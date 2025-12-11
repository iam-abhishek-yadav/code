import { createHash } from "crypto";

/**
 * Generates a SHA-256 hash of the input string
 * @param input - The string to hash
 * @returns Hexadecimal hash string
 */
export function generateHash(input: string): string {
  return createHash("sha256").update(input).digest("hex");
}

/**
 * Converts a portion of a hash string to a number
 * @param hash - The hexadecimal hash string
 * @param length - Number of hex characters to use (default: 8 for 32 bits)
 * @returns The numeric value of the hash substring
 */
export function hashToNumber(hash: string, length: number = 8): number {
  const substring = hash.substring(0, length);
  return parseInt(substring, 16);
}
