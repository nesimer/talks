import { toConsumerGenerator } from "../tools.js";

/**
 * Log data passed
 * 
 * @param {*} data
 */
function logger(data) {
  console.log(data);
  return data;
}

export default toConsumerGenerator(logger)