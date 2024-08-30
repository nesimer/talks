import { toConsumerGenerator } from "../tools.js";

/**
 * JSON parse raw data
 * 
 * @param {*} input - stream data
 */
export default toConsumerGenerator(JSON.parse);

// OR

export const parser = async function* (input) {
  for await (const chunk of input) {
    yield JSON.parse(chunk);
  }
}