import { existsSync, FSWatcher, mkdirSync, readdirSync, readFileSync, rmSync, Stats, statSync, watch, writeFileSync } from "node:fs";
import fs from "node:fs/promises";
import path from "node:path";
import { assert, color, exit, normPath, resolve_then } from "../../src/utils/common";
import { JSONstringify, prettier_format } from "./common";

export function cmd(filepath: string | undefined, frompath = "") {
	return normPath(path.relative(frompath, normPath(filepath ?? ""))) || ".";
}

export function isExternalPath(filepath: string) {
	return cmd(filepath).startsWith("..");
}

export function pathContains(a: string, b: string) {
	const r = path.relative(a, b);
	return !r || (!r.startsWith("..") && !path.isAbsolute(r));
}

interface FsOptions {
	sync: boolean;
	external: boolean;
}
interface FsContentOptions extends FsOptions {
	prettier: boolean;
}

interface ReadOptions extends FsContentOptions {}
export interface WriteOptions extends FsContentOptions {}
interface RmOptions extends FsOptions {}
type FsOp<O extends { sync?: boolean }, T> = O extends { sync: false } ? Promise<T> : T;

export function read_file<T extends Partial<ReadOptions>>(
	filepath: string,
	{ sync = true, external = false, prettier = false }: T = {} as any,
	init?: () => string
): FsOp<T, string> {
	assert_safe("read", filepath, external);
	return resolve_then((sync ? fsStatSync : fsStat)(filepath), (stat) => {
		if (!stat) {
			assert(typeof init === "function", `Attempted to read non-existing file at '${cmd(filepath)}'`);
			return resolve_then(maybe_create_dir(path.dirname(filepath), { sync, external, force: true }), () =>
				create_file(filepath, init(), { sync, prettier })
			);
		}
		assert_file("read", filepath, stat);
		return resolve_then(((sync ? readFileSync : fs.readFile) as any)(filepath, "utf-8"), (content) =>
			maybe_format(content, prettier, filepath)
		);
	}) as any;
}

export function read_json(filepath: string) {
	return JSON.parse(read_file(filepath, { sync: true }));
}

export function write_file_temp(filepath: string, content: any): string {
	const target = filepath + (is_temp(filepath) ? "" : ".temp");
	write_file(target, typeof content === "string" ? content : JSONstringify(content));
	return target;
}
function is_temp(filepath: string) {
	return /[\.\/]?temp[\.\/]?/.test(filepath);
}
function __write_file<T extends Partial<WriteOptions>>(
	filepath: string,
	content: string | ((prev: string) => string),
	opts: T | undefined,
	is_write: boolean
): T extends { sync: false } ? Promise<boolean> : boolean {
	const { sync = true, prettier = false } = opts ?? {};
	let created = false;
	return resolve_then(
		read_file(filepath, opts, is_write ? () => ((created = true), maybe_format(content as string, prettier, filepath)) : undefined),
		function (content_prev) {
			const content_next = created
				? content_prev
				: maybe_format(typeof content === "string" ? content : content(content_prev), prettier, filepath);
			return (
				content_prev !== content_next &&
				resolve_then((sync ? writeFileSync : fs.writeFile)(filepath, content_next), () => {
					const b = path.basename(filepath);
					const d = content_next.length - content_prev.length;
					const m =
						color.cyan("Updated") +
						(" " + color.link(cmd(filepath).slice(0, -b.length) + b)) +
						(" (" + color.yellow((d >= 0 ? "+" : "") + d) + " characters)");
					fs_log(sync, color.grey(is_temp(filepath) ? color.unstyle(m) : m));
					return true;
				})
			);
		}
	) as any;
}

export function write_file<T extends Partial<WriteOptions>>(filepath: string, content: string, opts?: T): FsOp<T, boolean> {
	return __write_file(filepath, content, opts, true);
}
export function update_file<T extends Partial<WriteOptions>>(
	filepath: string,
	content: string | ((prev: string) => string),
	opts?: T
): FsOp<T, boolean> {
	return __write_file(filepath, content, opts, false);
}

