
import type { Request, Response } from "express";
import { shortCodeService } from "../services/shortCodeService";
import { CODE_LENGTH } from "../config/constants";
import type { ShortenUrlResponse, ErrorResponse } from "../types";

/**
 * Validates if a string is a valid URL
 * @param urlString - The string to validate
 * @returns True if valid URL, false otherwise
 */
function isValidUrl(urlString: string): boolean {
  try {
    const url = new URL(urlString);
    return url.protocol === "http:" || url.protocol === "https:";
  } catch {
    return false;
  }
}

/**
 * POST /shorten - Shorten a URL
 */
export function shortenUrl(req: Request, res: Response<ShortenUrlResponse | ErrorResponse>) {
  try {
    const { url } = req.body;

    if (!url || typeof url !== "string") {
      return res.status(400).json({ error: "URL is required" });
    }

    if (!isValidUrl(url)) {
      return res.status(400).json({ error: "Invalid URL format" });
    }

    console.log("Received request to shorten URL:", url);

    const shortCode = shortCodeService.getUniqueShortCode(url);
    const shortenedUrl = `http://localhost:3000/${shortCode}`;

    console.log("Generated short code:", shortCode);

    return res.json({
      shortCode,
      shortenedUrl,
      originalUrl: url,
    });
  } catch (error) {
    console.error("Error shortening URL:", error);
    return res.status(500).json({ error: "Internal server error" });
  }
}

/**
 * GET /:shortCode - Retrieve and redirect to original URL
 */
export function getOriginalUrl(req: Request, res: Response) {
  try {
    const { shortCode } = req.params;

    if (!shortCode || shortCode.length !== CODE_LENGTH) {
      return res.status(400).json({ error: "Invalid short code" });
    }

    const originalUrl = shortCodeService.getOriginalUrl(shortCode);

    if (!originalUrl) {
      return res.status(404).json({ error: "Short code not found" });
    }

    return res.redirect(originalUrl);
  } catch (error) {
    console.error("Error retrieving URL:", error);
    return res.status(500).json({ error: "Internal server error" });
  }
}
