### Testowanie równości za pomocą makr assert_eq! i assert_ne!

Powszechnym sposobem testowania funkcjonalności jest porównywanie wyniku kodu, który testujemy, z oczekiwanym wynikiem, aby upewnić się, że są sobie równe. Można to zrobić za pomocą makra `assert!`, przekazując do niego wyrażenie z operatorem `==`. Jednakże, ponieważ jest to tak częsta technika testowania, biblioteka standardowa udostępnia parę makr: `assert_eq!` i `assert_ne!`, które umożliwiają przeprowadzanie takich testów w bardziej wygodny sposób. Te makra porównują dwa argumenty pod względem równości lub nierówności odpowiednio. Dodatkowo, jeśli asercja nie powiedzie się, wyświetlają oba wartości, co ułatwia zrozumienie _dlaczego_ test nie przeszedł; z kolei makro `assert!` jedynie wskazuje, że wartość wyrażenia `==` była `false`, nie podając wartości, które doprowadziły do tego wyniku.

W poniższym fragmencie kodu piszemy funkcję o nazwie `add_two`, która dodaje `2` do swojego parametru i zwraca wynik. Następnie testujemy tę funkcję za pomocą makra `assert_eq!`.

```rust
    pub fn add_two(a: i32) -> i32 {
        a + 2
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_adds_two() {
            assert_eq!(4, add_two(2));
        }
    }
```

##### Przykład testowania funkcji add_two za pomocą makra assert_eq!

Sprawdźmy, czy test przechodzi!

```text
    running 1 test
    test tests::it_adds_two ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Pierwszy argument przekazany do makra `assert_eq!`, `4`, jest równy wynikowi wywołania `add_two(2)`. Linia dotycząca tego testu to `test tests::it_adds_two ... ok`, a tekst `ok` wskazuje, że nasz test przeszedł!

Wprowadźmy teraz błąd do naszego kodu, aby zobaczyć, jak wygląda sytuacja, gdy test z użyciem `assert_eq!` nie przechodzi. Zmień implementację funkcji `add_two`, aby dodawała `3` zamiast `2`:

```rust
    pub fn add_two(a: i32) -> i32 {
        a + 3
    }
```

Uruchommy ponownie testy:

```text
running 1 test
test tests::it_adds_two ... FAILED

failures:

---- tests::it_adds_two stdout ----
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `4`,
 right: `5`', src/lib.rs:11:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::it_adds_two

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Nasz test wykrył błąd! Test `it_adds_two` nie przeszedł, wyświetlając komunikat `assertion failed: '(left == right)'` i wskazując, że `left` wynosiło `4`, a `right` było `5`. Ten komunikat jest przydatny i pomaga rozpocząć debugowanie: oznacza to, że argument `left` do `assert_eq!` wynosił `4`, ale argument `right`, gdzie mieliśmy `add_two(2)`, wynosił `5`.

Zwróć uwagę, że w niektórych językach programowania i frameworkach testowych parametry funkcji sprawdzających równość dwóch wartości nazywane są `expected` i `actual`, a kolejność, w jakiej podajemy argumenty, ma znaczenie. W Rust te parametry nazywane są `left` i `right`, a kolejność, w jakiej podajemy wartości oczekiwane oraz wynik z testowanego kodu, nie ma znaczenia. Możemy zapisać asercję w tym teście jako `assert_eq!(add_two(2), 4)`, co spowoduje komunikat błędu o treści `assertion failed: '(left == right)'`, z informacją, że `left` wynosiło `5`, a `right` było `4`.

Makro `assert_ne!` przejdzie test, jeśli podane dwie wartości są różne, a test zakończy się niepowodzeniem, jeśli będą równe. To makro jest najbardziej przydatne w przypadkach, gdy nie jesteśmy pewni, jaka wartość _będzie_ wynikiem, ale wiemy, jaka wartość na pewno _nie_ powinna wystąpić, jeśli nasz kod działa zgodnie z założeniami. Przykładowo, jeśli testujemy funkcję, która gwarantuje zmianę swojego wejścia w jakiś sposób, ale sposób tej zmiany zależy od dnia tygodnia, w którym uruchamiamy testy, najlepszym sposobem na sprawdzenie może być potwierdzenie, że wynik działania funkcji nie jest równy wejściu.

Pod powierzchnią makra `assert_eq!` i `assert_ne!` używają operatorów `==` i `!=`, odpowiednio. Gdy asercje nie przejdą, te makra drukują swoje argumenty przy użyciu formatowania debugowania, co oznacza, że porównywane wartości muszą implementować cechy `PartialEq` i `Debug`. Wszystkie typy prymitywne oraz większość typów z biblioteki standardowej implementują te cechy. W przypadku struktur (structs) i wyliczeń (enums), które samodzielnie definiujesz, musisz zaimplementować `PartialEq`, aby móc sprawdzić, czy wartości tych typów są równe lub różne. Musisz także zaimplementować `Debug`, aby wydrukować wartości w przypadku, gdy asercja się nie powiedzie. Ponieważ obie te cechy są cechami, które można automatycznie wyprowadzić (derivable traits), jak wspomniano w liście "Dodawanie adnotacji do wyprowadzenia cechy `Debug` i drukowanie instancji `Rectangle` za pomocą formatowania debugowania" w rozdziale "Struktury", sekcja "Przykładowe struktury", jest to zazwyczaj tak proste, jak dodanie adnotacji `#[derive(PartialEq, Debug)]` do definicji swojej struktury lub wyliczenia. Szczegóły na temat tych oraz innych cech możliwych do wyprowadzenia znajdziesz w załączniku C, [„Cechy możliwe do wyprowadzenia,”](https://doc.rust-lang.org/stable/book/appendix-03-derivable-traits.html).