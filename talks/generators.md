---
marp: true
theme: uncover
header: "Les generators: Simplifier le traitement des flux de données"
footer: "Nicolas Remise - Jug Summer Camp 2024 ![width:30px](./assets/logo-summercamp.png)"
paginate: true
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

## 🎁

> L'usage des generators est justifiée !

---

## Découverte des generators

![bg right](./assets/spongebob-rainbow.webp)

---

Released en 2015 ➡️ **ES6**

> Et oui, déjà :cry:

---

#### Mais Kezako ?

> Les generators sont des fonctions qui peuvent être **interrompues** et **reprises** ultérieurement.

#### Quel intérêt?

> Simplification de la gestion des états, lecture séquentielle des données, meilleure gestion des ressources, ...

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

### Protocole `Iterable`

Un objet est `Iterable` s'il implémente une méthode `[Symbol.iterator]` qui renvoie un objet conforme au protocole `Iterator`.

## 🤯

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
    console.log(value); // Hello, World
}
```

---

### Protocole `Iterator`

Un objet est un `Iterator` s'il a une méthode `next` qui retourne un objet avec deux propriétés : `value` et `done`.

---

```javascript
const iterator = iterable[Symbol.iterator]();
console.log(iterator.next()); // { value: 'Hello', done: false }
console.log(iterator.next()); // { value: 'World', done: false }
console.log(iterator.next()); // { value: undefined, done: true }
```

---

### Et les generators dans tout ça?

Les generators implémentent le protocole `Iterator` en fournissant une méthode `next` pour contrôler l'exécution et renvoyer les valeurs successives.

Un generator est aussi un objet `Iterable` car il implémente `[Symbol.iterator]`, ce qui permet son utilisation dans des boucles `for...of` 😉.

---

![bg right](./assets/woodworking.jpg)

## Dans la pratique

---

### Concepts clés 🔑

`yield` : Pause et retourne une valeur.

`next()` : Reprend l'exécution du generator.

`function* ()` : Definit une fonction comme étant une fonction génératrice.

---

![bg left](./assets/opposition.jpg)

**Différences avec des fonctions classiques**

---

Les fonctions classiques s'exécutent entièrement à l'appel.

```js
function myFunction() {
    return [1, 2, 3];
}
```

---

Les generators permettent des pauses et des reprises, rendant possible la _**lazy evaluation**_.

```js
function* generatorFunction() {
    yield 1;
    yield 2;
    yield 3;
}
```

<!-- ---

## Cas pratiques

![width:500px](./assets/akindofmagic.gif)

---

### Chainer des generators avec async/await

Consommer plusieurs generators de manière asynchrone et chainer leurs résultats.

---

```javascript
async function* fetchDataGenerator(urls) {
    for (let url of urls) {
        const response = await fetch(url);
        yield await response.json();
    }
}

const urls = ['https://api.com/data1', 'https://api.com/data2'];
const generator = fetchDataGenerator(urls);

async function consumeGenerator(gen) {
    for await (let data of gen) {
        handleData(data);
    }
}

consumeGenerator(generator);
```

---

- `fetchDataGenerator` est un generator asynchrone qui itère sur un tableau d'URLs, effectue une requête HTTP pour chaque URL et `yield` les données JSON récupérées.

- `consumeGenerator` utilise `for await...of` pour itérer de manière asynchrone sur les résultats produits par `fetchDataGenerator`, traitant chaque ensemble de données de manière séquentielle.

---

### Pagination de données asynchrones

Paginer les résultats provenant d'une API asynchrone.

---

```javascript
async function* paginateResults(url) {
    let page = 1;
    while (true) {
        const response = await fetch(`${url}?page=${page}`);
        const data = await response.json();
        if (data.length === 0) break;
        yield data;
        page++;
    }
}

const url = 'https://api.example.com/data';
const generator = paginateResults(url);

