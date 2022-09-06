import { rs as rs_esm } from "../dist/index.js";
import { testBuilds } from "../scripts/utils/build";
import { read_file, write_file_temp } from "../scripts/utils/fs.js";
import { rs } from "../src";
import { is_LocArray } from "../src/utils/ast/index.js";

if (false)
	(() => {
		// check how builds get stringified
		const filepath = "tests/samples/errors/foo.rs";
		write_file_temp(
			"tests/sample.temp.json",
			stringify(read_file(filepath), filepath, rs) //
		);
	})();

await testBuilds(
	rs,
	{
		esm: rs_esm,
		cjs: (await import("../dist/index.cjs")).default.rs,
	},
	function (file, rs) {
		return { ext: "json", content: stringify(file.content, file.cmd, rs) };
	},
	["tests/samples"]
);

function stringify(content: string, filepath: string, rs: typeof import("../src").rs) {
	var res: any;
	try {
		res = rs.parseFile(content, { filepath });
		if ("__devonly" in res.program) delete res.program.__devonly;
	} catch (e: any) {
		res = {
			name: e.name,
			message: e.message,
			loc: e.loc,
			ctx: e.ctx,
			toString: e.toString(),
		};
		// res.toString = res.toString.split(/\n/g)
	}
	return JSON.stringify(res, function (key, value) {
		if (is_LocArray(value)) return { ...value };
		if (value === undefined) return "undefined";
		return value;
	});
}
