### Sprawdzanie paniki za pomocą `should_panic`

Oprócz sprawdzania, czy nasz kod zwraca oczekiwane wartości, ważne jest również, aby upewnić się, że kod obsługuje warunki błędu zgodnie z oczekiwaniami. Na przykład weźmy pod uwagę typ `Guess`, który stworzyliśmy w sekcji "Panika czy jej brak" rozdziału "Błędy do odzyskania i nieodwracalne", w fragmencie "Typ `Guess`, który akceptuje tylko wartości pomiędzy 1 a 100". Inny kod korzystający z `Guess` zakłada gwarancję, że instancje `Guess` zawierają wyłącznie wartości pomiędzy 1 a 100. Możemy napisać test, który zapewnia, że próba utworzenia instancji `Guess` z wartością poza tym zakresem powoduje panikę.

Robimy to, dodając kolejny atrybut `should_panic` do naszej funkcji testowej. Ten atrybut sprawia, że test przechodzi, jeśli kod wewnątrz funkcji powoduje panikę; test zakończy się niepowodzeniem, jeśli kod nie spowoduje paniki.

Poniższy fragment kodu pokazuje test, który sprawdza, że warunki błędów w `Guess::new` występują wtedy, kiedy się tego spodziewamy.

```rust
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Wartość Guess musi mieścić się w przedziale 1-100, otrzymano {}.", value);
            }

            Guess {
                value
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        #[should_panic]
        fn greater_than_100() {
            Guess::new(200);
        }
    }
```

##### Przykład testowania, że dany warunek powoduje `panic!`

Umieszczamy atrybut `#[should_panic]` po atrybucie `#[test]` i przed funkcją testową, której dotyczy. Spójrzmy na wynik, gdy ten test przechodzi pomyślnie:

```text
    running 1 test
    test tests::greater_than_100 ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Wygląda dobrze! Teraz wprowadźmy błąd w naszym kodzie, usuwając warunek mówiący, że funkcja `new` powinna wywołać panikę, jeśli wartość jest większa niż 100:

```rust
    // --snip--

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1  {
                panic!("Wartość Guess musi mieścić się w przedziale 1-100, otrzymano {}.", value);
            }

            Guess {
                value
            }
        }
    }
```

Kiedy uruchamiamy test z "Przykładu testowania, że dany warunek powoduje `panic!`", test zakończy się niepowodzeniem:

```text
running 1 test
test tests::greater_than_100 ... FAILED

failures:

---- tests::greater_than_100 stdout ----
note: test did not panic as expected

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

W tym przypadku nie dostajemy zbyt pomocnego komunikatu, ale gdy spojrzymy na funkcję testową, widzimy, że jest oznaczona atrybutem `#[should_panic]`. Otrzymana porażka oznacza, że kod w funkcji testowej nie spowodował paniki.

Testy wykorzystujące `should_panic` mogą być niedokładne, ponieważ wskazują jedynie, że kod wywołał jakąś panikę. Test `should_panic` przejdzie nawet wtedy, gdy panika występuje z innego powodu niż ten, którego się spodziewaliśmy. Aby testy `should_panic` były bardziej dokładne, możemy dodać opcjonalny parametr `expected` do atrybutu `should_panic`. Narzędzie testowe upewni się, że komunikat błędu zawiera podany tekst. Na przykład, weźmy zmodyfikowany kod dla `Guess` (poniżej), w którym funkcja `new` generuje różne komunikaty w zależności od tego, czy wartość jest za mała, czy za duża.

```rust
    // --snip--

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 {
                panic!("Wartość Guess musi być większa lub równa 1, otrzymano {}.", value);
            } else if value > 100 {
                panic!("Wartość Guess musi być mniejsza lub równa 100, otrzymano {}.", value);
            }

            Guess {
                value
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        #[should_panic(expected = "Wartość Guess musi być mniejsza lub równa 100")]
        fn greater_than_100() {
            Guess::new(200);
        }
    }
```

##### Przykład testowania, że dany warunek powoduje panikę z określoną wiadomością

Ten test przejdzie, ponieważ wartość, którą umieściliśmy w parametrze `expected` atrybutu `should_panic`, jest podciągiem komunikatu generowanego przez funkcję `Guess::new` w przypadku paniki. Moglibyśmy wskazać cały komunikat, którego się spodziewamy, który w tym przypadku brzmiałby: `Wartość Guess musi być mniejsza lub równa 100, otrzymano 200.`. To, co zdecydujemy się określić w parametrze `expected` dla `should_panic`, zależy od tego, jak specyficzny lub dynamiczny jest komunikat o panice oraz jak precyzyjni chcemy być w naszym teście. W tym przypadku podciąg komunikatu wystarczy, aby upewnić się, że kod w funkcji testowej wykonuje przypadek `else if value > 100`.

Aby zobaczyć, co się dzieje, gdy test `should_panic` z komunikatem `expected` nie przechodzi, jeszcze raz wprowadzimy błąd w naszym kodzie, zamieniając treść bloków `if value < 1` i `else if value > 100`:

```rust
    if value < 1 {
        panic!("Wartość Guess musi być mniejsza lub równa 100, otrzymano {}.", value);
    } else if value > 100 {
        panic!("Wartość Guess musi być większa lub równa 1, otrzymano {}.", value);
    }
```

Tym razem, gdy uruchomimy test `should_panic`, zakończy się on niepowodzeniem:

```text
running 1 test
test tests::greater_than_100 ... FAILED

failures:

---- tests::greater_than_100 stdout ----
thread 'main' panicked at 'Wartość Guess musi być większa lub równa 1, otrzymano 200.', src/lib.rs:13:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: panic did not contain expected string
      panic message: `"Wartość Guess musi być większa lub równa 1, otrzymano 200."`,
 expected substring: `"Wartość Guess musi być mniejsza lub równa 100"`

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Komunikat o błędzie wskazuje, że test rzeczywiście wywołał panikę, tak jak się spodziewaliśmy, ale komunikat o panice nie zawierał oczekiwanego podciągu `Wartość Guess musi być mniejsza lub równa 100`. Komunikat o panice, który otrzymaliśmy w tym przypadku, brzmiał: `Wartość Guess musi być większa lub równa 1, otrzymano 200.`. Teraz możemy zacząć diagnozować, gdzie znajduje się błąd!