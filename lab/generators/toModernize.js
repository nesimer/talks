import readline from "node:readline";
import fs from "node:fs";
import { readFile } from "node:fs/promises";
import path from "node:path";

async function bootstrap() {
  const fileStream = fs.createReadStream(path.join(import.meta.dirname, "./assets/athletes.txt"));
  const stream = readline.createInterface({ input: fileStream, crlfDelay: Infinity, output: process.stdout });

  const countries = JSON.parse(await readFile(path.join(import.meta.dirname, './assets/countries.json')))

  stream.on('line', (l) => {
    const { country: countryId, ...data } = JSON.parse(l);

    const enrichedData = { ...data, country: countries.find(i => i.id === countryId) }

    const stringified = `[${enrichedData.country.name}] [${enrichedData.sport}] ${enrichedData.firstName} ${enrichedData.lastName.toUpperCase()}`;

    console.log(stringified);
  })
}

await bootstrap();