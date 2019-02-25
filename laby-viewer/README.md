# laby-viewer

Tile:
	Address (16 bit / 2 bytes) = ef00 = u16
	Color for each pixel = (x = 0-15 , y = 0-15) = (255 = u8, 0 = u8, 0 = u8)

1. Load tiles in array addressable by u16
2. Load DA1 file as 2 bytes each
3. Draw 10 tiles in each column

## Usage

```
cargo run DIG-PART.DA1 DIG-LABY.40
```

## Research

DIG-LABY.10 = Anfang?

DIG-LABY.11-15 = 5

DIG-LABY.21-26 = 6

DIG-LABY.31-38 = 8 (Längste Strecke = erste Abbiegung?)

DIG-LABY.40 = Ende?

Sichtbarer Strassenbereich: 320x160
Macht in Tiles je 16x16: 20x10

## Strecken

### Strecke 1 (beim der ersten Seitenstraße nach oben abbiegen)

Beschreibung 1.1:
- Einbahnstraße
- Achtung (Kinder)
- Schule
- Zebrastreifen
- Krankenwagen
- Geradeaus
- Ampeln
- Telefonhäuschen

Beschreibung 1.2:
- Achtung (Fahrrad)
- Telefonhäuschen
- Rechtsabbiegen
- Tomate

Beschreibung 1.3:
- Geradeaus
- Telefonhäuschen
- LKW
- Achtung (Kinder)
- Schule
- Bus
- Busschild
- Geradeaus
- Rechtsabbiegen
- Fahrschule
- Linksabbiegen
- Ampeln

Beschreibung 1.4:
- Geradeaus
- Bump
- Planierraupe
- Linksabbiegen
- Stop
- Vorfahrtschild von hinten

Beschreibung 1.5:
- Kirche
- Rechtsabbiegen
- Ampel
- Tomate

Beschreibung 1.6:
- Spielstraße
- Telefonhäuschen
- Linksabbiegen
- Einfahrt verboten (nicht Tomate)

Beschreibung 1.7:
- Bus
- Busschild
- Rechtsabbiegen
- Tomate

Beschreibung 1.8:
- 30
- Sandkasten
- Baustellen-LKW
- Krankenhaus
- Krankenwagen 2x
- 30 aufgehoben
- Rote Mauer
- Rechtsabbiegen
- Ampel

### Strecke 2 (beim der zweiten Seitenstraße nach oben abbiegen)

Beschreibung 2.1 (DIG-LABY.11?)
- Bump fa01 fb01 2a00 2b00
- Achtung (Fahrrad) cd00
- Oranges Auto (andere Fahrbahn) a501 a601 a701 a801 c901 ca01 cb01 cc01
- Türkises Auto (selbe Fahrbahn) 6b01 6c01 6d01 6e01 8301 8401 8501 8601
- Einbahnstraße d300
- Türkises Auto (andere Fahrbahn) 1e02 1f02 2002 2102 c901 ca01 cb01 cc01

Beschreibung 2.2:
- Geradeaus
- Ampeln
- Achtung (Kinder)
- Schule
- Zebrastreifen
- Stop
- Tomate

Beschreibung 2.3:
- Tomate
- Telefonhäuschen
- Spielstraße
- Linksabbiegen
- Tomate

Beschreibung 2.4:
- Bus
- Bushaltestelle
- Rechtsabbiegen
- Tomate

Beschreibung 2.5:
- 30
- Sandkasten
- Baustellen-LKW
- Krankenhaus
- Krankenwagen 2x
- 30 aufgehoben
- Rote Mauer
- Rechtsabbiegen
- Ampel

## Tile-Addressierung

### Achtung (Fahrrad)

In LABY file: 01ad hex = 173 dec

+32?

In DAT file: cd00 hex = 205 dec

### Einbahnstraße

In LABY file: 01b3 hex = 179 dec

+32?

In DAT file: d300 hex = 211 dec

## Mögliche Addressen in DIG-LABY.40

DIG-LABY.40 enthält keine Tiles aus dem 02 und 03 Block. Daher kann man davon ausgehen, dass es DA1 benutzt.

b6 = Oberseite Ortsausgangsschild

c8 = Unterseite Ortsausgangsschild

ad = Oberseite Andreaskreuz

bf = Unterseite Andreaskreuz

### Parkplatz

```
3e 3f 40 41
56 57 58 59
64 65 66 67
```
