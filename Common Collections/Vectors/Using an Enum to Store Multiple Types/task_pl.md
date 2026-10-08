### Używanie enum do przechowywania wielu typów

Na początku tego rozdziału wspomnieliśmy, że wektory mogą przechowywać jedynie wartości tego samego typu. Może to być niewygodne; istnieją przypadki, w których potrzebujemy przechowywać listę elementów różnych typów. Na szczęście warianty enum są zdefiniowane w ramach tego samego typu enum, więc kiedy musimy przechowywać elementy różnych typów w wektorze, możemy zdefiniować i użyć enum!

Na przykład załóżmy, że chcemy pobrać wartości z wiersza arkusza kalkulacyjnego, gdzie niektóre kolumny w wierszu zawierają liczby całkowite, inne liczby zmiennoprzecinkowe, a jeszcze inne ciągi znaków. Możemy zdefiniować enum, którego warianty będą przechowywać różne rodzaje wartości, a wszystkie warianty enum zostaną uznane za ten sam typ: typ tego enum. W ten sposób możemy stworzyć wektor, który przechowuje ten enum i w konsekwencji różne typy. Zademonstrowaliśmy to poniżej.

```rust
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("niebieski")),
        SpreadsheetCell::Float(10.12),
    ];
```

#### Definiowanie enum do przechowywania wartości różnych typów w jednym wektorze

Rust musi znać typy, które znajdą się w wektorze za czasów kompilacji, aby wiedzieć, ile dokładnie pamięci na stercie będzie potrzebne do przechowywania każdego elementu. Dodatkową zaletą jest to, że możemy określić jawnie, jakie typy są dozwolone w tym wektorze. Gdyby Rust pozwalał wektorowi przechowywać dowolny typ, istniałoby ryzyko, że jeden lub więcej typów spowoduje błędy przy operacjach wykonywanych na elementach wektora. Użycie enum w połączeniu z wyrażeniem `match` oznacza, że Rust zapewni podczas kompilacji obsługę każdego możliwego przypadku, co zostało omówione w rozdziale "Enumy".

Pisząc program, jeśli nie znasz pełnego zestawu typów, które program podczas wykonania otrzyma do przechowywania w wektorze, technika enum nie zadziała. W takim przypadku możesz użyć obiektu cech (trait object), który omówimy w Rozdziale 17 w [książce o Rust][book].

Teraz, gdy omówiliśmy niektóre z najczęstszych sposobów używania wektorów, koniecznie zapoznaj się z [dokumentacją API][vec-api] zawierającą wiele przydatnych metod zdefiniowanych na `Vec<T>` przez standardową bibliotekę. Na przykład, oprócz metody `push`, metoda `pop` usuwa i zwraca ostatni element.

[vec-api]: https://doc.rust-lang.org/std/vec/struct.Vec.html  
[book]: https://doc.rust-lang.org/stable/book/