import { AST_NODE_TYPES as NT, simpleTraverse } from "@typescript-eslint/typescript-estree";
import { Options } from "tsup";
import { assert, color, exit, last_of } from "../../src/utils/common";
import { File, for_each_rs_file } from "./common";
import { read_file, write_file } from "./fs";
import { create_console_line } from "./stdout";
import { replace_node, TSEdit, ts_edit_code } from "./typescript";

export function createStripPlugin({
	labels = [],
	functionCalls = [],
}: {
	labels: string[];
	functionCalls: string[];
}): (Options["plugins"] & {})[number] {
	return {
		name: "strip-devonly",
		esbuildOptions(options) {
			assert(!!options.plugins);
			const builtinPlugin_setup = last_of(options.plugins).setup;
			last_of(options.plugins).setup = function (build) {
				build.onLoad({ filter: /\.ts$/ }, (args) => ({
					loader: "ts",
					contents: edit(read_file(args.path)),
				}));
				return builtinPlugin_setup.call(this, build);
			};
		},
	};
	function edit(code: string) {
		return ts_edit_code(code, function (body) {
			const edits: TSEdit[] = [];
			for (const node of body) {
				simpleTraverse(node, {
					enter(node) {
						if (node.type === NT.LabeledStatement && labels.includes(node.label.name)) {
							edits.push(replace_node(node, "void 0;"));
						} else if (
							node.type === NT.CallExpression &&
							node.callee.type === NT.Identifier &&
							functionCalls.includes(node.callee.name)
						) {
							edits.push(replace_node(node, "void 0"));
						}
					},
				});
			}
			return edits;
		});
	}
}

export function testBuilds<T>(
	ts: T,
	builds: { [format: string]: T },
	toValue: (file: File, module: T) => { ext: string; content: string },
	samples: string[]
) {
	const line_log = create_console_line("...");
	const failed: string[] = [];
	let total = 0;
	for_each_rs_file(samples, (file) => {
		line_log.set(color.grey(`(${total}) ` + file.cmd));

		var expected = toValue(file, ts);
		for (var format in builds) {
			var actual = toValue(file, builds[format]);

			if (expected.content !== actual.content) {
				if (failed.push(`${file.cmd} (${format})`) === 1) {
					write_file(`tests/expected.temp.${expected.ext}`, expected.content);
					write_file(`tests/actual.${format}.temp.${expected.ext}`, actual.content);
				}
			}
		}

		total++;
	}).then(() => {
		if (failed.length) {
			line_log.delete();
			if (failed.length < 16) console.log(failed);
			exit(color.red(`The build produced different output in ${failed.length / 2}/${total} samples`));
		} else {
			line_log.set(color.green(`✓ The build passed ${total} samples`));
		}
	});
}
