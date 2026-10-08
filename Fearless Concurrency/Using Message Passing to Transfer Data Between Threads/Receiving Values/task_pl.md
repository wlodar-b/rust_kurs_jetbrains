## Odbieranie wartości

W następnym przykładzie pobierzemy wartość z odbiornika kanału w głównym wątku. To jest jak wyławianie gumowej kaczki z wody na końcu rzeki lub jak odbieranie wiadomości czatu.

```rust
    use std::thread;
    use std::sync::mpsc;

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();
        println!("Otrzymano: {}", received);
    }
```

##### Odbieranie wartości „hi” w głównym wątku i jej wyświetlanie

Odbiornik kanału ma dwie przydatne metody: `recv` i `try_recv`. Używamy tutaj `recv`, skrótu od _receive_ (odbierz), która blokuje wykonywanie głównego wątku i czeka, aż dowolna wartość zostanie przesłana przez kanał. Gdy wartość zostanie przesłana, `recv` zwróci ją jako `Result<T, E>`. W momencie zamknięcia nadawcy kanału, `recv` zwraca błąd, informując, że więcej wartości już nie nadejdzie.

Metoda `try_recv` nie blokuje, lecz natychmiast zwraca wynik `Result<T, E>`: wartość `Ok` zawierającą wiadomość, jeśli jest dostępna, oraz wartość `Err`, jeśli w tym momencie nie ma żadnych wiadomości. Użycie `try_recv` jest przydatne, jeśli wątek ma inne zadania do wykonania podczas oczekiwania na wiadomości. Możemy napisać pętlę, która co jakiś czas wywołuje `try_recv`, obsługuje wiadomość, jeśli jest dostępna, a w przeciwnym razie zajmuje się innymi zadaniami przez chwilę, zanim sprawdzi ponownie.

W tym przykładzie użyliśmy `recv` dla uproszczenia: w głównym wątku nie mamy nic innego do roboty poza oczekiwaniem na wiadomości, dlatego blokowanie go jest odpowiednie.

Kiedy uruchomimy kod z poniższego fragmentu, zobaczymy wartość wyświetloną przez główny wątek:

```text
    Otrzymano: hi
```

Doskonale!