---
marp: true
theme: uncover
header: "Les generators: Simplifier le traitement des flux de données - Nicolas Remise"
footer: "![width:100px](./assets/codeurs-en-seine-logo.png)"
paginate: true
style: |
  h3>strong {
    color: darkgrey !important;
  }
---
<!-- _header: "" -->
<!-- _paginate: false -->

# Les generators

Simplifier le traitement des flux de données

![height:100px](./assets/js.png)

![bg left](./assets/stream.jpg)

---

### **N**icolas **Remise**

![bg right](./assets/me.jpg)

Tech Lead JS/TS/Data

![height:100px](./assets/darva.png)

---

 ![height:150px](./assets/ts.png)  ![height:150px](./assets/js.png)
 ![height:150px](./assets/rust.png)  
 ![height:150px](./assets/mongodb.png)

---

## 🤷‍♂️

> Mais pourquoi gérer des flux de données en `JS`?

---

## Le contexte

Grosse volumetrie de documents journaliers au format `JSON`

Pas d'ETL pour effectuer les transformations sur les données pour les rendre plus _"interprétables"_

---
![bg left](./assets/js-big.png)

## Le choix du langage

---

## 👍

- Integration native du JSON
- Langage polyvalent et accessible

---

## 👎

- Complexité de la gestion des gros flux de données asynchrones
- Importance de maintenir un code lisible et facile à maintenir (**n** transformations, rarement très grosses unitairement)
- API `Stream` native vieillissante

---

## Découverte des generators

![bg right](./assets/spongebob-rainbow.webp)

---

Released en 2015 ➡️ **ES6**

> Et oui, déjà :cry:

---

### Mais Kezako ?

> Les generators sont des fonctions qui peuvent être **interrompues** et **reprises** ultérieurement.

---

### Un exemple simple pour commencer

```javascript
function* feelingGenerator() {
    yield "I";
    yield "love";
    yield "coding";
}

const g = feelingGenerator();
console.log(g.next().value); // "I"
console.log(g.next().value); // "love"
console.log(g.next().value); // "coding"
console.log(g.next().value); // undefined
```

---
![bg left](./assets/books.jpg)

## Protocoles `Iterator` et `Iterable`

---

Un generator est un objet `Iterable` car il implémente `[Symbol.iterator]`, ce qui permet son utilisation dans des boucles `for...of` 😉.

---

Les generators implémentent le protocole `Iterator` en fournissant une méthode `next` par le biais de la méthode `next()` fournit par `[Symbol.iterator]`.

---

```javascript
const iterable = {
    [Symbol.iterator]: function() {
        let step = 0;
        return {
            next: function() {
                step++;
                if (step === 1) {
                    return { value: 'Hello', done: false };
                } else if (step === 2) {
                    return { value: 'World', done: false };
                }
                return { value: undefined, done: true };
            }
        };
    }
};

for (const value of iterable) {
    console.log(value); // "Hello" puis "World"
}
```

---

![bg right](./assets/woodworking.jpg)

## Dans la pratique

---

### Keywords 🔑

`yield` : Pause et retourne une valeur.

`next()` : Reprend l'exécution du generator.

`function* ()` : Definit une fonction comme étant une fonction génératrice.

---

### Traitement chaîné sur flux de données

---

### Problématique

Un flux de données arrive en entrée

Plusieurs transformations/actions sont à effectuer sur chacune de ces données

Les transformations/actions doivent être les plus simple à maintenir

---

```javascript
async function toModernize() {
  const file = await open(
    path.join(import.meta.dirname, "myfile.txt")
    );

  const countries = JSON.parse(await readFile(
    path.join(import.meta.dirname, 'countries.json')
    ));

  for await (const line of file.readLines()) {
    const { country: countryId, ...data } = JSON.parse(line);
    const enrichedData = { ...data, country: countries.find(i => i.id === countryId) };

    const country = enrichedData.country.name;
    const sport = enrichedData.sport;
    const name = `${enrichedData.firstName} ${enrichedData.lastName.toUpperCase()}`;
    const stringified = `[${country}] [${sport}] ${names}`;

    console.log(stringified);
  }
}
```

---

## Les Transformations

1. parsing
2. ajout des details des pays
3. construction de la sortie
4. logging

---

### To generator's world

```js
/**
 * Read input file data
 */
export default async function* reader() {
  const file = await fs.open(
    path.join(import.meta.dirname, "myfile.txt")
    );
  yield* file.readLines();
}
```

```js
for await (const data of reader()){}
```

---

### Transformation

```js
const { country: countryId, ...data } = JSON.parse(line);
```

to

```js
async function* parser(input) {
  for await (const chunk of input) {
    yield JSON.parse(chunk);
  }
}
```

---

```js
for await (const data of parser(reader())){}
```

🤔

---

2 problèmes:

- Les actions sont moins lisibles à cause de la consommation du generator précédent
- La consommation du generator globale est lourde pour appeler le generator `n` avec la valeur du generator `n-1`

---

### toConsumerGenerator

```js
/**
 * Build a generator that apply `fn` passed 
 * on each data from input that will received
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
```

---

```js
function parserFn(data){
  return JSON.parse(data)
}

export const parserGenerator = toConsumerGenerator(parserFn)
```

- lisible
- maintenable
- testable

🤩

---

<!-- _header: "" -->
<!-- _footer: "" -->

### chain

```js
/**
 * Chain list of generators and return result
 * @param  fns - ordered array of operations
 * @returns result data (array or unique data)
 */
export async function chain(...fns) {
  const dataPipe = fns.filter(Boolean).reduce(
    (accumulatedData, fn) => fn(accumulatedData),
    undefined
  );
  let results = [];
  for await (const data of dataPipe) {
    results.push(data);
  }

  return results.length === 1 ? results[0] : results;
}
```

---

```js
await chain(reader, parser, ...);
```

🚀

---

## Ce qui donne ... 🎉

---

```js
const parser = toConsumerGenerator(JSON.parse);
```

---

```js
function loggerFn (data) {
  console.log(data);
  return data;
}
const logger = toConsumerGenerator(loggerFn);
```

---

```js
function addCountryDetailsFn (data) {
  const { country: countryId, ...rest } = data;
  const country = await getCountryById(countryId);
  return { ...rest, country }
};

const addCountryDetails = toConsumerGenerator(addCountryDetailsFn);
```

---

```js
function stringifierFn (data) {
  const { country, sport, lastName, firstName } = data;
  const fullName = `${firstName} ${lastName.toUpperCase()}`;
  return `[${country.name}] [${sport}] ${fullName}`;
}

const stringifier = toConsumerGenerator(stringifierFn);
```

---

```js
await chain(
  reader,
  parser,
  addCountryDetails,
  stringifier,
  logger
);
```

---

## Conclusion

- 👍 séparation des logiques
- 👍 briques logiques **plug'n play** 🧩
- 👍 performances équivalentes à l'utilisation de `Stream` classiques (voir mieux selon les runs)
- 🥼 🧪 utilisation de features innovantes 😉
- 🤩 Functional Programming
  
---
<!-- header: "" -->
<!-- footer: "" -->

## ![img](./assets/codeurs-en-seine-logo.png)

**Merci de votre écoute!**

[📁 nesimer/talks](https://github.com/nesimer/talks)
