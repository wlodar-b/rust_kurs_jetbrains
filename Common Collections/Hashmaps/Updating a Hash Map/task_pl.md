### Aktualizowanie HashMapy

Chociaż liczba kluczy i wartości może się zwiększać, każdy klucz może mieć przypisaną tylko jedną wartość w danym momencie. Jeśli chcesz zmienić dane w HashMapie, musisz zdecydować, jak postąpić w przypadku, gdy klucz już ma przypisaną wartość. Możesz zastąpić starą wartość nową, całkowicie ignorując starą wartość. Możesz zachować starą wartość i zignorować nową, dodając nową wartość jedynie, jeśli klucz *nie* ma jeszcze przypisanej wartości. Albo możesz połączyć starą wartość z nową. Przyjrzyjmy się, jak zrealizować każdą z tych opcji!

#### Nadpisywanie wartości

Jeśli dodamy klucz i wartość do HashMapy, a następnie dodamy ten sam klucz z inną wartością, wartość przypisana do tego klucza zostanie zastąpiona. Nawet jeśli w poniższym kodzie funkcja `insert` jest wywoływana dwa razy, HashMapa będzie zawierać tylko jedną parę klucz/wartość, ponieważ za każdym razem dodajemy wartość dla klucza drużyny Blue.

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
```

##### Zastępowanie wartości przechowywanej dla określonego klucza

Ten kod wypisze `{"Blue": 25}`. Oryginalna wartość `10` została nadpisana.

#### Dodawanie wartości tylko, jeśli klucz nie ma przypisanej wartości

Często sprawdzamy, czy dany klucz ma przypisaną wartość, a jeśli nie, dodajemy wartość dla tego klucza. HashMapa posiada specjalne API na taką okoliczność, zwane `entry`, które przyjmuje klucz jako parametr sprawdzany pod kątem istnienia przypisanej wartości. Wynikiem działania metody `entry` jest typ wyliczeniowy (`enum`) zwany `Entry`, który reprezentuje wartość, która może, ale nie musi istnieć. Załóżmy, że chcemy sprawdzić, czy klucz dla drużyny Yellow ma przypisaną wartość, a jeśli nie, dodajemy wartość 50, podobnie jak dla drużyny Blue. Przy użyciu API `entry` kod wygląda następująco:

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
```

##### Korzystanie z metody entry, by dodać wartość tylko, jeśli klucz nie ma przypisanej wartości

Metoda `or_insert` na typie `Entry` zwraca zmienną referencję do wartości przypisanej do klucza, jeśli taki istnieje, a jeśli nie, dodaje nową wartość i zwraca zmienną referencję do tej nowej wartości. Ta technika jest znacznie bardziej przejrzysta niż samodzielne pisanie logiki i dodatkowo lepiej współgra z mechanizmem pożyczania (`borrow checker`).

Uruchomienie powyższego kodu wypisze `{"Yellow": 50, "Blue": 25}`. Pierwsze wywołanie `entry` doda klucz dla drużyny Yellow z wartością 50, ponieważ drużyna Yellow nie ma jeszcze przypisanej wartości. Drugie wywołanie `entry` nie zmieni HashMapy, ponieważ drużyna Blue już ma przypisaną wartość 25.

#### Aktualizacja wartości na podstawie starej wartości

Innym powszechnym scenariuszem użycia HashMapy jest wyszukiwanie wartości przypisanej do klucza i jej zaktualizowanie na podstawie starej wartości. Na przykład, w poniższym kodzie pokazujemy, jak policzyć, ile razy każde słowo pojawia się w tekście. Używamy HashMapy, w której słowa są kluczami, a wartościami są liczniki śledzące ilość wystąpień każdego słowa. Jeśli spotykamy dane słowo po raz pierwszy, najpierw dodajemy wartość 0.

```rust
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
```

##### Liczenie wystąpień słów z wykorzystaniem HashMapy zawierającej słowa i liczniki

Ten kod wypisze `{"world": 2, "hello": 1, "wonderful": 1}`. Metoda `or_insert` faktycznie zwraca zmienną referencję (`&mut V`) do wartości dla danego klucza. Tutaj przechowujemy tę referencję w zmiennej `count`, więc aby przypisać nową wartość, musimy najpierw dereferencjonować `count` za pomocą gwiazdki (`*`). Zmienna referencja wychodzi z zakresu pod koniec pętli `for`, więc wszystkie te zmiany są bezpieczne i zgodne z zasadami mechanizmu pożyczania.