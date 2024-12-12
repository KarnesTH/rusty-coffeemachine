# Erste Implementierungen

> 🎯 **Ziel**: Grundlegende Funktionalität für unsere Strukturen implementieren

## Implementierung der Rezepte

Zuerst erweitern wir unsere Rezeptstruktur in `reciepes.rs`:

```rust
impl Reciepes {
    /// Erstellt ein neues Rezept
    pub fn new(name: String, ingredients: IngredientsContainer) -> Result<Self, std::io::Error> {
        Ok(Reciepes { name, ingredients })
    }

    /// Erstellt eine Liste von Standard-Rezepten
    pub fn get_reciepes() -> Result<Vec<Reciepes>, std::io::Error> {
        let init_reciepes = vec![
            Reciepes::new(
                "Espresso".to_string(),
                IngredientsContainer {
                    water: 30.0,
                    coffee: 30.0,
                    milk: 0.0,
                    sugar: 0.0,
                    cacao: 0.0,
                },
            )?,
            // Weitere Rezepte folgen...
        ];
        Ok(init_reciepes)
    }
}
```

## Implementierung der Kaffeemaschine

In `coffeemachine.rs` implementieren wir die Basisfunktionalität:

```rust
impl CoffeeMachine {
    /// Erstellt eine neue Kaffeemaschine
    pub fn new() -> Result<Self, std::io::Error> {
        let machine = CoffeeMachine {
            ingredients_container: IngredientsContainer {
                water: 100.0,
                coffee: 100.0,
                milk: 100.0,
                sugar: 100.0,
                cacao: 100.0,
            },
            garbage_container: GarbageContainer {
                coffee_grounds: 0.0,
            },
            reciepes: Reciepes::get_reciepes()?,
        };

        Ok(machine)
    }

    /// Prüft ob genügend Zutaten vorhanden sind
    fn check_ingredients(
        &self,
        ingredients: &IngredientsContainer,
    ) -> Result<bool, std::io::Error> {
        let result = self.ingredients_container.water >= ingredients.water
            && self.ingredients_container.coffee >= ingredients.coffee
            && self.ingredients_container.milk >= ingredients.milk
            && self.ingredients_container.sugar >= ingredients.sugar
            && self.ingredients_container.cacao >= ingredients.cacao;

        Ok(result)
    }
}
```

## Erste Tests

Fügen wir einige Tests hinzu:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coffee_machine_new() {
        let machine = CoffeeMachine::new().unwrap();
        assert_eq!(machine.ingredients_container.water, 100.0);
        assert_eq!(machine.garbage_container.coffee_grounds, 0.0);
    }

    #[test]
    fn test_check_ingredients() {
        let machine = CoffeeMachine::new().unwrap();
        let test_ingredients = IngredientsContainer {
            water: 30.0,
            coffee: 30.0,
            milk: 0.0,
            sugar: 0.0,
            cacao: 0.0,
        };
        assert!(machine.check_ingredients(&test_ingredients).unwrap());
    }
}
```

## Warum Unit-Tests?

Bevor wir unsere Implementierung überprüfen, ist es wichtig zu verstehen, warum wir Tests schreiben:

1. **Funktionalitätssicherung**:
   - Stellen sicher, dass unsere Kaffeemaschine wie erwartet funktioniert
   - Fangen Fehler früh im Entwicklungsprozess ab
   - Geben uns Vertrauen in unseren Code

2. **Entwicklungsunterstützung**:
   - Dienen als "lebende" Dokumentation
   - Erleichtern spätere Änderungen am Code
   - Helfen beim Verstehen des erwarteten Verhaltens

> 💡 **Pro-Tipp**: Tests sind nicht nur zum Finden von Fehlern da - sie helfen uns auch, besseren Code zu schreiben!

## Überprüfung der Implementierung

```bash
# Führe die Tests aus
cargo test

# Überprüfe die Dokumentation
cargo doc --open
```

## Was wir erreicht haben

1. **Grundfunktionalität**:
   - Erstellung neuer Kaffeemaschinen
   - Verwaltung von Rezepten
   - Überprüfung von Zutaten

2. **Testabdeckung**:
   - Grundlegende Tests implementiert
   - Funktionalität verifiziert
   - Basis für weitere Tests geschaffen

3. **Dokumentation**:
   - Dokumentierte Funktionen
   - Klare Beschreibungen
   - Beispiele in Docs

## Nächste Schritte

✅ Wir haben:
- Basisfunktionalität implementiert
- Tests geschrieben
- Dokumentation erstellt

👉 Im nächsten Kapitel werden wir uns mit der [Grundfunktionalität](../kapitel-2.md) beschäftigen.

> 💡 **Pro-Tipp**: Schreibe Tests parallel zur Implementierung. Das hilft dir, Fehler früh zu erkennen und sicherzustellen, dass deine Funktionen wie erwartet arbeiten.
