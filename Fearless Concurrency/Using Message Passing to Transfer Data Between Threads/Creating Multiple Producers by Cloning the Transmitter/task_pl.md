### Tworzenie wielu producentów poprzez klonowanie nadajnika

Wcześniej wspomnieliśmy, że `mpsc` jest akronimem od _multiple producer, single consumer_ (wielu producentów, jeden konsument). Użyjmy `mpsc` i rozszerzmy kod z poprzedniego przykładu, aby stworzyć wiele wątków, które wszystkie wysyłają wartości do tego samego odbiornika. Możemy to osiągnąć, klonując nadającą część kanału, jak pokazano poniżej:

```rust
    // --snip--

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("cześć"),
            String::from("z"),
            String::from("tego"),
            String::from("wątku"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("więcej"),
            String::from("wiadomości"),
            String::from("dla"),
            String::from("ciebie"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Odebrano: {}", received);
    }

    // --snip--
```

##### Wysyłanie wielu wiadomości od wielu producentów

Tym razem, zanim utworzymy pierwszy uruchamiany wątek, wywołujemy `clone` na nadającej części kanału. Spowoduje to utworzenie nowego uchwytu do nadawania, który możemy przekazać do pierwszego tworzonego wątku. Oryginalną część nadającą kanału przekazujemy do drugiego tworzonego wątku. Dzięki temu mamy dwa wątki, z których każdy wysyła różne wiadomości do odbierającej części kanału.

Gdy uruchomisz kod, wynik powinien wyglądać mniej więcej tak:

```text
    Odebrano: cześć
    Odebrano: więcej
    Odebrano: z
    Odebrano: wiadomości
    Odebrano: dla
    Odebrano: tego
    Odebrano: wątku
    Odebrano: ciebie
```

Możesz zobaczyć wartości w innej kolejności; zależy to od twojego systemu. Właśnie to czyni współbieżność zarówno interesującą, jak i trudną. Eksperymentując z `thread::sleep`, nadając mu różne wartości w różnych wątkach, każde uruchomienie kodu staje się bardziej niedeterministyczne i za każdym razem generuje inny wynik.

Teraz, gdy przeanalizowaliśmy, jak działają kanały, przyjrzyjmy się innej metodzie współbieżności.

Możesz zapoznać się z poniższym rozdziałem w książce The Rust Programming Language: _[Using Message Passing to Transfer Data Between Threads](https://doc.rust-lang.org/book/ch16-02-message-passing.html#using-message-passing-to-transfer-data-between-threads)_