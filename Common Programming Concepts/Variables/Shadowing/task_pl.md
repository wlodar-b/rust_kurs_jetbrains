## Zasłanianie (Shadowing)

W języku Rust możesz zadeklarować nową zmienną o tej samej nazwie co wcześniejsza zmienna, a nowa zmienna zasłania poprzednią zmienną. Rustaceanie mówią, że pierwsza zmienna jest _zasłonięta_ przez drugą, co oznacza, że wartość drugiej zmiennej jest używana, gdy wywołujemy tę nazwę. Możemy zasłonić zmienną, używając tej samej nazwy i powtarzając użycie słowa kluczowego `let`, jak pokazano poniżej:

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("Wartość zmiennej x to: {}", x);
}
```

Ten program najpierw wiąże `x` z wartością `5`. Następnie zasłania `x`, powtarzając `let x =` i dodając `1` do pierwotnej wartości, tak że `x` przyjmuje wartość `6`. Trzecie oświadczenie `let` również zasłania `x`, mnożąc poprzednią wartość przez `2`, aby nadać `x` ostateczną wartość `12`. Po uruchomieniu tego programu zobaczymy następujący wynik:

```text
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/variables`
Wartość zmiennej x to: 12
```

Zasłanianie różni się od oznaczania zmiennej jako `mut`, ponieważ otrzymamy błąd kompilacji, jeśli przez przypadek spróbujemy ponownie przypisać wartość do zmiennej bez użycia słowa kluczowego `let`. Używając `let`, możemy wykonać kilka transformacji na wartości, ale po zakończeniu tych transformacji zmienna pozostaje niemutowalna.

Inną różnicą między `mut` a zasłanianiem jest to, że ponieważ w rzeczywistości tworzymy nową zmienną za każdym razem, gdy ponownie używamy słowa kluczowego `let`, możemy zmienić typ wartości, ale zachować tę samą nazwę. Na przykład, załóżmy, że nasz program pyta użytkownika, ile spacji chce pomiędzy tekstem, wprowadzając znaki spacji, ale my chcemy przechowywać to wejście jako liczbę:

```rust
let spaces = "   ";
let spaces = spaces.len();
```

Taka konstrukcja jest dozwolona, ponieważ pierwsza zmienna `spaces` jest typu string, a druga `spaces`, która jest zupełnie nową zmienną, mającą tę samą nazwę co pierwsza, jest typu liczbowego. Zasłanianie oszczędza nam konieczności wymyślania różnych nazw, takich jak `spaces_str` i `spaces_num`; zamiast tego możemy ponownie użyć prostszej nazwy `spaces`. Jednak gdybyśmy spróbowali użyć `mut`, jak pokazano tutaj, otrzymamy błąd kompilacji:

```rust
let mut spaces = "   ";
spaces = spaces.len();
```

Błąd mówi, że nie możemy zmienić typu zmiennej:

```text
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected &str, found usize
  |
  = note: expected type `&str`
             found type `usize`
```

_Możesz zapoznać się z następnym rozdziałem w książce Rust Programming Language: [Shadowing](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html#shadowing)_

Teraz, gdy omówiliśmy, jak działają zmienne, zastosujmy naszą wiedzę w praktyce.