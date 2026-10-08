### Używanie klauzul move z wątkami

Klauzula `move` jest często używana wraz z `thread::spawn`, ponieważ pozwala na wykorzystanie danych z jednego wątku w innym wątku.

W Rozdziale 13 wspomnieliśmy, że możemy użyć słowa kluczowego `move` przed listą parametrów klauzuli, aby wymusić przejęcie własności przez klauzulę wartości, z których korzysta w swoim otoczeniu. Technika ta jest szczególnie użyteczna podczas tworzenia nowych wątków po to, by przenieść własność wartości z jednego wątku do drugiego.

Zwróć uwagę w poniższym przykładzie dotyczącego tworzenia wątku, że klauzula, którą przekazujemy do `thread::spawn`, nie przyjmuje żadnych argumentów: nie korzystamy z żadnych danych z głównego wątku w kodzie uruchamianym w nowo utworzonym wątku. Aby użyć danych z głównego wątku w nowo utworzonym wątku, klauzula nowego wątku musi przechwycić potrzebne jej wartości. Poniższy fragment pokazuje próbę utworzenia wektora w głównym wątku i użycia go w nowo utworzonym wątku. Jednak ten kod jeszcze nie zadziała, jak zobaczysz za chwilę.

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

##### Próba użycia wektora stworzonego w głównym wątku w innym wątku

Klauzula używa `v`, więc przechwyci `v` i włączy go do swojego środowiska. Ponieważ `thread::spawn` uruchamia tę klauzulę w nowym wątku, powinniśmy być w stanie uzyskać dostęp do `v` w tym nowym wątku. Ale gdy skompilujemy ten przykład, otrzymamy następujący błąd:

```text
error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
 --> src/main.rs:6:32
  |
6 |     let handle = thread::spawn(|| {
  |                                ^^ may outlive borrowed value `v`
7 |         println!("Here's a vector: {:?}", v);
  |                                           - `v` is borrowed here
  |
note: function requires argument type to outlive `'static`
 --> src/main.rs:6:18
  |
6 |       let handle = thread::spawn(|| {
  |  __________________^
7 | |         println!("Here's a vector: {:?}", v);
8 | |     });
  | |______^
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ^^^^^^^
```

Rust _wywnioskował_, jak przechwycić `v`, a ponieważ `println!` potrzebuje jedynie referencji do `v`, klauzula próbuje pożyczyć `v`. Jednak pojawia się problem: Rust nie jest w stanie określić, jak długo będzie działał nowo utworzony wątek, więc nie wie, czy referencja do `v` zawsze będzie ważna.

Poniższy przykład obrazuje scenariusz, w którym referencja do `v` prawdopodobnie nie będzie ważna:

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    drop(v); // o nie!

    handle.join().unwrap();
}
```

##### Wątek z klauzulą, która próbuje przechwycić referencję do `v` z głównego wątku, gdzie `v` jest usuwane

Gdybyśmy mogli uruchomić ten kod, istnieje możliwość, że nowo utworzony wątek zostałby natychmiast przeniesiony do tła i w ogóle by się nie wykonał. Nowo utworzony wątek ma w sobie referencję do `v`, ale główny wątek natychmiast usuwa `v`, używając funkcji `drop`, którą opisaliśmy w Rozdziale 15. Potem, kiedy nowo utworzony wątek zacznie się wykonywać, `v` nie będzie już ważne, więc referencja do niego również będzie nieprawidłowa. O nie!

Aby naprawić błąd kompilacji w przykładzie dotyczącym przekazywania wektora między wątkami, możemy zastosować sugestię zawartą w komunikacie o błędzie:

```text
help: to force the closure to take ownership of `v` (and any other referenced
variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ^^^^^^^
```

Dodając słowo kluczowe `move` przed klauzulą, wymuszamy na klauzuli przejęcie własności wartości, z których korzysta, zamiast pozwalać Rustowi wnioskować, że powinien pożyczyć te wartości. Zmodyfikowany kod pokazany w poniższym przykładzie zostanie skompilowany i uruchomiony zgodnie z naszym zamiarem:

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

##### Użycie słowa kluczowego move, aby wymusić na klauzuli przejęcie własności wartości, z których korzysta

Co by się stało z kodem w przykładzie z wątkiem i klauzulą, gdzie główny wątek wywołał `drop`, gdybyśmy użyli klauzuli `move`? Czy `move` naprawiłoby tę sytuację? Niestety, nie; otrzymalibyśmy inny błąd, ponieważ to, co próbuje zrobić ten kod, nie jest dozwolone z innego powodu. Gdybyśmy dodali `move` do klauzuli, przenieślibyśmy `v` do środowiska klauzuli, i nie moglibyśmy już wywołać `drop` na nim w głównym wątku. Zamiast tego otrzymalibyśmy taki błąd kompilacji:

```text
error[E0382]: use of moved value: `v`
  --> src/main.rs:10:10
   |
4  |     let v = vec![1, 2, 3];
   |         - move occurs because `v` has type `Vec<i32>`, which does not implement the `Copy` trait
5  | 
6  |     let handle = thread::spawn(move || {
   |                                ------- value moved into closure here
7  |         println!("Here's a vector: {:?}", v);
   |                                           - variable moved due to use in closure
...
10 |     drop(v); // o nie!
   |          ^ wartość użyta tutaj po przeniesieniu
```

Zasady własności Rusta ponownie nas uratowały! Otrzymaliśmy błąd w kodzie dotyczącym przekazywania wektora między wątkami, ponieważ Rust był ostrożny i pożyczył jedynie `v` dla wątku, co oznaczało, że główny wątek mógł teoretycznie unieważnić referencję do `v` w nowo utworzonym wątku. Informując Rusta, aby przekazał własność `v` do nowo utworzonego wątku, gwarantujemy mu, że główny wątek nie będzie już korzystał z `v`. Jeśli zmienimy przykład użycia klauzuli w ten sposób, naruszymy zasady własności, gdy spróbujemy użyć `v` w głównym wątku. Słowo kluczowe `move` zastępuje konserwatywne domyślne zachowanie Rusta dotyczące pożyczania; nie pozwala nam jednak naruszyć zasad własności.

Mając podstawowe zrozumienie wątków i API wątków, przyjrzyjmy się, co możemy _zrobić_ z wątkami.

Możesz odnieść się do następującego rozdziału w Książce o Języku Programowania Rust:
[Korzystanie z wątków do jednoczesnego uruchamiania kodu](https://doc.rust-lang.org/book/ch16-01-threads.html#using-threads-to-run-code-simultaneously)