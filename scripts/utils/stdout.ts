import { clearLine, clearScreenDown, cursorTo, moveCursor } from "node:readline";

const __console_log = console.log.bind(console);
console.log = (...args) => {
	_effect(() => {
		__console_log(...args);
	});
};

let l = 0;
const items: Line[] = [];
function _moveTo(line_index: number) {
	cursorTo(process.stdout, 0);
	if (l !== 0) {
		moveCursor(process.stdout, 0, -l + line_index);
		l = line_index;
	}
}
function _effect(fn: () => void) {
	if (items.length) {
		_moveTo(0);
		clearScreenDown(process.stdout);
	} else {
		cursorTo(process.stdout, 0);
		clearLine(process.stdout, 1);
	}
	fn();
	if (items.length) {
		for (const item of items) item.dispatch();
	}
}

interface Line {
	set(value: string): void;
	dispatch(): void;
	delete(): void;
}

export function create_console_line(str?: string) {
	const line = {
		set(value: string) {
			current = value.replace(/\n/g, " ");
			line.dispatch();
		},
		dispatch() {
			if (enabled) {
				_moveTo(items.indexOf(this));
				clearLine(process.stdout, 1);
				process.stdout.write(current);
			} else {
				enabled = true;
				_effect(() => {
					items.push(line);
				});
			}
		},
		delete() {
			if (enabled) {
				enabled = false;
				_effect(() => {
					items.splice(items.indexOf(line), 1);
				});
			}
		},
	};

	let enabled = false;
	let current = "";

	if (str) line.set(str);

	return line;
}

var Error_prepareStackTrace = Error.prepareStackTrace;
Error.prepareStackTrace = function (err, calls) {
	_moveTo(0);
	return Error_prepareStackTrace ? Error_prepareStackTrace.call(this, err, calls) : calls;
};

process.on("exit", () => {
	if (items.length) {
		process.stdout.write("\n");
	}
});
