## Wycinki łańcuchów znaków (String Slices)

_Wycinek łańcucha znaków_ to referencja do części `String`, która wygląda tak:

```rust
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
```

Jest to podobne do wzięcia referencji na cały `String`, lecz z dodatkowym elementem `[0..5]`. Zamiast referencji do całego `String`, jest to referencja do części `String`.

Wycinek możemy stworzyć, wykorzystując zakres wewnątrz nawiasów kwadratowych, podając `[starting_index..ending_index]`, gdzie `starting_index` oznacza pierwszą pozycję w wycinku, a `ending_index` ostatnią pozycję w wycinku + 1. Wewnątrz struktury danych wycinka przechowuje się pozycję początkową oraz długość wycinka, co odpowiada różnicy `ending_index` i `starting_index`. Tak więc w przypadku `let world = &s[6..11];`, `world` będzie wycinkiem, który zawiera wskaźnik na 7-ty bajt (licząc od 1) `s` oraz informację o długości wynoszącą 5.

Rysunek 6 ilustruje to na diagramie.

<img alt="world zawierający wskaźnik na 6. bajt ciągu s i długość 5" src="https://doc.rust-lang.org/stable/book/img/trpl04-06.svg" class="center" style="width: 50%;">

##### Rysunek 6: Wycinek łańcucha znaków odnoszący się do części String

Dzięki składni zakresów `..` w Rust, jeśli chcemy zacząć od pierwszego indeksu (zera), możemy pominąć wartość przed dwoma kropkami. Innymi słowy, poniższe przykłady są równoważne:

```rust
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];
```

Analogicznie, jeśli Twój wycinek obejmuje ostatni bajt w `String`, możesz pominąć liczbę zamykającą zakres. Oznacza to, że poniższe przykłady są równe:

```rust
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];
```

Możesz także pominąć obie wartości, aby wziąć wycinek całego łańcucha znaków. Dlatego poniższe przykłady są równoważne:

```rust
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];
```

> Uwaga: Indeksy zakresu wycinków łańcuchów znaków muszą występować na prawidłowych granicach znaków kodowanych w UTF-8. Jeśli spróbujesz utworzyć wycinek łańcucha w środku wielobajtowego znaku, Twój program zakończy się błędem. W tej sekcji, omawiając podstawy wycinków łańcuchów, zakładamy tylko kodowanie ASCII; bardziej szczegółowa dyskusja na temat obsługi UTF-8 znajduje się w sekcji [„Przechowywanie tekstu kodowanego w UTF-8 za pomocą String”](https://doc.rust-lang.org/stable/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings) w rozdziale „Typowe Kolekcje”.

Z uwzględnieniem powyższych informacji, przeimplementujmy funkcję `first_word`, aby zwracała wycinek. Typ oznaczający „wycinek łańcucha znaków” zapisuje się jako `&str`:

```rust
    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
```

Indeks końca słowa uzyskujemy w ten sam sposób, co w poprzednim przykładzie, wyszukując pierwsze wystąpienie spacji. Gdy znajdziemy spację, zwracamy wycinek łańcucha znaków, używając początku ciągu i indeksu spacji jako granic.

Teraz, kiedy wywołujemy `first_word`, otrzymujemy pojedynczą wartość związaną z bazowymi danymi. Wartość ta składa się z referencji do punktu początkowego wycinka i liczby elementów w wycinku.

Zwracanie wycinka zadziałałoby również z funkcją `second_word`:

```rust
    fn second_word(s: &String) -> &str {
```

Dysponujemy teraz prostym API, które trudniej użyć w niepoprawny sposób, ponieważ kompilator upewni się, że referencje do `String` pozostaną ważne. Przypomnij sobie błąd w programie z przykładu „Przechowywanie wyniku wywołania funkcji first_word, a następnie zmienianie zawartości String”, gdy uzyskaliśmy indeks końca pierwszego słowa, ale następnie wyczyściliśmy łańcuch, przez co nasz indeks był nieprawidłowy? Kod był logicznie błędny, ale nie generował od razu błędów. Problemy pojawiłyby się później, jeśli próbowalibyśmy nadal używać indeksu pierwszego słowa z pustym łańcuchem. Wycinki eliminują ten błąd i pozwalają wykryć problem w naszym kodzie znacznie szybciej. Użycie wersji wycinka `first_word` spowoduje błąd w czasie kompilacji:

```rust
    fn main() {
        let mut s = String::from("hello world");

        let word = first_word(&s);

        s.clear(); // błąd!

        println!("pierwsze słowo to: {}", word);
    }
```

Oto błąd kompilatora:

```text
    error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
      --> src/main.rs:18:5
       |
    16 |     let word = first_word(&s);
       |                           -- immutable borrow occurs here
    17 |
    18 |     s.clear(); // błąd!
       |     ^^^^^^^^^ mutable borrow occurs here
    19 |
    20 |     println!("pierwsze słowo to: {}", word);
       |                                       ---- immutable borrow later used here
```

Przypomnij sobie zasady pożyczania, według których jeśli mamy niezmienną referencję do czegoś, nie możemy również wziąć referencji zmiennej. Ponieważ `clear` musi skrócić `String`, potrzebuje zmiennej referencji. Rust na to nie pozwala, a kompilacja kończy się niepowodzeniem. Rust nie tylko sprawił, że nasze API stało się łatwiejsze w użyciu, ale także wyeliminował całą klasę błędów już podczas kompilacji!