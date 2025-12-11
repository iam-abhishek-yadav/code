import { CODE_LENGTH, MAX_COLLISION_ATTEMPTS, BASE62_CHARS } from "../config/constants";
import { toBase62 } from "../utils/encoding";
import { generateHash, hashToNumber } from "../utils/hashing";
import type { IShortCodeService } from "../types";

const urlStore = new Map<string, string>();
const urlToCode = new Map<string, string>();

/**
 * Generates a short code from a URL using hash-based approach
 * @param url - The original URL
 * @param attempt - Collision resolution attempt number (default: 0)
 * @returns A short code of CODE_LENGTH characters
 */
function generateShortCode(url: string, attempt: number = 0): string {
  const hashInput = attempt === 0 ? url : `${url}:${attempt}`;
  const hash = generateHash(hashInput);
  const num = hashToNumber(hash, 8);
  const base62 = toBase62(num);
  const code = base62.substring(0, CODE_LENGTH).padStart(CODE_LENGTH, BASE62_CHARS[0]);
  return code;
}

class ShortCodeService implements IShortCodeService {
  /**
   * Generates a short code from a URL
   * @param url - The original URL
   * @param attempt - Collision resolution attempt number
   * @returns A short code
   */
  generateShortCode(url: string, attempt: number = 0): string {
    return generateShortCode(url, attempt);
  }

  /**
   * Generates a unique short code with collision handling
   * @param url - The original URL
   * @returns A unique short code
   */
  getUniqueShortCode(url: string): string {
    if (urlToCode.has(url)) {
      return urlToCode.get(url)!;
    }

    let attempt = 0;
    let code: string;

    do {
      code = generateShortCode(url, attempt);
      attempt++;

      if (attempt > MAX_COLLISION_ATTEMPTS) {
        throw new Error(
          `Failed to generate unique code after ${MAX_COLLISION_ATTEMPTS} attempts`
        );
      }
    } while (urlStore.has(code) && urlStore.get(code) !== url);

    if (urlStore.has(code) && urlStore.get(code) === url) {
      return code;
    }

    urlStore.set(code, url);
    urlToCode.set(url, code);

    return code;
  }

  /**
   * Retrieves the original URL for a given short code
   * @param shortCode - The short code to look up
   * @returns The original URL or undefined if not found
   */
  getOriginalUrl(shortCode: string): string | undefined {
    return urlStore.get(shortCode);
  }
}

export const shortCodeService = new ShortCodeService();
