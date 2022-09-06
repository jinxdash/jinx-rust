import fs from "node:fs/promises";
import path from "node:path";
import prettier from "prettier";
import {
	assert,
	clamp,
	color,
	each,
	edit_string,
	exit,
	flat,
	getTabAddedWidth,
	MaybeFlatten,
	normPath,
	print_string,
	toArray,
} from "../../src/utils/common";
import { cmd, fsStat, pathContains, read_file, read_json, remove_dir, remove_file, update_file, WriteOptions, write_ErrTemp } from "./fs";

const default_prettier_config: prettier.Options = { parser: "typescript", ...read_json("package.json").prettier };
function getPrintWidth() {
	return default_prettier_config.printWidth!;
}
export function prettier_format(str: string, options: prettier.Options = {}) {
	try {
		return prettier.format(str, { ...default_prettier_config, ...options });
	} catch (err) {
		if (options.filepath) write_ErrTemp(options.filepath, str);
		exit("" + err);
	}
}
export function JSONstringify(value: any) {
	return prettier_format(JSON.stringify(value), { parser: "json" });
}
export function cmd_comment(target_path: string, source_dir?: string, name = "") {
	let p = cmd(target_path, source_dir);
	if (!/^[\.\/]/.test(p)) p = `./${p}`;
	return `// ${name && `${name}: `}"${p}"`;
}

export function format_headers(filepath: string) {
	const src = read_file(filepath, { prettier: true })!;
	const lines = src.split(/\n/g);
	const errors: { line_index: number; error: string }[] = [];
	let changed = false;
	let line_index = 0;

	function addError(error: string) {
		errors.push({ line_index, error });
	}
	function replace_line(next: string) {
		if (next !== lines[line_index]) {
			changed = true;
			lines[line_index] = next;
		}
	}
	each(lines, (line, index) => {
		line_index = index;
		if (line.charCodeAt(2) !== 35) return;
		const match = /^\/\/#((?:end)?region ?===| ?---)/.exec(line);
		if (match === null) return;
		const is_region_start = match[1].startsWith("region");
		const is_region_end = !is_region_start && match[1].startsWith("endregion");
		const [START, FILLER, CENTER_LEFT, CENTER_RIGHT, END] = is_region_start
			? ["//#region ", "=", "[", "]", "``--."]
			: !is_region_end
			? ["//#", "-", "+", "+", "."]
			: ["//#endregion ", "=", "", "", "..--'"];

		let end = line.length;
		let start = match[0].length;
		{
			if (line.endsWith(END)) end -= END.length;
			while (line.charAt(end - 1) === FILLER) end--;
			if (line.endsWith(CENTER_RIGHT, end)) end -= CENTER_RIGHT.length;
			while (line.charAt(end - 1) === " ") end--;

			while (line.charAt(start) === FILLER) start++;
			if (line.startsWith(CENTER_LEFT, start)) start += CENTER_LEFT.length;
			while (line.charAt(start) === " ") start++;
		}
		const title = line.slice(start, end).trim();
		if (title && is_region_end) {
			addError("#endregion cannot have a title");
		} else if (title && title.includes(FILLER + FILLER)) {
			addError("failed to find title / bad formatting");
		} else if (!title && !is_region_end) {
			addError("missing title");
		} else {
			replace_line(
				header_line([START, CENTER_LEFT, title, CENTER_RIGHT, END], FILLER) //
			);
		}
	});

	if (errors.length) {
		each(errors, ({ line_index, error }) => {
			console.log(`Error: ${color.red(error)}\n\t at ${cmd(filepath)}:${line_index + 1}\n`);
		});
	} else {
		if (changed) {
			update_file(filepath, lines.join("\n"));
		}
	}
	function header_line(
		[LINE_START, TITLE_LEFT, TITLE, TITLE_RIGHT, LINE_END]: [string, string, string, string, string],
		fillerChar: string,
		PRINT_WIDTH: number = getPrintWidth()
	): string {
		const length = clamp(80, 200, PRINT_WIDTH);
		const center = Math.max(8, Math.floor((20 - TITLE.length) / 2));
		const fill = " ".repeat(center);
		const _TITLE = TITLE ? TITLE_LEFT + fill + TITLE + fill + TITLE_RIGHT : "";
		const half = Math.max(0, Math.floor(length - _TITLE.length) / 2);
		return (LINE_START.padEnd(half, fillerChar) + _TITLE).padEnd(length, fillerChar) + LINE_END;
	}
}

export function update_file_section<T extends Partial<WriteOptions>>(
	filepath: string,
	section_start: string,
	content: string,
	section_end: string,
	opts: T = {} as T
) {
	return update_file<T>(
		filepath,
		update_section(
			read_file(filepath),
			section_start,
			content, //
			section_end,
			false
		),
		opts
	);
	function update_section(str: string, start: string, update: string, end: string, forbidden_characters: RegExp | false) {
		try {
			const [i0, i1] = find_section(str, start, end, forbidden_characters);
			return edit_string(str, i0, start + update + end, i1);
		} catch (e) {
			write_ErrTemp(null, str);
			throw e;
		}
		function find_section(str: string, start: string, end: string, forbidden_characters: false | RegExp) {
			const i0 = str.indexOf(start);
			const i1 = str.indexOf(end, i0 + start.length) + end.length;
			const prev = str.slice(i0, i1);
			if (i0 === -1) throw new Error(`Could not find section start '${print_string(start)}' in ${cmd(filepath)}`);
			if (i1 - end.length === -1) throw new Error(`Could not find section end '${print_string(end)}' in ${cmd(filepath)}`);
			if (forbidden_characters && forbidden_characters.test(prev.slice(start.length, -end.length))) {
				const section = `'${print_string(start)}...${print_string(end)}'`;
				throw new Error(
					`Found section ${section} in ${cmd(filepath)} but it includes forbidden characters (${forbidden_characters.toString()})`
				);
			}
			return [i0, i1];
		}
	}
}

