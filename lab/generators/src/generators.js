import { appendFile, access, rm, open } from "fs/promises";
import path from "path";
import { toConsumerGenerator, getCountryById } from "./tools.js";

const output = "JO.output.txt";

/**
 * Read input file data
 */
async function* reader() {
  const file = await open(path.join(import.meta.dirname, "../assets/athletes.txt"));
  yield* file.readLines();
}

/**
 * Replace country id by country data 
 * 
 * @param {*} data
 */
async function addCountryDetails(data) {
  const { country: countryId, ...rest } = data;
  const country = await getCountryById(countryId);
  return { ...rest, country }
}

/**
 * Build generator that write each data received in output file
 * and return data
 */
async function writer(data) {
  await appendFile(output, data + "\n");
  return data;
}

/**
 * Stringify with defined format data passed
 */
function stringifier(data) {
  const { country, sport, lastName, firstName } = data;
  return `[${country.name}] [${sport}] ${firstName} ${lastName.toUpperCase()}`
}

/**
 * JSON parse raw data of input
 * 
 * @param {*} input - stream data
 */
export const parser = async function* (input) {
  for await (const chunk of input) {
    yield JSON.parse(chunk);
  }
}

/**
 * Log data passed
 */
function logger(data) {
  console.log(data);
  return data;
}

export default {
  addCountryDetails: toConsumerGenerator(addCountryDetails),
  logger: toConsumerGenerator(logger),
  parser,
  reader,
  stringifier: toConsumerGenerator(stringifier),
  writer: toConsumerGenerator(writer)
}

/*
 * It checks if out file exits already or not
 * If it exists, remove it
 */
try {
  await access(output);
  await rm(output);
} catch (_e) { }
