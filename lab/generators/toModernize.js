import { readFile, open } from "node:fs/promises";
import path from "node:path";

async function bootstrap() {
  const file = await open(path.join(import.meta.dirname, "./assets/athletes.txt"));

  const countries = JSON.parse(await readFile(path.join(import.meta.dirname, './assets/countries.json')))

  for await (const line of file.readLines()) {
    const { country: countryId, ...data } = JSON.parse(line);
    const enrichedData = { ...data, country: countries.find(i => i.id === countryId) };

    const stringified = `[${enrichedData.country.name}] [${enrichedData.sport}] ${enrichedData.firstName} ${enrichedData.lastName.toUpperCase()}`;

    console.log(stringified);
  }
}

await bootstrap();