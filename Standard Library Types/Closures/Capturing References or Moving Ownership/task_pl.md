### Przechwytywanie referencji lub przenoszenie własności

Closury mogą przechwytywać wartości ze swojego środowiska na trzy sposoby, które odpowiadają trzem sposobom przekazywania parametrów w funkcji: przez niezmienne wypożyczenie, zmienne wypożyczenie oraz przejęcie własności. Closura zdecyduje się na odpowiednią metodę w zależności od tego, co jej ciało robi z przechwyconymi wartościami.

Poniższy przykład definiuje closurę, która przechwytuje niezmienne wypożyczenie wektora o nazwie `list`, ponieważ potrzebuje jedynie niezmiennego wypożyczenia, aby wydrukować wartość. Ten przykład ilustruje również, że zmienna może zostać powiązana z definicją closury, a closura może być później wywołana za pomocą nazwy zmiennej i nawiasów, tak jakby nazwa zmiennej była nazwą funkcji:

```rust
fn main() {
    let list = vec![1, 2, 3];
    println!("Przed zdefiniowaniem closury: {:?}", list);

    let only_borrows = || println!("Z closury: {:?}", list);

    println!("Przed wywołaniem closury: {:?}", list);
    only_borrows();
    println!("Po wywołaniu closury: {:?}", list);
}
```

##### Przykład definiowania i wywoływania closury, która przechwytuje niezmienne wypożyczenie

`list` jest nadal dostępna dla kodu przed definicją closury, po definicji closury, lecz przed jej wywołaniem, oraz po wywołaniu closury, ponieważ możemy mieć wiele jednoczesnych niezmiennych wypożyczeń `list` w tym samym czasie. Ten kod kompiluje się, uruchamia i wypisuje:

```console
$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/closure-example`
Przed zdefiniowaniem closury: [1, 2, 3]
Przed wywołaniem closury: [1, 2, 3]
Z closury: [1, 2, 3]
Po wywołaniu closury: [1, 2, 3]
```

Kolejny przykład zmienia definicję closury tak, aby wymagała zmiennego wypożyczenia, ponieważ jej ciało dodaje element do wektora `list`:

```rust
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Przed zdefiniowaniem closury: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("Po wywołaniu closury: {:?}", list);
}
```

##### Przykład definiowania i wywoływania closury, która przechwytuje zmienne wypożyczenie

Ten kod kompiluje się, uruchamia i wypisuje:

```console
$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/closure-example`
Przed zdefiniowaniem closury: [1, 2, 3]
Po wywołaniu closury: [1, 2, 3, 7]
```

Zauważ, że między definicją a wywołaniem closury `borrows_mutably` nie ma już instrukcji `println!`: kiedy `borrows_mutably` jest definiowana, przechwytuje zmienną referencję do `list`. Po wywołaniu closury, ponieważ nie używamy jej później, zmienne wypożyczenie kończy się. Między definicją a wywołaniem closury nie jest dozwolone wykonanie niezmiennego wypożyczenia do wydruku, ponieważ gdy istnieje zmienne wypożyczenie, inne wypożyczenia są niedozwolone. Spróbuj dodać tam instrukcję `println!`, aby zobaczyć, jaki komunikat o błędzie otrzymasz!

Jeśli chcesz wymusić, aby closura przejęła własność wartości, które wykorzystuje w środowisku, nawet jeśli ciało closury nie wymaga jej w sposób ścisły, możesz użyć słowa kluczowego `move` przed listą parametrów. Ta technika jest szczególnie przydatna podczas przekazywania closury do nowego wątku w celu przekazania danych na własność do nowego wątku. Więcej przykładów użycia closur `move` znajdziesz w rozdziale 16, kiedy będziemy omawiać współbieżność.