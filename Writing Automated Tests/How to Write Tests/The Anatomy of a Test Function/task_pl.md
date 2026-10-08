## Jak pisać testy

Testy w języku Rust to funkcje, które sprawdzają, czy kod produkcyjny działa zgodnie z oczekiwaniami. Treść funkcji testowych zazwyczaj obejmuje trzy kroki:

1. Przygotowanie potrzebnych danych lub stanu.
2. Uruchomienie kodu, który chcemy przetestować.
3. Sprawdzenie, czy wyniki są zgodne z oczekiwaniami.

Przyjrzyjmy się funkcjom, które Rust oferuje specjalnie do pisania testów realizujących te kroki, w tym atrybutowi `test`, kilku makrom oraz atrybutowi `should_panic`.

### Budowa funkcji testowej

W najprostszej postaci test w języku Rust to funkcja oznaczona atrybutem `test`. Atrybuty to metadane dotyczące elementów kodu w Rust; jednym z przykładów jest atrybut `derive`, którego używaliśmy ze strukturami w Rozdziale 5. Aby zamienić funkcję w funkcję testową, należy dodać `#[test]` w linii bezpośrednio przed deklaracją `fn`. Gdy uruchamiasz testy za pomocą polecenia `cargo test`, Rust tworzy binarkę uruchamiającą testy, która wykonuje funkcje oznaczone atrybutem `test` i informuje, czy każda funkcja testowa zakończyła się sukcesem, czy porażką.

Gdy tworzymy nowy projekt biblioteki za pomocą Cargo, automatycznie generowany jest moduł testowy zawierający funkcję testową. Ten moduł pozwala od razu zacząć pisać testy, bez konieczności sprawdzania struktury i składni funkcji testowych za każdym razem, gdy zaczynamy nowy projekt. Możemy dodawać dowolną liczbę dodatkowych funkcji testowych i modułów testowych.

Zgłębimy działanie testów, eksperymentując z wygenerowanym szablonem testu, bez rzeczywistego testowania kodu produkcyjnego. Następnie napiszemy prawdziwe testy wywołujące fragmenty naszego kodu i sprawdzimy poprawność jego działania.

Zmieńmy zawartość pliku _src/lib.rs_ tak, aby wyglądała jak poniższy fragment kodu:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

##### Przykład modułu testowego i funkcji automatycznie generowanych przez cargo new

Na razie pomińmy dwie pierwsze linie i skupmy się na samej funkcji, aby zrozumieć, jak działa. Zauważ adnotację `#[test]` przed linią `fn`: ten atrybut informuje, że jest to funkcja testowa, dzięki czemu wykonujący testy wie, że ma ją traktować jako test. W module `tests` moglibyśmy mieć również funkcje pomocnicze, które ułatwiają tworzenie wspólnych scenariuszy lub operacji, dlatego musimy oznaczać funkcje testowe atrybutem `#[test]`.

Ciało funkcji korzysta z makra `assert_eq!`, aby sprawdzić, czy suma 2 + 2 rzeczywiście wynosi 4. To sprawdzenie stanowi przykład formatu typowego testu. Uruchommy ten kod, aby zobaczyć, że test przechodzi pomyślnie.

Kliknij prawym przyciskiem myszy zadanie 'Jak pisać testy', wybierz **Open in Terminal** i uruchom polecenie `cargo test`. Powinieneś zobaczyć wynik podobny do poniższego:

