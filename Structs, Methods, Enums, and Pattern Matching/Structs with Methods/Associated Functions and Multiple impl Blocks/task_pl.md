### Funkcje powiązane

Kolejną przydatną cechą bloków `impl` jest to, że możemy definiować funkcje wewnątrz bloków `impl`, które *nie* przyjmują `self` jako parametru. Nazywamy je *funkcjami powiązanymi* (ang. *associated functions*), ponieważ są powiązane ze strukturą. Nadal są to funkcje, a nie metody, ponieważ nie działają na instancji tej struktury. Już wcześniej używałeś funkcji powiązanej `String::from`.

Funkcje powiązane są często używane jako konstruktory zwracające nową instancję struktury. Na przykład moglibyśmy dostarczyć funkcję powiązaną, która przyjmuje jeden parametr reprezentujący wymiar i używa go zarówno jako szerokości, jak i wysokości, co ułatwiłoby stworzenie kwadratowego `Rectangle`, zamiast określania tej samej wartości dwa razy:

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

Aby wywołać tę funkcję powiązaną, używamy składni `::` z nazwą struktury; `let sq = Rectangle::square(3);` jest tego przykładem. Ta funkcja jest z przestrzeni nazw struktury: składnia `::` jest używana zarówno dla funkcji powiązanych, jak i przestrzeni nazw tworzonych przez moduły. Omówimy moduły w rozdziale "Moduły".

### Wiele bloków `impl`

Każda struktura może mieć wiele bloków `impl`. Na przykład listing "Implementacja metody `can_hold` na `Rectangle`, która przyjmuje jako parametr inną instancję `Rectangle`" jest równoważny z poniższym kodem, gdzie każda metoda znajduje się w osobnym bloku `impl`.

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

#### Przepisanie listingu "Implementacja metody `can_hold` na `Rectangle`, która przyjmuje jako parametr inną instancję `Rectangle`" za pomocą wielu bloków `impl`

Nie ma powodu, aby w tym przypadku rozdzielać te metody na wiele bloków `impl`, ale taka składnia jest poprawna. Zobaczymy przypadek, w którym wiele bloków `impl` jest przydatne, w rozdziale "Typy generyczne, cechy i czas życia".