import { chain } from "./src/tools.js";
import addCountryDetails from "./src/generators/addCountryDetails.js";
import logger from "./src/generators/logger.js";
import parser from "./src/generators/parser.js";
import stringifier from "./src/generators/stringifier.js";
import reader from "./src/generators/reader.js";
import writer from "./src/generators/writer.js";


await chain(
  reader,
  parser,
  addCountryDetails,
  logger,
  stringifier,
  process.env.DEBUG ? logger : writer
);

//OR without tools

import fs from 'node:fs/promises';
import path from "path";

async function bootstrap() {
  const file = await fs.open(path.join(import.meta.dirname, "./assets/athletes.txt"));
  const stream = file.readLines();

  for await (const chunk of stringifier(logger(addCountryDetails(parser(stream))))) {
    console.log(chunk)
  }
}

// await bootstrap();

