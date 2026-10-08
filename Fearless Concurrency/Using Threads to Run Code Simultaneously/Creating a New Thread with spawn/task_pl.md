### Tworzenie nowego wątku za pomocą `spawn`

Aby utworzyć nowy wątek, wywołujemy funkcję `thread::spawn` i przekazujemy jej zamknięcie (mówiliśmy o zamknięciach w Rozdziale 13) zawierające kod, który chcemy uruchomić w nowym wątku. Przykład w poniższym fragmencie kodu wypisuje tekst z głównego wątku i inny tekst z nowego wątku:

```rust
    use std::thread;
    use std::time::Duration;

    fn main() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("cześć, numer {} z nowo utworzonego wątku!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("cześć, numer {} z głównego wątku!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
```

##### Tworzenie nowego wątku, aby wypisywać jedno, podczas gdy główny wątek wypisuje coś innego

Zwróć uwagę, że w przypadku tej funkcji nowy wątek zostanie przerwany, gdy główny wątek zakończy działanie, niezależnie od tego, czy zakończył swoje działanie, czy nie. Wynik działania tego programu może nieznacznie się różnić za każdym razem, ale będzie podobny do poniższego:

```text
    cześć, numer 1 z głównego wątku!
    cześć, numer 1 z nowo utworzonego wątku!
    cześć, numer 2 z głównego wątku!
    cześć, numer 2 z nowo utworzonego wątku!
    cześć, numer 3 z głównego wątku!
    cześć, numer 3 z nowo utworzonego wątku!
    cześć, numer 4 z głównego wątku!
    cześć, numer 4 z nowo utworzonego wątku!
    cześć, numer 5 z nowo utworzonego wątku!
```

Wywołania `thread::sleep` wymuszają zatrzymanie wątku na krótki czas, umożliwiając działanie innemu wątkowi. Wątki prawdopodobnie będą wykonywać się naprzemiennie, ale nie jest to gwarantowane: zależy to od tego, jak system operacyjny obsługuje harmonogram wątków. W tym przypadku główny wątek wypisywał jako pierwszy, mimo że w kodzie wywołanie wypisywania z nowo utworzonego wątku występuje wcześniej. I mimo że nakazaliśmy nowo utworzonemu wątkowi wypisywać liczby aż do `i = 9`, zatrzymał się on na liczbie 5, ponieważ główny wątek zakończył swoje działanie.

Jeśli po uruchomieniu tego kodu zobaczysz jedynie wyjście z głównego wątku lub nie będzie żadnych przeplatających się wypisów, spróbuj zwiększyć liczby w zakresach, aby dać systemowi operacyjnemu więcej możliwości na przełączanie między wątkami.