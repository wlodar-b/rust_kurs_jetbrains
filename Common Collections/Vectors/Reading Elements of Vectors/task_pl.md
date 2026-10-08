### Odczytywanie elementów wektorów

Teraz, gdy wiesz już, jak tworzyć, aktualizować i usuwać wektory, dobrym kolejnym krokiem będzie nauczenie się, jak odczytywać ich zawartość. Istnieją dwa sposoby odwoływania się do wartości przechowywanych w wektorze. W przykładach oznaczyliśmy typy wartości zwracanych przez te funkcje, aby zwiększyć ich przejrzystość.

Poniższy kod pokazuje obie metody uzyskiwania dostępu do wartości w wektorze: za pomocą składni indeksowania lub metody `get`.

```rust
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("Trzeci element to {}", third);

    match v.get(2) {
        Some(third) => println!("Trzeci element to {}", third),
        None => println!("Nie ma trzeciego elementu."),
    }
```

#### Korzystanie ze składni indeksowania lub metody get w celu uzyskania dostępu do elementu w wektorze

Zwróć uwagę na dwa szczegóły. Po pierwsze, używamy wartości indeksu `2`, aby uzyskać trzeci element: wektory są indeksowane liczbami zaczynającymi się od zera. Po drugie, dwoma sposobami uzyskania trzeciego elementu są zastosowanie `&` i `[]`, co daje nam referencję, lub skorzystanie z metody `get`, do której przekazujemy indeks jako argument, co daje nam `Option<&T>`.

Rust oferuje dwa sposoby odwoływania się do elementu, aby umożliwić wybór zachowania programu, gdy spróbujemy użyć indeksu, dla którego wektor nie ma przypisanego elementu. Na przykład zobaczmy, co zrobi program, jeśli ma wektor zawierający pięć elementów, a następnie spróbuje uzyskać dostęp do elementu o indeksie 100, jak pokazano poniżej.

```rust,should_panic,panics
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
```

#### Próba uzyskania dostępu do elementu o indeksie 100 w wektorze zawierającym pięć elementów

Po uruchomieniu tego kodu pierwsza metoda `[]` spowoduje panikę programu, ponieważ odnosi się do nieistniejącego elementu. Tę metodę najlepiej używać, gdy chcesz, aby program zakończył działanie w przypadku próby uzyskania dostępu do elementu wykraczającego poza końcowy zakres wektora.

Kiedy do metody `get` zostanie przekazana wartość indeksu spoza wektora, zwraca ona `None` bez wywoływania paniki. Tę metodę stosuje się, gdy próby uzyskania dostępu do elementu poza zakresem wektora zdarzają się sporadycznie w normalnych warunkach. Twój kod może wtedy zawierać logikę obsługi sytuacji z `Some(&element)` lub `None`, jak omówiono w rozdziale „Enumy”. Na przykład, indeks może pochodzić od osoby podającej liczbę. Jeśli przypadkowo wpisze ona za dużą liczbę, a program otrzyma wartość `None`, możesz poinformować użytkownika, ile elementów znajduje się obecnie w wektorze, i dać mu kolejną szansę na wprowadzenie poprawnej wartości. Takie podejście jest bardziej przyjazne użytkownikowi niż zakończenie działania programu z powodu literówki!