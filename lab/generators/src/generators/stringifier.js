import { toConsumerGenerator } from "../tools.js";

function stringifier(data) {
  const { country, sport, lastName, firstName } = data;
  return `[${country.name}] [${sport}] ${firstName} ${lastName.toUpperCase()}`
}

/**
 * Stringify data to showable formatting
 * 
 * @param {*} input - stream data
 */
export default toConsumerGenerator(stringifier);