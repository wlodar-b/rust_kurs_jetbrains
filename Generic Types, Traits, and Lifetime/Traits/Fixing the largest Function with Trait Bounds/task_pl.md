### Naprawa funkcji `largest` za pomocą ograniczeń na cechach

Skoro już wiesz, jak określić wymagane zachowanie przy użyciu ograniczeń na parametrach typu generycznego, wróćmy do [definicji funkcji `largest`](course://Generic+Types,+Traits,+and+Lifetime/Generic+Data+Types/In+Function+Definitions), która używa parametru typu generycznego! Ostatnim razem, gdy próbowaliśmy uruchomić ten kod, otrzymaliśmy następujący błąd:

```text
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  |             ^^^^^^^^^^^^^^^^^^^^^^
```

W ciele funkcji `largest` chcieliśmy porównać dwie wartości typu `T` za pomocą operatora większy niż (`>`). Ponieważ ten operator jest zdefiniowany jako metoda domyślna na cechy `std::cmp::PartialOrd` standardowej biblioteki, musimy określić `PartialOrd` jako ograniczenie cechy dla `T`, aby funkcja `largest` mogła działać na wycinkach dowolnego typu, który można porównać. Nie musimy wprowadzać `PartialOrd` do zakresu, ponieważ znajduje się w prelude. Zmień sygnaturę funkcji `largest`, aby wyglądała następująco:

```rust,ignore
fn largest<T: PartialOrd>(list: &[T]) -> T {
```

Tym razem po skompilowaniu kodu otrzymujemy inny zestaw błędów:

```console
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0508]: cannot move out of type `[T]`, a non-copy slice
 --> src/main.rs:2:23
  |
2 |     let mut largest = list[0];
  |                       ^^^^^^^
  |                       |
  |                       cannot move out of here
  |                       move occurs because `list[_]` has type `T`, which does not implement the `Copy` trait
  |                       help: consider borrowing here: `&list[0]`

error[E0507]: cannot move out of a shared reference
 --> src/main.rs:4:18
  |
4 |     for &item in list {
  |         -----    ^^^^
  |         ||
  |         |data moved here
  |         |move occurs because `item` has type `T`, which does not implement the `Copy` trait
  |         help: consider removing the `&`: `item`

Some errors have detailed explanations: E0507, E0508.
For more information about an error, try `rustc --explain E0507`.
error: could not compile `chapter10` due to 2 previous errors
```

Kluczowym fragmentem w tym błędzie jest `cannot move out of type [T], a non-copy slice`. W wersjach niegenerycznych funkcji `largest` próbowaliśmy znaleźć największy `i32` lub `char`. Jak omówiono w podsekcji “Dane jedynie na stosie: Copy” zadania [Clone and Copy](course://Understanding Ownership/What is ownership/Clone and Copy), typy takie jak `i32` oraz `char`, które mają znany rozmiar, mogą być przechowywane na stosie, więc implementują cechę `Copy`. Jednak gdy uczyniliśmy funkcję `largest` generyczną, stało się możliwe, że parametr `list` będzie zawierał typy, które nie implementują cechy `Copy`. W rezultacie, nie mogliśmy przenieść wartości z `list[0]` do zmiennej `largest`, co skutkowało tym błędem.

Aby wywołać ten kod jedynie dla typów, które implementują cechę `Copy`, możemy dodać `Copy` do ograniczeń cechy `T`! Poniższy fragment kodu pokazuje pełną implementację funkcji generycznej `largest`, która będzie kompilowana, o ile typy wartości w wycinku przekazanym do funkcji będą implementowały cechy `PartialOrd` *i* `Copy`, tak jak robią to `i32` oraz `char`.

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("Największa liczba to {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("Największy znak to {}", result);
}
```

#### Działająca definicja funkcji `largest`, która działa na dowolnym typie generycznym implementującym cechy `PartialOrd` i `Copy`

Jeśli nie chcemy ograniczać funkcji `largest` do typów implementujących cechę `Copy`, moglibyśmy określić, że `T` ma ograniczenie cechy `Clone` zamiast `Copy`. Następnie moglibyśmy kopiować każdą wartość w wycinku, gdy chcemy, aby funkcja `largest` miała własność. Użycie funkcji `clone` oznacza, że potencjalnie wykonywalibyśmy więcej operacji alokacji na stercie w przypadku typów, które posiadają dane na stercie, takich jak `String`, a operacje alokacji na stercie mogą być powolne, jeśli pracujemy z dużą ilością danych.

Innym sposobem implementacji `largest` jest zmiana funkcji tak, aby zwracała referencję do wartości typu `T` w wycinku. Gdybyśmy zmienili typ zwracany na `&T` zamiast `T`, zmieniając jednocześnie ciało funkcji, aby zwracała referencję, nie potrzebowalibyśmy ograniczeń cech `Clone` lub `Copy`, a także moglibyśmy uniknąć alokacji na stercie. Spróbuj zaimplementować te alternatywne rozwiązania we własnym zakresie!