import { pretty_join } from "../utils/common";
import { exit } from "./error";

export interface ParserOptions {
	filepath?: string | undefined;
}

const defaultParserOptions: Required<ParserOptions> = { filepath: undefined };
export function checkOptions(O: ParserOptions) {
	for (var k in O)
		k in defaultParserOptions ||
			exit(
				"" +
					`Unknown parser option "${k}"\n` +
					`  Valid options: ${pretty_join(Object.keys(defaultParserOptions), { quote: true })}`,
				O
			);
}
