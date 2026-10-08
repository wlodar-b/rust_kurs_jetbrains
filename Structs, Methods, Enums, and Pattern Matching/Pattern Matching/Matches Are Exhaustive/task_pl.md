### Dopasowania muszą być wyczerpujące

Jest jeszcze jeden aspekt `match`, który musimy omówić. Spójrzmy na tę wersję funkcji `plus_one`, która zawiera błąd i nie skompiluje się:

```rust,ignore,does_not_compile
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

Nie uwzględniliśmy przypadku `None`, więc ten kod spowoduje wystąpienie błędu. Na szczęście jest to błąd, który Rust potrafi wykryć. Jeśli spróbujemy skompilować ten kod, otrzymamy następujący błąd:

```console
error[E0004]: non-exhaustive patterns: `None` not covered
   --> src/main.rs:3:15
    |
3   |         match x {
    |               ^ pattern `None` not covered
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `Option<i32>`
```

Rust wie, że nie uwzględniliśmy wszystkich możliwych przypadków, a nawet wie, który wzorzec pominęliśmy! Dopasowania w Rust są *wyczerpujące*: musimy uwzględnić każdą możliwą sytuację, aby kod był poprawny. Zwłaszcza w przypadku `Option<T>`, kiedy Rust uniemożliwia nam zapomnienie o jawnej obsłudze przypadku `None`, chroni nas przed założeniem, że mamy wartość, gdy w rzeczywistości jest to null. Dzięki temu popełnienie błędu za miliard dolarów, omawianego wcześniej, staje się niemożliwe.