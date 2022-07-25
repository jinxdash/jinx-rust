import { rs } from "../src";
import { color } from "../src/utils/common";
import { File, for_each_rs_file } from "./utils/common";
import { tempState } from "./utils/fs";

const ignored = [
	// rust-lang
	"src/test/pretty/ast-stmt-expr-attr.rs",
	"src/test/rustdoc-ui/test-compile-fail3.rs",

	"src/test/ui/associated-consts/assoc-const-eq-missing.rs",
	"src/test/ui/associated-consts/assoc-const-ty-mismatch.rs",
	"src/test/ui/associated-consts/assoc-const.rs",
	"src/test/ui/associated-types/associated-type-projection-from-multiple-supertraits.rs",
	"src/test/ui/associated-types/issue-36499.rs",

	"src/test/ui/async-await/issue-77993-2.rs",
	"src/test/ui/attribute-with-no-generics-in-parameter-list.rs",
	"src/test/ui/bounds-lifetime.rs",
	"src/test/ui/c-variadic/issue-86053-1.rs",
	"src/test/ui/cast/issue-88621.rs",
	"src/test/ui/cfg/cfg_stmt_expr.rs",
	"src/test/ui/const-generics/generic_arg_infer/array-repeat-expr.rs",

	"src/test/ui/editions/edition-keywords-2015-2015.rs",
	"src/test/ui/editions/edition-keywords-2015-2018.rs",
	"src/test/ui/try-block/try-is-identifier-edition2015.rs",

	"src/test/ui/issues/issue-74564-if-expr-stack-overflow.rs",
	"src/test/ui/lint/special-upper-lower-cases.rs",
	"src/test/ui/macros/macro-comma-support-rpass.rs",
	"src/test/ui/macros/type-macros-hlist.rs",
	"src/test/ui/parser/extern-crate-async.rs",
	"src/test/ui/parser/utf8_idents-rpass.rs",
	"src/test/ui/pattern/rest-pat-syntactic.rs",
	"src/test/ui/rfc-2565-param-attrs/param-attrs-pretty.rs",

	"src/tools/rustfmt/tests/parser/issue_4418.rs",
	"src/tools/rustfmt/tests/parser/issue-4126/invalid.rs",
	"src/tools/rustfmt/tests/parser/unclosed-delims/issue_4466.rs",
	"src/tools/rustfmt/tests/source/configs/disable_all_formatting/true.rs",
	"src/tools/rustfmt/tests/source/macro_rules.rs",
	"src/tools/rustfmt/tests/source/type-ascription.rs",
	"src/tools/rustfmt/tests/source/type.rs",
	"src/tools/rustfmt/tests/target/macro_rules.rs",
	"src/tools/rustfmt/tests/target/type-ascription.rs",
	"src/tools/rustfmt/tests/target/type.rs",
];

const state = tempState(`scripts/dev.parser.cache.temp.json`, (prev: string[] | undefined) => {
	let current = "";
	const cache = new Set(prev);
	console.log(color.grey(`Cache: ${cache.size} entries`));
	return {
		toJSON() {
			if (current) console.log(current);
			return [...cache];
		},
		withFile(file: File, fn: (file: File) => void, ignore: (file: File) => boolean) {
			if (!cache.has((current = file.cmd))) {
				if (ignore(file)) console.log(color.grey(`Ignored file "${file.cmd}"`));
				else fn(file);
				cache.add(current);
			}
			current = "";
		},
	};
});

for_each_rs_file(
	[
		`E:/dev/github/rust/rust-lang/`,
		// `E:/dev/github/rust/tokio/`,
		// `E:/dev/github/rust/syn/`,
		// `E:/dev/github/rust/serde/`,
		// `E:/dev/github/rust/rand/`,
		// `E:/dev/github/rust/bevy/`,
	],
	(file) =>
		state.withFile(
			file,
			(file) => rs.parseFile(file.content, { filepath: file.cmd }),
			(file) =>
				ignored.some((i) => file.path.endsWith(i)) ||
				/\/\/[~^ ]+(error|ERROR|expected)/.test(file.content) ||
				/#!\[feature\([^\)]*(type_ascription)[^\)]*\)\]/.test(file.content) ||
				["// ignore-test: not a test", "unterminated raw string", "terminating raw string", "fn(mut self)"].some((str) =>
					file.content.includes(str)
				)
		),
	[
		"await-keyword",
		"dyn-keyword",
		// "test",
		// "tests",
	]
);
