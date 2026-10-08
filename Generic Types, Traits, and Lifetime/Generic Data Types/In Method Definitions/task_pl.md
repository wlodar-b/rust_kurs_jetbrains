### W definicjach metod

Możemy implementować metody na strukturach i enumeracjach (jak to zrobiliśmy w rozdziale "Struktury") oraz używać typów generycznych w ich definicjach. Poniższy fragment kodu pokazuje strukturę `Point<T>`, którą zdefiniowaliśmy wcześniej, z zaimplementowaną metodą o nazwie `x`.

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

#### Implementacja metody nazwanej `x` w strukturze `Point<T>`, która zwróci referencję do pola `x` typu `T`.

Tutaj zdefiniowaliśmy metodę nazwaną `x` dla `Point<T>`, która zwraca referencję do danych w polu `x`.

Zwróć uwagę, że musimy zadeklarować `T` zaraz po `impl`, abyśmy mogli wskazać, że implementujemy metody dla typu `Point<T>`. Deklarując `T` jako typ generyczny po `impl`, Rust może rozpoznać, że typ w nawiasach kątowych w `Point` jest typem generycznym, a nie konkretnym typem.

Możemy na przykład zaimplementować metody wyłącznie dla instancji `Point<f32>` zamiast dla `Point<T>` z dowolnym typem generycznym. Poniżej używamy konkretnego typu `f32`, co oznacza, że nie deklarujemy żadnych typów po `impl`.

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

#### Blok `impl`, który dotyczy wyłącznie struktury z określonym konkretnym typem dla generycznego parametru typu `T`.

Kod ten oznacza, że typ `Point<f32>` będzie miał metodę nazwaną `distance_from_origin`, a inne instancje `Point<T>`, gdzie `T` nie jest typu `f32`, nie będą miały tej metody zdefiniowanej. Metoda mierzy, jak daleko nasz punkt znajduje się od punktu o współrzędnych (0.0, 0.0) i wykorzystuje operacje matematyczne dostępne tylko dla typów zmiennoprzecinkowych.