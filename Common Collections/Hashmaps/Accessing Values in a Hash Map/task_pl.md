### Dostęp do wartości w HashMap

Możemy uzyskać wartość z hash mapy, podając jej klucz metodzie `get`, jak pokazano poniżej.

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
```

#### Dostęp do wyniku dla drużyny Blue przechowywanego w hash mapie

Tutaj, `score` będzie miało wartość przypisaną do drużyny Blue, a wynik będzie `Some(&10)`. Wynik jest opakowany w `Some`, ponieważ `get` zwraca `Option<&V>`; jeśli w hash mapie nie ma wartości dla tego klucza, `get` zwróci `None`. Program będzie musiał obsłużyć typ `Option` w jeden z omówionych w rozdziale „Enumy” sposobów.

Możemy iterować przez każdą parę klucz/wartość w hash mapie w sposób podobny jak w przypadku wektorów, używając pętli `for`:

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
```

Ten kod wypisze każdą parę w dowolnej kolejności:

```text
Yellow: 50
Blue: 10