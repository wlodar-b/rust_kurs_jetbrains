### Kanały i przekazywanie własności

Zasady dotyczące własności odgrywają kluczową rolę w przesyłaniu wiadomości, ponieważ pomagają pisać bezpieczny, współbieżny kod. Zapobieganie błędom w programowaniu współbieżnym to zaleta wynikająca z uwzględniania własności w całych programach w języku Rust. Zróbmy eksperyment, aby pokazać, jak kanały i własność współpracują, by zapobiegać problemom: spróbujemy użyć wartości `val` w utworzonym wątku _po tym_, jak wyślemy ją przez kanał. Spróbuj skompilować kod w poniższym przykładzie, aby zrozumieć, dlaczego ten kod nie jest dozwolony:

```rust
    use std::thread;
    use std::sync::mpsc;

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
            println!("val to {}", val);
        });

        let received = rx.recv().unwrap();
        println!("Otrzymano: {}", received);
    }
```

##### Próba użycia val po wysłaniu jej przez kanał

Tutaj próbujemy wypisać `val` po tym, jak wysłaliśmy ją przez kanał za pomocą `tx.send`. Pozwolenie na to byłoby złym pomysłem: gdy wartość została wysłana do innego wątku, ten wątek mógłby ją zmodyfikować lub usunąć, zanim ponownie spróbujemy jej użyć. Potencjalnie modyfikacje w innym wątku mogłyby spowodować błędy lub nieoczekiwane wyniki z powodu niespójnych lub nieistniejących danych. Jednak Rust wyświetli nam błąd, jeśli spróbujemy skompilować poniższy kod:

```text
    error[E0382]: borrow of moved value: `val`
      --> src/main.rs:10:31
       |
    8  |         let val = String::from("hi");
       |             --- operacja przeniesienia następuje, ponieważ `val` ma typ `String`, który nie implementuje cechy `Copy`
    9  |         tx.send(val).unwrap();
       |                 --- wartość została przeniesiona tutaj
    10 |         println!("val to {}", val);
       |                               ^^^ wartość została pożyczona tutaj po operacji przeniesienia

```

Nasz błąd związany ze współbieżnością spowodował błąd w czasie kompilacji. Funkcja `send` przejmuje własność swojego parametru, a kiedy wartość jest przeniesiona, odbiornik przejmuje jej własność. Dzięki temu nie możemy przez przypadek ponownie użyć tej wartości po jej wysłaniu; system własności upewnia się, że wszystko jest w porządku.