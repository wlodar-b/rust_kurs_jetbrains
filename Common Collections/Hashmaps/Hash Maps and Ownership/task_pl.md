### HashMapy i własność

Dla typów implementujących cechę `Copy`, takich jak `i32`, wartości są kopiowane do hash mapy. Dla posiadanych wartości, takich jak `String`, wartości zostaną przeniesione, a hash mapa stanie się ich właścicielem, jak pokazano poniżej.

```rust
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name i field_value są w tym momencie nieprawidłowe, spróbuj ich użyć i
    // zobacz, jaki błąd kompilatora otrzymasz!
```

#### Pokazanie, że klucze i wartości należą do hash mapy po ich wstawieniu

Nie możemy używać zmiennych `field_name` i `field_value` po ich przeniesieniu do hash mapy przez wywołanie `insert`.

Jeśli wstawimy referencje do wartości do hash mapy, wartości te nie zostaną przeniesione do hash mapy. Wartości, do których odnoszą się referencje, muszą być ważne przynajmniej tak długo, jak długo ważna jest hash mapa. Więcej o tych zagadnieniach możesz przeczytać w sekcji [„Walidacja referencji przy użyciu
czasów życia”][validating-references-with-lifetimes]<!-- ignore --> w Rozdziale 10 książki o języku Rust.

[validating-references-with-lifetimes]:
https://github.com/rust-lang/book/blob/master/src/ch10-03-lifetime-syntax.md