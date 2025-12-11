import { BASE62_CHARS } from "../config/constants";

/**
 * Converts a number to base-62 encoding
 * @param num - The number to convert
 * @returns Base-62 encoded string
 */
export function toBase62(num: number): string {
  if (num === 0) {
    return BASE62_CHARS.charAt(0);
  }

  let result = "";
  while (num > 0) {
    result = BASE62_CHARS.charAt(num % 62) + result;
    num = Math.floor(num / 62);
  }
  return result;
}

/**
 * Converts a base-62 string back to a number
 * @param str - The base-62 encoded string
 * @returns The decoded number
 */
export function fromBase62(str: string): number {
  let result = 0;
  for (let i = 0; i < str.length; i++) {
    const char = str.charAt(i);
    const charIndex = BASE62_CHARS.indexOf(char);
    if (charIndex === -1) {
      throw new Error(`Invalid base-62 character: ${char}`);
    }
    result = result * 62 + charIndex;
  }
  return result;
}
