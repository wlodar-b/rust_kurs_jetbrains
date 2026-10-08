### Operator glob

Jeśli chcemy wprowadzić _wszystkie_ publiczne elementy zdefiniowane w danej ścieżce do zakresu (scope), możemy określić tę ścieżkę, a następnie użyć `*`, czyli operatora glob:

```rust
    use std::collections::*;
```

To polecenie `use` wprowadza wszystkie publiczne elementy zdefiniowane w `std::collections` do bieżącego zakresu. Zachowaj ostrożność przy korzystaniu z operatora glob! Operator glob może utrudnić określenie, jakie nazwy znajdują się w zakresie i gdzie w Twoim programie dana nazwa została zdefiniowana.

Operator glob jest często używany przy testowaniu, aby wprowadzić wszystko poddawane testom do modułu `tests`; omówimy to w sekcji [„Jak pisać testy”](https://doc.rust-lang.org/stable/book/ch11-01-writing-tests.html#how-to-write-tests) w rozdziale 11. Operator glob bywa także wykorzystywany w ramach wzorca prelude: więcej informacji o tym wzorcu znajdziesz w [dokumentacji standardowej biblioteki](https://doc.rust-lang.org/std/prelude/index.html#other-preludes).

_Możesz odnieść się do następującego rozdziału w książce „The Rust Programming Language”: [Importowanie ścieżek za pomocą słowa kluczowego use](https://doc.rust-lang.org/stable/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#bringing-paths-into-scope-with-the-use-keyword)_