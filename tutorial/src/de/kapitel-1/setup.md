# Projekt aufsetzen

> 🎯 **Ziel**: Ein neues Rust-Projekt erstellen und die grundlegende Struktur einrichten

## Neues Projekt erstellen

Zuerst erstellen wir ein neues Rust-Projekt mit Cargo:

```bash
cargo new rusty-coffeemachine
cd rusty-coffeemachine
```

## Projektstruktur

Wir werden folgende Dateistruktur erstellen:

```
rusty-coffeemachine/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── lib.rs
    ├── coffeemachine.rs
    ├── containers.rs
    └── reciepes.rs
```

## Cargo.toml konfigurieren

Öffne `Cargo.toml` und füge folgende Konfiguration hinzu:

```toml
[package]
name = "rusty-coffeemachine"
version = "0.1.0"
edition = "2021"
authors = ["Dein Name <deine@email.com>"]
description = "Eine virtuelle Kaffeemaschine in Rust"

[dependencies]
# Hier werden später ggf. Abhängigkeiten hinzugefügt
```

## Modulstruktur erstellen

1. Erstelle `lib.rs` mit den Modul-Deklarationen:

```rust
pub mod coffeemachine;
pub mod containers;
pub mod reciepes;

// Re-exportiere wichtige Strukturen
pub use coffeemachine::CoffeeMachine;
pub use containers::{GarbageContainer, IngredientsContainer};
pub use reciepes::Reciepes;
```

2. Erstelle eine minimale `main.rs`:

```rust
use rusty_coffeemachine::CoffeeMachine;

fn main() -> Result<(), std::io::Error> {
    let mut machine = CoffeeMachine::new()?;
    machine.run()?;

    Ok(())
}
```

3. Erstelle leere Moduldateien:
   - `coffeemachine.rs`
   - `containers.rs`
   - `reciepes.rs`

## Git-Repository einrichten

1. Initialisiere Git:
```bash
git init
```

2. Erstelle `.gitignore`:
```
/target
Cargo.lock
**/*.rs.bk
```

3. Erster Commit:
```bash
git add .
git commit -m "Initial commit: Basic project structure"
```

## Projekt testen

Überprüfe, ob das Projekt kompiliert:

```bash
cargo check
```

> ⚠️ An dieser Stelle wird das Projekt noch nicht kompilieren, da wir die Strukturen noch nicht implementiert haben.

## Nächste Schritte

✅ Wir haben:
- Ein neues Rust-Projekt erstellt
- Die Projektstruktur aufgesetzt
- Git-Versionierung eingerichtet

👉 Als Nächstes werden wir die [grundlegenden Datenstrukturen](strukturen.md) implementieren.

> 💡 **Pro-Tipp**: Mache regelmäßige Git-Commits während der Entwicklung. Das hilft dir, Änderungen nachzuverfolgen und bei Bedarf zurückzurollen.
