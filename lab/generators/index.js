import readline from "node:readline";
import fs from "node:fs";
import path from "node:path";

import { piper } from "./src/tools.js";

import addCountryDetails from "./src/generators/addCountryDetails.js";
import logger from "./src/generators/logger.js";
import parser from "./src/generators/parser.js";
import toString from "./src/generators/toString.js";

async function bootstrap() {
  const fileStream = fs.createReadStream(path.join(import.meta.dirname, "./assets/athletes.txt"));
  const stream = readline.createInterface({ input: fileStream, crlfDelay: Infinity });

  const pipe = await piper(
    parser,
    addCountryDetails,
    process.env.DEBUG ? logger : undefined,
    toString
  );

  for await (const athlete of pipe(stream)) {
    console.log(athlete);
  }
}

await bootstrap();