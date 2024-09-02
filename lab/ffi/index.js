import { join } from "path";
import tokenize from "./src/tokenize.js";
import rustTokenize from "./src/rustTokenize.js";
import { tokenize as napiTokenize } from "./src/napi-tokenize/index.js";

const prefixAssetsPath = join(import.meta.dirname, "./assets");

async function bootstrap() {
  const jsLabel = "js";
  const rustLabel = "rust";
  const napiLabel = "napi";

  const inputPath = join(prefixAssetsPath, "large-file.json");
  const getOutputPath = (label) => join(prefixAssetsPath, `large-file-${label}.json`);

  const jsStartDate = Date.now()
  await tokenize(inputPath, getOutputPath(jsLabel));
  const jsDuration = Date.now() - jsStartDate;

  const rustStartDate = Date.now()
  rustTokenize(inputPath, getOutputPath(rustLabel));
  const rustDuration = Date.now() - rustStartDate;

  const napiStartDate = Date.now()
  napiTokenize(inputPath, getOutputPath(napiLabel));
  const napiDuration = Date.now() - napiStartDate;

  console.table([
    {
      type: jsLabel, durationInMs: jsDuration
    },
    {
      type: rustLabel, durationInMs: rustDuration
    },
    {
      type: napiLabel, durationInMs: napiDuration
    },
  ])
}

await bootstrap();