function fs_log(sync: boolean, msg: string) {
	if (sync) console.log(msg);
	else {
		setImmediate(() => {
			console.log(msg);
		});
	}
}

export function fsStat(target: string) {
	return fs.stat(target).catch((e) => {
		if (e.code === "ENOENT") return undefined;
		throw e;
	});
}
export function fsStatSync(target: string) {
	try {
		return statSync(target);
	} catch (e) {
		if ((e as any).code === "ENOENT") return undefined;
		throw e;
	}
}
export function remove_file<T extends Partial<RmOptions>>(
	filepath: string,
	{ sync = true, external = false }: T = {} as T
): T["sync"] extends false ? Promise<void> : void {
	assert_safe("remove", filepath, external);
	return resolve_then((sync ? fsStatSync : fsStat)(filepath), (stat) => {
		if (!stat) return;
		assert_file("remove", filepath, stat);
		return resolve_then((sync ? rmSync : fs.rm)(filepath), () => fs_log(sync, color.magenta(`Removed file "${cmd(filepath)}"`)));
	}) as any;
}

export function remove_dir<T extends Partial<RmOptions>>(
	dirpath: string,
	{ sync = true, external = false }: T = {} as T
): T["sync"] extends false ? Promise<void> : void {
	assert_safe("remove", dirpath, external);
	return resolve_then((sync ? fsStatSync : fsStat)(dirpath), (stat) => {
		if (!stat) return;
		assert_dir("remove", dirpath, stat);
		return resolve_then((sync ? rmSync : fs.rm)(dirpath, { recursive: true }), () =>
			fs_log(sync, color.magenta(`Removed directory "${cmd(dirpath)}"`))
		);
	}) as any;
}

function create_file<T extends Partial<WriteOptions>>(
	filepath: string,
	content: string,
	{ sync = true, prettier = false }: T = {} as T
): T["sync"] extends false ? Promise<string> : string {
	assert_safe("create a file", filepath, false);
	const CONTENT = maybe_format(content, prettier, filepath);
	return resolve_then((sync ? writeFileSync : fs.writeFile)(filepath, CONTENT), () => {
		const b = path.basename(filepath);
		fs_log(
			sync,
			grey_if_tempfile(
				filepath,
				color.grey(
					color.magenta("Created") + //
						(" " + color.link(cmd(filepath).slice(0, -b.length) + b))
				)
			)
		);
		return CONTENT;
	}) as any;
}

function grey_if_tempfile(filepath: string, msg: string): string {
	return path.basename(filepath).includes(".temp.") ? color.grey(color.unstyle(msg)) : msg;
}

const pending_create_dir: { [dir: string]: Promise<void> | void } = {};
function maybe_create_dir(dir: string, { sync = true, external = false, force = false }) {
	assert_safe("create a directory", dir, external);
	const id = `${+sync}${+force}:${path.resolve(dir)}`;
	return (pending_create_dir[id] ??= resolve_then((sync ? fsStatSync : fsStat)(dir), (stat) => {
		if (!stat && force) {
			return (pending_create_dir[id] = resolve_then((sync ? mkdirSync : fs.mkdir)(dir, { recursive: true }), () => {
				delete pending_create_dir[id];
				fs_log(sync, color.grey(`Created directory ${color.underline(cmd(dir))}`));
			}));
		}
		delete pending_create_dir[id];
		if (stat) assert_dir("read", dir, stat);
		else exit(`Attempted to create directory at "${cmd(dir)}"` + `\tDid you mean to use '${color.magenta("force: true")}' ?`);
	}));
}

export function write_ErrTemp(og_pth: string | null, content: string) {
	const temp = `error.temp${(og_pth && path.extname(og_pth)) || ".ts"}`;
	writeFileSync(temp, content);
	console.log(color.red("Inspect this error at: " + cmd(temp)));
}

