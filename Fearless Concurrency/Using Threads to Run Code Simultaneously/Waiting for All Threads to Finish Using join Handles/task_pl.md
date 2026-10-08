### Oczekiwanie na zakończenie wszystkich wątków za pomocą uchwytów join

Kod powyżej nie tylko przerywa wcześniej utworzony wątek przedwcześnie w większości przypadków z powodu zakończenia wątku głównego, ale także nie gwarantuje, że nowo utworzony wątek w ogóle zostanie uruchomiony. Powodem tego jest brak gwarancji co do kolejności uruchamiania wątków!

Możemy rozwiązać problem braku uruchamiania wątku (lub jego niepełnego wykonania), zapisując wartość zwrotną funkcji `thread::spawn` w zmiennej. Typ zwrotny funkcji `thread::spawn` to `JoinHandle`. `JoinHandle` to wartość posiadana, na której możemy wywołać metodę `join`, aby poczekać, aż utworzony wątek się zakończy. Przykład poniżej pokazuje, jak użyć obiektu `JoinHandle` dla wątku, który wcześniej utworzyliśmy, i wywołać `join`, aby upewnić się, że wątek zakończy się przed zakończeniem funkcji `main`:

```rust
    use std::thread;
    use std::time::Duration;

    fn main() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("Cześć numer {} z nowo utworzonego wątku!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("Cześć numer {} z wątku głównego!", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();
    }
```

##### Zapisywanie obiektu JoinHandle zwróconego przez thread::spawn, aby zagwarantować pełne wykonanie wątku

Wywołanie metody `join` na uchwycie blokuje aktualnie uruchomiony wątek do czasu zakończenia wątku reprezentowanego przez ten uchwyt. _Blokowanie_ wątku oznacza, że wątek ten nie może wykonywać pracy ani się zakończyć. Ponieważ umieściliśmy wywołanie `join` po pętli `for` w wątku głównym, uruchomienie tego kodu powinno dać wynik podobny do tego:

```text
    Cześć numer 1 z wątku głównego!
    Cześć numer 2 z wątku głównego!
    Cześć numer 1 z nowo utworzonego wątku!
    Cześć numer 3 z wątku głównego!
    Cześć numer 2 z nowo utworzonego wątku!
    Cześć numer 4 z wątku głównego!
    Cześć numer 3 z nowo utworzonego wątku!
    Cześć numer 4 z nowo utworzonego wątku!
    Cześć numer 5 z nowo utworzonego wątku!
    Cześć numer 6 z nowo utworzonego wątku!
    Cześć numer 7 z nowo utworzonego wątku!
    Cześć numer 8 z nowo utworzonego wątku!
    Cześć numer 9 z nowo utworzonego wątku!
```

Dwa wątki nadal na przemian wykonują swoje zadania, ale wątek główny czeka na zakończenie nowo utworzonego wątku dzięki wywołaniu `handle.join()` i nie kończy się, dopóki ten wątek nie zakończy pracy.

Teraz zobaczmy, co się stanie, jeśli przeniesiemy `handle.join()` przed pętlę `for` w funkcji `main`, jak tutaj:

```rust
    use std::thread;
    use std::time::Duration;

    fn main() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("Cześć numer {} z nowo utworzonego wątku!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        handle.join().unwrap();

        for i in 1..5 {
            println!("Cześć numer {} z wątku głównego!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
```

Wątek główny poczeka na zakończenie nowo utworzonego wątku, a następnie wykona swoją pętlę `for`, więc wynik nie będzie już przeplatany, jak pokazano tutaj:

```text
    Cześć numer 1 z nowo utworzonego wątku!
    Cześć numer 2 z nowo utworzonego wątku!
    Cześć numer 3 z nowo utworzonego wątku!
    Cześć numer 4 z nowo utworzonego wątku!
    Cześć numer 5 z nowo utworzonego wątku!
    Cześć numer 6 z nowo utworzonego wątku!
    Cześć numer 7 z nowo utworzonego wątku!
    Cześć numer 8 z nowo utworzonego wątku!
    Cześć numer 9 z nowo utworzonego wątku!
    Cześć numer 1 z wątku głównego!
    Cześć numer 2 z wątku głównego!
    Cześć numer 3 z wątku głównego!
    Cześć numer 4 z wątku głównego!
```

Małe szczegóły, takie jak miejsce wywołania metody `join`, mogą wpływać na to, czy wątki będą działały jednocześnie.