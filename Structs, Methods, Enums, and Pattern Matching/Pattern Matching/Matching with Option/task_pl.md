### Dopasowywanie za pomocą `Option<T>`

W poprzedniej sekcji chcieliśmy uzyskać wewnętrzną wartość `T` z przypadku `Some` podczas używania `Option<T>`; możemy także obsłużyć `Option<T>` za pomocą `match`, tak jak to zrobiliśmy z wyliczeniem `Coin`! Zamiast porównywać monety, będziemy porównywać warianty `Option<T>`, ale sposób, w jaki działa wyrażenie `match`, pozostaje taki sam.

Załóżmy, że chcemy napisać funkcję, która przyjmuje `Option<i32>` i, jeśli istnieje wartość w środku, dodaje do niej 1. Jeśli wartość nie istnieje, funkcja powinna zwrócić wartość `None` i nie próbować przeprowadzać żadnych operacji.

Dzięki `match` ta funkcja jest bardzo łatwa do napisania i będzie wyglądać jak kod poniżej.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

#### Funkcja używająca wyrażenia `match` na `Option<i32>`

Przeanalizujmy pierwsze wykonanie `plus_one` bardziej szczegółowo. Kiedy wywołujemy `plus_one(five)`, zmienna `x` w ciele funkcji `plus_one` będzie miała wartość `Some(5)`. Następnie porównujemy ją z każdą gałęzią `match`.

```rust,ignore
None => None,
```

Wartość `Some(5)` nie pasuje do wzorca `None`, więc przechodzimy do kolejnej gałęzi.

```rust,ignore
Some(i) => Some(i + 1),
```

Czy `Some(5)` pasuje do `Some(i)`? Tak, jak najbardziej! Mamy ten sam wariant. `i` zostaje powiązane z wartością zawartą w `Some`, więc `i` przyjmuje wartość `5`. Kod w tej gałęzi `match` zostaje wykonany, więc dodajemy 1 do wartości `i` i tworzymy nową wartość `Some` z naszym wynikiem `6` wewnątrz.

Teraz rozważmy drugie wywołanie `plus_one` w Listing 6-5, gdzie `x` ma wartość `None`. Wchodzimy w `match` i porównujemy z pierwszą gałęzią.

```rust,ignore
None => None,
```

Pasuje! Nie ma wartości, do której moglibyśmy coś dodać, więc program zatrzymuje się i zwraca wartość `None` po prawej stronie `=>`. Ponieważ pierwsza gałąź pasuje, inne gałęzie nie są już porównywane.

Łączenie `match` i enumeratyw jest przydatne w wielu sytuacjach. Często zobaczysz ten wzorzec w kodzie napisanym w Rust: `match` używane na enumeracie, powiązanie zmiennej z danymi w środku, a następnie wykonanie kodu w oparciu o te dane. Na początku może to być nieco trudne, ale gdy się przyzwyczaisz, będziesz sobie życzył, żeby było to dostępne we wszystkich językach. Jest to konsekwentnie ulubiona funkcja użytkowników.