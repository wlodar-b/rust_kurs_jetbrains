### Nadawanie nowych nazw za pomocą słowa kluczowego `as`

Istnieje inne rozwiązanie problemu wprowadzania do tego samego zakresu dwóch typów o tej samej nazwie za pomocą `use`: po ścieżce możemy określić nowe lokalne imię, czyli alias, dla typu, używając `as`. W poniższym przykładzie pokazano inny sposób zapisania kodu poprzez nadanie nowej nazwy jednemu z dwóch typów `Result` za pomocą `as`.

```rust
    use std::fmt::Result;
    use std::io::Result as IoResult;

    fn function1() -> Result {
    }

    fn function2() -> IoResult<()> {
    }
```

##### Zmiana nazwy typu podczas wprowadzania go do zakresu za pomocą słowa kluczowego as

W drugiej instrukcji `use` wybraliśmy nową nazwę `IoResult` dla typu `std::io::Result`, która nie będzie kolidowała z `Result` z `std::fmt`, który również został wprowadzony do zakresu. Oba powyższe przykłady są uważane za idiomatyczne, więc wybór należy do Ciebie!