async function handleGenerator(gen) {
    for await (let data of gen) {
        handlePageData(data);
    }
}

handleGenerator(generator);
```

---

- `paginateResults` est un generator asynchrone qui itère sur les pages d'une url tant qu'il y a de la données et `yield` les données de chaque page.

- `handleGenerator` utilise `for await...of` pour itérer de manière asynchrone sur les résultats produits par `paginateResults`, traitant ainsi chaque page de données.

---

### Génération de séquences infinies de données

Générer des séquences de données potentiellement infinies.

---

```javascript
async function* generateRandomNumbers() {
    while (true) {
        await new Promise(resolve => setTimeout(resolve, 1000)); // Attente d'une seconde
        yield Math.random();
    }
}

const generator = generateRandomNumbers();

async function handleGenerator(gen) {
    for await (let number of gen) {
        handleRandom(number);
        if (someCondition) {
            gen.return();
        }
    }
}

handleGenerator(generator);
```

---

- `generateRandomNumbers` est une generator qui itère de manière infini en effectuant pour chaque **loop** un `yield` d'un nombre aléatoire.
- `handleGenerator` consomme les donnnées provenant du generator via `for await ... of`, les traite et si une condition est remplie, coupe le generator infini. -->

---
<!-- _header: "" -->
<!-- _footer: "" -->
![bg](./assets/final.jpg)

---

## Traitement chaîné sur flux de données

---

### Problématique
(Rappel)

Un flux de données arrive en entrée

Plusieurs transformations/actions sont à effectuer sur chacune de ces données

Les transformations/actions doivent être les plus simple à maintenir

---

### Le traitement du flux

```js
async function bootstrap(){
    const dataStream = /* flux de données async */

    const results = [];
    for await (const chunk of dataStream)(
        // TODO SIMPLIFY with generator!
        let data1 = await action1(chunk);
        let data2 = await action2(data1);
        /* ...actionN(dataN-1) */
        results.push(data2)
    )
}
```

---

### Nos generators

```js
async function* action1Generator(input) {
  for await (const chunk of input) {
    yield action1(chunk);
  }
}

async function* action2Generator(input) {
  for await (const chunk of input) {
    yield action2(chunk);
  }
}
```

---

### Le chainage ou la création du pipe

```js
async function bootstrap(){
    const dataStream = /* mon flux de données async (fichiers, bdd, etc...) */
    const results = [];

    const pipe = piper(
        action1Generator, 
        action2Generator 
        /* , ...actionNGenerator*/
    );
    for await (const chunk of pipe(dataStream))(
        results.push(chunk)
    )
}
```

---

## 😎

La logique métier est sorti du process de run

---

### La magical touch 🪄

```js
function piper(...fns) {
    return stream => 
    fns
        .filter(Boolean)
        .reduce(
            (accStream, fn) => fn(accStream), 
            stream
        );
} 
```

---

La fonction `piper` permet de **chainer de manière sequentielle et dans l'ordre fournies** les différentes opérations passées en paramètre.

Il ne reste plus qu'à éxécuter la closure retournée avec le stream. 🎉

---

### Demo time 🧪

---

### Et les tests dans tout ça?

---

```js
it("should ...", async () => {
    const streamData = [/* nos test data*/];
    expect.assertions(streamData.length * 
    /* total d'assertions par lot de data */
    );
    for await (const chunk of action1Generator(streamData.values())){
        expect(/*...*/).toBe(/*...*/);
    }
});
```

---

## Conclusion

- 👍 séparation des logiques
- 👍 briques logiques **plug'n play** 🧩
- 👍 performances équivalentes à l'utilisation de `Stream` classiques (voir mieux selon les runs)
- 🥼 🧪 utilisation de features innovantes 😉
  
---
<!-- header: "" -->
<!-- footer: "" -->

![bg](./assets/bg-jug.png)

## Fin

Merci de votre écoute!

Des questions?
