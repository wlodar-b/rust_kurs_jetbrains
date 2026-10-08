## Ujednolicenie czasów życia

Jeżeli kompilator tylko sprawdza poprawność referencji przekazanych do oznaczonych parametrów oraz typu zwracanego, co musimy zmienić?

<div class="hint">
Pamiętaj, że generyczny czas życia <code>'a</code> otrzyma konkretny czas życia równy mniejszemu z czasów życia <code>x</code> i <code>y</code>.
</div>

<div class="hint">
Możesz obrać co najmniej dwie ścieżki, aby osiągnąć pożądany rezultat, przy zachowaniu wewnętrznego bloku:

1. Przenieś deklarację `string2`, aby istniała tak długo, jak `string1` (w jaki sposób zadeklarowano `result`?).
2. Przenieś `println!` do wewnętrznego bloku.

</div>