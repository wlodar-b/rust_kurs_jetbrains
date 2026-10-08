## Używanie if w instrukcji let

Ponieważ `if` jest wyrażeniem, możemy użyć go po prawej stronie instrukcji `let`, jak pokazano poniżej:

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("Wartość zmiennej number to: {}", number);
}
```

##### Przykład przypisania wyniku wyrażenia if do zmiennej

Zmienna `number` zostanie powiązana z wartością w zależności od wyniku wyrażenia `if`. Uruchom ten kod, aby zobaczyć, co się stanie:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/branches`
Wartość zmiennej number to: 5
```

Pamiętaj, że bloki kodu są oceniane na podstawie ostatniego wyrażenia w nich, a same liczby również są wyrażeniami. W tym przypadku wartość całego wyrażenia `if` zależy od tego, który blok kodu zostanie wykonany. Oznacza to, że wartości z potencjalnym wynikiem z każdej gałęzi `if` muszą być tego samego typu; w poprzednim fragmencie kodu wyniki zarówno gałęzi `if`, jak i `else` były liczbami całkowitymi typu `i32`. Jeśli typy są niezgodne, jak w poniższym przykładzie, pojawi się błąd:

```rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("Wartość zmiennej number to: {}", number);
}
```

Kiedy spróbujemy skompilować ten kod, otrzymamy błąd. Gałęzie `if` i `else` mają typy wartości, które są niezgodne, a Rust wskazuje dokładnie, gdzie znaleźć problem w programie:

```text
error[E0308]: `if` i `else` mają niekompatybilne typy
 --> src/main.rs:4:44
  |
4 |     let number = if condition { 5 } else { "six" };
  |                                 -          ^^^^^ oczekiwano liczby całkowitej, znaleziono `&str`
  |                                 |
  |                                 oczekiwano na podstawie tego
```

Wyrażenie w bloku `if` jest oceniane jako liczba całkowita, a wyrażenie w bloku `else` jako ciąg znaków. To nie zadziała, ponieważ zmienne muszą mieć jeden typ. Rust musi znać w czasie kompilacji, jaki typ ma zmienna `number`, jednoznacznie, aby mógł zweryfikować w czasie kompilacji, że jej typ jest poprawny wszędzie, gdzie używamy `number`. Rust nie byłby w stanie tego zrobić, jeśli typ zmiennej `number` byłby określany dopiero w czasie wykonywania; kompilator byłby bardziej skomplikowany i zapewniałby mniej gwarancji dotyczących kodu, gdyby musiał śledzić wiele hipotetycznych typów dla dowolnej zmiennej.

_Możesz odnieść się do następującego rozdziału w książce o języku Rust: [Using if in a let Statement](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#using-if-in-a-let-statement)_