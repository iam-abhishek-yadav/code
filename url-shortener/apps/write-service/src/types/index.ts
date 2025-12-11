export interface ShortenUrlRequest {
  url: string;
}

export interface ShortenUrlResponse {
  shortCode: string;
  shortenedUrl: string;
  originalUrl: string;
}

export interface ErrorResponse {
  error: string;
}

export interface IShortCodeService {
  generateShortCode(url: string, attempt?: number): string;
  getUniqueShortCode(url: string): string;
  getOriginalUrl(shortCode: string): string | undefined;
}
