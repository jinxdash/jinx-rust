import { createParserPrinter, rs_print_samples } from "../scripts/utils";

rs_print_samples(["tests/samples/"], "tests/output/", createParserPrinter());
