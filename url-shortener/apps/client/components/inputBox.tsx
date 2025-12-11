"use client";

import { useState } from "react";
import { Button } from "./ui/button";
import { Input } from "./ui/input";
import { shorten_the_url } from "@/action";

const InputBox = () => {
  const [url, setUrl] = useState("");
  const [shortenedUrl, setShortenedUrl] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);
  const [copied, setCopied] = useState(false);

  const isValidUrl = (urlString: string): boolean => {
    try {
      const url = new URL(urlString);
      return url.protocol === "http:" || url.protocol === "https:";
    } catch {
      return false;
    }
  };

  const handleShorten = async () => {
    if (!url.trim()) {
      setError("Please enter a URL");
      return;
    }

    if (!isValidUrl(url)) {
      setError("Please enter a valid URL (must start with http:// or https://)");
      return;
    }

    setError("");
    setLoading(true);
    setShortenedUrl("");
    setCopied(false);

    try {
      const result = await shorten_the_url(url);
      setShortenedUrl(result.shortenedUrl);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to shorten URL");
    } finally {
      setLoading(false);
    }
  };

  const handleCopy = async () => {
    if (!shortenedUrl) return;

    try {
      await navigator.clipboard.writeText(shortenedUrl);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      setError("Failed to copy to clipboard");
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      handleShorten();
    }
  };

  return (
    <div className="bg-white p-8 rounded-lg shadow-xl flex flex-col items-center gap-6 w-full max-w-md">
      <h1 className="text-2xl font-bold text-gray-800">URL Shortener</h1>
      
      <div className="w-full flex flex-col gap-3">
        <Input
          type="text"
          placeholder="https://example.com"
          value={url}
          onChange={(e) => {
            setUrl(e.target.value);
            setError("");
          }}
          onKeyPress={handleKeyPress}
          className="w-full"
          disabled={loading}
          aria-invalid={error ? "true" : "false"}
        />
        
        {error && (
          <p className="text-sm text-red-600" role="alert">
            {error}
          </p>
        )}
      </div>

      <Button
        onClick={handleShorten}
        disabled={loading || !url.trim()}
        className="w-full"
      >
        {loading ? "Shortening..." : "Shorten URL"}
      </Button>

      {shortenedUrl && (
        <div className="w-full flex flex-col gap-3 p-4 bg-gray-50 rounded-md border">
          <p className="text-sm font-medium text-gray-700">Shortened URL:</p>
          <div className="flex gap-2">
            <Input
              type="text"
              value={shortenedUrl}
              readOnly
              className="flex-1 bg-white"
            />
            <Button
              onClick={handleCopy}
              variant={copied ? "secondary" : "outline"}
              size="sm"
            >
              {copied ? "Copied!" : "Copy"}
            </Button>
          </div>
        </div>
      )}
    </div>
  );
};

export default InputBox;