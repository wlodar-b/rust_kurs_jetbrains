#### Operator `?` można używać tylko w funkcjach zwracających `Result`

Operator `?` można stosować jedynie w funkcjach, których typ zwracany to `Result`, ponieważ został on zdefiniowany tak, aby działał w ten sam sposób, co wyrażenie `match`, które omówiliśmy na przykładzie powodującym błąd w kodzie przy użyciu `match`. Częścią `match`, która wymaga zwracania `Result`, jest `return Err(e)`. Dlatego typ zwracany funkcji musi być `Result`, aby był kompatybilny z tą formą `return`.

Spójrzmy, co się stanie, jeśli użyjemy operatora `?` w funkcji `main`, która, jak zapewne pamiętasz, ma typ zwracany `()`:

```rust
    use std::fs::File;

    fn main() {
        let f = File::open("hello.txt")?;
    }
```

Podczas kompilacji tego kodu otrzymamy następujący komunikat błędu:

```text
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `Try`)
 --> src/main.rs:4:13
  |
3 | / fn main() {
4 | |     let f = File::open("hello.txt")?;
  | |             ^^^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?` operator in a function that returns `()`
5 | | }
  | |_- this function should return `Result` or `Option` to accept `?`
  |
  = help: the trait `Try` is not implemented for `()`
  = note: required by `from_error`
```

Błąd ten wskazuje, że możemy używać operatora `?` jedynie w funkcji, która zwraca `Result` lub `Option` albo inny typ implementujący `std::ops::Try`. Jeśli piszesz kod w funkcji, która nie zwraca jednego z tych typów, a chcesz użyć operatora `?` podczas wywoływania innych funkcji zwracających `Result<T, E>`, masz dwie możliwości naprawienia tego problemu. Jedną z technik jest zmiana typu zwracanego Twojej funkcji na `Result<T, E>`, jeśli nie ma żadnych ograniczeń uniemożliwiających to. Drugą techniką jest użycie wyrażenia `match` lub jednej z metod `Result<T, E>` w celu obsłużenia `Result<T, E>` w odpowiedni sposób.

Funkcja `main` jest wyjątkowa i istnieją pewne ograniczenia dotyczące możliwego typu zwracanego. Jednym z dopuszczalnych typów zwracanych dla `main` jest `()`, a wygodnie, innym dopuszczalnym typem zwracanym jest `Result<T, E>`, jak pokazano tutaj:

```rust
    use std::error::Error;
    use std::fs::File;

    fn main() -> Result<(), Box<dyn Error>> {
        let f = File::open("hello.txt")?;

        Ok(())
    }
```

Typ `Box<dyn Error>` nazywa się _obiektem cechy_ (ang. _trait object_), co omówiono w rozdziale 17 w sekcji [„Korzystanie z obiektów cech, które umożliwiają wartości różnych typów”](https://doc.rust-lang.org/stable/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types). Na razie możesz traktować `Box<dyn Error>` jako określenie „dowolnego rodzaju błędu”. Użycie operatora `?` w funkcji `main` z tym typem zwracanym jest dozwolone.

Teraz, gdy omówiliśmy szczegóły dotyczące wywoływania `panic!` lub zwracania `Result`, wróćmy do tematu, jak zdecydować, które podejście należy zastosować w określonych przypadkach.

_Możesz odnieść się do rozdziału w książce „Język programowania Rust”: [Błędy odzyskiwalne za pomocą Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#recoverable-errors-with-result)_.