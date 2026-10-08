## Obsługa wielu warunków za pomocą else if

Możesz obsługiwać wiele warunków, łącząc `if` i `else` w wyrażeniu `else if`. Na przykład:

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("liczba jest podzielna przez 4");
    } else if number % 3 == 0 {
        println!("liczba jest podzielna przez 3");
    } else if number % 2 == 0 {
        println!("liczba jest podzielna przez 2");
    } else {
        println!("liczba nie jest podzielna przez 4, 3 ani 2");
    }
}
```

Ten program może wykonać jedną z czterech możliwych ścieżek. Po uruchomieniu zobaczysz następujący wynik:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/branches`
liczba jest podzielna przez 3
```

Kiedy ten program się wykonuje, sprawdza każde wyrażenie `if` po kolei i wykonuje pierwszy blok kodu, dla którego warunek jest spełniony. Zwróć uwagę, że mimo iż liczba 6 jest podzielna przez 2, nie widzimy komunikatu 'liczba jest podzielna przez 2', ani tekstu 'liczba nie jest podzielna przez 4, 3 ani 2' z bloku `else`. Dzieje się tak, ponieważ Rust wykonuje blok tylko dla pierwszego spełnionego warunku i pomija sprawdzanie pozostałych po znalezieniu jednego.

Użycie zbyt wielu wyrażeń `else if` może zaśmiecić kod, więc jeśli masz ich więcej niż jedno, warto rozważyć refaktoryzację kodu. [Rozdział 6](https://doc.rust-lang.org/stable/book/ch06-00-enums.html) opisuje potężną konstrukcję rozgałęzień w Rust o nazwie `match`, która nadaje się do takich przypadków.

_Możesz odnieść się do następującego rozdziału w książce „The Rust Programming Language”: [Obsługa wielu warunków za pomocą else if](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#handling-multiple-conditions-with-else-if)_