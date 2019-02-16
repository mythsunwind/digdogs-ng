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
