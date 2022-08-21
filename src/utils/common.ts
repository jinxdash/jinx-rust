import { CharCode } from "./CharCode";
import { createCustomError } from "./debug";

declare global {
	var console: { log(...args: any[]): void; error(...args: any[]): void };
	interface ErrorConstructor {
		captureStackTrace(targetObject: object, constructorOpt?: Function): void;
		prepareStackTrace?: ((err: Error, stackTraces: any[]) => any) | undefined;
		stackTraceLimit: number;
	}
	interface ImportMeta {
		url: string;
	}
}

export function Narrow<T extends R, R = unknown>(value: R): asserts value is T {}
export function AssertTypesEq<A extends B, B>(...args: [B] extends [A] ? [] : [RIGHT_TYPES_NOT_ASSIGNABLE_TO_LEFT: Exclude<B, A>]) {}

// prettier-ignore
type indexof<A> = A extends readonly any[] ? A extends 0 ? any : keyof A & number : A extends Set<unknown> ? never : A extends Map<infer U, unknown> ? U 
	: A extends Iterable<unknown> ? never : A extends object ? keyof A & (number | string) : never;
// prettier-ignore
type valueof<A> = A extends ReadonlyArray<infer U> ? A extends 0 ? any : U : A extends Set<infer U> ? U : A extends Map<unknown, infer U> ? U 
	: A extends Iterable<infer U> ? U : A extends object ? A[keyof A & (number | string)] : never;
// prettier-ignore
type vObject<V extends unknown = unknown, K extends unknown = unknown> = | object | readonly V[] | { [key: string]: V } | anySet<V> | anyMap<K, V>;
export type vIterable<V = unknown, K = unknown> = vObject<V, K> | Iterable<V>;
export type itfn<A, R> = (value: valueof<A>, key: indexof<A>) => R;
type anySet<V extends unknown = unknown> = Set<V>;
type anyMap<K extends unknown = unknown, V extends unknown = unknown> = Map<K, V>;
type anyfunction<A extends any[] = unknown[], R = unknown> = (...args: A) => R;
type objlike = object | anyfunction;
type anymap<K extends unknown = unknown, V extends unknown = unknown> = K extends objlike ? Map<K, V> | WeakMap<K, V> : Map<K, V>;

let __parser_maybe_exit: ParserMaybeExit;
type ParserMaybeExit = (message: string, ctx: any[]) => void | never;
export function __SET_PARSER_ERROR_MNGR(_maybe_exit: ParserMaybeExit) {
	__DEV__: assert(!__parser_maybe_exit);
	__parser_maybe_exit = _maybe_exit;
}

export function exit(message: string, ...ctx: any[]): never {
	__parser_maybe_exit?.(message, ctx);
	if (ctx.length > 0) console.log("Error context:", { ...ctx });
	throw createCustomError({ message });
}
exit.never = function never(...ctx: any[]): never {
	exit("Reached unreachable code", ...ctx);
};
export function assert(predicate: boolean, err?: string, ...ctx: any[]): asserts predicate {
	__DEV__: if (typeof predicate !== "boolean") exit("Expected boolean", predicate);
	if (false === predicate) exit(err ?? "Assertion failed", ...ctx);
}
assert.isNode = function isNode(target: any): asserts target is { nodeType: number } {
	if (!(is_object(target) && "nodeType" in target)) exit("Expected Node", target);
};
assert.every = function every<A extends vIterable>(data: A, condition: itfn<A, boolean>, err?: string, ...ctx: any[]) {
	if (some(data, (value, index) => !condition(value, index))) {
		if (some(data, (value, index) => typeof index !== "string" && typeof index !== "number")) {
			const failed: [any, any][] = [];
			each(data, (value, index) => {
				if (!condition(value, index)) {
					failed.push([index, value]);
				}
			});
			ctx.unshift(failed);
		} else {
			const failed: { [key: string | number]: any } = {};
			each(data, (value, index) => {
				if (!condition(value, index)) {
					failed[index as any] = value;
				}
			});
			ctx.unshift(failed);
		}
		exit(`Assertion failed${err && ` : ${err}`}`, ...ctx);
	}
};

interface PrettyJoinOpts {
	quote?: boolean;
	tail?: string;
	empty?: string;
}

