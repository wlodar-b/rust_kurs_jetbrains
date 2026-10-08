## Iteracja przez zakresy za pomocą for

Bezpieczeństwo i zwięzłość pętli `for` sprawiają, że są one najczęściej używaną konstrukcją pętli w języku Rust. Nawet w sytuacjach, w których chcesz uruchomić kod określoną liczbę razy, jak w przykładzie odliczania, który wykorzystywał pętlę `while` w sekcji "Przykład użycia pętli while do wykonywania kodu, gdy warunek jest spełniony", większość Rustacean zastosowałaby pętlę `for`. Można to zrobić za pomocą typu `Range`, dostarczanego przez bibliotekę standardową, który generuje wszystkie liczby w sekwencji, zaczynając od jednej liczby i kończąc przed drugą liczbą.

Oto jak wyglądałoby odliczanie z użyciem pętli `for` oraz innej metody, której jeszcze nie omawialiśmy – `rev`, do odwracania zakresu:

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

Ten kod jest trochę przyjemniejszy, prawda?