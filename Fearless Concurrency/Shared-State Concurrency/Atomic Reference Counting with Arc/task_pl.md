#### Atomowe liczenie referencji za pomocą `Arc<T>`

Na szczęście `Arc<T>` *jest* typem podobnym do `Rc<T>`, który jest bezpieczny w użyciu w sytuacjach wymagających współbieżności. Litera *a* oznacza *atomiczny*, co wskazuje, że jest to typ z *atomowym liczeniem referencji*. Atomiki to rodzaj prymitywów współbieżności, których nie omówimy tutaj szczegółowo; szczegóły można znaleźć w dokumentacji biblioteki standardowej dla [`std::sync::atomic`]. Na tym etapie wystarczy wiedzieć, że atomiki działają jak typy prymitywne, ale są bezpieczne do użycia w różnych wątkach.

[`std::sync::atomic`]: https://doc.rust-lang.org/std/sync/atomic/

Możesz się teraz zastanowić, dlaczego wszystkie typy prymitywne nie są atomowe oraz dlaczego typy z biblioteki standardowej nie są domyślnie zaimplementowane z użyciem `Arc<T>`. Powodem jest to, że bezpieczeństwo współbieżności wiąże się z obniżeniem wydajności, którego nie chcemy, jeśli naprawdę nie jest to potrzebne. Jeśli przeprowadzasz operacje na wartościach tylko w jednym wątku, Twój kod może działać szybciej, jeżeli nie musi zapewniać gwarancji, jakie oferują atomiki.

Wróćmy do naszego przykładu: `Arc<T>` i `Rc<T>` mają takie samo API, więc możemy naprawić nasz program, zmieniając linię `use`, wywołanie `new` oraz wywołanie `clone`. Poniższy kod wreszcie się skompiluje i uruchomi:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
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

##### Użycie Arc<T> do opakowania Mutex<T>, aby móc dzielić własność między wieloma wątkami

Ten kod wypisze następujący wynik:

```text
    Result: 10
```

Udało się! Zliczyliśmy liczby od 0 do 10, co może nie wydawać się zbyt imponujące, ale nauczyło nas wiele o `Mutex<T>` i bezpieczeństwie wątków. Możesz także wykorzystać strukturę tego programu do wykonywania bardziej skomplikowanych operacji niż tylko inkrementacja licznika. Korzystając z tej strategii, możesz podzielić obliczenie na niezależne części, rozdzielić je między wątki, a następnie użyć `Mutex<T>`, aby każdy wątek zaktualizował wynik końcowy swoimi danymi.