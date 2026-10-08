## Podmoduły w testach integracyjnych

W miarę dodawania kolejnych testów integracyjnych, możesz chcieć utworzyć więcej niż jeden plik w katalogu *tests*, aby pomóc w ich organizacji; na przykład możesz grupować funkcje testujące według funkcjonalności, którą sprawdzają. Jak wspomniano wcześniej, każdy plik w katalogu *tests* jest kompilowany jako oddzielny crate.

Traktowanie każdego pliku testów integracyjnych jako osobnego crate'a jest przydatne do tworzenia oddzielnych zakresów, które lepiej odwzorowują sposób, w jaki użytkownicy końcowi będą korzystać z Twojego crate'a. Jednak oznacza to również, że pliki w katalogu *tests* nie dzielą tych samych zachowań, co pliki w *src*, co zauważyłeś w rozdziale "Moduły i Makra/Moduły" dotyczącym organizowania kodu w moduły i pliki.

Różnica w zachowaniu plików w katalogu *tests* jest najbardziej zauważalna, gdy masz zestaw pomocniczych funkcji, które byłyby przydatne w wielu plikach testów integracyjnych, i próbujesz zastosować kroki opisane w sekcji „Podział modułów na różne pliki” w rozdziale "Moduły i Makra/Moduły", aby wyodrębnić je do wspólnego modułu. Na przykład, jeśli utworzymy plik *tests/common.rs* i umieścimy w nim funkcję o nazwie `setup`, możemy dodać do niej kod, który chcemy wywoływać w wielu funkcjach testujących w różnych plikach testów:

```rust
pub fn setup() {
    // tutaj umieścisz kod przygotowawczy specyficzny dla testów Twojej biblioteki
}
```

Po ponownym uruchomieniu testów zobaczymy nową sekcję w wynikach testów dotyczącą pliku *common.rs*, mimo że plik ten nie zawiera żadnych funkcji testujących ani nie wywołaliśmy nigdzie funkcji `setup`:

```text
   Compiling test_organization v0.1.0
    Finished test [unoptimized + debuginfo] target(s) in 0.81s
     Running target/debug/deps/test_organization-61f5d8d60ccbcc19

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/common-b5e4eefa9d201089

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-5843d720c5feeb7a

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests test_organization

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Pojawienie się `common` w wynikach testów z wyświetlonym komunikatem `running 0 tests` nie jest tym, czego chcieliśmy. Chcieliśmy tylko współdzielić pewien kod z innymi plikami testów integracyjnych. W następnej sekcji dowiesz się, jak uniknąć pojawiania się `common` w wynikach testów i jak odpowiednio zorganizować testy.

Aby uniknąć pojawienia się `common` w wynikach testów, zamiast tworzyć plik *tests/common.rs*, utworzymy *tests/common/mod.rs*. Jest to alternatywna konwencja nazewnictwa, którą Rust również rozumie. Nadanie plikowi takiej nazwy mówi Rustowi, aby nie traktował modułu `common` jako pliku testów integracyjnych. Po przeniesieniu kodu funkcji `setup` do *tests/common/mod.rs* i usunięciu pliku *tests/common.rs*, sekcja w wynikach testów przestanie się pojawiać. Pliki w podkatalogach katalogu *tests* nie są kompilowane jako oddzielne crate'y ani nie posiadają sekcji w wynikach testów.

Po utworzeniu *tests/common/mod.rs* możemy korzystać z niego jako z modułu w dowolnych plikach testów integracyjnych. Oto przykład wywołania funkcji `setup` z testu `it_adds_two` w pliku *tests/integration_test.rs*:

```rust,ignore
use test_organization_part_2;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_organization_part_2::add_two(2));
}
```

Zwróć uwagę, że deklaracja `mod common;` jest taka sama, jak deklaracja modułu przedstawiona w przykładzie "Deklarowanie modułu front_of_house którego kod znajduje się w _src/front_of_house.rs" w sekcji "Podział modułów na różne pliki" w rozdziale "Moduły". Następnie w funkcji testującej możemy wywołać funkcję `common::setup()`.

Wynik działania `cargo test` po utworzeniu *tests/common/mod.rs* i wywołaniu funkcji `setup` z testu `it_adds_two` w pliku *tests/integration_test.rs*:

```text
Compiling submodules v0.1.0 
    Finished test [unoptimized + debuginfo] target(s) in 0.50s
     Running target/debug/deps/submodules-c44b35b673c8053d

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-31048908068047a2

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests submodules

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

##### Testy integracyjne dla crate'ów binarnych

Jeżeli Twój projekt jest crate'em binarnym, który zawiera jedynie plik *src/main.rs* i nie ma pliku *src/lib.rs*, nie możesz tworzyć testów integracyjnych w katalogu *tests* ani przenosić funkcji zdefiniowanych w pliku *src/main.rs* do zakresu za pomocą instrukcji `use`. Tylko crate'y biblioteczne udostępniają funkcje, które mogą być używane przez inne crate'y; crate'y binarne są przeznaczone do samodzielnego uruchamiania.

To jeden z powodów, dla których projekty Rust, które dostarczają binaria, posiadają prosty plik *src/main.rs*, który wywołuje logikę zawartą w *src/lib.rs*. Dzięki takiej strukturze testy integracyjne *mogą* testować crate biblioteczny za pomocą `use`, aby uzyskać dostęp do kluczowej funkcjonalności. Jeżeli kluczowa funkcjonalność działa, mała ilość kodu w pliku *src/main.rs* również zadziała poprawnie, a ten niewielki fragment kodu nie wymaga testowania.