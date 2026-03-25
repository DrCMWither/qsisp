# qƨisp

[DE](README_DE.md)|[EN](README_EN.md)|[FR](README_FR.md)|[ZH](README.md)

### Langage de programmation : créez le vôtre !

## Qu’est-ce que qƨisp ?

**qƨisp** (si votre police ne le supporte pas, appelez-le *qU+01A8isp*. Et non, *qsisp* n’est pas le nom officiel) est un dialecte Lisp multilingue basé sur la locale.

Il s’adapte à la langue de votre système :

* Mots-clés;
* Parenthèses;
* Chaînes de caractères;
* Commentaires;
* Votre état mental!


## Caractéristiques

### Syntaxe basée sur la locale

Écrivez du code dans votre propre langue! Vous n’avez pas besoin d’apprendre qƨisp, ni même les mots-clés Lisp: Vous n’avez peut-être même pas besoin de connaître l’anglais. qƨisp s’adapte à vous.


## Effets secondaires

* Vous ne regarderez plus jamais les parenthèses ASCII de la même manière;
* Effondrement mental;
* IDE : **je démissionne**


## Exemple

```lisp
« début
  « définir x ‹ 10 ›
  « si  ‹ <= x 10 ›
    « imprimer ‹ x ›
 » » » »
```

Essayez de le réécrire en version allemande.

Ou mélangez toutes les langues.

Nous vous déconseillons fortement de le faire.


## Exécution locale / Développement

1. Clonez le dépôt :

```bash
git clone https://github.com/DrCMWither/qsisp
cd qsisp
```

2. Assurez-vous que votre environnement local est à jour. Ce projet nécessite `Rust >= 1.75` ainsi que `cargo`.

3. Exécutez directement ou construisez une version release :

```bash
cargo run -- test.qs
cargo build --release
```

## Récompenses

* Prix d’horreur des langages interculturels
* Mention spéciale : attaque psychologique au niveau du lecteur
* Dialecte Lisp le plus indigeste de 2026

## Feuille de route

* Analyse RTL correcte;
* AST multilingue;
* Plugin IDE (pratiquement inutilisable);
* Sémantique formelle (si quelqu’un survit).


*Au fait — le nom de ce langage n’est pas un palindrome.*

