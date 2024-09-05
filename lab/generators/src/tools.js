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
export async function getCountryById(id) {
  if (!countries) {
    countries = JSON.parse(await readFile(path.join(import.meta.dirname, '../assets/countries.json')));
  }

  return countries.find(i => i.id === id)
    || { name: "Not found" };
}

/**
 * Chain list of generators and return result
 * 
 * @param  {...(GeneratorFunction|AsyncGeneratorFunction)} fns - ordered array of operations
 * @returns result data (array or unique data)
 */
export async function chain(...fns) {
  const dataPipe = fns.filter(Boolean).reduce(
    (accumulatedData, fn) => fn(accumulatedData),
    undefined
  );

  let results = [];
  for await (const result of dataPipe) {
    results.push(result);
  }

  return results.length === 1 ? results[0] : results;
}

/**
 * Build a generator that apply fn passed on each data from input that will received
 * 
 * @param {Function} fn 
 * @returns {AsyncGenerator} generator
 */
export function toConsumerGenerator(fn) {
  return async function* (input) {
    for await (const chunk of input) {
      yield fn(chunk);
    }
  }
}