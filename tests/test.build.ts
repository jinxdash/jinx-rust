import { rs as rs_esm } from "../dist/index.js";
import { testBuilds } from "../scripts/utils/build";
import { read_file, write_file_temp } from "../scripts/utils/fs.js";
import { rs } from "../src";
import { is_LocArray } from "../src/utils/ast/index.js";

// check how builds get stringified
if (false)
	write_file_temp(
		"tests/sample.temp.json",
		stringify(rs.parseFile(read_file("tests/samples/expressions/block.rs"))) //
	);

testBuilds(
	rs,
	{
		esm: rs_esm,
		cjs: (await import("../dist/index.cjs")).default.rs,
	},
	function (file, rs) {
		const res = rs.parseFile(file.content, { filepath: file.cmd });
		if ("__devonly" in res.program) delete res.program.__devonly;
		return { ext: "json", content: stringify(res) };
	},
	["tests/samples"]
);

function stringify(data: any) {
	return JSON.stringify(data, function (key, value) {
		if (is_LocArray(value)) return { ...value };
		if (value === undefined) return "undefined";
		return value;
	});
}
