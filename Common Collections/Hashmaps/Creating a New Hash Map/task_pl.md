### Tworzenie nowej mapy skrótów

Możesz utworzyć pustą mapę skrótów za pomocą `new` i dodawać elementy za pomocą `insert`. W kodzie poniżej śledzimy wyniki dwóch drużyn o nazwach Blue i Yellow. Drużyna Blue zaczyna od 10 punktów, a drużyna Yellow od 50 punktów.

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
```

#### Tworzenie nowej mapy skrótów i dodawanie kluczy oraz wartości

Zwróć uwagę, że najpierw musimy użyć (`use`) `HashMap` z części kolekcji standardowej biblioteki. Spośród trzech najczęściej używanych przez nas kolekcji, ta jest najrzadziej stosowana, więc nie jest domyślnie dołączona do funkcji dostępnych w prelude. Mapy skrótów mają również mniej wsparcia ze strony standardowej biblioteki; na przykład, nie istnieje wbudowany makro do ich konstrukcji.

Podobnie jak wektory, mapy skrótów przechowują swoje dane na stercie. Ta `HashMap` ma klucze typu `String` i wartości typu `i32`. Tak jak wektory, mapy skrótów są jednorodne: wszystkie klucze muszą mieć ten sam typ, a wszystkie wartości muszą mieć ten sam typ.

Innym sposobem utworzenia mapy skrótów jest użycie iteratorów i metody `collect` na wektorze krotek, gdzie każda krotka składa się z klucza i odpowiadającej mu wartości. Omówimy iteratory i ich powiązane metody bardziej szczegółowo w sekcji „Iteratory” rozdziału „Iteratory i zamknięcia”. Metoda `collect` zbiera dane do różnych typów kolekcji, w tym `HashMap`. Na przykład, jeśli mielibyśmy nazwy drużyn i początkowe wyniki w dwóch oddzielnych wektorach, moglibyśmy użyć metody `zip`, aby stworzyć wektor krotek, w którym „Blue” będzie sparowane z 10 i tak dalej. Następnie moglibyśmy użyć metody `collect`, aby przekształcić ten wektor krotek w mapę skrótów, jak pokazano w poniższym przykładzie.

```rust
    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
```

#### Tworzenie mapy skrótów z listy drużyn i listy wyników

Adnotacja typu `HashMap<_, _>` jest tutaj potrzebna, ponieważ istnieje możliwość `collect` do wielu różnych struktur danych i Rust nie wie, którą chcesz, chyba że jawnie to określisz. Jednak dla parametrów typu klucza i wartości używamy podkreśleń, a Rust może wnioskować o typach, jakie powinna zawierać mapa skrótów, na podstawie typów danych w wektorach. W powyższym kodzie typ klucza będzie `String`, a typ wartości `i32`, tak samo jak w pierwszym przykładzie z tej sekcji.