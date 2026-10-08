## Kontrolowanie sposobu uruchamiania testów

Podobnie jak `cargo run` kompiluje Twój kod i uruchamia powstały plik wykonywalny, `cargo test` kompiluje Twój kod w trybie testowym i uruchamia powstały binarny plik testowy. Możesz określić opcje w wierszu poleceń, aby zmienić domyślne zachowanie `cargo test`. Na przykład domyślne działanie binarnego pliku wynikowego z `cargo test` polega na uruchamianiu wszystkich testów równolegle i przechwytywaniu wyjścia generowanego podczas uruchamiania testów, co zapobiega wyświetlaniu wyjścia oraz ułatwia czytanie wyników testów.

Niektóre opcje wiersza poleceń dotyczą `cargo test`, a inne wynikowego binarnego pliku testowego. Aby rozdzielić te dwa rodzaje argumentów, należy podać argumenty dla `cargo test`, po których następuje separator `--` i argumenty dla pliku testowego. Uruchomienie `cargo test --help` wyświetla opcje, które można zastosować z `cargo test`, a uruchomienie `cargo test -- --help` pokazuje opcje, które możesz użyć po separatorze `--`.

### Uruchamianie testów równolegle lub kolejno

Podczas uruchamiania wielu testów domyślnie są one uruchamiane równolegle z użyciem wątków. Oznacza to, że testy zakończą się szybciej, co pozwala szybciej uzyskać informację zwrotną, czy Twój kod działa. Ponieważ testy są uruchamiane jednocześnie, upewnij się, że nie zależą one od siebie nawzajem ani od żadnego współdzielonego stanu, w tym środowiska, takich jak aktualny katalog roboczy lub zmienne środowiskowe.

Na przykład załóżmy, że każdy test uruchamia kod, który tworzy plik na dysku o nazwie *test-output.txt* i zapisuje do niego dane. Następnie każdy test odczytuje dane z tego pliku i weryfikuje, czy plik zawiera określoną wartość, która różni się w każdym teście. Ponieważ testy są uruchamiane jednocześnie, jeden test może nadpisać plik między momentem, gdy inny test go zapisuje i odczytuje. W rezultacie drugi test się nie powiedzie, nie z powodu błędu w kodzie, ale dlatego, że testy zakłóciły się nawzajem podczas uruchamiania równoległego. Jednym z rozwiązań jest upewnienie się, że każdy test zapisuje do innego pliku; innym rozwiązaniem jest uruchamianie testów kolejno, jeden po drugim.

Jeśli nie chcesz uruchamiać testów równolegle lub chcesz mieć bardziej granularną kontrolę nad liczbą używanych wątków, możesz przekazać flagę `--test-threads` oraz liczbę wątków do pliku testowego. Spójrz na poniższy przykład:

```console
$ cargo test -- --test-threads=1
```

Ustawiliśmy liczbę wątków testowych na `1`, co oznacza, że program nie użyje równoległości. Uruchamianie testów przy użyciu jednego wątku zajmie więcej czasu niż równoległe, ale testy nie będą się wzajemnie zakłócały, jeśli współdzielą stan.

### Wyświetlanie wyjścia funkcji

Domyślnie, jeśli test przechodzi, biblioteka testowa Rust przechwytuje wszystko, co zostało wydrukowane na standardowe wyjście. Na przykład jeśli w teście wywołamy `println!`, a test przejdzie, nie zobaczymy wyjścia `println!` w terminalu; zobaczymy tylko linię wskazującą, że test przeszedł. Jeśli test się nie powiedzie, zobaczymy wszystko, co zostało wydrukowane na standardowe wyjście wraz z resztą komunikatu o niepowodzeniu.

Poniższy kod zawiera prostą funkcję, która drukuje wartość swojego parametru i zwraca 10, oraz test, który przechodzi, i test, który się nie powodzi:

```rust,panics
fn prints_and_returns_10(a: i32) -> i32 {
    println!("Otrzymałem wartość {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
```

##### Testy dla funkcji wywołującej `println!`

Uruchamiając te testy z `cargo test`, zobaczymy następujące wyjście:

```text
running 2 tests
test tests::this_test_will_fail ... FAILED
test tests::this_test_will_pass ... ok

failures:

---- tests::this_test_will_fail stdout ----
Otrzymałem wartość 8
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:19:9
note: wykonaj polecenie z `RUST_BACKTRACE=1`, aby wyświetlić szczegóły błędu


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Zauważ, że w tym wyjściu nie widzimy tekstu `Otrzymałem wartość 4`, który jest drukowany, gdy przechodzi test. To wyjście zostało przechwycone. Wyjście z testu, który się nie powiódł, czyli `Otrzymałem wartość 8`, pojawia się w sekcji podsumowania wyników testów, która również pokazuje przyczynę niepowodzenia testu.

Jeśli chcemy zobaczyć wydrukowane wartości również dla przechodzących testów, możemy nakazać Rustowi wyświetlenie wyjść udanych testów na końcu za pomocą `--show-output`.

```text
$ cargo test -- --show-output
```

Podczas ponownego uruchamiania testów z flagą `--show-output` zobaczymy następujące wyjście:

```text
running 2 tests
test tests::this_test_will_fail ... FAILED
test tests::this_test_will_pass ... ok

successes:

---- tests::this_test_will_pass stdout ----
Otrzymałem wartość 4


successes:
    tests::this_test_will_pass

failures:

---- tests::this_test_will_fail stdout ----
Otrzymałem wartość 8
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:19:9
note: wykonaj polecenie z `RUST_BACKTRACE=1`, aby zobaczyć stack trace


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

### Uruchamianie podzbioru testów na podstawie nazwy

Czasami uruchamianie całego zestawu testów może zająć dużo czasu. Jeśli pracujesz nad kodem w określonej części projektu, możesz uruchamiać tylko te testy, które dotyczą tego kodu. Możesz wybrać, które testy uruchomić, przekazując do `cargo test` nazwę lub nazwy testów jako argument.

Aby zademonstrować uruchamianie podzbioru testów, utworzymy trzy testy dla naszej funkcji `add_two`, jak przedstawiono poniżej, i wybierzemy, które z nich uruchomić:

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
```

##### Trzy testy z różnymi nazwami

Jeśli uruchomimy testy bez podawania żadnych argumentów, jak widzieliśmy wcześniej, wszystkie testy zostaną uruchomione równolegle:

```text
running 3 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok
test tests::one_hundred ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

#### Uruchamianie pojedynczych testów

Możemy przekazać nazwę dowolnej funkcji testowej do `cargo test`, aby uruchomić tylko ten test, na przykład `cargo test one_hundred`:

```text
running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out
```

Uruchomiono tylko test o nazwie `one_hundred`; pozostałe dwa testy nie pasowały do tej nazwy. Wyjście testu informuje nas, że mieliśmy więcej testów niż te uruchomione przez tę komendę, wyświetlając `2 filtered out` na końcu podsumowania.

Nie możemy określić nazw wielu testów w ten sposób; użyta będzie tylko pierwsza wartość przekazana do `cargo test`. Ale istnieje sposób na uruchomienie wielu testów.

#### Filtrowanie w celu uruchomienia wielu testów

Możemy określić część nazwy testu i każdy test, którego nazwa zawiera tę wartość, zostanie uruchomiony. Na przykład, ponieważ dwie nazwy testów zawierają `add`, możemy uruchomić te dwa za pomocą `cargo test add`:

```text
running 2 tests
test tests::add_two