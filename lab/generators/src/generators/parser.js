/**
 * JSON parse raw data
 * 
 * @param {*} input - stream data
 */
export default async function* parser(input) {
  for await (const chunk of input) {
    yield JSON.parse(chunk);
  }
}