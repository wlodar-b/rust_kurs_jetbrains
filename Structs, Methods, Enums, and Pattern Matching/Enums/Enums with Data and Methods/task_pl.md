### Enumy z danymi i metodami

Spójrzmy na kolejny przykład enuma w poniższym zestawieniu: ten zawiera szeroką różnorodność typów osadzonych w jego wariantach.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

#### Enum `Message`, którego warianty przechowują różne ilości i typy wartości

Ten enum ma cztery warianty z różnymi typami:

* `Quit` nie ma żadnych związanych z nim danych.
* `Move` zawiera anonimową strukturę wewnątrz.
* `Write` zawiera pojedynczy obiekt `String`.
* `ChangeColor` zawiera trzy wartości typu `i32`.

Definiowanie enuma z wariantami, takimi jak te w powyższym przykładzie, jest podobne do definiowania różnych rodzajów struktur (`struct`), z tą różnicą, że enum nie używa słowa kluczowego `struct`, a wszystkie warianty są połączone w ramach typu `Message`. Następujące struktury mogłyby przechowywać te same dane, które przechowują wymienione wcześniej warianty enuma:

```rust
struct QuitMessage; // struktura jednostkowa
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // struktura krotek
struct ChangeColorMessage(i32, i32, i32); // struktura krotek
```

Jednak gdybyśmy używali różnych struktur, z których każda miałaby swój typ, nie moglibyśmy tak łatwo zdefiniować funkcji, która przyjmowałaby dowolny z tych rodzajów wiadomości, jak mogliśmy to zrobić z enumem `Message` zdefiniowanym w wcześniejszym przykładzie kodu, który jest pojedynczym typem.

Istnieje jeszcze jedno podobieństwo między enumami a strukturami: tak jak możemy definiować metody dla struktur za pomocą `impl`, możemy również definiować metody dla enumów. Oto metoda nazwana `call`, którą moglibyśmy zdefiniować dla naszego enuma `Message`:

```rust
impl Message {
    fn call(&self) {
        // ciało metody byłoby zdefiniowane tutaj
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

Ciało metody używa `self`, aby uzyskać wartość, dla której wywołaliśmy metodę. W tym przykładzie utworzyliśmy zmienną `m`, która ma wartość `Message::Write(String::from("hello"))`, i to właśnie ta wartość będzie przypisana do `self` w ciele metody `call`, gdy zostanie wywołana za pomocą `m.call()`.

Przyjrzyjmy się innemu enumowi z biblioteki standardowej, który jest bardzo powszechny i użyteczny: `Option`.