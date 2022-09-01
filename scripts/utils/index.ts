import { spawn } from "node:child_process";
import { watch } from "node:fs";
import path from "node:path";
import prettier from "prettier";
import { rs } from "../../src";
import { ast_debug_print } from "../../src/debug/print";
import type { Program } from "../../src/parser/nodes";
import { ArrayProps } from "../../src/utils/ast";
import { assert, color, each, pretty_timespan, toArray, trycatch } from "../../src/utils/common";
import { overrideDefaultError } from "../../src/utils/debug";
import { cmd_comment, for_each_rs_file, JSONstringify, remove_unknown_files } from "./common";
import { fsStatSync, pathContains, watch_dir, write_file } from "./fs";
import { create_console_line } from "./stdout";

overrideDefaultError(true);

/**
 * ATTENTION: Respawns the current process (necessary to re-load deps)
 * Make sure to not do anything beside calling this function in that process
 */
export function rs_createREPL(
	printers: PrinterFn | PrinterFn[] = [createParserPrinter(), createASTtoJSONPrinter()],
	{ cwd = process.cwd(), replPath = "repl", depsPath = "src", initFile = "\nfn main() {\n\tlet i: u8 = 0;\n}\n" } = {}
) {
	const samples_dir = path.join(cwd, replPath, "samples");
	const output_dir = path.join(cwd, replPath, "output");
	if (process.env.isChildProcess) {
		let pending = false;
		let fs_updates = 0;
		const watched = new Set<string>();
		const linelog = create_console_line("% REPL starting ...");

		watch_dir(
			samples_dir,
			() => {
				dispatch();
				return init(samples_dir);
			},
			() => void write_file(path.join(samples_dir, "main.rs"), initFile),
			{ persistent: true }
		);

		function init(dir: string) {
			watched.add(dir);
			return (name: string, exists: boolean) => {
				const target = path.join(dir, name);
				if (exists && !watched.has(target) && fsStatSync(target)?.isDirectory()) {
					const watcher = watch_dir(
						target,
						() => init(target),
						() => {
							watcher.close();
							watched.delete(target);
						}
					);
				} else {
					dispatch();
				}
			};
		}

		function dispatch() {
			++fs_updates;
			if (pending !== (pending = true)) {
				(function retry() {
					const u = fs_updates;
					setTimeout(() => {
						if (u !== fs_updates) return retry();
						rs_print_samples([samples_dir], output_dir, printers, { debug: true, linelog })
							.catch(console.error)
							.then(() => {
								if (u !== fs_updates) return retry();
								pending = false;
								linelog.set(color.magenta("% REPL watching for updates ..."));
							});
					}, 100);
				})();
			}
		}
	} else {
		let current = spawn_repl();
		let last_info = "";
		let last_reload = 0;
		watch(path.join(cwd, depsPath), { recursive: true }, (event, filename) => {
			const t = Date.now();
			const info = `${event}:${filename}`;
			if (last_info === (last_info = info) && t - last_reload < 1000) return;
			console.log(`[${info}] reloading REPL`);
			last_reload = t;
			current.kill();
			current = spawn_repl();
		});
		function spawn_repl() {
			return spawn(process.argv[0], [...process.execArgv, process.argv.slice(1)] as any, {
				env: { isChildProcess: "1" },
				stdio: "inherit",
			});
		}
	}
}

interface PrintOpts {
	debug?: boolean;
	linelog?: { set(value: string): void };
}

export function createParserPrinter(): PrinterFn {
	return (updateOutput, { name, ast }) => {
		updateOutput({ printed: `${name}.p.rs` }, (addInfo) => {
			if (!ast.program.__devonly) addInfo("Program.__devonly: undefined");
			else if (!ast.program.__devonly.stats) addInfo("Program.__devonly.stats: undefined");
			else each(ast.program.__devonly.stats, (value, key) => addInfo(`${key}: ${value}`));
			return { printed: ast_debug_print(ast) };
		});
	};
}

export function createASTtoJSONPrinter(): PrinterFn {
	return (updateOutput, { name, ast }) => {
		updateOutput({ ast: `${name}.json` }, () => ({ ast }));
	};
}

