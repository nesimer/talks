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

## Conclusion

- 👍 séparation des logiques
- 👍 briques logiques **plug'n play** 🧩
- 👍 performances équivalentes à l'utilisation de `Stream` classiques (voir mieux selon les runs)
- 🥼 🧪 utilisation de features innovantes 😉
  
---
<!-- header: "" -->
<!-- footer: "" -->

![bg](./assets/bg-jug.png)

## ![img](./assets/JugSummerCamp%202024%20-%20Les%20generators.png)

**Merci de votre écoute!**
Des questions?

[📁 Ressources](https://github.com/nesimer/talks)