export function pretty_join(ARR: any[], { quote = false, tail = "and", empty = "" }: PrettyJoinOpts = {}) {
	if (ARR.length === 0) return empty;
	const MAP = quote
		? function wrap_str(str: string) {
				return typeof str === "string" ? (str.includes('"') ? `'${str}'` : `"${str}"`) : "" + str;
		  }
		: Identity;
	const DEDUPED = dedupe(ARR);
	var res = "" + MAP(ARR[0]);
	if (DEDUPED.length === 1) return res;
	for (var i = 1; i < DEDUPED.length - 1; ++i) res += ", " + MAP(ARR[i]);
	return res + " " + tail + " " + MAP(last_of(ARR));
}

export function pretty_timespan(n: number) {
	if (0 === n) return "now";
	if (n < 0 || isNaN(n)) return "Indefinite timespan";
	if (Infinity === n) return "Infinite timespan";
	if (1e-3 > n) return `${Math.trunc(n * 1e6)}µs`;
	if (1 > n) return `${Math.trunc(n * 1e3)}ns`;
	if (1e3 > n) return `${n.toFixed(0)}ms`;
	if (10e3 > n) return xs(+num(n / 1e3), "second");
	if (6e4 > n) return `${Math.trunc(n / 1e3)} seconds`;
	if (36e5 > n) return `${xs(Math.trunc(n / 6e4), "minute")}${n % 6e4 >= 1e3 ? ` and ${pretty_timespan(n % 6e4)}` : ""}`;
	if (432e5 > n) return xs(+num(n / 36e5), "hour");
	if (864e6 > n) return `over ${Math.trunc(n / 36e5)} hours`;
	return xs(Math.trunc(n / 864e6), "day");
	function xs(data: number, thing: string) {
		return data + " " + thing + (data !== 1 ? "s" : "");
	}
	function num(n: number): string {
		return n < 0 ? "-" + num(-n) : "" + (n | 0) + (n < 10 && n - (n | 0) >= 0.1 ? `.${((n - (n | 0)) * 10) | 0}` : "");
	}
}

function charCodeAtLast(str: string) {
	__DEV__: assert(str.length > 0);
	return str.charCodeAt(str.length - 1);
}

export function Identity<T>(v: T): T {
	return v;
}

export function last_of<T extends ArrayLike<any>>(arr: T): T extends readonly [...infer A, infer U] ? U : T[number] {
	__DEV__: isArrayLike(arr) || exit("Expected Array"), arr.length > 0 || exit("Attempted to retrieve last item of an empty array", arr);
	return arr[arr.length - 1];
}
export function maybe_last_of<T extends readonly any[] | undefined>(
	arr: T
): T extends any[] ? (T extends readonly [...infer A, infer U] ? U : T[number]) : undefined {
	return undefined === arr || 0 === arr.length ? undefined : last_of(arr as any[]);
}

