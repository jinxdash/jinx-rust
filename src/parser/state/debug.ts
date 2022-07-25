import { assert } from "../../utils/common";
import { Node, __DevonlyObject } from "../nodes";

type StatsItem = __DevonlyObject["stats"];

interface DebugState {
	[state_name: string]: unknown;
}

interface SetupItem {
	stats?: () => StatsItem;
	debug?: () => DebugState;
	check?: (ast: Node) => void;
}

let itemsLength = 0;
const initDevItems: (() => SetupItem)[] = [];
const currentItems: SetupItem[] = [];

function aggregate<K extends "stats" | "debug">(targetKey: K, sort: boolean): ReturnType<NonNullable<SetupItem[K]>> {
	if (sort) {
		return createObjectWithSortedKeys(gen) as any;
	} else {
		const o = {};
		gen(
			(k, v) => void (o[k] = v),
			(k) => k in o
		);
		return o as any;
	}
	function gen(addKeyValue: (key: string, value: unknown) => void, hasKey: (key: string) => boolean) {
		for (var i = 0; i < itemsLength; i++) {
			if (targetKey in currentItems[i]) {
				try {
					var itemValue = currentItems[i][targetKey]!();
				} catch (e) {
					console.log(e);
					itemValue = {};
				}
				for (var key in itemValue) {
					addKeyValue(getSafeKey(key, hasKey), itemValue[key]);
				}
			}
		}
	}

	function createObjectWithSortedKeys<V>(create: (add: (key: string, value: V) => void, hasKey: (key: string) => boolean) => void) {
		const names: string[] = [];
		const nameToValue = new Map<string, V>();
		create(
			function (key, value) {
				__DEV__: assert(typeof key === "string" && key !== "" && !nameToValue.has(key), "", { key, names, nameToValue });
				nameToValue.set(key, value);
				names.push(key);
			},
			function (key) {
				return nameToValue.has(key);
			}
		);

		names.sort(function (a, b) {
			return a.localeCompare(b, "en");
		});

		const obj: { [name: string]: V } = {};
		for (var i = 0; i < names.length; i++) {
			const name = names[i];
			obj[name] = nameToValue.get(name)!;
		}

		return obj;
	}

	function getSafeKey(key: string, hasKey: (key: string) => boolean) {
		if (hasKey(key)) {
			let i = 1;
			while (hasKey(`${key}_${i}`)) i++;
			return `${key}_${i}`;
		}
		return key;
	}
}

export function devonly<T extends SetupItem | {}>(run: () => T): T {
	__DEV__: {
		const prev = (currentItems[itemsLength] = {} as any);
		const next = (initDevItems[itemsLength++] = run)();
		for (var key in next) prev[key] = next[key];
		return prev;
	}
}

export function devonly_check(ast: Node) {
	__DEV__: {
		for (var i = 0; i < itemsLength; i++) currentItems[i].check?.(ast);
	}
}

export function devonly_cleanup() {
	__DEV__: {
		for (var i = 0; i < itemsLength; i++) {
			const prev = currentItems[i];
			const next = initDevItems[i]();
			for (var key in prev) prev[key] = next[key];
		}
	}
}

export function devonly_getDebugState() {
	__DEV__: return aggregate("debug", false);
}

export function devonly_getDevonlyObject(): __DevonlyObject {
	__DEV__: return { stats: aggregate("stats", true) };
}
