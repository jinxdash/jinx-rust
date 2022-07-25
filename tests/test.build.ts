import { rs as rs_esm } from "../dist/index.js";
import { testBuilds } from "../scripts/utils/build";
import { rs } from "../src";

testBuilds(
	rs,
	{
		esm: rs_esm,
		cjs: (await import("../dist/index.cjs")).default.rs,
	},
	function (file, rs) {
		const res = rs.parseFile(file.content, { filepath: file.cmd });
		if ("__devonly" in res.program) delete res.program.__devonly;
		return { ext: "json", content: JSON.stringify(res) };
	},
	["tests/samples"]
);