export function join_wrap({
	start,
	content,
	end,
	sep,
	indent = "\t",
}: {
	start: string;
	content: string[];
	end: string;
	sep: string;
	indent?: string;
}) {
	const PRETTIER_IGNORE = "// prettier-ignore\n";
	if (content.length === 0) return PRETTIER_IGNORE + start.trimEnd() + end.trimStart();

	const sepEol = sep.trimEnd();
	const target_length = getPrintWidth() - (indent.length + getTabAddedWidth(indent) + sepEol.length);

	let res = "";
	let line = "";

	if (start) {
		if (start.endsWith("\n")) {
			res += start;
			line += `${indent}${content[0]}`;
		} else {
			line += `${start}${content[0]}`;
		}
	} else {
		line += `${indent}${content[0]}`;
	}

	for (let item of content.slice(1)) {
		item = item.trimEnd();
		if (line.length + item.length > target_length) {
			res += `${line}${sepEol}`;
			line = `\n${indent}${item}`;
		} else {
			line += `${sep}${item}`;
		}
	}

	return PRETTIER_IGNORE + res + line + end;
}

function Promise_map<T, R>(arr: T[], fn: (v: T) => R): Promise<Awaited<MaybeFlatten<Awaited<R>>>[]> {
	return Promise.all(arr.map(fn)).then((r) => flat(r)) as any;
}

function crawl_directory(dir: string, ext: string, filter_out: string[]): Promise<string[]> {
	return fs
		.readdir(dir)
		.then((items) =>
			Promise_map(items, function (each_item) {
				const target = path.join(dir, each_item);
				if (each_item.endsWith(ext)) return target;
				if (filter_out.includes(each_item)) return [];
				return fsStat(target)
					.then((stat) => (stat?.isDirectory() ? crawl_directory(target, ext, filter_out) : []))
					.catch(() => []);
			}).catch(() => [])
		)
		.catch(() => []);
}

export function remove_unknown_files(target_root_dirs: string | string[], known_files: Set<string>) {
	target_root_dirs = toArray(target_root_dirs).map((dir) => path.resolve(dir));
	for (const file of known_files) {
		const resolvedPath = path.resolve(file);
		if (resolvedPath !== file) known_files.delete(file), known_files.add(resolvedPath);
		if (!target_root_dirs.some((dir) => pathContains(dir, file)))
			exit("", { known_files: [...known_files], file, r: resolvedPath, target_root_dirs });
	}
	return Promise.all(
		target_root_dirs.map((target_root_dir) =>
			(async function processDirectory(TARGET_DIR: string) {
				const items = await fs.readdir(TARGET_DIR);
				let files_count = 0;
				for (const item of items) {
					const target = path.join(TARGET_DIR, item);
					if ((await fsStat(target))?.isDirectory()) {
						processDirectory(target);
					} else {
						if (known_files.has(target)) {
							files_count++;
						} else {
							remove_file(target);
						}
					}
				}
				if (files_count === 0 && TARGET_DIR !== target_root_dir) {
					assert(!known_files.has(TARGET_DIR));
					remove_dir(TARGET_DIR);
				}
			})(target_root_dir)
		)
	);
}

export interface File {
	name: string;
	path: string;
	cmd: string;
	content: string;

	dir: string;
}

function formFile(filepath: string, dir: string): File {
	return {
		dir,
		name: path.basename(filepath),
		path: normPath(filepath),
		cmd: cmd(filepath),
		content: "",
	};
}

function _each_x_file(ext: string, dirs: string[], each_file: (file: File) => void, filter_out: string[] = []) {
	if (dirs.length > 1) assert(dirs.every((dir, i, a) => i === 0 || (!pathContains(dir, a[i - 1]) && !pathContains(a[i - 1], dir))));
	return Promise_map(dirs, (dir) =>
		crawl_directory(dir, ext, filter_out).then((files) => files.sort().map((filepath) => formFile(filepath, dir)))
	).then(async function (files) {
		const unread = new Set(files);
		while (unread.size > 0) {
			await Promise.all(
				[...unread].map((file) =>
					fs
						.readFile(file.path, "utf8")
						.then((content) => {
							unread.delete(file);
							file.content = content;
							each_file(file);
						})
						.catch(function (error) {
							if (error.code !== "EMFILE") throw error;
						})
				)
			);
		}
	});
}

export async function for_each_rs_file(crates: string[], each_file: (file: File) => void, filter_out: string[] = []) {
	return _each_x_file(".rs", crates, each_file, filter_out);
}

export async function for_each_ts_file(dir: string, each_file: (file: File) => void, filter_out: string[] = []) {
	return _each_x_file(".ts", [dir], each_file, filter_out);
}
