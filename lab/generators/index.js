import { chain } from "./src/tools.js";
import generators from "./src/generators.js";

const { reader, parser, addCountryDetails, logger, stringifier, writer } = generators;

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

