import { readFile } from 'node:fs/promises';
import path from 'node:path';

let countries;
/**
 * Load countries if not present yet
 * Find country associated to id passed in params
 * 
 * @param {*} id 
 * @returns identified country or default value
 */
async function getCountryById(id) {
  if (!countries) {
    countries = JSON.parse(await readFile(path.join(import.meta.dirname, '../../assets/countries.json')));
  }

  return countries.find(i => i.id === id)
    || { name: "Not found" };
}

/**
 * Replace country id by country data 
 * 
 * @param {*} input - stream data
 */
export default async function* (input) {
  for await (const chunk of input) {
    const { country: countryId, ...data } = chunk;
    const country = await getCountryById(countryId);
    yield { ...data, country };
  }
}
