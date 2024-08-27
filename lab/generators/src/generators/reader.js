import readline from "node:readline";
import fs from "node:fs";
import path from "node:path";

/**
 * Read input file data
 */
export default async function* reader() {
  const fileStream = fs.createReadStream(path.join(import.meta.dirname, "../../assets/athletes.txt"));
  yield* readline.createInterface({ input: fileStream, crlfDelay: Infinity });
}