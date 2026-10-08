## Warunki

Decydowanie, czy kod powinien zostać wykonany w zależności od tego, czy warunek jest spełniony, to podstawowy element w większości języków programowania. Najczęściej używane konstrukcje, które pozwalają kontrolować przepływ wykonywania kodu w języku Rust, to wyrażenia `if`.

### Wyrażenia if

Wyrażenie `if` pozwala na rozgałęzianie kodu w zależności od spełnienia określonych warunków. Podajesz warunek, a następnie wskazujesz: „Jeśli ten warunek jest spełniony, wykonaj ten blok kodu. Jeżeli warunek nie jest spełniony, nie wykonuj tego bloku kodu.”

W pliku _src/main.rs_ wprowadź następujący kod:

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

Wszystkie wyrażenia `if` zaczynają się od słowa kluczowego `if`, po którym znajduje się warunek. W tym przypadku warunek sprawdza, czy zmienna `number` ma wartość mniejszą niż 5. Blok kodu, który chcemy wykonać, gdy warunek jest spełniony, umieszczamy bezpośrednio za warunkiem w nawiasach klamrowych. Bloki kodu związane z warunkami w wyrażeniach `if` często nazywane są gałęziami, podobnie jak gałęzie w wyrażeniach `match`.

Opcjonalnie możemy również dołączyć wyrażenie `else`, co też zrobiliśmy tutaj, aby przekazać programowi alternatywny blok kodu do wykonania, jeśli warunek zwróci wartość false. Jeśli nie podamy wyrażenia `else` i warunek będzie fałszywy, program po prostu pominie blok `if` i przejdzie do następnego fragmentu kodu.

Spróbuj uruchomić ten kod; powinieneś zobaczyć następujący wynik:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/branches`
condition was true
```

Spróbujmy zmienić wartość `number` na taką, która powoduje, że warunek stanie się fałszywy, aby zobaczyć, co się stanie:

```rust
let number = 7;
```

Uruchom program ponownie i spójrz na wynik:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/branches`
condition was false
```

Warto również zauważyć, że warunek w tym kodzie musi być typu `bool`. Jeśli warunek nie będzie typu `bool`, pojawi się błąd. Na przykład spróbuj uruchomić następujący kod:

```rust
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```

Wyrażenie `if` tym razem ocenia warunek na wartość `3`, a Rust zgłasza błąd:

```text
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected `bool`, found integer

error: aborting due to previous error
```

Błąd oznacza, że Rust oczekiwał `bool`, ale otrzymał liczbę całkowitą. W przeciwieństwie do języków takich jak Ruby czy JavaScript, Rust nie próbuje automatycznie konwertować typów innych niż logiczne na typ logiczny. Musisz być jednoznaczny i zawsze dostarczyć wyrażeniu `if` wartość logiczną jako warunek. Jeśli chcemy, aby blok kodu `if` został wykonany tylko wtedy, gdy liczba nie jest równa `0`, możemy zmienić wyrażenie `if` na następujące:

```rust
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}
```

Uruchomienie tego kodu wyświetli `number was something other than zero`.

_Możesz odwołać się do następującego rozdziału w książce o języku Rust: [Control Flow - if Expressions](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#if-expressions)_