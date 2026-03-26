<img src=https://github.com/DrCMWither/qsisp/blob/main/docs/logo/qsisp.png width=300 />

# qƨisp

[DE](README_DE.md)|[EN](README_EN.md)|[FR](README_FR.md)|[ZH](README.md)

### Programmiersprache: Erschaffe deine eigene!

---

## Was ist qƨisp?

**qƨisp** (falls deine Schriftart es nicht unterstützt, nenne es *qU+01A8isp*. Und nein, *qsisp* ist nicht der offizielle Name) ist ein lokalitätsbasiertes, mehrsprachiges Lisp-Dialekt.

Es passt sich an deine Systemsprache an:

* Schlüsselwörter;
* Klammern;
* Zeichenketten;
* Kommentare;
* Deinen mentalen Zustand!

## Eigenschaften

### Lokalitätsbasierte Syntax

Schreibe Code in deiner eigenen Sprache! Du musst weder qƨisp noch Lisp-Schlüsselwörter lernen: Vielleicht musst du nicht einmal Englisch können. qƨisp passt sich dir an.

## Nebenwirkungen

* Du wirst ASCII-Klammern nie wieder normal sehen;
* Mentaler Zusammenbruch;
* IDE: **Ich kündige.**

## Beispiel

```lisp
„beginn
  „definieren x ‚10‘
  „wenn ‚<= x 10‘
    „drucken‚x‘
““““
```

Versuche, das in der französischen Version neu zu schreiben.

Oder mische alle Sprachen.

Wir raten dringend davon ab.


## Lokales Ausführen / Entwickeln

1. Klonen Sie dieses Repository:

```bash
git clone https://github.com/DrCMWither/qsisp.git
cd qsisp
```

2. Aktualisieren Sie Ihre lokale Umgebung. Dieses Projekt erfordert `Rust >= 1.75` sowie `cargo`.

3. Führen Sie das Projekt direkt aus oder erstellen Sie eine Release-Version:

```bash
cargo run -- example/test.qs
cargo build --release
```

## Auszeichnungen

* Preis für interkulturellen Programmierhorror
* Sonderpreis für psychologische Angriffe auf Reader-Ebene
* Unverdaulichster Lisp-Dialekt 2026

## Zukunftspläne

* Echte RTL-Analyse
* Mehrsprachiger AST
* IDE-Plugin (praktisch unbenutzbar)
* Formale Semantik (falls noch jemand lebt)

*Und übrigens — der Name dieser Sprache ist kein Palindrom.*