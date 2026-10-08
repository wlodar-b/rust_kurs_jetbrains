### Kontroler Pożyczania (Borrow Checker)

Kompilator Rust korzysta z *kontrolera pożyczania* (borrow checker), który porównuje zakresy, aby określić, czy wszystkie pożyczki są prawidłowe. Poniższy kod przedstawia ten sam fragment co poprzedni, ale z adnotacjami pokazującymi okresy życia (lifetimes) zmiennych.

```rust,ignore,does_not_compile
{
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

#### Adnotacje okresów życia zmiennych `r` i `x`, nazwanych odpowiednio `'a` i `'b`

Tutaj oznaczyliśmy okres życia zmiennej `r` jako `'a`, a `x` jako `'b`. Jak widać, wewnętrzny blok `'b` jest znacznie mniejszy niż zewnętrzny blok okresu życia `'a`. Podczas kompilacji Rust porównuje długość obu okresów życia i zauważa, że `r` ma okres życia `'a`, ale odwołuje się do pamięci z okresem życia `'b`. Program zostaje odrzucony, ponieważ `'b` jest krótsze od `'a`: obiekt odwołania nie żyje tak długo, jak samo odwołanie.

Poniższy fragment kodu poprawia ten błąd, dzięki czemu nie ma wiszącego odwołania (dangling reference) i kompiluje się bez błędów.

```rust
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
```

#### Prawidłowe odwołanie, ponieważ dane mają dłuższy okres życia niż odwołanie

Tutaj zmienna `x` ma okres życia `'b`, który w tym przypadku jest dłuższy niż `'a`. Oznacza to, że `r` może odwoływać się do `x`, ponieważ Rust wie, że odwołanie w `r` będzie zawsze ważne, gdy tylko `x` będzie ważne.

Teraz, gdy wiesz, jak wyglądają okresy życia odwołań i jak Rust analizuje okresy życia, aby zapewnić, że odwołania zawsze będą prawidłowe, przejdźmy do eksplorowania generycznych okresów życia parametrów i wartości zwracanych w kontekście funkcji.