### Aktualizowanie Stringa

Obiekt typu `String` może powiększać swój rozmiar, a jego zawartość może się zmieniać, podobnie jak w przypadku zawartości `Vec<T>`, jeśli dodamy do niego więcej danych. Dodatkowo można wygodnie używać operatora `+` lub makra `format!`, aby scalać wartości typu `String`.

#### Dodawanie do Stringa za pomocą metod `push_str` i `push`

Możemy powiększyć obiekt `String`, używając metody `push_str`, aby dodać do niego wycinek stringa, jak pokazano w poniższym fragmencie kodu.

```rust
    let mut s = String::from("foo");
    s.push_str("bar");
```

##### Przykład dodania wycinka stringa do Stringa za pomocą metody push_str

Po wykonaniu tych dwóch linii kodu zmienna `s` będzie zawierała `foobar`. Metoda `push_str` przyjmuje wycinek stringa, ponieważ nie zawsze chcemy przejąć własność przekazanego parametru. Na przykład, poniższy fragment kodu pokazuje, że byłoby niekorzystne, gdybyśmy nie mogli użyć `s2` po dodaniu jego zawartości do `s1`.

```rust
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
```

##### Używanie wycinka stringa po dodaniu jego zawartości do Stringa

Gdyby metoda `push_str` przejmowała własność `s2`, nie bylibyśmy w stanie wypisać jego wartości w ostatniej linii. Jednak ten kod działa zgodnie z oczekiwaniami!

Metoda `push` przyjmuje pojedynczy znak jako parametr i dodaje go do obiektu `String`. Poniżej znajduje się fragment kodu, który dodaje literę _l_ do obiektu `String`, używając metody `push`.

```rust
    let mut s = String::from("lo");
    s.push('l');
```

##### Dodawanie jednego znaku do wartości String za pomocą metody push

W wyniku tego kodu zmienna `s` będzie zawierała `lol`.

#### Konkatenacja za pomocą operatora `+` lub makra format!

Często będziesz chciał połączyć dwie istniejące wartości typu `String`. Jednym ze sposobów jest użycie operatora `+`, jak pokazano poniżej.

```rust
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // uwaga: s1 zostało tutaj przeniesione i nie może być dalej używane
```

##### Używanie operatora + do połączenia dwóch wartości String w nową wartość String

W wyniku tego kodu, zmienna `s3` będzie zawierała `Hello, world!`. Powodem, dla którego `s1` jest już nieważne po dodaniu, oraz powodem, dla którego użyliśmy referencji do `s2`, jest sygnatura metody wywoływanej, gdy używamy operatora `+`. Operator `+` korzysta z metody `add`, której sygnatura wygląda mniej więcej tak:

```rust
    fn add(self, s: &str) -> String {
```

Nie jest to dokładna sygnatura znajdująca się w standardowej bibliotece: w bibliotece standardowej, `add` jest zdefiniowane z użyciem typów generycznych. Tutaj patrzymy na sygnaturę `add` z podstawionymi konkretnymi typami zamiast generycznych, co dzieje się, gdy wywołujemy tę metodę z wartościami `String`. Omawiamy generyki w rozdziale "Typy generyczne, cechy i czas życia". Ta sygnatura daje nam wskazówki, które są potrzebne do zrozumienia zawiłości operatora `+`.

Po pierwsze, `s2` jest poprzedzone `&`, co oznacza, że dodajemy _referencję_ drugiego stringa do pierwszego stringa ze względu na parametr `s` w funkcji `add`: możemy dodawać tylko `&str` do `String`, nie możemy dodawać dwóch wartości `String` do siebie. Ale zaraz – typ `&s2` to `&String`, a nie `&str`, jak określono w drugim parametrze funkcji `add`. Dlaczego więc kompiluje się fragment "Używanie operatora + do połączenia dwóch wartości String w nową wartość String"?

Powodem, dla którego możemy użyć `&s2` w wywołaniu `add`, jest to, że kompilator potrafi _przekształcić_ argument `&String` w `&str`. Gdy wywołujemy metodę `add`, język Rust używa mechanizmu _deref coercion_, który w tym przypadku zamienia `&s2` w `&s2[..]`. Dereferencję omawiamy bardziej szczegółowo w rozdziale 15 książki [Rust](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html). Ponieważ `add` nie przejmuje własności parametru `s`, `s2` pozostanie ważną wartością typu `String` po tej operacji.

Po drugie, widać w sygnaturze, że `add` przejmuje własność `self`, ponieważ `self` nie jest poprzedzone `&`. To oznacza, że `s1` w przykładzie "Używanie operatora + do połączenia dwóch wartości String w nową wartość String" zostanie przeniesione do wywołania `add` i nie będzie już ważne. Tak więc, chociaż deklaracja `let s3 = s1 + &s2;` wygląda, jakby miała skopiować oba stringi i stworzyć nowy, to w rzeczywistości instrukcja ta przejmuje własność `s1`, dodaje kopię zawartości `s2`, a następnie zwraca własność wyniku. Innymi słowy, wydaje się, że wykonuje dużo kopiowania, ale faktycznie implementacja jest bardziej wydajna niż kopiowanie.

Jeśli potrzebujemy dodać więcej stringów, działanie operatora `+` staje się nieporęczne:

```rust
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
```

W tym przypadku `s` będzie zawierało `tic-tac-toe`. Przy wszystkich znakach `+` i `"` trudno jest się zorientować, co się dzieje. W bardziej złożonych przypadkach łączenia stringów możemy użyć makra `format!`:

```rust
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
```

Ten kod również ustawia zmienną `s` na `tic-tac-toe`. Makro `format!` działa w taki sam sposób jak `println!`, ale zamiast wypisywać wynik na ekran, zwraca `String` z zawartością. Wersja kodu z użyciem `format!` jest znacznie czytelniejsza i nie przejmuje własności żadnego z parametrów.