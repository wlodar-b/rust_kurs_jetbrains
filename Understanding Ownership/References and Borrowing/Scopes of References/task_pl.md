## Zakresy referencji

Pamiętaj, że zakres referencji rozpoczyna się od momentu jej wprowadzenia i trwa do ostatniego użycia tej referencji. Na przykład poniższy kod skompiluje się, ponieważ ostatnie użycie niezmiennych referencji następuje przed wprowadzeniem zmiennej referencji:

```rust
    let mut s = String::from("hello");

    let r1 = &s; // bez problemu
    let r2 = &s; // bez problemu
    println!("{} and {}", r1, r2);
    // r1 i r2 nie są już używane po tym miejscu

    let r3 = &mut s; // bez problemu
    println!("{}", r3);
```

Zakresy niezmiennych referencji `r1` i `r2` kończą się po wywołaniu `println!`, gdzie są one ostatni raz używane, co następuje przed utworzeniem zmiennej referencji `r3`. Te zakresy się nie nakładają, więc ten kod jest dopuszczalny.  
Zdolność kompilatora do wykrywania, że referencja nie jest już używana w danym miejscu przed końcem jej zakresu, nazywa się **Non-Lexical Lifetimes** (skrót NLL), o czym możesz przeczytać więcej w [Przewodniku po edycjach](https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/non-lexical-lifetimes.html).

Chociaż błędy związane z wypożyczaniem mogą być czasami frustrujące, pamiętaj, że kompilator Rust wskazuje potencjalne błędy wcześnie (w czasie kompilacji, a nie w czasie działania programu) i dokładnie pokazuje, gdzie tkwi problem. Dzięki temu nie musisz szukać powodu, dla którego dane nie są zgodne z Twoimi oczekiwaniami.