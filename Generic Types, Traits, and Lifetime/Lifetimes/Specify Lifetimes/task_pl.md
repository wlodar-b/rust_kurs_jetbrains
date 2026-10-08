## Określanie okresów życia

Kompilator Rust musi wiedzieć, jak sprawdzić, czy dostarczone referencje są
prawidłowe, aby mógł poinformować programistę, jeśli referencja jest zagrożona
przekroczeniem zakresu przed jej użyciem. Pamiętaj, referencje to pożyczki
i nie są właścicielami swoich danych. Co, jeśli ich właściciel wyjdzie poza zakres?

<div class="hint">
Pozwól kompilatorowi Cię poprowadzić.

Spójrz również na [książkę](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html), jeśli potrzebujesz pomocy.
</div>