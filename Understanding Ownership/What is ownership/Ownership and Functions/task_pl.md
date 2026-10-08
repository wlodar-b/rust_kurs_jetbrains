## Własność i funkcje

Semantyka przekazywania wartości do funkcji jest podobna do przypisywania wartości do zmiennej. Przekazanie zmiennej do funkcji spowoduje przeniesienie lub skopiowanie jej wartości, tak jak w przypadku przypisania. Poniższy przykład zawiera adnotacje pokazujące, gdzie zmienne wchodzą i wychodzą z zakresu.

```rust
fn main() {
    let s = String::from("hello");  // s wchodzi w zakres

    takes_ownership(s);             // wartość s zostaje przeniesiona do funkcji...
    // ... i dlatego nie jest już tu dostępna

    let x = 5;                      // x wchodzi w zakres

    makes_copy(x);                  // x zostałby przeniesiony do funkcji,
    // ale i32 jest typu Copy, więc można
    // nadal używać x później

} // Tutaj, x wychodzi z zakresu, a potem s. Ale ponieważ wartość s została przeniesiona, 
// nic szczególnego się nie dzieje.

fn takes_ownership(some_string: String) { // some_string wchodzi w zakres
    println!("{}", some_string);
} // Tutaj, some_string wychodzi z zakresu i wywoływana jest funkcja `drop`. Pamięć
// rezerwowa zostaje zwolniona.

fn makes_copy(some_integer: i32) { // some_integer wchodzi w zakres
    println!("{}", some_integer);
} // Tutaj, some_integer wychodzi z zakresu. Nic szczególnego się nie dzieje.
```

##### Funkcje z adnotacjami dotyczącymi własności i zakresu

Jeśli spróbowalibyśmy użyć `s` po wywołaniu funkcji `takes_ownership`, Rust zgłosiłby błąd kompilacji. Te statyczne kontrole chronią nas przed błędami. Spróbuj dodać kod do funkcji `main`, który używa `s` i `x`, aby zobaczyć, gdzie można ich używać, a gdzie zasady dotyczące własności uniemożliwiają ich użycie.