import { toConsumerGenerator } from "../tools.js";

/**
 * JSON parse raw data
 * 
 * @param {*} input - stream data
 */
export default toConsumerGenerator(JSON.parse);