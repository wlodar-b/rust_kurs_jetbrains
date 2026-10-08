### W definicjach struktur

Możemy także definiować struktury tak, aby używały parametru typu generycznego w jednym lub więcej polach, używając składni `<>`. Kod poniżej pokazuje, jak zdefiniować strukturę `Point<T>` do przechowywania wartości współrzędnych `x` i `y` dowolnego typu.

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

#### Struktura `Point<T>` przechowująca wartości `x` i `y` typu `T`

Składnia używania generyków w definicjach struktur jest podobna do tej używanej w definicjach funkcji. Najpierw deklarujemy nazwę parametru typu w nawiasach trójkątnych bezpośrednio po nazwie struktury. Następnie możemy użyć generycznego typu w definicji struktury tam, gdzie normalnie określilibyśmy konkretne typy danych.

Warto zauważyć, że ponieważ użyliśmy tylko jednego generycznego typu do zdefiniowania `Point<T>`, ta definicja oznacza, że struktura `Point<T>` jest generyczna względem jakiegoś typu `T`, a pola `x` i `y` są *tego samego* typu, jakikolwiek by on nie był. Jeśli utworzymy instancję `Point<T>` z wartościami różnych typów, jak to pokazano poniżej, nasz kod nie skompiluje się.

```rust,ignore,does_not_compile
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}
```

#### Pola `x` i `y` muszą być tego samego typu, ponieważ oba mają ten sam generyczny typ danych `T`.

W tym przykładzie, gdy przypisujemy wartość całkowitą 5 do `x`, informujemy kompilator, że generyczny typ `T` będzie w tej instancji `Point<T>` typu całkowitego. Następnie, kiedy przypisujemy wartość 4.0 do `y`, które zostało zdefiniowane jako tego samego typu co `x`, otrzymamy błąd niedopasowania typu, jak poniżej:

```console
error[E0308]: mismatched types
 --> src/main.rs:7:38
  |
7 |     let wont_work = Point { x: 5, y: 4.0 };
  |                                      ^^^ expected integer, found floating-point number
```

Aby zdefiniować strukturę `Point`, gdzie `x` i `y` również są generyczne, ale mogą mieć różne typy, możemy użyć wielu parametrów typu generycznego. Na przykład w poniższym kodzie możemy zmienić definicję `Point`, aby była generyczna względem typów `T` i `U`, gdzie `x` jest typu `T`, a `y` jest typu `U`.

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

#### Struktura `Point<T, U>` generyczna względem dwóch typów, dzięki czemu `x` i `y` mogą być wartościami różnych typów

Teraz wszystkie pokazane instancje `Point` są dozwolone! Możesz używać dowolnej liczby parametrów typu generycznego w definicji, ale użycie ich w zbyt dużej liczbie sprawia, że kod staje się trudny do odczytania. Jeśli potrzebujesz wielu generyków w swoim kodzie, może to być wskazówka, że kod wymaga restrukturyzacji na mniejsze fragmenty.