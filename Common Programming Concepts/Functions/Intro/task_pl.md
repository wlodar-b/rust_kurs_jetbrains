## Funkcje

Funkcje są wszechobecne w kodzie Rust. Już widziałeś jedną z najważniejszych funkcji w tym języku: funkcję `main`, która jest punktem wejścia wielu programów. Widziałeś również słowo kluczowe `fn`, które pozwala na deklarowanie nowych funkcji.

Kod w języku Rust używa _stylu snake case_ jako konwencji nazewnictwa funkcji i zmiennych. W stylu snake case wszystkie litery są małe, a słowa oddzielone są znakami podkreślenia. Oto program zawierający przykład definicji funkcji:

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

Definicje funkcji w Rust zaczynają się od `fn` i zawierają parę nawiasów za nazwą funkcji. Nawiasy klamrowe wskazują kompilatorowi, gdzie zaczyna się i kończy ciało funkcji.

Możemy wywoływać każdą funkcję zdefiniowaną w programie, wpisując jej nazwę, a następnie parę nawiasów. Ponieważ funkcja `another_function` została zdefiniowana w programie, można ją wywołać z wnętrza funkcji `main`. Zauważ, że funkcja `another_function` została zdefiniowana _po_ funkcji `main` w kodzie źródłowym; mogła być również zdefiniowana wcześniej. Rust nie zwraca uwagi na to, gdzie zdefiniujesz funkcje, o ile są one zdefiniowane gdzieś w programie.

Uruchommy kod z powyższego przykładu, aby bliżej przyjrzeć się funkcjom. Umieść przykład `another_function` w pliku src/main.rs i uruchom go. Powinieneś zobaczyć następujące wyjście:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28 secs
     Running `target/debug/functions`
Hello, world!
Another function.
```

Linie są wykonywane w kolejności, w jakiej znajdują się w funkcji `main`. Najpierw zostanie wydrukowany komunikat „Hello, world!”, a następnie wywoływana jest funkcja `another_function`, która drukuje swój komunikat.