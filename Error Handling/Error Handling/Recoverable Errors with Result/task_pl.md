## Błędy odzyskiwalne z Result

Większość błędów nie jest na tyle poważna, aby wymagać zatrzymania programu. Czasami, gdy funkcja zawiedzie, dzieje się tak z powodu, który można łatwo zinterpretować i na który można odpowiedzieć. Na przykład, jeśli próbujesz otworzyć plik, a operacja ta kończy się niepowodzeniem, ponieważ plik nie istnieje, możesz chcieć utworzyć plik zamiast przerywać działanie programu.

Enum `Result` jest zdefiniowany z dwoma wariantami, `Ok` i `Err`, w następujący sposób:

```rust
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
```

`T` i `E` to ogólne parametry typów: omówimy szczegóły generyków w lekcji „Typy generyczne, cechy i czas życia”. Na tym etapie musisz wiedzieć, że `T` reprezentuje typ wartości, która zostanie zwrócona w przypadku sukcesu w wariancie `Ok`, a `E` reprezentuje typ błędu, który zostanie zwrócony w przypadku niepowodzenia w wariancie `Err`. Ponieważ `Result` ma te ogólne parametry typów, możemy używać typu `Result` i funkcji zdefiniowanych dla niego w bibliotece standardowej w wielu różnych sytuacjach, w których wartość sukcesu i wartość błędu, które chcemy zwrócić, mogą się różnić.

Wywołajmy funkcję, która zwraca wartość `Result`, ponieważ funkcja może zakończyć się niepowodzeniem. W poniższym fragmencie kodu próbujemy otworzyć plik.

```rust
    use std::fs::File;

    fn main() {
        let f = File::open("hello.txt");
    }
```

##### Otwieranie pliku

Skąd wiemy, że `File::open` zwraca `Result`? Możemy sprawdzić [dokumentację API biblioteki standardowej](https://doc.rust-lang.org/std/index.html) albo zapytać o to kompilator! Jeśli zastosujemy adnotację typu dla zmiennej `f`, używając typu, który _nie_ jest typem zwracanym przez funkcję, a następnie spróbujemy skompilować kod, kompilator poinformuje nas, że typy się nie zgadzają. Wiadomość o błędzie powie nam wtedy, jaki typ ma zmienna `f`. Spróbujmy! Wiemy, że typ zwracany przez `File::open` nie jest typu `u32`, więc zmieńmy deklarację `let f` w następujący sposób:

```rust
    let f: u32 = File::open("hello.txt");
```

Próba kompilacji teraz spowoduje wyświetlenie następującego komunikatu:

```text
error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |            ---   ^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found enum `std::result::Result`
  |            |
  |            expected due to this
  |
  = note: expected type `u32`
             found enum `std::result::Result<File, std::io::Error>`
```

To pokazuje, że typ zwracany przez funkcję `File::open` to `Result<T, E>`. Parametr generyczny `T` został tutaj wypełniony typem wartości sukcesu, `std::fs::File`, który jest uchwytem pliku. Typ `E` używany dla wartości błędu to `std::io::Error`.

Taki typ zwrotny oznacza, że wywołanie `File::open` może się powieść i zwrócić uchwyt pliku, który można odczytywać lub zapisywać. Wywołanie funkcji może również zakończyć się niepowodzeniem: na przykład plik może nie istnieć albo możemy nie mieć uprawnień do dostępu do pliku. Funkcja `File::open` musi mieć sposób na poinformowanie nas, czy się powiodła, czy nie, a jednocześnie przekazać uchwyt pliku lub informacje o błędzie. Te informacje są dokładnie tym, co przekazuje enum `Result`.

W przypadku, gdy `File::open` zakończy się sukcesem, wartość w zmiennej `f` będzie instancją `Ok`, która zawiera uchwyt do pliku. W przypadku, gdy operacja się nie powiedzie, wartością w `f` będzie instancja `Err`, zawierająca więcej informacji na temat rodzaju błędu, który wystąpił.

Aby wziąć różne działania w zależności od wartości zwróconej przez `File::open`, musimy uzupełnić kod w przedstawionym fragmencie. Poniższy kod pokazuje jeden ze sposobów obsługi `Result` za pomocą podstawowego narzędzia - wyrażenia `match`, które omówiliśmy w części „Operator Match” (["The Match Operator"](course://Enums/Enums and Pattern Matching/The Match Operator)).

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

##### Używanie wyrażenia match do obsługi wariantów Result, które mogą zostać zwrócone

Należy zauważyć, że podobnie jak w przypadku enuma `Option`, enum `Result` i jego warianty zostały wprowadzone do przestrzeni nazw przez prelude, więc nie musimy określać `Result::` przed wariantami `Ok` i `Err` w ramionach `match`.

W tym miejscu informujemy Rust, że gdy wynik to `Ok`, zwracamy wartość wewnętrzną `file` z wariantu `Ok`, a następnie przypisujemy tę wartość uchwytu pliku do zmiennej `f`. Po instrukcji `match` możemy używać uchwytu pliku do odczytu lub zapisu.

Drugie ramię `match` obsługuje przypadek, w którym otrzymujemy wartość `Err` z `File::open`. W tym przykładzie wybraliśmy użycie makra `panic!`. Jeśli nie istnieje plik o nazwie _hello.txt_ w naszym bieżącym katalogu i uruchomimy ten kod, zobaczymy następujący wynik z makra `panic!`:

```text
thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:8:23
```

Jak zwykle, to wyjście dokładnie informuje nas, co poszło nie tak.