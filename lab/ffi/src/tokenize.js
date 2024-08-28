import { readFile, writeFile } from "node:fs/promises";
import jwt from "jsonwebtoken";

export default async function (inputPath, outputPath) {
  const fileData = await readFile(inputPath, { encoding: "utf-8" });
  const parsedData = JSON.parse(fileData);
  const builtData = parsedData.map(i => jwt.sign(i, "secret"));
  await writeFile(outputPath, JSON.stringify(builtData, null, "\t"));
}