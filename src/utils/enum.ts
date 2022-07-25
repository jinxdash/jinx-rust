// prettier-ignore
import { DelimKind } from "../parser/nodes";
import { CharCode } from "./CharCode";
import { exit, strChar } from "./common";
import { is_UNICODE_XID_Continue, is_UNICODE_XID_Start } from "./unicode";

// prettier-ignore
export function is_number(char: number) {
	switch (char) {
		case 48: case 49: case 50: case 51: case 52: case 53: case 54: case 55: case 56: case 57: return true; // 0-9
		default: return false;
	}
}
// prettier-ignore
export function is_whitespaceOrSlash(char: number) {
	switch (char) {
		// case CharCode["\t"]: case CharCode["\v"]: case CharCode["\f"]: case CharCode["\r"]: case CharCode[" "]:
		// #ws
		case 9: case 10: case 11: case 12: case 13: case 32: case 47: case 133: case 8206: case 8207: case 8232: case 8233: return true;
		default: return false;
	}
}
// prettier-ignore
export function is_whitespaceStrict(char: number) {
	switch (char) {
		// #ws -1
		case 9: case 10: case 11: case 12: case 13: case 32: /*'/'*/ case 133: case 8206: case 8207: case 8232: case 8233: return true;
		default: return false;
	}
}
// prettier-ignore
export function is_UpperCase(char: number) {
	switch (char) {
		case 65: case 66: case 67: case 68: case 69: case 70: case 71: case 72: case 73: case 74: 
		case 75: case 76: case 77: case 78: case 79: case 80: case 81: case 82: case 83: case 84: 
		case 85: case 86: case 87: case 88: case 89: case 90: // A-Z
			return true;
		default:
			return false;
	}
}
// prettier-ignore
export function is_XID_Start(char: number) {
	switch (char) {
		case 65: case 66: case 67: case 68: case 69: case 70: case 71: case 72: case 73: case 74: 
		case 75: case 76: case 77: case 78: case 79: case 80: case 81: case 82: case 83: case 84: 
		case 85: case 86: case 87: case 88: case 89: case 90: // A-Z
		case 95: // _
		case 97: case 98: case 99: 
		case 100: case 101: case 102: case 103: case 104: case 105: case 106: case 107: case 108: 
		case 109: case 110: case 111: case 112: case 113: case 114: case 115: case 116: case 117: 
		case 118: case 119: case 120: case 121: case 122: // a-z
			return true;
		default:
			return 128 < char && is_UNICODE_XID_Start(char);
	}
}
// prettier-ignore
export function is_XID_Continue(char: number) {
	switch (char) {
		case 48: case 49: case 50: case 51: case 52: case 53: case 54: case 55: case 56: case 57: // 0-9
		case 65: case 66: case 67: case 68: case 69: case 70: case 71: case 72: case 73: case 74: 
		case 75: case 76: case 77: case 78: case 79: case 80: case 81: case 82: case 83: case 84: 
		case 85: case 86: case 87: case 88: case 89: case 90: // A-Z
		case 95: // _
		case 97: case 98: case 99: 
		case 100: case 101: case 102: case 103: case 104: case 105: case 106: case 107: case 108: 
		case 109: case 110: case 111: case 112: case 113: case 114: case 115: case 116: case 117: 
		case 118: case 119: case 120: case 121: case 122: // a-z
			return true;
		default:
			return (128 < char) && is_UNICODE_XID_Continue(char);
	}
}

// prettier-ignore
export function is_hex(char: number) {
	switch (char) {
		case 48: case 49: case 50: case 51: case 52: case 53: case 54: case 55: case 56: case 57: // 0-9
		case 65: case 66: case 67: case 68: case 69: case 70: // A-F
		case 97: case 98: case 99: case 100: case 101: case 102: // a-f
			return true;
		default: 
			return false;
	}
}

// prettier-ignore
export function is_oct(char: number) {
	switch (char) {
		case 48: case 49: case 50: case 51: case 52: case 53: case 54: case 55: return true; // 0-7
		default: return false;
	}
}

// prettier-ignore
export function is_bin(char: number) {
	switch (char) {
		case 48: case 49: return true; // 0-1
		default: return false;
	}
}

// prettier-ignore
export function getDelimKind(startChar: number): DelimKind {
	switch (startChar) {
		case CharCode["("]: return DelimKind["()"];
		case CharCode["["]: return DelimKind["[]"];
		case CharCode["{"]: return DelimKind["{}"];
		case CharCode["<"]: return DelimKind["<>"];
		case CharCode["|"]: return DelimKind["||"];
	}
	return DelimKind["None"];
}

export type SomeDelimKind = Exclude<DelimKind, DelimKind.None>;
// prettier-ignore
export function getSomeDelimKind(startChar: number): SomeDelimKind {
	switch (startChar) {
		case CharCode["("]: return DelimKind["()"];
		case CharCode["["]: return DelimKind["[]"];
		case CharCode["{"]: return DelimKind["{}"];
		case CharCode["<"]: return DelimKind["<>"];
		case CharCode["|"]: return DelimKind["||"];
	}
	exit(`Assertion failed: Expected SomeDelimKind start, got '${strChar(startChar)}'`);
}

export type GroupDelimKind = typeof DelimKind["()"] | typeof DelimKind["[]"] | typeof DelimKind["{}"];
// prettier-ignore
export function getGroupDelimKind(startChar: number): GroupDelimKind {
	switch (startChar) {
		case CharCode["("]: return DelimKind["()"];
		case CharCode["["]: return DelimKind["[]"];
		case CharCode["{"]: return DelimKind["{}"];
	}
	exit(`Assertion failed: Expected GroupDelim start`);
}

// prettier-ignore
export function getDelimStartCharCode(kind: SomeDelimKind) {
	switch (kind as DelimKind) {
		case DelimKind["()"]: return CharCode["("];
		case DelimKind["[]"]: return CharCode["["];
		case DelimKind["{}"]: return CharCode["{"];
		case DelimKind["<>"]: return CharCode["<"];
		case DelimKind["||"]: return CharCode["|"];
	}
	exit(`Assertion failed: Expected SomeDelimKind`, kind);
}

// prettier-ignore
export function getDelimEndCharCode(kind: SomeDelimKind) {
	switch (kind as DelimKind) {
		case DelimKind["()"]: return CharCode[")"];
		case DelimKind["[]"]: return CharCode["]"];
		case DelimKind["{}"]: return CharCode["}"];
		case DelimKind["<>"]: return CharCode[">"];
		case DelimKind["||"]: return CharCode["|"];
	}
	exit(`Assertion failed: Expected SomeDelimKind`, kind);
}