```text
$ cargo test
  Compiling how_to_write_tests v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.38s
     Running target/debug/deps/intro-c8e247c4dd65e48f

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

##### Przykład wyniku uruchomienia automatycznie wygenerowanego testu

Cargo skompilowało i uruchomiło test. Po liniach `Compiling`, `Finished` i `Running` pojawia się linia `running 1 test`. Następnie widzimy nazwę wygenerowanej funkcji testowej, czyli `it_works`, oraz wynik tego testu: `ok`. Ogólne podsumowanie działania testów pokazuje się na końcu. Tekst `test result: ok.` oznacza, że wszystkie testy zakończyły się sukcesem, a część `1 passed; 0 failed` podaje liczbę testów, które przeszły lub zawiodły.

Ponieważ nie mamy żadnych testów oznaczonych jako zignorowane, podsumowanie pokazuje `0 ignored`. Nie zastosowaliśmy również filtrowania testów, więc na końcu podsumowania widnieje `0 filtered out`. O ignorowaniu i filtrowaniu testów opowiemy w sekcji "Uruchamianie testów".

Statystyka `0 measured` dotyczy testów benchmarkowych sprawdzających wydajność. Testy benchmarkowe są w momencie pisania tego tekstu dostępne wyłącznie w nocnej wersji Rust. Więcej informacji znajdziesz w [dokumentacji dotyczącej testów benchmarkowych](https://doc.rust-lang.org/unstable-book/library-features/test.html).

Kolejna część wyniku testów, rozpoczynająca się od `Doc-tests how_to_write_tests`, dotyczy wyników testów dokumentacyjnych. Nie mamy jeszcze żadnych takich testów, ale Rust jest w stanie kompilować przykłady kodu zamieszczone w dokumentacji API. Ta funkcja pomaga utrzymywać spójność dokumentacji i kodu. Tworzenie testów dokumentacyjnych omówimy w sekcji [„Komentarze dokumentacyjne jako testy”](https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests) Rozdziału 14 księgi Rust. Na razie zignorujemy wyniki `Doc-tests`.

Zmieńmy nazwę naszego testu, aby zobaczyć, jak to wpływa na wynik testów. Zmień nazwę funkcji `it_works` na inną, na przykład `exploration`, jak poniżej:

```rust
    #[cfg(test)]
    mod tests {
        #[test]
        fn exploration() {
            assert_eq!(2 + 2, 4);
        }
    }
```

Następnie ponownie uruchom `cargo test`. Wynik teraz pokazuje `exploration` zamiast `it_works`:

```text
Compiling how_to_write_tests v0.1.0
   Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running target/debug/deps/intro-c8e247c4dd65e48f

running 1 test
test tests::exploration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Dodajmy kolejny test, tym razem taki, który zakończy się niepowodzeniem! Testy nie przechodzą pomyślnie, jeśli coś w funkcji testowej wywołuje panikę. Każdy test uruchamiany jest w nowym wątku, a gdy główny wątek wykryje, że wątek testowy się zakończył, test oznaczany jest jako nieudany. Najprostszy sposób na wywołanie paniki, omówiony w Rozdziale 9, to użycie makra `panic!`. Wprowadź nowy test, `another`, aby Twój plik _src/lib.rs_ wyglądał tak:

```rust
    #[cfg(test)]
    mod tests {
        #[test]
        fn exploration() {
            assert_eq!(2 + 2, 4);
        }

        #[test]
        fn another() {
            panic!("Make this test fail");
        }
    }
```

##### Przykład dodania drugiego testu, który zakończy się niepowodzeniem, ponieważ wywołuje makro panic!

Uruchom test ponownie za pomocą `cargo test`. Wynik powinien wyglądać jak poniżej:

```text
Compiling how_to_write_tests v0.1.0
   Finished dev [unoptimized + debuginfo] target(s) in 0.34s
     Running target/debug/deps/intro-c8e247c4dd65e48f

running 2 tests
test tests::exploration ... ok
test tests::another ... FAILED

failures:

---- tests::another stdout ----
thread 'tests::another' panicked at 'Make this test fail', Writing Automated Tests/Tests/How to Write Tests/src/lib.rs:9:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

##### Przykład wyniku testów, gdy jeden test przechodzi, a drugi kończy się błędem

Zamiast `ok`, linia `test tests::another` pokazuje `FAILED`. Między poszczególnymi wynikami a podsumowaniem pojawiają się dwie nowe sekcje: pierwsza pokazuje szczegółowy powód każdego nieudanego testu. W tym przypadku test `another` zakończył się niepowodzeniem, ponieważ `panicked at "Make this test fail"`, co miało miejsce w linii 10 pliku _src/lib.rs_. Druga sekcja wypisuje tylko nazwy wszystkich nieudanych testów, co jest przydatne, gdy mamy dużo testów i dużo szczegółowych komunikatów o błędach. Nazwę nieudanego testu możemy użyć do uruchomienia tylko tego testu, aby łatwiej go debugować; więcej na ten temat znajdziesz w sekcji "Uruchamianie testów".

W podsumowaniu na końcu widzimy, że ogólny wynik testów to `FAILED`. Jeden test się powiódł, a drugi zakończył się niepowodzeniem.

Teraz, gdy widziałeś, jakie wyniki dają testy w różnych scenariuszach, przyjrzyjmy się innym makrom niż `panic!`, które są przydatne w testach.