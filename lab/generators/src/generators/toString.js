/**
 * Stringify data to showable formatting
 * 
 * @param {*} input - stream data
 */
export default async function* toString(input) {
  for await (const { country, sport, lastName, firstName } of input) {
    yield `[${country.name}] [${sport}] ${firstName} ${lastName.toUpperCase()}`;
  }
}