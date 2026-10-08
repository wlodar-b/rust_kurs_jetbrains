### Sprawdzanie wyników za pomocą makra assert! 

Makro `assert!`, dostarczone przez bibliotekę standardową, jest przydatne, gdy chcesz upewnić się, że pewien warunek w teście zostanie oceniony jako `true` (prawdziwy). Przekazujemy do makra `assert!` argument, który jest wyrażeniem oceniającym się do wartości logicznej (Boolean). Jeśli wartość jest `true`, `assert!` nie robi nic, a test przechodzi pomyślnie. Jeśli wartość jest `false`, makro `assert!` wywołuje makro `panic!`, co powoduje, że test się nie powodzi. Używanie makra `assert!` pomaga sprawdzić, czy nasz kod działa zgodnie z oczekiwaniami.

W rozdziale "Struktury/Skladnia Metod" w sekcji "Implementacja metody `can_hold` dla `Rectangle`, która przyjmuje instancję innej `Rectangle` jako parametr", używaliśmy struktury `Rectangle` i metody `can_hold`, które są tutaj powtórzone. Umieśćmy ten kod w pliku _src/lib.rs_ i napiszmy dla niego kilka testów, korzystając z makra `assert!`.

```rust
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
```

##### Przykład użycia struktury `Rectangle` i jej metody `can_hold` z rozdziału "Struktury/Skladnia Metod"

Metoda `can_hold` zwraca wartość logiczną, co oznacza, że jest to idealny przypadek użycia makra `assert!`. W poniższym fragmencie kodu piszemy test sprawdzający metodę `can_hold` poprzez utworzenie instancji `Rectangle` o szerokości 8 i wysokości 7 oraz testowanie, czy ta instancja może pomieścić inną instancję `Rectangle` o szerokości 5 i wysokości 1.

```rust
   #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn larger_can_hold_smaller() {
            let larger = Rectangle { width: 8, height: 7 };
            let smaller = Rectangle { width: 5, height: 1 };

            assert!(larger.can_hold(&smaller));
        }
    }
```

##### Przykład testu dla `can_hold`, który sprawdza, czy większy prostokąt może rzeczywiście pomieścić mniejszy prostokąt

Zwróć uwagę, że dodaliśmy nową linię w module `tests`: `use super::*;`. Moduł `tests` to zwykły moduł, który przestrzega standardowych reguł widoczności, omówionych we wstępie do rozdziału "Moduły" (rozdział "Moduły i Makra"). Ponieważ moduł `tests` jest modułem wewnętrznym, musimy wprowadzić do jego zakresu kod testowany znajdujący się w zewnętrznym module. Użyliśmy tutaj symbolu glob (gwiazdki), aby wszystko, co zdefiniujemy w zewnętrznym module, było dostępne w module `tests`.

Nazwaliśmy nasz test `larger_can_hold_smaller`, a następnie stworzyliśmy dwie instancje `Rectangle`, które są nam potrzebne. Następnie wywołaliśmy makro `assert!` i przekazaliśmy mu wynik wywołania `larger.can_hold(&smaller)`. Wyrażenie to powinno zwrócić `true`, dlatego nasz test powinien przejść. Sprawdźmy to!

```text
    running 1 test
    test tests::larger_can_hold_smaller ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Test przeszedł! Dodajmy teraz inny test, tym razem sprawdzający, czy mniejszy prostokąt nie może pomieścić większego:

```rust
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn larger_can_hold_smaller() {
            // --snip--
        }

        #[test]
        fn smaller_cannot_hold_larger() {
            let larger = Rectangle { width: 8, height: 7 };
            let smaller = Rectangle { width: 5, height: 1 };

            assert!(!smaller.can_hold(&larger));
        }
    }
```

Ponieważ poprawny wynik funkcji `can_hold` w tym przypadku to `false`, musimy zanegować ten wynik, zanim przekażemy go do makra `assert!`. W rezultacie nasz test przejdzie, jeśli `can_hold` zwróci `false`:

```text
    running 2 tests
    test tests::smaller_cannot_hold_larger ... ok
    test tests::larger_can_hold_smaller ... ok

    test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Dwa testy przeszły! Zobaczmy teraz, co stanie się z wynikami naszych testów, gdy wprowadzimy błąd w kodzie. Zmieńmy implementację metody `can_hold`, zastępując znak większy znakiem mniejszy w porównaniu szerokości:

```rust
    // --snip--

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width < other.width && self.height > other.height
        }
    }
```

Uruchomienie testów teraz daje następujący wynik:

```text
    running 2 tests
    test tests::smaller_cannot_hold_larger ... ok
    test tests::larger_can_hold_smaller ... FAILED

    failures:

    ---- tests::larger_can_hold_smaller stdout ----
    thread 'tests::larger_can_hold_smaller' panicked at 'assertion failed:
    larger.can_hold(&smaller)', src/lib.rs:22:9
    note: Run with `RUST_BACKTRACE=1` for a backtrace.

    failures:
        tests::larger_can_hold_smaller

    test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Nasze testy wykryły błąd! Ponieważ `larger.width` wynosi 8, a `smaller.width` wynosi 5, porównanie szerokości w `can_hold` zwraca teraz `false`: 8 nie jest mniejsze od 5.