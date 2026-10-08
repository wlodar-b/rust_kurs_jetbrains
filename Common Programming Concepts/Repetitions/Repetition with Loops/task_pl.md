## Powtórzenie z pętlami

Często przydatne jest wykonanie bloku kodu więcej niż raz. Do tego zadania Rust oferuje kilka _pętli_. Pętla wykonuje kod znajdujący się w korpusie pętli do końca, a następnie natychmiast zaczyna ponownie od początku.

Rust obsługuje trzy rodzaje pętli: `loop`, `while` i `for`. Wypróbujmy każdą z nich.

### Powtarzanie kodu przy użyciu loop

Słowo kluczowe `loop` mówi Rustowi, aby wykonywał blok kodu w nieskończoność, dopóki nie zostanie wyraźnie poinformowany, aby przestać.

Na przykład zmień plik `src/main.rs`, aby wyglądał w ten sposób:

```rust
fn main() {
     loop {
         println!("znowu!");
     }
 }
```

Kiedy uruchomimy ten program, zobaczymy `znowu!` wypisywane w kółko, aż do ręcznego zatrzymania programu. Większość terminali obsługuje skrót klawiaturowy <span class="keystroke">ctrl-c</span>, aby przerwać program, który utknął w ciągłej pętli:

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
znowu!
znowu!
znowu!
znowu!
^Cznowu!
```
Symbol `^C` oznacza moment, w którym naciśnięto <span class="keystroke">ctrl-c</span>. Możesz lub nie możesz zobaczyć słowa `znowu!` wypisanego po `^C`, w zależności od tego, gdzie kod znajdował się w pętli, gdy otrzymał sygnał przerwania.

Na szczęście Rust oferuje inny, bardziej niezawodny sposób zakończenia pętli. Możesz użyć słowa kluczowego `break` wewnątrz pętli, aby wskazać programowi, kiedy zakończyć wykonywanie pętli.

### Zwracanie wartości z pętli

Jednym z zastosowań `loop` jest ponowne próbowanie operacji, która może się nie powieść, na przykład sprawdzanie, czy wątek zakończył swoje zadanie. Jednak możesz potrzebować przekazać wynik tej operacji do reszty swojego kodu. Aby to zrobić, możesz dodać wartość, którą chcesz zwrócić, po wyrażeniu `break`, którego używasz do przerwania pętli; ta wartość zostanie zwrócona z pętli, dzięki czemu możesz jej użyć, jak pokazano tutaj:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Wynik to {}", result);
}
```

Przed pętlą deklarujemy zmienną o nazwie `counter` i inicjalizujemy ją wartością `0`. Następnie deklarujemy zmienną o nazwie `result`, aby przechowywać wartość zwróconą z pętli. Na każdej iteracji pętli dodajemy `1` do zmiennej `counter`, a następnie sprawdzamy, czy licznik jest równy `10`. Gdy tak jest, używamy słowa kluczowego `break` z wartością `counter * 2`. Po zakończeniu pętli używamy średnika, aby zakończyć instrukcję przypisania wartości do `result`. Na koniec wypisujemy wartość zmiennej `result`, która w tym przypadku wynosi 20.

_Możesz odwołać się do następnego rozdziału w książce "The Rust Programming Language": [Repetition with Loops](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#repetition-with-loops)_