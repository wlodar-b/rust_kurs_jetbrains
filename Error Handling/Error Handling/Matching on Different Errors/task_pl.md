### Dopasowywanie różnych błędów

Kod w następnym przykładzie wywoła `panic!` niezależnie od tego, dlaczego `File::open` się nie powiodło. Zamiast tego chcemy podjąć różne działania w zależności od przyczyny błędu: jeśli `File::open` nie powiodło się, ponieważ plik nie istnieje, chcemy utworzyć plik i zwrócić uchwyt do nowego pliku. Jeśli `File::open` nie powiodło się z innego powodu – na przykład dlatego, że nie mieliśmy uprawnień do otwarcia pliku – chcemy nadal, aby kod wywołał `panic!` w taki sam sposób, jak w poprzednim fragmencie. Spójrz na poniższy kod, który dodaje wewnętrzną instrukcję `match`.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem z utworzeniem pliku: {:?}", e),
            },
            other_error => {
                panic!("Problem z otwarciem pliku: {:?}", other_error)
            }
        },
    };
}
```

##### Obsługiwanie różnych rodzajów błędów na różne sposoby

Typ wartości zwracanej przez `File::open` wewnątrz wariantu `Err` to `io::Error`, czyli struktura dostarczana przez standardową bibliotekę. Ta struktura posiada metodę `kind`, którą możemy wywołać, aby otrzymać wartość `io::ErrorKind`. Enum `io::ErrorKind` jest dostarczany przez standardową bibliotekę i zawiera warianty reprezentujące różne rodzaje błędów, które mogą wystąpić podczas operacji wejścia/wyjścia. Wariant, którego chcemy użyć, to `ErrorKind::NotFound`, który wskazuje, że plik, który próbujemy otworzyć, jeszcze nie istnieje. Dopasowujemy się do `f`, ale dodatkowo mamy wewnętrzne dopasowanie do `error.kind()`.

Warunek, który chcemy sprawdzić w wewnętrznym dopasowaniu, to czy wartość zwrócona przez `error.kind()` jest wariantem `NotFound` wyliczenia `ErrorKind`. Jeśli tak, próbujemy utworzyć plik za pomocą `File::create`. Jednakże, ponieważ `File::create` również może się nie powieść, potrzebujemy drugiego przypadku w wewnętrznej instrukcji `match`. Kiedy plik nie może być utworzony, wyświetlana jest inna wiadomość o błędzie. Drugi przypadek zewnętrznej instrukcji `match` pozostaje bez zmian, więc program wywoła `panic!` w przypadku każdego błędu innego niż brak pliku.

To bardzo dużo `match`! Wyrażenie `match` jest bardzo użyteczne, ale jednocześnie bardzo prymitywne. W sekcji ["Typy standardowej biblioteki/Zamykania"](course://Standard Library Types/Closures) poznasz zamykania; typ `Result<T, E>` posiada wiele metod akceptujących zamykania i zaimplementowanych przy użyciu wyrażeń `match`. Korzystanie z tych metod sprawi, że twój kod będzie bardziej zwięzły. Bardziej doświadczony programista Rust mógłby napisać ten kod w następujący sposób:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem z utworzeniem pliku: {:?}", error);
            })
        } else {
            panic!("Problem z otwarciem pliku: {:?}", error);
        }
    });
}
```

Chociaż ten kod zachowuje się tak samo jak poprzedni, nie zawiera żadnych wyrażeń `match` i jest łatwiejszy do odczytania. Wróć do tego przykładu po zapoznaniu się z sekcją ["Typy standardowej biblioteki/Zamykania"](course://Standard Library Types/Closures) i sprawdź metodę `unwrap_or_else` w dokumentacji standardowej biblioteki. Wiele innych metod może uprościć ogromne, zagnieżdżone wyrażenia `match`, gdy masz do czynienia z błędami.