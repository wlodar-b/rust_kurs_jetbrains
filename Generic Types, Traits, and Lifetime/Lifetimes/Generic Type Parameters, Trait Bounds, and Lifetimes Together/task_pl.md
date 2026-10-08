## Parametry typu generycznego, ograniczenia traitów i lifetimes razem

Przyjrzyjmy się krótko składni definiowania parametrów typu generycznego, ograniczeń traitów i lifetimes w jednej funkcji!

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Ogłoszenie! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

To jest funkcja `longest`, którą omawialiśmy wcześniej i która zwraca dłuższy z dwóch wycinków tekstu (string slice). Teraz jednak posiada dodatkowy parametr o nazwie `ann` typu generycznego `T`, który może być dowolnym typem implementującym trait `Display`, jak określa to klauzula `where`. Ten dodatkowy parametr zostanie wypisany przed porównaniem długości wycinków tekstu, co jest powodem, dla którego ograniczenie traitu `Display` jest konieczne. Ponieważ lifetimes są rodzajem typów generycznych, deklaracje parametru lifetime `'a` oraz parametru generycznego typu `T` znajdują się w tej samej liście w nawiasach ostrokątnych po nazwie funkcji.