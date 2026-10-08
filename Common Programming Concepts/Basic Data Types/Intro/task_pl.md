## Typy danych

Każda wartość w Rust ma określony _typ danych_, który informuje Rust, jaki rodzaj danych jest określony, dzięki czemu wie, jak z nimi pracować. W tej lekcji przyjrzymy się typom skalarowym, które reprezentują pojedynczą wartość.

Należy pamiętać, że Rust jest _językiem statycznie typowanym_, co oznacza, że musi znać typy wszystkich zmiennych w czasie kompilacji. Kompilator zazwyczaj jest w stanie wywnioskować, jakiego typu chcemy użyć, na podstawie wartości i sposobu jej wykorzystania. W przypadkach, gdy możliwych jest wiele typów, na przykład gdy konwertujemy `String` na typ numeryczny używając `parse`, musimy dodać adnotację typu, taką jak ta:

```rust
let guess: u32 = "42".parse().expect("Nie jest liczbą!");
```

Jeśli tutaj nie dodamy adnotacji typu, Rust wyświetli następujący błąd, co oznacza, że kompilator potrzebuje od nas więcej informacji, aby wiedzieć, jakiego typu chcemy użyć:

```text
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Nie jest liczbą!");
  |         ^^^^^
  |         |
  |         nie można wywnioskować typu dla `_`
  |         rozważ dodanie typu do `guess`
```

Zobaczysz różne adnotacje typów dla innych typów danych.

### Typy skalarowe

Typ _skalarowy_ reprezentuje pojedynczą wartość. Rust posiada cztery główne typy skalarowe: liczby całkowite, liczby zmiennoprzecinkowe, wartości logiczne (Boole) oraz znaki (characters). Możliwe, że rozpoznasz je z innych języków programowania. Przyjrzyjmy się, jak działają w Rust.