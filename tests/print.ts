import { createParserPrinter, rs_print_samples } from "../scripts/utils";

// for_each_ts_file(path.resolve("src"), (file) => {
// 	console.log(cmd(file.path), file.content.includes("\r"));
// 	write_file(file.path, file.content.replace(/\r/g, ""));
// });

rs_print_samples(["tests/samples/"], "tests/output/", createParserPrinter());
