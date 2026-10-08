### Przenoszenie końca nadawczego

Przenieśmy koniec nadawczy do nowo utworzonego wątku i wyślijmy jeden ciąg znaków, aby nowo utworzony wątek komunikował się z głównym wątkiem, jak pokazano w poniższym fragmencie kodu. To jak wrzucenie gumowej kaczki do rzeki w górę strumienia lub wysłanie wiadomości czatu z jednego wątku do drugiego.

```rust
    use std::thread;
    use std::sync::mpsc;

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });
    }
```

##### Przenoszenie `tx` do nowo utworzonego wątku i wysyłanie „hi”

Ponownie używamy `thread::spawn`, aby utworzyć nowy wątek, a następnie korzystamy z `move`, aby przenieść `tx` do domknięcia, tak aby nowo utworzony wątek posiadał `tx`. Nowy wątek musi posiadać koniec nadawczy kanału, aby móc wysyłać wiadomości przez ten kanał.

Koniec nadawczy posiada metodę `send`, która przyjmuje wartość, którą chcemy wysłać. Metoda `send` zwraca typ `Result<T, E>`, więc jeśli koniec odbiorczy został już zamknięty i nie ma miejsca, gdzie można wysłać wartość, operacja wysyłania zwróci błąd. W tym przykładzie wywołujemy `unwrap`, aby wymusić panikę w przypadku błędu. Jednak w prawdziwej aplikacji odpowiednio byśmy to obsłużyli: wróć do Rozdziału 9, aby przejrzeć strategie właściwego obsługiwania błędów.