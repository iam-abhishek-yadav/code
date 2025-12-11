import type { NextFunction, Request, Response } from "express";

const express = require("express");
const app = express();

app.use(express.json());

app.use((req: Request, res: Response, next: NextFunction) => {
  res.header("Access-Control-Allow-Origin", "*");
  res.header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS");
  res.header("Access-Control-Allow-Headers", "Content-Type, Authorization");
  if (req.method === "OPTIONS") {
    return res.sendStatus(200);
  }
  next();
});

import { shortenUrl, getOriginalUrl } from "./routes/urlRoutes";

app.post("/shorten", shortenUrl);
app.get("/:shortCode", getOriginalUrl);

app.get("/", (req: Request, res: Response) => {
  res.send("Write service is running");
});

const PORT = 3000;
app.listen(PORT, () => {
  console.log(`Write service is running on port ${PORT}`);
});
