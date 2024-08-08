/**
 * Log each data passed from  input stream
 * 
 * @param {*} input - stream data
 */
export default async function* logger(input) {
  for await (const chunk of input) {
    console.log(chunk);
    yield chunk;
  }
}