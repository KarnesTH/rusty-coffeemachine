# Kapitel 2: Grundfunktionalität

> 🎯 **Ziel**: Implementierung der Kernfunktionalität unserer Kaffeemaschine

## Überblick

In diesem Kapitel werden wir:
1. Die Verwaltung der Kaffeerezepte implementieren
2. Ein System für die Zutatenverwaltung entwickeln
3. Eine robuste Fehlerbehandlung einbauen

## Was wir lernen werden

### Rezeptverwaltung
- Definition verschiedener Kaffeerezepte
- Implementierung der Rezeptauswahl
- Verwaltung der Rezeptdatenbank

### Zutatenverwaltung
- Kontrolle der Zutatenmengen
- Verbrauchsberechnung
- Füllstandsüberwachung

### Fehlerbehandlung
- Behandlung von Ressourcenmangel
- Validierung von Benutzereingaben
- Sichere Fehlerausgabe

## Kapitelstruktur

```mermaid
graph LR
    A[Rezeptverwaltung] --> B[Zutatenverwaltung]
    B --> C[Fehlerbehandlung]
```

### Unterkapitel
1. [Rezeptverwaltung](kapitel-2/rezepte.md)
   - Implementierung der Rezeptlogik
   - Verwaltung der Rezeptdatenbank
   - Rezeptauswahl und -validierung

2. [Zutatenverwaltung](kapitel-2/zutaten.md)
   - Verwaltung der Zutatenbestände
   - Verbrauchsberechnung
   - Füllstandsüberwachung

3. [Fehlerbehandlung](kapitel-2/fehler.md)
   - Implementierung der Fehlertypen
   - Fehlerbehandlungsstrategien
   - Benutzerfreundliche Fehlermeldungen

## Was wir am Ende haben werden

Nach diesem Kapitel verfügt unsere Kaffeemaschine über:
- Ein vollständiges Rezeptverwaltungssystem
- Eine zuverlässige Zutatenverwaltung
- Eine robuste Fehlerbehandlung

> 💡 **Pro-Tipp**: Die Grundfunktionalität ist das Herz unserer Anwendung. Nimm dir Zeit, sie sorgfältig zu implementieren!

---

Bereit für die Details? Lass uns mit der [Rezeptverwaltung](kapitel-2/rezepte.md) beginnen!
