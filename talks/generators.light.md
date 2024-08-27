---
marp: true
theme: uncover
header: "Les generators: Simplifier le traitement des flux de données"
footer: "Nicolas Remise - Jug Summer Camp 2024 ![width:30px](./assets/logo-summercamp.png)"
paginate: true
style: |
  h3>strong {
    color: darkgrey !important;
  }
---
<!-- _header: "" -->
<!-- _paginate: false -->

# Les generatorss

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

## Demo time 🧪

---

### Traitement chaîné sur flux de données

---

### Problématique

Un flux de données arrive en entrée

Plusieurs transformations/actions sont à effectuer sur chacune de ces données

Les transformations/actions doivent être les plus simple à maintenir

---

### Et les tests dans tout ça?

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