# BMD Import Info (CSV)

Diese Dokumentation fasst das Wissen zur Erstellung der BMD-kompatiblen CSV-Importdatei zusammen, wie sie in diesem Projekt implementiert ist.

## Allgemeine Formatvorgaben

- **Dateiformat:** CSV (Semikolon-separiert `;`)
- **Encoding:** `Windows-1252` (ANSI), nicht UTF-8!
- **Zahlenformat:** Europäisch (Komma als Dezimaltrenner, zwei Nachkommastellen, z.B. `123,45`).
- **Datumsformat:** `TT.MM.JJJJ` (z.B. `10.04.2026`).

## CSV-Struktur (Header)

Die Datei enthält folgende Spalten in dieser exakten Reihenfolge:

| Spalte | Feldname | Beschreibung / Beispiel |
| :--- | :--- | :--- |
| 1 | `satzart` | Immer `0` für Buchungssätze. |
| 2 | `konto` | Personenkonto (Kreditor/Debitor), z.B. `33001`. |
| 3 | `gkonto` | Gegenkonto (Sachkonto), z.B. `5000`. |
| 4 | `buchdatum` | Datum der Buchung (`TT.MM.JJJJ`). |
| 5 | `belegdatum` | Datum auf dem Beleg (`TT.MM.JJJJ`). |
| 6 | `belegnr` | Interne laufende Nummer im BMD. |
| 7 | **`betrag`** | Bruttobetrag. **Wichtig:** Für normale Eingangsrechnungen (ER) mit Buchcode 1 muss der Betrag **negativ** sein (`-123,45`). |
| 8 | `text` | Buchungstext (Symbol + Lieferant + BelegID). |
| 9 | `buchsymbol` | z.B. `ER` oder ein benutzerdefiniertes Symbol. |
| 10 | **`buchcode`** | `1` für Standard-Eingangsrechnungen. |
| 11 | `periode` | Monat der Buchung (1-12). |
| 12 | `uva-periode` | Jahr und Monat (`JJJJMM`). |
| 13 | `uva-kursdatum` | Meist identisch mit dem Belegdatum. |
| 14 | `steuercode` | Optional (leer, wenn über Prozentsatz oder Konto gesteuert). |
| 15 | `prozent` | Steuersatz (z.B. `20,00`). |
| 16 | `steuer` | Steuerbetrag (meist leer, BMD rechnet selbst). |
| 17 | `uidnr` | UID-Nummer des Geschäftspartners (optional). |
| 18 | `dokument` | Pfad zum PDF-Beleg. |
| 19 | `extbelegnr` | Externe Belegnummer (vom Lieferanten). |

## Besonderheiten

### Betrag und Vorzeichen
Für den BMD-Import einer Eingangsrechnung (Buchcode 1) auf ein Kreditorenkonto muss der Betrag in der Importdatei **negativ** multipliziert werden, damit er im BMD korrekt als Verbindlichkeit gebucht wird.

### Dokumentenpfade (`dokument`)
BMD kann Dateien direkt öffnen, wenn der Pfad korrekt hinterlegt ist.
- **Normaler Pfad:** Absoluter Pfad (z.B. `C:\Belege\2026\Rechnung_123.pdf`).
- **Terminal Server (`\\tsclient\`):** In speziellen Remote-Umgebungen muss dem Pfad das Präfix `\\tsclient\` vorangestellt werden (z.B. `\\tsclient\C\Belege\...`), damit der Server auf lokale Dateien des Clients zugreifen kann. Das Programm transformiert `C:\` automatisch in `\\tsclient\C`.

### UVA-Periode
Wird im Format `JJJJMM` angegeben (z.B. `202604` für April 2026), um die steuerliche Zuordnung sicherzustellen.
