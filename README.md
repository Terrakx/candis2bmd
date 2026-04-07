# Candis2BMD - ER-Import Konverter

Ein Desktop-Tool (Rust/Tauri) zur Konvertierung von Candis-Daten in das BMD-Format (ER-Import).

## Features
- Automatischer Import von Candis CSV/Export Dateien
- Konvertierung in das BMD-kompatible Format
- Einfache Benutzeroberfläche für schnelle Abwicklung

## Voraussetzungen
Stelle sicher, dass folgende Software auf deinem System installiert ist:
- [Node.js](https://nodejs.org/) (für das Frontend)
- [Rust](https://www.rust-lang.org/) (für den Tauri Core)

## Installation & Start (Development)

1. Abhängigkeiten installieren:
   ```powershell
   npm install
   ```

2. Entwicklungsmodus starten:
   ```powershell
   npm run tauri dev
   ```

## Build (Production)
Um eine installierbare `.exe` Datei zu erstellen:
```powershell
npm run tauri build
```

## Lizenz
Dieses Projekt steht unter der **MIT-Lizenz**. Siehe [LICENSE](./LICENSE) für Details.