export function normPath(filepath: string) {
	return filepath.replace(/^file:\/\/\//, "").replace(/\\\\?/g, "/");
}

// prettier-ignore
export function strChar(sequence: string | CharCode | CharCode[]): string {
	switch (typeof sequence) {
		case "number": return print_string(String.fromCharCode(sequence));
		case "object": return print_string(String.fromCharCode(...sequence));
		default: return print_string(sequence);
	}
}
export function strToken(sequence: CharCode[]): string {
	return sequence.includes(CharCode["'"]) ? `\`${strChar(sequence)}\`` : `'${strChar(sequence)}'`;
}
export function print_string(str: string) {
	return /[\u0000-\u0020]/.test(str)
		? str
				.replace(/ /g, "•")
				.replace(/\n/g, "↲")
				.replace(/\t/g, "╚")
				.replace(/[\u0000-\u0020]/g, "□")
		: str;
}

export function getLineStarts(str: string): number[] {
	for (var arr = [0], i = 0, j = 0; i < str.length - 1; i++) if (10 === str.charCodeAt(i)) arr[++j] = i + 1;
	return arr;
}
export function urlAt(filepath: string, lineStarts?: number[], index?: number) {
	const line = undefined === lineStarts || undefined === index || index < 0 ? -1 : getLineIndex(lineStarts, index);
	return normPath(filepath) + (-1 === line ? "" : `:${1 + line}:${1 + (index! - lineStarts![line])}`);
}

function isArrayLike(value: any): value is ArrayLike<unknown> {
	return is_object(value) && oisArrayLike(value);
}

function oisArrayLike(value: {}): value is ArrayLike<unknown> {
	return "length" in value && (0 === (value as any).length || "0" in value);
}

export function binarySearch(array: ArrayLike<number>, target: number) {
	if (isEmpty(array)) return -1;
	let i = 0;
	let low = 0;
	let high = array.length - 1;
	if (target >= array[high]) return high;
	else high--;
	while (low <= high) {
		i = low + ((high - low) >> 1);
		if (target === array[i]) return i;
		if (target > array[i]) low = i + 1;
		else high = i - 1;
	}
	return low - 1;
}
export function binarySearchIn<T extends {}>(array: ArrayLike<T>, target: number, toValue: (item: T) => number) {
	if (isEmpty(array)) return -1;
	let i = 0;
	let low = 0;
	let high = array.length - 1;
	let value = toValue(array[high]);
	if (target >= value) return high;
	else high--;
	while (low <= high) {
		i = low + ((high - low) >> 1);
		value = toValue(array[i]);
		if (target === value) return i;
		if (target > value) low = i + 1;
		else high = i - 1;
	}
	return low - 1;
}

export function binaryInsertIn<T extends {}>(array: T[], item: T, toValue: (item: T) => number) {
	if (0 === array.length || toValue(item) >= toValue(array[array.length - 1])) array[array.length] = item;
	else array.splice(1 + binarySearchIn(array, toValue(item), toValue), 0, item);
}

export function binaryInsertEach<T extends {}>(array: T[], items: T[], toValue: (item: T) => number) {
	for (var i = 0; i !== items.length; i++) binaryInsertIn(array, items[i], toValue);
}

export function getLineIndex(lineStarts: number[], index: number) {
	__DEV__: (isArrayLike(lineStarts) && lineStarts[0] === 0 && index >= 0) || exit("", lineStarts, index);
	return binarySearch(lineStarts, index);
}

export function getLineChar(lineStarts: number[], index: number): { line: number; char: number } {
	const line = getLineIndex(lineStarts, index);
	return { line, char: index - lineStarts[line] };
}
export function getTerminalWidth(fallbackWidth = 200) {
	return globalThis?.process?.stdout?.columns ?? fallbackWidth;
}

export type ColorFn = (str: string) => string;
// @ts-ignore
const isBrowser = typeof window !== "undefined" && typeof window.document !== "undefined";
export const color = ((cfn, mfn) => ({
	black: cfn(30),
	red: cfn(31),
	green: cfn(32),
	yellow: cfn(33),
	blue: cfn(34),
	magenta: cfn(35),
	cyan: cfn(36),
	white: cfn(37),
	grey: cfn(90),
	bold: mfn(1, 22),
	italic: mfn(3, 23),
	underline: mfn(4, 24),
	hidden: mfn(8, 28),
	hiddenCursor: (str: string) => `\x1B[?25l${str}\x1B[?25h`,
	unstyle: (str: string) => str.replace(/\x1B\[[0-9][0-9]?m/g, ""),
	unstyledLength: (str: string) => str.replace(/\x1B\[[0-9][0-9]?m/g, "").length,
	link: (str: string) => color.underline(color.blue(str)),
}))(
	(c1: number) => (isBrowser ? Identity : (str: string) => `\x1B[${c1}m${str.replace(/\x1B\[39m/g, `\x1B[${c1}m`)}\x1B[39m`),
	(c1: number, c2: number) => (isBrowser ? Identity : (str: string) => `\x1B[${c1}m${str}\x1B[${c2}m`)
);
export function dedupe<T>(arr: Iterable<T>): T[] {
	return [...new Set(arr)];
}
export function Map_add<K, V>(map: anymap<K, V[]>, key: K, value: V): V[] {
	if (map.has(key)) {
		var array = map.get(key)!;
		Array_add(array, value);
	} else {
		var array = [value];
		map.set(key, array);
	}
	return array;
}
export function Map_set<K, V>(map: anymap<K, V>, key: K, value: V): void {
	__DEV__: assert(!map.has(key));
	map.set(key, value);
}
export function Map_includes<K, V>(map: anymap<K, V[]>, key: K, value: V): boolean {
	return map.has(key) && map.get(key)!.includes(value);
}
export function Map_get<K extends object, V>(map: WeakMap<K, V>, key: K, init: (key: K) => V): V;
export function Map_get<K, V>(map: Map<K, V>, key: K, init: (key: K) => V): V;
export function Map_get<K, V>(map: anymap<K, V>, key: K, init: (key: K) => V): V {
	if (!map.has(key)) map.set(key, init(key));
	return map.get(key)!;
}
export function isEmpty(array: ArrayLike<any>): boolean {
	__DEV__: assert(isArrayLike(array));
	return 0 === array.length;
}
export function Array_add<T>(arr: T[], item: T): void {
	__DEV__: assert(!arr.includes(item), "Array already includes item", { array: arr, item: item });
	arr.push(item);
}
export function Array_splice<T extends any[]>(array: T, target: T[number], index: number = array.indexOf(target)) {
	__DEV__: {
		const i = arguments.length === 2 ? array.indexOf(target) : index;
		assert(i === index && i !== -1 && i === array.lastIndexOf(target), "", { array, target, index, i });
	}
	array.splice(index, 1);
}
export function Array_replace<T extends any[]>(array: T, target: T[number], ...replacements: T[number][]) {
	const i = array.indexOf(target);
	__DEV__: if (i === -1 || i !== array.lastIndexOf(target))
		exit("Array_replace", { index: i, lastIndex: array.lastIndexOf(target), array, target, replacements });
	array.splice(array.indexOf(target), 1, ...replacements);
}
export function Array_find_splice<T extends any[]>(array: T, test: (item: T[number], index: number, array: T) => boolean): T[number] {
	const i = array.findIndex(test);
	__DEV__: assert(i !== -1 && array.slice(i + 1).findIndex(test) === -1);
	const item = array[i];
	array.splice(i, 1);
	return item;
}
export function Array_pop<T extends any[]>(arr: T, item: T[number]) {
	__DEV__: assert(last_of(arr) === item);
	arr.pop();
}
export function toArray<T>(data: T | T[] | undefined): T[] {
	return is_defined(data) ? (is_array(data) ? data : [data]) : [];
}
export function toArrayFlat<T>(data: T | T[] | undefined): FlatArray<T> {
	return (is_defined(data) ? (is_array(data) ? flat(data) : [data]) : []) as FlatArray<T>;
}
export function has_key<T extends object, K extends T extends never ? never : keyof T>(
	o: T,
	k: K
): o is K extends never
	? never
	: T extends { [k in K]: any }
	? T & { [k in K]: any }
	: T extends { [k in K]?: any }
	? T & { [k in K]: NonNullable<T[k]> }
	: never {
	return k in o;
}
export function has_key_undefined<T extends object, K extends T extends never ? never : keyof T>(
	o: T,
	k: K
): o is K extends never
	? never
	: T extends { [k in K]: any }
	? T & { [k in K]: undefined }
	: T extends { [k in K]?: any }
	? T & { [k in K]: undefined }
	: never {
	return k in o && undefined === o[k];
}
export function has_key_defined<T extends object, K extends T extends never ? never : keyof T>(
	o: T,
	k: K
): o is K extends never
	? never
	: T extends { [k in K]: any }
	? T & { [k in K]: {} }
	: T extends { [k in K]?: any }
	? T & { [k in K]: {} }
	: never {
	return k in o && undefined !== o[k];
}

export function has_key_non_empty_array<T extends object, K extends T extends never ? never : keyof T>(
	o: T,
	k: K
): o is K extends never
	? never
	: T extends { [k in K]: any }
	? T & { [k in K]: any[] }
	: T extends { [k in K]?: any }
	? T & { [k in K]: any[] }
	: never {
	return k in o && undefined !== o[k] && "length" in o[k] && 0 !== (o[k] as any).length;
}

export function is_defined<T>(data: T | undefined): data is T {
	return undefined !== data;
}

export function is_defined_non_empty<T>(data: T[] | undefined): data is T[] {
	return undefined !== data && 0 !== data.length;
}

export function is_object(data: unknown): data is object | ({ [key: string]: unknown } | unknown[]) {
	return "object" === typeof data && null !== data;
}

export function is_array(data: unknown): data is any[] {
	return Array.isArray(data);
}
export function is_non_empty_array(data: unknown): data is any[] {
	return Array.isArray(data) && data.length !== 0;
}

export function is_string(data: unknown): data is string {
	return "string" === typeof data;
}

function ois_vobject(data: any) {
	__DEV__: assert(is_object(data));
	switch (data.constructor) {
		case Array:
		case Object:
		case Set:
		case Map:
			return true;
		default:
			return false;
	}
}

export function is_thenable(value: any): value is PromiseLike<unknown> {
	return typeof value === "object" && value !== null && "then" in value && typeof value.then === "function";
}

// export function resolveEval<T, R>(evalValue: () => T, then_: (r: Awaited<T>) => R, catch_?: (e: any) => R): T | R {
// 	if (catch_) {
// 		try {
// 			var res = evalValue();
// 		} catch (e) {
// 			return catch_(e) as any;
// 		}
// 		return (is_thenable(res) ? res.then(then_).catch(catch_) : then_(res)) as any;
// 	} else {
// 		var res = evalValue();
// 		return (is_thenable(res) ? res.then(then_) : then_(res)) as any;
// 	}
// }

export function resolve_then<T, R>(
	value: T,
	then: (v: Awaited<T>) => R
): T extends Promise<any> ? Promise<Awaited<R>> : R extends Promise<any> ? R : R {
	return is_thenable(value) ? value.then(then) : (then(value as any) as any);
}

export function each<A extends vObject>(data: A, callback: itfn<A, void>): void;
export function each(data: any, callback: (value: any, index: any) => void): void {
	__DEV__: assert(ois_vobject(data));
	// prettier-ignore
	switch (data.constructor) {
		case Array: { let i = 0; for (; i < data.length; i++) callback(data[i], i); return; }
		case Object: { let k; for (k in data) callback(data[k], k); return; }
		case Set: { let d; for (d of data) callback(d, undefined!); return; }
		case Map: { let e; for (e of data) callback(e[1], e[0]); return; }
		default:  { let x; for (x of data) callback(x, undefined!); return; }
	}
}

export function some<A>(data: A, test: itfn<A, boolean>): boolean;
export function some(data: any, test: (value: any, index: any) => boolean): boolean {
	__DEV__: assert(ois_vobject(data));
	// prettier-ignore
	switch (data.constructor) {
		case Array: { let i = 0; for (; i < data.length; i++) if (test(data[i], i)) return true; return false; }
		case Object: { let k; for (k in data) if (test(data[k], k)) return true; return false; }
		case Set: { let d; for (d of data) if (test(d, undefined!)) return true; return false; }
		case Map: { let e; for (e of data) if (test(e[1], e[0])) return true; return false; }
		default:  { let x; for (x of data) if (test(x, undefined!)) return true; return false; }
	}
}

export function every<A extends vIterable>(data: A, test: itfn<A, boolean>): boolean;
export function every(data: any, test: (value: any, index: any) => boolean): boolean {
	__DEV__: assert(ois_vobject(data));
	// prettier-ignore
	switch (data.constructor) {
		case Array: { let i = 0; for (; i < data.length; i++) if (!test(data[i], i)) return false; return true; }
		case Object: { let k; for (k in data) if (!test(data[k], k)) return false; return true; }
		case Set: { let d; for (d of data) if (!test(d, undefined!)) return false; return true; }
		case Map: { let e; for (e of data) if (!test(e[1], e[0])) return false; return true; }
		default:  { let x; for (x of data) if (!test(x, undefined!)) return false; return true; }
	}
}

export function iLast(index: number, array: any[]) {
	return 1 + index === array.length;
}

export function find_last<T>(arr: T[], test: itfn<T[], boolean>): T | undefined {
	for (var i = arr.length; --i !== -1; ) if (test(arr[i], i)) return arr[i];
}

export function find_last_index<T>(arr: T[], test: itfn<T[], boolean>): number {
	for (var i = arr.length; --i !== -1; ) if (test(arr[i], i)) return i;
	return -1;
}

export function try_eval<T>(fn: () => T): T | undefined {
	try {
		return fn();
	} catch (e) {
		return undefined;
	}
}

export function trycatch<T, C>(tryFn: () => T, catchFn: (e: Error) => C): T | C;
export function trycatch<T>(tryFn: () => T): T | Error;
export function trycatch(tryFn: () => any, catchFn: (e: Error) => any = (e) => e) {
	try {
		return tryFn();
	} catch (e) {
		return catchFn(e as Error);
	}
}

export function count_occurences(str: string, re: RegExp) {
	return str.match(re)?.length ?? 0;
}

export function get_extra_indent(str: string) {
	return count_occurences(str, /\t/g) * 3;
}
export function get_tab_aware_printWidth(str: string) {
	return str.length + get_extra_indent(str);
}

export function clamp(min: number, max: number, value: number) {
	return value > min ? (value < max ? value : max) : min;
}
export function min(a: number, b: number) {
	return a < b ? a : b;
}
export function max(a: number, b: number) {
	return a > b ? a : b;
}

type MaybeNested<T> = T | MaybeNested<T>[];
export type MaybeFlatten<T> = T extends ReadonlyArray<infer U> ? MaybeFlatten<Exclude<U, T>> : T;
export type DeepArray<T> = MaybeNested<T>[];
export type FlatArray<T> = MaybeFlatten<T>[];

export function flat<T extends readonly any[]>(arr: T): FlatArray<T> {
	return (arr as any as [any]).flat(Infinity);
}
export function flatMap<T extends readonly any[], R>(arr: T, mapFn: (item: T[number], index: number, array: T) => R): FlatArray<R> {
	return flat(arr.map(mapFn));
}

export function joinln(...arr: string[]): string {
	return arr.join("\n");
}

export function join_lines(fn: () => Generator<string, void, void>): string {
	return [...fn()].join("\n");
}

export function reduce_tagged_template<T>(args: [strings: TemplateStringsArray, ...values: T[]], map: (value: T) => string) {
	for (var str = "" + args[0][0], i = 1; i < args.length; i++) str += map(args[i] as T) + args[0][i];
	return str;
}

export function map_tagged_template<T, R>(args: [strings: TemplateStringsArray, ...values: T[]], map: (value: T) => R) {
	const arr: (R | string)[] = [args[0][0]];
	for (var i = 1; i < args.length; i++) arr.push(map(args[i] as T), args[0][i]);
	return arr;
}

export function edit_string(str: string, start: number, content: string, end: number) {
	return str.slice(0, start) + content + str.slice(end);
}

export function map_filter<T, R extends Exclude<unknown, undefined>>(
	arr: T[],
	map: (value: T, index: number, arr: T[]) => R,
	test: (value: R, index: number) => boolean = (v) => null != v
): R[] {
	for (var mapped: R[] = [], i = 0, j = 0; i < arr.length; i++) if (test((mapped[j] = map(arr[i], i, arr)), i)) j++;
	return (mapped.length = j), mapped;
}

export function array_filter_map_join<T extends any[]>(
	arr: T,
	test: (value: T[number], index: number) => boolean,
	map: (value: T[number], index: number) => string,
	separator: string
): string {
	var str = "";
	var i = 0;
	for (; i < arr.length; i++)
		if (test(arr[i], i)) {
			str += map(arr[i], i);
			while (++i < arr.length) if (test(arr[i], i)) str += separator + map(arr[i], i);
			break;
		}
	return str;
}

export function map_join<T extends any[]>(arr: T, map: (value: T[number], index: number) => string, separator: string): string {
	var str = "";
	var i = 0;
	if (arr.length !== 0) {
		str += map(arr[i], i);
		while (++i < arr.length) str += separator + map(arr[i], i);
	}
	return str;
}

export function spliceAll<T extends any[]>(array: T): [...T] {
	const r: [...T] = [...array];
	array.length = 0;
	return r;
}

export function spread<R>(fn: () => Iterable<R>): R[] {
	return [...fn()];
}

export function* each_match(re: RegExp, str: string): Generator<RegExpExecArray, void, void> {
	var match: RegExpExecArray | null = re.exec(str);
	while (null !== match) yield match, (match = re.exec(str));
}

export function indexOf(re: RegExp, str: string) {
	return re.exec(str)?.index ?? -1;
}
