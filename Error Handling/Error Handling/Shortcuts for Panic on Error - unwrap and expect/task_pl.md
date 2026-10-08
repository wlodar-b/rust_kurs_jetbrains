### Skróty do paniki w przypadku błędu: `unwrap` i `expect`

Używanie `match` działa wystarczająco dobrze, ale może być nieco rozbudowane i nie zawsze jasno przekazuje zamiar. Typ `Result<T, E>` ma wiele zdefiniowanych metod pomocniczych do wykonywania różnych zadań. Jedną z tych metod jest `unwrap`, która jest skróconą metodą zaimplementowaną dokładnie tak, jak wyrażenie `match`, o którym napisaliśmy w rozdziale "Użycie wyrażenia match do obsługi wariantów Result, które mogą zostać zwrócone". Jeśli wartość `Result` jest wariantem `Ok`, `unwrap` zwraca wartość wewnątrz `Ok`. Jeśli wartość `Result` jest wariantem `Err`, `unwrap` wywołuje makro `panic!`. Oto przykład działania `unwrap`:

```rust
    use std::fs::File;

    fn main() {
        let f = File::open("hello.txt").unwrap();
    }
```

Jeśli uruchomimy ten kod bez pliku _hello.txt_, zobaczymy komunikat błędu z wywołania `panic!`, które wykonuje metoda `unwrap`:

```text
    thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
    repr: Os { code: 2, message: "No such file or directory" } }',
    src/libcore/result.rs:906:4
```

Inna metoda, `expect`, podobna do `unwrap`, pozwala nam również określić komunikat błędu dla `panic!`. Korzystanie z `expect` zamiast `unwrap` i zapewnienie odpowiednich komunikatów o błędach może lepiej przekazać zamiar i ułatwić lokalizację źródła paniki. Składnia `expect` wygląda tak:

```rust
    use std::fs::File;

    fn main() {
        let f = File::open("hello.txt").expect("Failed to open hello.txt");
    }
```

Używamy `expect` w taki sam sposób jak `unwrap`: aby zwrócić uchwyt pliku lub wywołać makro `panic!`. Komunikat o błędzie używany przez `expect` w jego wywołaniu `panic!` będzie parametrem, który przekazaliśmy do `expect`, zamiast domyślnego komunikatu `panic!`, który wykorzystuje `unwrap`. Oto jak to wygląda:

```text
    thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
    2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```

Ponieważ ten komunikat o błędzie zaczyna się od tekstu, który określiliśmy, `Failed to open hello.txt`, łatwiej będzie znaleźć, skąd w kodzie pochodzi ten komunikat o błędzie. Jeśli używamy `unwrap` w wielu miejscach, może zająć więcej czasu ustalenie, które dokładnie wywołanie `unwrap` powoduje panikę, ponieważ wszystkie wywołania `unwrap`, które prowadzą do paniki, drukują ten sam komunikat.