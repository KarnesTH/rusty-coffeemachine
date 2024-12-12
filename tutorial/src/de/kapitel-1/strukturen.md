# Strukturen und Datentypen

> 🎯 **Ziel**: Definition der grundlegenden Datenstrukturen für unsere Kaffeemaschine

## Container-Strukturen

Zuerst implementieren wir die Behälter für Zutaten und Abfälle in `containers.rs`:

```rust
// Behälter für alle Zutaten
#[derive(Clone, Debug)]
pub struct IngredientsContainer {
    pub water: f32,   // Wasser in ml
    pub coffee: f32,  // Kaffeebohnen in g
    pub milk: f32,    // Milch in ml
    pub sugar: f32,   // Zucker in g
    pub cacao: f32,   // Kakao in g
}

// Behälter für Abfälle
#[derive(Debug)]
pub struct GarbageContainer {
    pub coffee_grounds: f32,  // Kaffeesatz in g
}
```

> 💡 **Erklärung**:
> - `#[derive(Debug)]` ermöglicht das Debugging der Strukturen
> - `#[derive(Clone)]` für IngredientsContainer erlaubt das Klonen von Rezepten
> - Wir verwenden `f32` für präzise Mengenangaben

## Rezept-Struktur

In `reciepes.rs` definieren wir die Struktur für unsere Kaffeerezepte:

```rust
use crate::containers::IngredientsContainer;

#[derive(Clone, Debug)]
pub struct Reciepes {
    pub name: String,
    pub ingredients: IngredientsContainer,
}
```

## Kaffeemaschinen-Struktur

Die Hauptstruktur in `coffeemachine.rs`:

```rust
use crate::containers::{GarbageContainer, IngredientsContainer};
use crate::reciepes::Reciepes;

#[derive(Debug)]
pub struct CoffeeMachine {
    pub ingredients_container: IngredientsContainer,
    pub garbage_container: GarbageContainer,
    pub reciepes: Vec<Reciepes>,
}
```

## Überprüfung der Strukturen

Lass uns testen, ob alles kompiliert:

```bash
cargo check
```

Wenn keine Fehler auftreten, haben wir unsere Grundstrukturen erfolgreich definiert!

## Warum diese Strukturierung?

1. **Modularität**:
   - Jede Struktur hat eine klare, einzelne Verantwortung
   - Änderungen an einer Komponente beeinflussen nicht direkt andere
   - Neue Funktionen können einfach hinzugefügt werden

2. **Datenkapselung**:
   - Klare Trennung zwischen Zutaten und Abfällen
   - Rezepte sind unabhängig von der Maschinenverwaltung
   - Jeder Container verwaltet seine eigenen Daten

3. **Erweiterbarkeit**:
   - Neue Zutaten können einfach im `IngredientsContainer` ergänzt werden
   - Zusätzliche Rezepte lassen sich problemlos hinzufügen
   - Neue Funktionalitäten können modular implementiert werden

4. **Wartbarkeit**:
   - Übersichtliche Codestruktur
   - Einfaches Debugging durch `Debug`-Trait
   - Gute Testbarkeit der einzelnen Komponenten

## Nächste Schritte

✅ Wir haben:
- Grundlegende Datenstrukturen definiert
- Eine modulare Basis geschaffen
- Die Voraussetzungen für Implementierungen erstellt

👉 Im nächsten Abschnitt werden wir die [ersten Implementierungen](implementierung.md) für diese Strukturen entwickeln.

> 💡 **Pro-Tipp**: Gut durchdachte Strukturen sind das Fundament für sauberen und wartbaren Code. Nimm dir Zeit, über die Beziehungen zwischen den Komponenten nachzudenken.
