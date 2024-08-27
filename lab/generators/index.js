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
  process.env.DEBUG ? logger : undefined,
  stringifier,
  process.env.DEBUG ? logger : writer
);