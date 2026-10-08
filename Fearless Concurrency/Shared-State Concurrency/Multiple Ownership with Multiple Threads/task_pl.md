### Współdzielenie własności z wieloma wątkami

W [rozdziale 15](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) książki o Rust, autorzy umożliwili wartością posiadanie wielu właścicieli, korzystając ze wskaźnika sprytnego `Rc<T>` do stworzenia referencyjnie zliczanego obiektu. Zróbmy to samo tutaj i zobaczmy, co się stanie. Opakujemy `Mutex<T>` w `Rc<T>` w poniższym przykładzie i sklonujemy `Rc<T>` przed przeniesieniem własności do wątku. Po przeanalizowaniu błędów wrócimy również do wykorzystania pętli `for`, zachowując słowo kluczowe `move` w zamknięciu.

```rust
    use std::rc::Rc;
    use std::sync::Mutex;
    use std::thread;

    fn main() {
        let counter = Rc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Rc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Wynik: {}", *counter.lock().unwrap());
    }
```

##### Próba użycia Rc<T> do umożliwienia wielu wątkom posiadanie Mutex<T>

Ponownie kompilujemy i otrzymujemy... inne błędy! Kompilator uczy nas bardzo wiele.

```text
error[E0277]: `Rc<Mutex<i32>>` nie może być bezpiecznie przenoszony między wątkami
   --> src/main.rs:11:22
    |
11  |           let handle = thread::spawn(move || {
    |  ______________________^^^^^^^^^^^^^_-
    | |                      |
    | |                      `Rc<Mutex<i32>>` nie może być bezpiecznie przenoszony między wątkami
12  | |             let mut num = counter.lock().unwrap();
13  | |
14  | |             *num += 1;
15  | |         });
    | |_________- w tym `[closure@src/main.rs:11:36: 15:10]`
    |
    = pomoc: w `[closure@src/main.rs:11:36: 15:10]` trait `Send` nie jest zaimplementowany dla `Rc<Mutex<i32>>`
    = notatka: wymagane, ponieważ pojawia się wewnątrz typu `[closure@src/main.rs:11:36: 15:10]`
```

Wow, ta wiadomość o błędzie jest bardzo rozwlekła! Oto najważniejsza część, na którą należy zwrócić uwagę: `` `Rc<Mutex<i32>>` nie może być bezpiecznie przenoszony między wątkami ``. Kompilator mówi nam również, dlaczego: `` trait `Send` nie jest zaimplementowany dla `Rc<Mutex<i32>>` ``. Porozmawiamy o `Send` w następnej sekcji: jest to jeden z traitów, który zapewnia, że typy używane z wątkami są odpowiednie do sytuacji współbieżnych.

Niestety, `Rc<T>` nie jest bezpieczne do współdzielenia między wątkami. Gdy `Rc<T>` zarządza licznikiem referencji, zwiększa jego wartość przy każdym wywołaniu `clone`, a zmniejsza, gdy każda kopia zostaje zniszczona. Nie używa jednak żadnych mechanizmów współbieżności, aby upewnić się, że zmiany w liczniku nie mogą zostać przerwane przez inny wątek. To może prowadzić do błędnego liczenia — subtelnych błędów, które z kolei mogą skutkować wyciekami pamięci lub usunięciem wartości przed jej zakończeniem. Potrzebujemy typu, który jest dokładnie jak `Rc<T>`, ale który wprowadza zmiany w liczniku referencji w sposób bezpieczny dla wątków.