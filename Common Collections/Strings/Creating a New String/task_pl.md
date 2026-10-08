### Tworzenie nowego Stringa

Wiele operacji dostępnych dla `Vec<T>` jest również dostępnych dla `String`, zaczynając od funkcji `new` do tworzenia stringa, jak pokazano poniżej.
```rust
    let mut s = String::new();
```

##### Przykład tworzenia nowego, pustego Stringa

Ten wiersz tworzy nowy, pusty string o nazwie `s`, do którego możemy później załadować dane. Często mamy jakieś początkowe dane, od których chcemy zacząć tworzenie stringa. W tym celu używamy metody `to_string`, która jest dostępna dla każdego typu implementującego cechę `Display`, tak jak literały tekstowe. Poniżej przedstawione są dwa przykłady.

```rust
    let data = "initial contents";

    let s = data.to_string();

    // metoda działa również bezpośrednio na literałach:
    let s = "initial contents".to_string();
```

##### Przykład użycia metody to_string do utworzenia Stringa z literału tekstowego

Ten kod tworzy string zawierający `initial contents`.

Możemy również użyć funkcji `String::from`, aby utworzyć `String` z literału tekstowego. Kod poniżej jest równoważny kodowi z poprzedniego przykładu, który używa metody `to_string`.

```rust
    let s = String::from("initial contents");
```

##### Przykład użycia funkcji String::from do utworzenia Stringa z literału tekstowego

Ponieważ stringi są wykorzystywane w wielu sytuacjach, możemy używać różnych uniwersalnych API do pracy z nimi, co daje nam wiele możliwości. Niektóre z tych opcji mogą wydawać się redundantne, ale każda z nich ma swoje zastosowanie! W tym przypadku `String::from` i `to_string` działają tak samo, więc wybór między nimi jest kwestią stylu.

Pamiętaj, że stringi są kodowane w UTF-8, więc możemy w nich zawrzeć dowolne prawidłowo zakodowane dane, jak pokazano poniżej.

```rust
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
```

##### Przykład przechowywania powitań w różnych językach w stringach

Wszystkie powyższe wartości są poprawnymi `String`.