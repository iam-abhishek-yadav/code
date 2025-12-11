"use server";

export interface ShortenUrlResponse {
  shortCode: string;
  shortenedUrl: string;
  originalUrl: string;
}

export const shorten_the_url = async (url: string): Promise<ShortenUrlResponse> => {
  if (!process.env.WRITE_SERVICE_URL) {
    throw new Error("WRITE_SERVICE_URL environment variable is not set");
  }

  const baseUrl = process.env.WRITE_SERVICE_URL.replace(/\/$/, "");
  const response = await fetch(`${baseUrl}/shorten`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ url }),
  });

  if (!response.ok) {
    const text = await response.text();
    let errorMessage = `Failed to shorten URL: ${response.status}`;
    try {
      const errorData = JSON.parse(text);
      errorMessage = errorData.error || errorMessage;
    } catch {
      errorMessage = text || errorMessage;
    }
    throw new Error(errorMessage);
  }

  return response.json();
};