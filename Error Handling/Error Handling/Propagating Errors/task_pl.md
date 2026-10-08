### Propagowanie błędów

Kiedy piszesz funkcję, której implementacja wywołuje coś, co może się nie powieść, zamiast obsługiwać błąd wewnątrz tej funkcji, możesz zwrócić błąd do kodu wywołującego, aby to on zdecydował, co zrobić. Jest to nazywane _propagowaniem_ błędu i daje więcej kontroli kodowi wywołującemu, gdzie może być dostępnych więcej informacji lub logiki określającej, jak powinien zostać obsłużony błąd, niż masz w kontekście swojego kodu.

Na przykład, poniższy fragment kodu pokazuje funkcję, która odczytuje nazwę użytkownika z pliku. Jeśli plik nie istnieje lub nie może zostać odczytany, ta funkcja zwróci te błędy do kodu, który ją wywołał.

```rust
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
```

##### Funkcja zwracająca błędy do kodu wywołującego za pomocą `match`

Tę funkcję można napisać znacznie krócej, ale zaczniemy od ręcznej implementacji, aby przyjrzeć się obsłudze błędów; na końcu pokażemy krótszą wersję. Przyjrzyjmy się najpierw typowi zwracanemu przez tę funkcję: `Result<String, io::Error>`. Oznacza to, że funkcja zwraca wartość typu `Result<T, E>`, gdzie parametr generyczny `T` został zastąpiony konkretnym typem `String`, a generyczny typ `E` konkretnym typem `io::Error`. Jeśli funkcja zakończy się powodzeniem bez żadnych problemów, kod wywołujący tę funkcję otrzyma wartość `Ok`, która zawiera `String` — nazwę użytkownika odczytaną z pliku. Jeśli funkcja napotka jakiekolwiek problemy, kod wywołujący otrzyma wartość `Err`, która zawiera instancję `io::Error` z dodatkowymi informacjami o problemach. Wybraliśmy `io::Error` jako typ zwracany dla tej funkcji, ponieważ jest to typ wartości błędu zwracanej zarówno przez funkcję `File::open`, jak i metodę `read_to_string` używane w ciele tej funkcji, które mogą się nie powieść.

Ciało funkcji rozpoczyna się od wywołania funkcji `File::open`. Następnie obsługujemy wartość `Result` zwróconą za pomocą `match` w sposób podobny do `match` w części "Użycie wyrażenia match do obsługi wariantów Result, które mogą zostać zwrócone", ale zamiast wywoływać `panic!` w przypadku `Err`, wcześnie opuszczamy tę funkcję, przekazując wartość błędu zwróconą z `File::open` do kodu wywołującego jako wartość błędu tej funkcji. Jeśli `File::open` zakończy się sukcesem, przechowujemy uchwyt do pliku w zmiennej `f` i kontynuujemy.

Następnie tworzymy nowy `String` w zmiennej `s` i wywołujemy metodę `read_to_string` na uchwycie pliku `f`, aby odczytać zawartość pliku do `s`. Metoda `read_to_string` również zwraca `Result`, ponieważ może się nie powieść, nawet jeśli `File::open` zakończyło się powodzeniem. Dlatego potrzebujemy kolejnego `match`, aby obsłużyć ten `Result`: jeśli `read_to_string` zakończy się sukcesem, funkcja zakończy się sukcesem i zwracamy nazwę użytkownika z pliku, która znajduje się w `s`, opakowaną w `Ok`. Jeśli `read_to_string` się nie powiedzie, zwracamy wartość błędu w taki sam sposób, jak zwróciliśmy wartość błędu w `match`, który obsługiwał wynik `File::open`. Nie musimy jednak explicite używać `return`, ponieważ jest to ostatnie wyrażenie w funkcji.

Kod, który wywołuje tę funkcję, obsłuży następnie otrzymanie wartości `Ok` zawierającej nazwę użytkownika lub wartości `Err`, która zawiera `io::Error`. Nie wiemy, co kod wywołujący zrobi z tymi wartościami. Jeśli kod wywołujący otrzyma wartość `Err`, może wywołać `panic!` i spowodować awarię programu, użyć domyślnej nazwy użytkownika lub uzyskać nazwę użytkownika z innego źródła niż plik, na przykład. Nie mamy wystarczających informacji o tym, co kod wywołujący faktycznie próbuje zrobić, więc przekazujemy dalej wszystkie informacje o sukcesie lub błędzie, aby mógł je odpowiednio obsłużyć.

Ten wzorzec propagowania błędów jest tak powszechny w Rust, że Rust dostarcza operator pytajnika `?`, aby to ułatwić.