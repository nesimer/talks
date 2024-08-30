import fs from "node:fs/promises";
import path from "node:path";

/**
 * Read input file data
 */
export default async function* reader() {
  const file = await fs.open(path.join(import.meta.dirname, "../../assets/athletes.txt"));
  yield* file.readLines();
}