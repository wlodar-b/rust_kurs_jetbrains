### Testy integracyjne

W języku Rust testy integracyjne są całkowicie zewnętrzne wobec twojej biblioteki. Korzystają z biblioteki w taki sam sposób, jak każdy inny kod, co oznacza, że mogą wywoływać jedynie funkcje będące częścią publicznego API twojej biblioteki. Ich celem jest sprawdzenie, czy różne elementy twojej biblioteki współpracują poprawnie. Jednostki kodu, które osobno działają prawidłowo, mogą sprawiać problemy po zintegrowaniu, dlatego ważne jest również pokrycie testami zintegrowanego kodu. Aby utworzyć testy integracyjne, najpierw musisz stworzyć katalog *tests*.

#### Katalog *tests*

Utworzyliśmy katalog *tests* w głównym katalogu naszego projektu, obok *src*. Cargo automatycznie szuka plików z testami integracyjnymi w tym katalogu. Możemy umieścić w nim dowolną liczbę plików testowych, a Cargo skompiluje każdy z tych plików jako osobną „szkatułkę” (*crate*).

Przyjrzyjmy się testowi integracyjnemu. Korzystając z kodu z listingu „Testowanie funkcji prywatnej” w pliku *src/lib.rs*, zajrzyj do katalogu *tests*, gdzie znajduje się plik nazwany *tests/integration_test.rs* z kodem z poniższego listingu.

```rust
use integration_tests;

#[test]
fn it_adds_two() {
    assert_eq!(4, integration_tests::add_two(2));
}
```

##### Test integracyjny funkcji w szkatułce `integration_tests`

Dodaliśmy `use integration_tests;` na początku kodu, czego nie musieliśmy robić w testach jednostkowych. Powodem jest to, że każdy plik w katalogu `tests` jest osobną szkatułką, więc musimy wprowadzić naszą bibliotekę do zakresu każdej szkatułki testowej.

Nie musimy oznaczać żadnego kodu w pliku *tests/integration_test.rs* przy użyciu `#[cfg(test)]`. Cargo traktuje katalog `tests` w sposób szczególny i kompiluje pliki w tym katalogu tylko wtedy, gdy uruchamiamy `cargo test`. Uruchom teraz `cargo test`:

```text
Compiling integration_tests v0.1.0 
    Finished test [unoptimized + debuginfo] target(s) in 0.54s
     Running target/debug/deps/integration_tests-61f5d8d60ccbcc19
     
running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_tests-d5df7484b111e79e

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests integration_tests

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Trzy sekcje wyników obejmują testy jednostkowe, test integracyjny oraz testy dokumentacyjne. Pierwsza sekcja dotycząca testów jednostkowych wygląda tak samo, jak dotychczas: jedna linia dla każdego testu jednostkowego (jeden nazwany `internal`, który dodaliśmy w „Testowaniu funkcji prywatnej”) i linia podsumowująca wyniki testów jednostkowych.

Sekcja testów integracyjnych zaczyna się linią `Running target/debug/deps/integration_tests-d5df7484b111e79e` (hash na końcu twojego wyniku będzie inny). Następnie widzimy jedną linię dla każdej funkcji testowej w danym teście integracyjnym oraz linię podsumowującą wyniki testu integracyjnego, tuż przed rozpoczęciem sekcji `Doc-tests integration_tests`.

Podobnie jak dodanie większej liczby funkcji testowych w testach jednostkowych dodaje więcej linii wynikowych w sekcji testów jednostkowych, dodanie większej liczby funkcji testowych do pliku testów integracyjnych dodaje więcej linii wyników w sekcji tego pliku testowego. Każdy plik testów integracyjnych ma swoją własną sekcję, więc jeśli dodamy więcej plików w katalogu *tests*, pojawi się więcej sekcji testów integracyjnych.

Wciąż możemy uruchomić konkretną funkcję testu integracyjnego, podając jej nazwę jako argument w poleceniu `cargo test`. Aby uruchomić wszystkie testy w konkretnym pliku testów integracyjnych, użyj argumentu `--test` polecenia `cargo test`, a następnie nazwę pliku (`cargo test --test integration_test`):

```text
running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

To polecenie uruchamia tylko testy w pliku *tests/integration_test.rs*.