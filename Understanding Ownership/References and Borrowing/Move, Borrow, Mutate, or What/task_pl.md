## Przekazywanie, pożyczanie, mutowanie, czyli co i jak

Spraw, aby kod się kompilował! Nie zmieniaj nic poza argumentami funkcji.

<div class="hint">
Zauważ, że <code>fn get_last_char</code> nie powinno przejmować własności, 
ponieważ <code>message</code> jest używana dalej w funkcji <code>main</code>. 
Zamiast tego możesz użyć referencji, aby ją wypożyczyć.
</div>

<div class="hint">
Zauważ, że <code>fn uppercase_and_print</code> jest ostatnim wywołaniem w <code>main</code>, więc możesz bezpiecznie
przenieść <code>message</code> do tej funkcji. W związku z tym nie ma potrzeby przekazywania referencji. 

Zwróć również uwagę, że ta funkcja modyfikuje zmienną `message`. Dlatego powinieneś zdefiniować ją jako mutowalną.
</div>