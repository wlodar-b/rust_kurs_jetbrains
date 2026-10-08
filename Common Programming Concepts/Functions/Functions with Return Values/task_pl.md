## Funkcje zwracające wartości

Funkcje mogą zwracać wartości do kodu, który je wywołuje. Nie nadajemy nazw wartościom zwracanym, ale deklarujemy ich typ po strzałce `(->)`. W języku Rust wartość zwracana przez funkcję jest równoznaczna z wartością ostatniego wyrażenia w bloku ciała funkcji. Możesz zakończyć działanie funkcji wcześniej za pomocą słowa kluczowego `return` i podając wartość, ale większość funkcji zwraca ostatnie wyrażenie domyślnie. Oto przykład funkcji, która zwraca wartość:

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("Wartość x to: {}", x);
}
```

Funkcja `five` nie zawiera wywołań innych funkcji, makr ani nawet instrukcji `let`—jest w niej tylko liczba 5. Jest to w pełni poprawna funkcja w języku Rust. Zauważ, że również określono typ zwracanej wartości jako `-> i32`. Spróbuj uruchomić ten kod; wynik powinien wyglądać następująco:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30 secs
     Running `target/debug/functions`
Wartość x to: 5
```

Liczba `5` w funkcji `five` to wartość zwracana przez tę funkcję, dlatego typ zwracanej wartości to `i32`. Przyjrzyjmy się temu bardziej szczegółowo. Istnieją dwa istotne elementy: po pierwsze, linia `let x = five();` pokazuje, że używamy wartości zwracanej przez funkcję do zainicjowania zmiennej. Ponieważ funkcja `five` zwraca `5`, linia ta jest równoważna z następującą:

```rust
let x = 5;
```

Po drugie, funkcja `five` nie przyjmuje żadnych parametrów i określa typ wartości zwracanej, ale ciało funkcji to samotna liczba `5` bez średnika, ponieważ jest to wyrażenie, którego wartość chcemy zwrócić.

Spójrzmy na inny przykład:

```rust
fn main() {
    let x = plus_one(5);

    println!("Wartość x to: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

Uruchomienie tego kodu wyświetli `Wartość x to: 6`. Ale jeśli dodamy średnik na końcu linii zawierającej `x + 1`, zmieniając ją z wyrażenia na instrukcję, pojawi się błąd.

```rust
fn main() {
    let x = plus_one(5);

    println!("Wartość x to: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

Kompilacja tego kodu powoduje następujący błąd:

```text
error[E0308]: mismatched types
 --> src/main.rs:7:24
  |
7 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ oczekiwano `i32`, otrzymano `()`
  |    |
  |    niejawnie zwraca `()` ponieważ ciało funkcji nie zawiera końcowego wyrażenia lub instrukcji `return`
8 |     x + 1;
  |          - sugestia: rozważ usunięcie tego średnika
```

Główna wiadomość o błędzie, „niedopasowane typy”, wskazuje na główny problem w tym kodzie. Definicja funkcji `plus_one` mówi, że zwróci ona `i32`, ale instrukcje nie zwracają wartości, co jest wyrażone jako `()`, pusta krotka. Zatem nic nie jest zwracane, co jest sprzeczne z definicją funkcji i prowadzi do błędu. W tym wyniku Rust dostarcza komunikat, który może pomóc rozwiązać problem: sugeruje usunięcie średnika, co naprawiłoby błąd.

_Możesz odnieść się do następującego rozdziału w książce „The Rust Programming Language”: [Functions with Return Values](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html#functions-with-return-values)_.