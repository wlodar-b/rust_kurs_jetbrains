#### Udostępnianie Mutex<T> między wieloma wątkami

Spróbujmy teraz udostępnić wartość pomiędzy wieloma wątkami za pomocą `Mutex<T>`. Uruchomimy 10 wątków, z których każdy zwiększy wartość licznika o 1, tak aby licznik przeszedł od 0 do 10\. Następny przykład spowoduje błąd kompilatora, a dzięki temu błędowi dowiemy się więcej o używaniu `Mutex<T>` i o tym, jak Rust pomaga nam używać go poprawnie. Poniżej znajduje się kod początkowego przykładu:

```rust
    use std::sync::Mutex;
    use std::thread;

    fn main() {
        let counter = Mutex::new(0);
        let mut handles = vec![];

        for _ in 0..10 {
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
```

##### Dziesięć wątków zwiększających licznik chroniony przez Mutex<T>

Tworzymy zmienną `counter`, która przechowuje typ `i32` wewnątrz `Mutex<T>`, tak jak zrobiliśmy to w pierwszym przykładzie. Następnie tworzymy 10 wątków, iterując po zakresie liczb. Używamy `thread::spawn` i przypisujemy wszystkim wątkom tę samą klamrę (closure), która przenosi licznik do wątku, uzyskuje blokadę na `Mutex<T>` za pomocą metody `lock`, a następnie dodaje 1 do wartości w mutexie. Gdy wątek zakończy działanie klamry, `num` wyjdzie z zakresu, zwalniając blokadę, aby inny wątek mógł ją uzyskać.

W głównym wątku zbieramy wszystkie uchwyty (join handles). Następnie wywołujemy `join` na każdym z nich, aby upewnić się, że wszystkie wątki zakończą działanie. W tym momencie główny wątek uzyska blokadę i wypisze wynik tego programu.

Wspomnieliśmy, że ten przykład się nie skompiluje. Sprawdźmy teraz, dlaczego!

```text
error[E0382]: use of moved value: `counter`
  --> src/main.rs:9:36
   |
5  |     let counter = Mutex::new(0);
   |         ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
...
9  |         let handle = thread::spawn(move || {
   |                                    ^^^^^^^ value moved into closure here, in previous iteration of loop
10 |             let mut num = counter.lock().unwrap();
   |                           ------- use occurs due to use in closure
```

Komunikat o błędzie informuje, że wartość `counter` została przeniesiona w poprzedniej iteracji pętli. Rust mówi nam więc, że nie możemy przenieść własności blokady `counter` do wielu wątków. Naprawmy ten błąd kompilatora za pomocą metody wielokrotnego współwłasności omawianej w [Rozdziale 15](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) Rust Book.