export function tempState<T extends { toJSON(): J }, J>(
	filepath: `${string}.temp.json`,
	init: (value: J | undefined) => T
): Omit<T, "toJSON"> {
	const OPTIONS = { external: false, sync: true, optional: true };
	const data = read_file(filepath, OPTIONS);
	const value = init(data === undefined ? data : JSON.parse(data));
	process.on("exit", () => {
		write_file(filepath, JSONstringify(value.toJSON()));
	});
	return value;
}

function maybe_format(content: string, should_format: boolean, filepath: string) {
	if (should_format && /\.json$/.test(filepath)) return prettier_format(content, { parser: "json", filepath });
	if (should_format && /\.([mc]?[jt]s|[jt]sx)$/.test(filepath)) return prettier_format(content, { parser: "typescript", filepath });
	return content;
}
function assert_safe(op: string, filepath: string, external: boolean) {
	if (!external && isExternalPath(filepath)) exit(`Attempted to ${op} at external path: "${cmd(filepath)}"`);
}
function assert_file(op: string, filepath: string, stat: Stats) {
	assert(stat.isFile(), `Attempted to ${op} a file but found a directory at "${cmd(filepath)}"`);
}
function assert_dir(op: string, dir: string, stat: Stats) {
	assert(stat.isDirectory(), `Attemped to ${op} a directory but found a file at "${cmd(dir)}"`);
}

function clampExists(fullpath: string) {
	let dir = fullpath;
	while (!existsSync(dir)) {
		assert(!!dir);
		dir = dirpath(dir);
	}
	return dir;
}

function dirpath(subpath: string) {
	return path.resolve(subpath, "..");
}

type FsUpdateFn = (name: string, exists: boolean) => void;
interface FSWatchMatching {
	[match: string]: FsUpdateFn;
}
export function watch_dir(dir: string, init: () => FsUpdateFn | FSWatchMatching, on_missing?: () => void, { persistent = false } = {}) {
	let current: FSWatcher;

	function watch_start() {
		if (!current) if (!existsSync(dir)) on_missing?.();
		current = existsSync(dir) ? watch_target() : watch_closest();
	}

	function watch_closest() {
		const closest = clampExists(dir);
		const missing = path.resolve(closest, path.relative(closest, dir).split(path.sep)[0]);
		return __watch(closest, function (name) {
			if (
				// removed closest
				(name === path.dirname(closest) && !existsSync(closest)) ||
				// created missing
				(name === path.dirname(missing) && existsSync(missing))
			) {
				this.close();
				watch_start();
			}
		});
	}

	function watch_target() {
		const watcher = __watch(dir, function (name) {
			const item_exists = existsSync(path.join(dir, name));
			const dir_removed = !item_exists && !existsSync(dir);
			if (dir_removed) {
				this.close();
				on_missing?.();
				watch_start();
			} else {
				on_update(name, item_exists);
			}
		});

		const watched_dir_items = init();
		readdirSync(dir).forEach((item) => on_update(item, true));

		return watcher;

		function on_update(name: string, item_exists: boolean) {
			if (typeof watched_dir_items === "function") {
				watched_dir_items(name, item_exists);
			} else {
				for (const match in watched_dir_items) {
					if (match_pathname(match, name)) {
						watched_dir_items[match](name, item_exists);
					}
				}
			}
			function match_pathname(match: string, name: string) {
				return match[0] === "*" ? name.endsWith(match.slice(1)) : match === name;
			}
		}
	}

	function __watch(dir: string, fn: (this: FSWatcher, name: string) => void) {
		const watcher = watch(dir, { persistent }, function (_event, name) {
			fn.call(watcher, name);
		});
		return watcher;
	}

	watch_start();
	return {
		close() {
			current.close();
		},
	};
}