export function createPrettierPrinter(prettier_config: prettier.Config & { parser: string; plugins: [{}] }, printDocs = false): PrinterFn {
	return (updateOutput, { name, ast, outdir }) => {
		updateOutput(
			{
				formatted: `${name}.f.rs`,
				second_pass: `${name}.f.1.rs`,
				doc: `${name}.js`,
			},
			(addInfo) => {
				const first_pass = rs_prettier_format(ast.code, ast.filepath!);

				const filepath2 = path.join(outdir, `${name}.f.rs`);

				const second_pass = trycatch(() => {
					const ast2 = rs_parseFile("" + first_pass, filepath2);

					for (const k of ["comments", "danglingAttributes", "attributes"] as (keyof ArrayProps<Program>)[]) {
						const d = (ast.program[k]?.length ?? 0) - (ast2.program[k]?.length ?? 0);
						if (d !== 0) addInfo(`format: lost ${d} ${k}`);
					}

					return withGreyedOutLogs(() => rs_prettier_format("" + first_pass, filepath2));
				});

				return {
					formatted: first_pass,
					second_pass: first_pass !== second_pass && second_pass,
					doc: printDocs && trycatch(() => withMutedLogs(() => rs_prettier_format_debug("" + first_pass, filepath2))),
				};

				function getPrettierConfig(filepath: string) {
					// custom config from shebang e.g. "#!{ printWidth: 120 }"
					return "shebang" in ast && ast.shebang.value[0] === "{" //
						? { ...prettier_config, filepath, ...eval(`(${ast.shebang.value})`) }
						: { ...prettier_config, filepath };
				}

				function rs_prettier_format(code: string, filepath: string) {
					return prettier.format(code, getPrettierConfig(filepath));
				}

				function rs_prettier_format_debug(code: string, filepath: string) {
					const { __debug } = prettier as any;
					const docs = __debug.printToDoc(code, getPrettierConfig(filepath));
					return __debug.formatDoc(docs, { printWidth: 100, tabWidth: 2 }) as string;
					// const str = __debug.printDocToString(docs, config).formatted as string;
					// return { doc, str };
				}
			}
		);
	};
}

type ContentOutput = false | string | object | Error;
type UpdateFn = <T extends { [key: string]: string | false }>(
	files: T,
	fn: (add_infoToOutput: (msg: string) => void) => { [K in keyof T]?: ContentOutput },
	extraLinksToOutput?: { [key: string]: string }
) => void;

export type PrinterFn = (addOutput: UpdateFn, context: { ast: ReturnType<typeof rs_parseFile>; name: string; outdir: string }) => void;

export function rs_print_samples(
	samples_dir: string[],
	printed_dir: string,
	printer: PrinterFn | PrinterFn[],
	{ debug: PRINT_DEBUG = false, linelog = create_console_line("% ... %") }: PrintOpts = {}
) {
	for (const dir of samples_dir) assert(!pathContains(dir, printed_dir) && !pathContains(printed_dir, dir));
	const tests_times = new Map<string, number>();
	const known_files = new Set<string>();

	return for_each_rs_file(samples_dir, function (file) {
		linelog.set(color.grey(`Printing sample: ${file.cmd}`));
		const TIMER_START = Date.now();

		const OUTDIR = path.resolve(printed_dir, path.relative(file.dir, path.dirname(file.path)));

		const context = {
			ast: rs_parseFile(file.content, file.cmd),
			name: file.name.slice(0, -3),
			outdir: OUTDIR,
		};

		for (const f of toArray(printer)) {
			f(function (files, fn, extraLinksToOutput = {}) {
				let extra = "";
				const Output = fn((msg) => void (extra += msg.replace(/^|\n\s*/g, "\n// ")));
				extra += getEndCommentInfo(extraLinksToOutput);
				each(files, (filepath, K) => {
					const content = Output[K];
					if (filepath && content) {
						const EXT = path.extname(filepath);
						const TARGET_PATH = path.join(OUTDIR, filepath);
						const is_JSON = EXT === ".json";
						known_files.add(TARGET_PATH);
						write_file(
							TARGET_PATH,
							content instanceof Error
								? color.unstyle(content.toString() + content.stack)
								: is_JSON
								? JSONstringify(content)
								: content.toString() + extra
						);
					}
				});
				function getEndCommentInfo(extraLinksToOutput: { [key: string]: string }) {
					let str = "\n" + cmd_comment(file.path, OUTDIR, "source");
					each(extraLinksToOutput, (linkFilepath, linkName: string) => {
						str += "\n" + cmd_comment(path.join(OUTDIR, linkFilepath), OUTDIR, linkName);
					});
					return str;
				}
			}, context);
		}

		tests_times.set(file.cmd, Date.now() - TIMER_START);
	}).then(() => {
		linelog.set(
			color.green(`✓ Processed samples in ${color.bold(pretty_timespan([...tests_times.entries()].reduce((p, n) => p + n[1], 0.0)))}`)
		);

		if (PRINT_DEBUG) {
			const sorted_times = [...tests_times.entries()].sort((a, b) => b[1] - a[1]);
			for (const { 0: cmd, 1: t } of sorted_times) if (t > 500) console.log(`${color.red(pretty_timespan(t))} ${color.grey(cmd)}`);
		}

		remove_unknown_files(printed_dir, known_files);
	});
}

function withGreyedOutLogs<R>(fn: () => R): R {
	const console_log = console.log;
	try {
		console.log = (...args) => {
			console_log(...args.map((v) => (typeof v === "string" ? color.grey(color.unstyle(v)) : v)));
		};
		return fn();
	} finally {
		console.log = console_log;
	}
}

function withMutedLogs<R>(fn: () => R): R {
	const console_log = console.log;
	try {
		console.log = (...args) => {};
		return fn();
	} finally {
		console.log = console_log;
	}
}

function rs_parseFile(code: string, filepath: string) {
	return rs.parseFile(code, { filepath });
}
