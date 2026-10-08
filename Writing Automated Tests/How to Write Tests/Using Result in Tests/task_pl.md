### Używanie Result<T, E> w testach

Do tej pory pisaliśmy testy, które wywołują panikę, gdy zawodzą. Możemy jednak również pisać testy wykorzystujące `Result<T, E>`! Oto test z [jednego z poprzednich zadań](course://Writing Automated Tests/How to Write Tests/The Anatomy of a Test Function), przerobiony tak, by korzystać z `Result<T, E>` i zwracać `Err` zamiast wywoływać panikę:

```rust
    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() -> Result<(), String> {
            if 2 + 2 == 4 {
                Ok(())
            } else {
                Err(String::from("dwa plus dwa nie równa się cztery"))
            }
        }
    }
```

Funkcja `it_works` teraz ma typ zwracanej wartości `Result<(), String>`. W ciele funkcji, zamiast wywoływać makro `assert_eq!`, zwracamy `Ok(())`, gdy test przechodzi, oraz `Err` z zawartością typu `String`, gdy test się nie powodzi.

Pisanie testów, które zwracają `Result<T, E>`, umożliwia użycie operatora znaku zapytania w ciele testów, co może być wygodnym sposobem na pisanie testów, które powinny zawodzić, jeśli jakakolwiek operacja wewnątrz nich zwróci wariant `Err`.

Nie można użyć adnotacji `#[should_panic]` w testach, które korzystają z `Result<T, E>`. Zamiast tego, należy bezpośrednio zwrócić wartość `Err`, gdy test powinien się nie powieść.

Teraz, gdy znasz różne sposoby pisania testów, przyjrzyjmy się, co dzieje się podczas uruchamiania testów, oraz różnym opcjom, jakie możemy wykorzystać z `cargo test`.