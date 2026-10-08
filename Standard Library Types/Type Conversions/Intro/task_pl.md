## Konwersje typów

Rust oferuje wiele sposobów na konwersję wartości jednego typu na inny.

Najprostszą formą konwersji typu jest wyrażenie rzutowania typu. Jest ono oznaczane za pomocą operatora binarnego `as`. Na przykład, `println!("{}", 1 + 1.0);` nie skompiluje się, ponieważ `1` jest liczbą całkowitą, a `1.0` to liczba zmiennoprzecinkowa. Jednak `println!("{}", 1 as f32 + 1.0)` powinno się skompilować. Ćwiczenie [`using_as`](using_as.rs) stara się to omówić.

Rust oferuje również cechy (traits), które ułatwiają konwersje typów po ich implementacji. Te cechy są dostępne w module [`convert`](https://doc.rust-lang.org/std/convert/index.html). Obejmują one następujące cechy:
- `From` i `Into`, omawiane w [`from_into`](from_into.rs)
- `TryFrom` i `TryInto`, omawiane w [`try_from_into`](try_from_into.rs)
- `AsRef` i `AsMut`, omawiane w [`as_ref_mut`](as_ref_mut.rs)

Ponadto moduł `std::str` oferuje cechę [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html), która pomaga w konwersji ciągów znaków na docelowe typy za pomocą metody `parse` w ciągach znaków. Jeśli cecha ta zostanie prawidłowo zaimplementowana dla danego typu `Person`, wówczas `let p: Person = "Mark,20".parse().unwrap()` powinno zarówno się skompilować, jak i uruchomić bez paniki.

To powinny być główne sposoby ***w ramach standardowej biblioteki*** na konwersję danych do żądanych typów.

#### Sekcje z Książki o Ruście

Nie są one bezpośrednio omawiane w książce, ale biblioteka standardowa posiada świetną dokumentację dotyczącą [konwersji tutaj](https://doc.rust-lang.org/std/convert/index.html). Cechę `FromStr` omówiono również [tutaj](https://doc.rust-lang.org/std/str/trait.FromStr.html).