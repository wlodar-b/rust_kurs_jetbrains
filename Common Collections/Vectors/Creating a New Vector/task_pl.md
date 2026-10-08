### Tworzenie nowego wektora

Aby utworzyć nowy, pusty wektor, możemy wywołać funkcję `Vec::new`, jak pokazano w poniższym kodzie.

```rust
    let v: Vec<i32> = Vec::new();
```

#### Tworzenie nowego, pustego wektora do przechowywania wartości typu i32

Zauważ, że dodaliśmy tutaj adnotację typu. Ponieważ nie wstawiamy żadnych wartości do tego wektora, Rust nie wie, jaki rodzaj elementów zamierzamy przechowywać. To jest ważny punkt. Wektory są zaimplementowane z użyciem generyków; omówimy, jak korzystać z generyków z własnymi typami w rozdziale „Typy generyczne, cechy i cykl życia”. Na razie wystarczy wiedzieć, że typ `Vec<T>` dostarczany przez bibliotekę standardową może przechowywać dowolny typ, a gdy konkretny wektor przechowuje konkretny typ, typ ten jest określany w nawiasach kątowych. W powyższym kodzie poinformowaliśmy Rust, że `Vec<T>` w zmiennej `v` będzie przechowywał elementy typu `i32`.

W bardziej realistycznym kodzie Rust często może wywnioskować typ wartości, które chcesz przechowywać, gdy tylko wprowadzisz do niego wartości, więc rzadko trzeba wykonywać taką adnotację typu. Częściej tworzy się `Vec<T>` z początkowymi wartościami, a Rust udostępnia makro `vec!` dla wygody. To makro utworzy nowy wektor, który przechowuje podane wartości. Poniższy listing tworzy nowy `Vec<i32>`, który przechowuje wartości `1`, `2` i `3`. Typ liczby całkowitej to `i32`, ponieważ jest to domyślny typ całkowity, jak omówiliśmy w sekcji „Typy danych” w rozdziale „Podstawowe zasady programowania”.

```rust
    let v = vec![1, 2, 3];
```

#### Tworzenie nowego wektora zawierającego wartości

Ponieważ podaliśmy początkowe wartości typu `i32`, Rust może wywnioskować, że typ zmiennej `v` to `Vec<i32>`, a adnotacja typu nie jest konieczna. Następnie przyjrzymy się, jak modyfikować wektor.