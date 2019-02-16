dat-viewer
==========

### DIG-PART.DAT

2 bytes = Dateikennung (b503)

2880 bytes = Addressierung der 948 tiles

* 3 Blöcke mit 256 mal 2 bytes
* Erster byte 00 - ff beschreibt Tilenummer
* Zweiter byte 00,01 oder 02 beschreibt Blocknummer
* 1 Block mit 180 mal 2 bytes
* Erster byte 00 - b4 beschreibt Tilenummer
* Zweiter byte 04 beschreibt Blocknummer

Aber:
* (256*3+180)*2 = 1896
* 984 bytes zusätzlich, die eventuell Zusatzinformation bestimmter Tiles speichern?

Danach: 948 Stück 16x16 Tiles

### DIG-PART.DA1

2 bytes = Dateikennung (e800)

2880 bytes = Addressierung der 231? tiles

* 1 Block mit 231 mal 2 bytes
* Erster byte 00 - e7 beschreibt Tilenummer
* Zweiter byte 00 beschreibt Blocknummer

Danach: Bis zum Ende der 2880 bytes: ?

Danach: 231? Stück 16x16 Tiles

## Beispieladdressierung

c600 Abbiegen Schild
d100 30 Schild
cd00 Baustellen Schild
d000 30 aufgehoben Schild

