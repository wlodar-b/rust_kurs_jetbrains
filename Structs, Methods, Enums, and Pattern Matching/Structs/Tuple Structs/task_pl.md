### Używanie struktur krotek bez nazwanych pól do tworzenia różnych typów

Możesz również definiować struktury, które wyglądają jak krotki, nazywane *strukturami krotek*. Struktury krotek mają dodatkowe znaczenie wynikające z nazwy struktury, ale nie mają nazw powiązanych z ich polami; zamiast tego mają tylko typy pól. Struktury krotek są przydatne, gdy chcesz nadać całej krotce nazwę i sprawić, by krotka była innym typem niż inne krotki, a nazwanie każdego pola jak w zwykłej strukturze byłoby rozwlekłe lub zbędne.

Aby zdefiniować strukturę krotki, zacznij od słowa kluczowego `struct` i nazwy struktury, a następnie podaj typy w krotce. Na przykład poniżej przedstawiono definicje i użycie dwóch struktur krotek o nazwach `Color` i `Point`:

```rust
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
```

Zauważ, że wartości `black` i `origin` są różnymi typami, ponieważ są to instancje różnych struktur krotek. Każda zdefiniowana przez ciebie struktura jest własnym typem, nawet jeśli pola w strukturze mają te same typy. Na przykład funkcja, która przyjmuje parametr typu `Color`, nie może przyjąć `Point` jako argumentu, chociaż oba typy składają się z trzech wartości `i32`. Poza tym instancje struktur krotek zachowują się jak krotki: możesz je rozpakować na pojedyncze elementy, możesz użyć kropki (`.`) wraz z indeksem do dostępu do konkretnej wartości i tak dalej.

### Struktury podobne do jednostki bez jakichkolwiek pól

Możesz również definiować struktury, które nie mają żadnych pól! Są one nazywane *strukturami podobnymi do jednostki* (ang. *unit-like structs*), ponieważ zachowują się podobnie do `()`, typu jednostki (*unit type*). Struktury podobne do jednostki mogą być przydatne w sytuacjach, w których musisz zaimplementować cechę (*trait*) dla jakiegoś typu, ale nie potrzebujesz przechowywać w tym typie żadnych danych. Omówimy cechy [później](course://Generic+Types,+Traits,+and+Lifetime/Traits/Traits).