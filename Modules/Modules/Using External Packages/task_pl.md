### Korzystanie z zewnętrznych pakietów

Załóżmy, że chcemy uzyskać losowe liczby. W tym celu będziemy potrzebować zewnętrznego pakietu o nazwie `rand`. Aby użyć `rand` w naszym projekcie, dodajemy tę linię do pliku _Cargo.toml_:

```toml
    [dependencies]
    rand = "0.8.5"
```

Dodanie `rand` jako zależności w _Cargo.toml_ informuje Cargo, aby pobrał pakiet `rand` oraz wszystkie jego zależności z _https://crates.io_ i udostępnił `rand` w naszym projekcie.

Następnie, aby wprowadzić definicje `rand` do zakresu naszego pakietu, dodajemy linię zaczynającą się od nazwy biblioteki, `rand`, i wymieniamy elementy, które chcemy wprowadzić do zakresu. Na przykład, wprowadzamy do zakresu cechę (`trait`) `Rng` i wywołujemy funkcję `rand::thread_rng`:

```rust
    use rand::Rng;
    fn main() {
        let secret_number = rand::thread_rng().gen_range(1..=100);
    }
```

Członkowie społeczności Rust udostępnili wiele pakietów na stronie _https://crates.io_, a importowanie któregokolwiek z nich do swojego pakietu wymaga wykonania tych samych kroków: wymienienia ich w pliku _Cargo.toml_ pakietu oraz użycia `use`, aby wprowadzić elementy z ich bibliotek do zakresu.

Zwróć uwagę, że standardowa biblioteka (`std`) jest również biblioteką zewnętrzną dla naszego pakietu. Ponieważ standardowa biblioteka jest dostarczana razem z językiem Rust, nie musimy zmieniać pliku _Cargo.toml_, aby uwzględnić `std`. Musimy jednak użyć `use`, aby wprowadzić elementy z tej biblioteki do zakresu naszego pakietu. Na przykład, aby użyć `HashMap`, napisalibyśmy:

```rust
    use std::collections::HashMap;
```

Jest to ścieżka absolutna zaczynająca się od `std`, czyli nazwy biblioteki standardowej.