## Typ wycinka (slice)

Innym typem danych, który nie przejmuje własności, jest _wycinek_ (slice). Wycinki pozwalają odnosić się do ciągłego fragmentu elementów w kolekcji zamiast do całej kolekcji.

Oto małe zadanie programistyczne: napisz funkcję, która przyjmuje ciąg znaków (string) i zwraca pierwsze słowo znalezione w tym ciągu. Jeśli funkcja nie znajdzie spacji w tekście, cały ciąg musi być jednym słowem, więc należy zwrócić cały tekst.

Zastanówmy się nad sygnaturą tej funkcji:

```rust
    fn first_word(s: &String) -> ?
```

Funkcja `first_word` przyjmuje `&String` jako parametr. Nie chodzi nam o własność, więc to rozwiązanie jest w porządku. Ale co powinniśmy zwrócić? Nie mamy rzeczywistego sposobu na wskazanie _części_ ciągu znaków. Możemy jednak zwrócić indeks końcowy słowa. Spróbujmy tego rozwiązania, jak pokazano w poniższym fragmencie kodu.

```rust
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }
```

##### Funkcja first_word zwracająca wartość indeksu bajtowego względem parametru String

Ponieważ musimy przejść przez elementy `String` jeden po drugim i sprawdzić, czy któryś z nich to spacja, przekształcimy nasz `String` w tablicę bajtów za pomocą metody `as_bytes`:

```rust
    let bytes = s.as_bytes();
```

Następnie tworzymy iterację nad tablicą bajtów za pomocą metody `iter`:

```rust
    for (i, &item) in bytes.iter().enumerate() {
```

Szczegóły o iteratorach omówimy w Rozdziale 13. Na ten moment wystarczy wiedzieć, że `iter` to metoda, która zwraca każdy element w kolekcji, a `enumerate` opakowuje wynik metody `iter` i zwraca każdy element jako część krotki. Pierwszym elementem krotki zwróconej przez `enumerate` jest indeks, a drugim jest referencja do elementu. Jest to bardziej wygodne niż obliczanie indeksu samodzielnie.

Ponieważ metoda `enumerate` zwraca krotkę, możemy użyć wzorców do destrukturyzacji tej krotki, tak jak w innych miejscach w Rust. W pętli `for` definiujemy wzorzec zawierający `i` jako indeks w krotce oraz `&item` jako pojedynczy bajt w tej krotce. Ponieważ z metody `.iter().enumerate()` otrzymujemy referencję do elementu, używamy `&` we wzorcu.

W pętli `for` szukamy bajtu, który reprezentuje spację, korzystając ze składni literału bajtowego. Jeśli znajdziemy spację, zwracamy pozycję. W przeciwnym razie zwracamy długość ciągu znaków za pomocą metody `s.len()`:

```rust
        if item == b' ' {
            return i;
        }
    }

    s.len()
```

Teraz mamy sposób na odnalezienie indeksu końca pierwszego słowa w ciągu znaków, ale mamy pewien problem. Zwracamy wartość `usize` jako samodzielną liczbę, ale ma ona znaczenie tylko w kontekście `&String`. Innymi słowy, ponieważ jest to oddzielna wartość od `String`, nie ma gwarancji, że będzie nadal ważna w przyszłości. Rozważ program w poniższym fragmencie kodu, który używa funkcji `first_word` z poprzedniego przykładu.

```rust
    fn main() {
        let mut s = String::from("hello world");

        let word = first_word(&s); // word uzyska wartość 5

        s.clear(); // to opróżnia String, czyniąc go równym ""

        // word nadal ma wartość 5, ale nie ma już ciągu znaków,
        // z którym moglibyśmy sensownie użyć tej wartości. teraz word jest całkowicie nieprawidłowy!
    }
```

##### Przechowywanie wyniku wywołania funkcji first_word i zmiana zawartości ciągu String

Ten program kompiluje się bez błędów i zrobiłby to również, gdybyśmy używali `word` po wywołaniu `s.clear()`. Ponieważ `word` nie jest powiązany ze stanem `s`, `word` nadal przechowuje wartość `5`. Moglibyśmy spróbować użyć tej wartości `5` w zmiennej `s`, aby wyodrębnić pierwsze słowo, ale byłby to błąd, ponieważ zawartość `s` zmieniła się od czasu zapisania wartości `5` w `word`.

Martwienie się o to, że indeks w `word` może nie być zsynchronizowany z danymi w `s`, jest uciążliwe i podatne na błędy! Zarządzanie tymi indeksami staje się jeszcze bardziej problematyczne, jeśli napiszemy funkcję `second_word`. Jej sygnatura musiałaby wyglądać tak:

```rust
    fn second_word(s: &String) -> (usize, usize) {
```

Teraz śledzimy zarówno indeks początkowy, jak i końcowy, co daje nam jeszcze więcej wartości obliczonych na podstawie danych w określonym stanie, ale niepowiązanych z tym stanem w żaden sposób. Mamy teraz trzy niezwiązane zmienne, które muszą być utrzymywane w synchronizacji.

Na szczęście Rust ma rozwiązanie tego problemu: wycinki tekstu (string slices).