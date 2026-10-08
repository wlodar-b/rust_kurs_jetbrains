#### Skrót propagowania błędów: operator ?

Poniższy fragment kodu przedstawia implementację funkcji `read_username_from_file`, która działa tak samo jak w poprzednim przykładzie, ale w tej implementacji wykorzystano operator `?`.

```rust
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
```

##### Funkcja zwracająca błędy do kodu wywołującego przy użyciu operatora ?

Operator `?` umieszczony po wartości typu `Result` działa prawie tak samo jak wyrażenia `match`, które definiowaliśmy wcześniej, aby obsłużyć wartości `Result` w przykładzie wywołującym błąd na `match`. Jeśli wartością `Result` jest `Ok`, wartość wewnątrz `Ok` zostanie zwrócona z wyrażenia, a program będzie kontynuował działanie. Jeśli natomiast wartością jest `Err`, `Err` zostanie zwrócony z całej funkcji, tak jakbyśmy użyli słowa kluczowego `return`, dzięki czemu wartość błędu zostaje propagowana do kodu wywołującego.

Istnieje różnica między tym, co robi wyrażenie `match` w tamtym przykładzie, a tym, co robi operator `?`: wartości błędów, na które wołany jest operator `?`, przechodzą przez funkcję `from`, zdefiniowaną w typie `From` w standardowej bibliotece, która jest używana do konwersji błędów z jednego typu na inny. Kiedy operator `?` wywołuje funkcję `from`, typ błędu otrzymany jest konwertowany na typ błędu określony w zwracanym typie bieżącej funkcji. Jest to przydatne, gdy funkcja zwraca jeden typ błędu, aby reprezentować wszystkie sposoby, w jakie funkcja może zakończyć się niepowodzeniem, nawet jeśli część z nich może nie działać z różnych przyczyn. Dopóki każdy typ błędu implementuje funkcję `from`, aby określić, jak skonwertować się na typ błędu zwracany przez funkcję, operator `?` automatycznie zajmuje się tą konwersją.

W kontekście ostatniego fragmentu kodu, znak `?` na końcu wywołania `File::open` zwróci wartość wewnątrz `Ok` do zmiennej `f`. Jeśli wystąpi błąd, operator `?` zakończy całą funkcję wcześniej i przekaże dowolną wartość `Err` do kodu wywołującego. To samo dotyczy `?` na końcu wywołania `read_to_string`.

Operator `?` eliminuje wiele zbędnego kodu i upraszcza implementację tej funkcji. Kod ten można nawet bardziej skrócić, łańcząc wywołania metod bezpośrednio po `?`, co pokazano w następnym przykładzie.

```rust
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }
```

##### Łączenie wywołań metod po operatorze ?

Przenieśliśmy utworzenie nowego obiektu `String` o nazwie `s` na początek funkcji; ta część pozostała bez zmian. Zamiast tworzenia zmiennej `f`, połączyliśmy wywołanie `read_to_string` bezpośrednio z wynikiem `File::open("hello.txt")?`. Nadal mamy `?` na końcu wywołania `read_to_string` i wciąż zwracamy wartość `Ok` zawierającą nazwę użytkownika w `s`, jeśli zarówno `File::open`, jak i `read_to_string` zakończą się sukcesem, zamiast zwrócenia błędu. Funkcjonalność ponownie jest taka sama jak w poprzednich przykładach; jest to po prostu inny, bardziej wygodny sposób napisania tej funkcji.

Mówiąc o różnych sposobach napisania tej funkcji, poniższy fragment kodu pokazuje, że istnieje jeszcze krótszy sposób.

```rust
    use std::io;
    use std::fs;

    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
```

##### Użycie fs::read_to_string zamiast otwierania i następnie odczytywania pliku

Odczytanie pliku jako ciągu znaków jest dość powszechną operacją, dlatego Rust dostarcza wygodną funkcję `fs::read_to_string`, która otwiera plik, tworzy nowy obiekt `String`, odczytuje zawartość pliku, zapisuje ją do tego `String` i zwraca go. Oczywiście użycie `fs::read_to_string` nie daje nam możliwości wyjaśnienia wszystkich aspektów obsługi błędów, dlatego najpierw pokazaliśmy dłuższą wersję.