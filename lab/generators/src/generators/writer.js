import { appendFile, access, rm } from "fs/promises";
import { toConsumerGenerator } from "../tools.js";

const output = "JO.output.txt";

/*
 * It checks if out file exits already or not
 * If it exists, remove it
 */
try {
  await access(output);
  await rm(output);
} catch (_e) { }

/**
 * Build generator that write each data received in output file
 * and return data
 */
async function writer(data) {
  await appendFile(output, data + "\n");
  return data;
}

export default toConsumerGenerator(writer);