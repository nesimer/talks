import { chain } from "./src/tools.js";
import generators from "./src/generators.js";

const { reader, parser, addCountryDetails, logger, stringifier, writer } = generators;

await chain(
  reader,
  parser,
  addCountryDetails,
  // logger,
  stringifier,
  process.env.DEBUG ? logger : writer
);

//OR without tools

async function bootstrap() {
  for await (const chunk of stringifier(logger(addCountryDetails(parser(reader()))))) {
    console.log(chunk)
  }
}

// await bootstrap();

