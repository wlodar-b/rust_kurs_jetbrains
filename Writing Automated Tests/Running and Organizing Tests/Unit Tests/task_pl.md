### Testy Jednostkowe

Celem testów jednostkowych jest testowanie każdej jednostki kodu w izolacji od reszty programu, aby szybko określić, gdzie kod działa zgodnie z oczekiwaniami, a gdzie nie. Umieszczasz testy jednostkowe w katalogu *src* w każdym pliku zawierającym kod, który testują. Konwencją jest tworzenie modułu o nazwie `tests` w każdym pliku, aby zawierał funkcje testowe, i anotowanie modułu za pomocą `cfg(test)`.

#### Moduł Testów i `#[cfg(test)]`

Anotacja `#[cfg(test)]` dla modułu testowego informuje Rust, aby kompilował i uruchamiał kod testowy tylko podczas uruchamiania `cargo test`, a nie podczas `cargo build`. Oszczędza to czas kompilacji, gdy chcesz jedynie zbudować bibliotekę, i zmniejsza rozmiar wynikowego skompilowanego pliku, ponieważ testy nie są do niego dołączane. Zauważysz, że ponieważ testy integracyjne znajdują się w innym katalogu, nie potrzebują anotacji `#[cfg(test)]`. Natomiast testy jednostkowe znajdują się w tych samych plikach co kod, dlatego używasz `#[cfg(test)]`, aby określić, że nie powinny być one uwzględnione w skompilowanym wyniku.

Przypomnij sobie przykład modułu testowego i funkcję automatycznie wygenerowaną przez `cargo new` w pierwszej sekcji tego rozdziału:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

Ten kod to automatycznie wygenerowany moduł testowy. Atrybut `cfg` oznacza *konfigurację* i informuje Rust, że poniższy element powinien być uwzględniony tylko przy określonej opcji konfiguracji. W tym przypadku opcją konfiguracji jest `test`, która jest dostarczana przez Rust do kompilowania i uruchamiania testów. Dzięki użyciu atrybutu `cfg`, Cargo kompiluje nasz kod testowy tylko wtedy, gdy aktywnie uruchamiamy testy za pomocą `cargo test`. Obejmuje to także wszelkie funkcje pomocnicze znajdujące się w tym module, oprócz funkcji oznaczonych `#[test]`.

#### Testowanie Prywatnych Funkcji

W społeczności zajmującej się testowaniem trwa debata na temat tego, czy prywatne funkcje powinny być testowane bezpośrednio, a inne języki czynią to trudnym lub niemożliwym. Niezależnie od ideologii testowania, którą wyznajesz, zasady prywatności w Rustr pozwalają na testowanie prywatnych funkcji. Poniżej przedstawiono kod z prywatną funkcją `internal_adder`.

```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

##### Testowanie prywatnej funkcji

Zauważ, że funkcja `internal_adder` nie jest oznaczona jako `pub`, ale ponieważ testy są jedynie kodem w Rust, a moduł `tests` jest po prostu innym modułem, możesz wprowadzić `internal_adder` w zakres testów i wywołać ją. Jeśli uważasz, że prywatne funkcje nie powinny być testowane, Rust nie zmusi cię do tego.