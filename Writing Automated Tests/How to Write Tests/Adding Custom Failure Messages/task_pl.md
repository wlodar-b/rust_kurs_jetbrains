## Dodawanie niestandardowych komunikatów błędu

Możesz również dodać niestandardowy komunikat, który zostanie wyświetlony wraz z komunikatem o niepowodzeniu, jako opcjonalny argument w makrach `assert!`, `assert_eq!` oraz `assert_ne!`. Wszystkie argumenty podane po wymaganym argumencie w `assert!` lub dwóch wymaganych argumentach w `assert_eq!` i `assert_ne!` są przekazywane do makra `format!` (omówionego w Rozdziale 8 w sekcji [„Konkatenacja za pomocą operatora `+` lub makra `format!`”](https://doc.rust-lang.org/stable/book/ch08-02-strings.html#concatenation-with-the--operator-or-the-format-macro) Książki), więc można podać ciąg formatowania zawierający placeholdery `{}` oraz wartości, które wypełnią te placeholdery. Niestandardowe komunikaty są przydatne do dokumentowania, co oznacza dane wyrażenie asercji; jeśli test zawiedzie, łatwiej będzie zrozumieć, jaki dokładnie problem wystąpił w kodzie.

Na przykład, załóżmy, że mamy funkcję, która wita ludzi po imieniu, i chcemy przetestować, czy imię przekazane do funkcji pojawia się w wyniku:

```rust
    pub fn greeting(name: &str) -> String {
        format!("Hello {}!", name)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn greeting_contains_name() {
            let result = greeting("Carol");
            assert!(result.contains("Carol"));
        }
    }
```

Wymagania dla tego programu nie zostały jeszcze ustalone, i mamy niemal pewność, że tekst `Hello` na początku powitania ulegnie zmianie. Podjęliśmy decyzję, że nie chcemy aktualizować testu po każdej zmianie wymagań, więc zamiast sprawdzania dokładnej równości wartości zwracanej przez funkcję `greeting`, po prostu sprawdzimy, czy wynik zawiera tekst przekazanego parametru wejściowego.

Wprowadźmy teraz błąd do tego kodu, zmieniając `greeting`, tak aby nie zawierał `name`, aby sprawdzić, jak wygląda wynik niepowodzenia testu:

```rust
    pub fn greeting(name: &str) -> String {
        String::from("Hello!")
    }
```

Uruchomienie tego testu daje następujący wynik:

```text
running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
thread 'main' panicked at 'assertion failed: result.contains("Carol")', src/lib.rs:12:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::greeting_contains_name
```

Ten wynik wskazuje jedynie, że wyrażenie asercji zawiodło, oraz na której linii znajduje się to wyrażenie. Bardziej użyteczny komunikat błędu w takim przypadku zawierałby wartość zwróconą przez funkcję `greeting`. Zmieńmy funkcję testującą, dodając do niej niestandardowy komunikat błędu utworzony na podstawie ciągu formatowania z placeholderem wypełnionym faktyczną wartością, którą otrzymaliśmy z funkcji `greeting`:

```rust
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}
```

Teraz, po uruchomieniu testu, otrzymamy bardziej informacyjny komunikat o błędzie:

```text
---- tests::greeting_contains_name stdout ----
thread 'main' panicked at 'Greeting did not contain name, value was `Hello!`', src/lib.rs:12:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Widzimy wartość, którą faktycznie otrzymaliśmy w wyniku testu, co pomoże nam debugować, co się stało, zamiast tego, czego się spodziewaliśmy.