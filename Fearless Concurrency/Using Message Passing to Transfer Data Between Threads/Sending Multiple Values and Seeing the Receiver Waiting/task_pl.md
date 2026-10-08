### Wysyłanie wielu wartości i obserwowanie oczekiwania odbiorcy

Kod w przykładowym fragmencie, który odbiera "hi" w głównym wątku, kompilował się i działał, ale nie pokazywał jasno, że dwa osobne wątki komunikują się ze sobą za pośrednictwem kanału. W poniższym fragmencie wprowadziliśmy pewne modyfikacje, które udowodnią, że kod działa równolegle: wątek uruchomiony za pomocą `spawn` teraz wysyła wiele wiadomości i robi 1-sekundową przerwę między wysłaniem kolejnych wiadomości.

```rust
    use std::thread;
    use std::sync::mpsc;
    use std::time::Duration;

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            // Wysyłanie każdej wartości i oczekiwanie 1 sekundy
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        // Odbieranie wartości jako iterator i ich drukowanie
        for received in rx {
            println!("Got: {}", received);
        }
    }
```

##### Wysyłanie wielu wiadomości i wstrzymywanie się między nimi

Tym razem uruchomiony wątek posiada wektor ciągów znaków (`vector of strings`), które chcemy wysłać do głównego wątku. Iterujemy po nich, wysyłając każdą wartość pojedynczo, a między kolejnymi wiadomościami zatrzymujemy się, wywołując funkcję `thread::sleep` z parametrem `Duration` o wartości 1 sekundy.

W wątku głównym nie wywołujemy już funkcji `recv` bezpośrednio: zamiast tego traktujemy `rx` jako iterator. Dla każdej odebranej wartości wywołujemy jej drukowanie. Gdy kanał zostanie zamknięty, iteracja się zakończy.

Po uruchomieniu kodu z ostatniego przykładu powinieneś zobaczyć następujące wyjście z 1-sekundową przerwą między każdą linią:

```text
    Got: hi
    Got: from
    Got: the
    Got: thread
```

Ponieważ w wątku głównym nie mamy żadnego kodu, który by wprowadzał zatrzymanie lub opóźnienie w pętli `for`, możemy stwierdzić, że wątek główny czeka na odebranie wartości z uruchomionego wątku.