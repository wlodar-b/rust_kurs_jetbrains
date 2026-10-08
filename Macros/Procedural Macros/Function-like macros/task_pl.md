### Makra przypominające funkcje

Makra przypominające funkcje definiują makra, które wyglądają jak wywołania funkcji. Podobnie jak makra `macro_rules!`, są bardziej elastyczne od funkcji; na przykład mogą przyjmować nieokreśloną liczbę argumentów. Jednak makra `macro_rules!` mogą być definiowane jedynie przy użyciu składni przypominającej wzorce, którą omówiliśmy w sekcji [„Deklaratywne makra z `macro_rules!` do ogólnego metaprogramowania”](https://doc.rust-lang.org/stable/book/ch19-06-macros.html#declarative-macros-with-macro_rules-for-general-metaprogramming). Makra przypominające funkcje przyjmują parametr `TokenStream`, a ich definicja manipuluje tym `TokenStream` za pomocą kodu Rust, podobnie jak dwa inne typy makr proceduralnych. Przykładem makra przypominającego funkcję jest makro `sql!`, które mogłoby zostać wywołane w poniższy sposób:

```rust
    let sql = sql!(SELECT * FROM posts WHERE id=1);
```

To makro analizowałoby instrukcję SQL znajdującą się wewnątrz i sprawdzało, czy jest ona składniowo poprawna, co wymaga znacznie bardziej zaawansowanego przetwarzania niż to możliwe w przypadku makra `macro_rules!`. Makro `sql!` zostałoby zdefiniowane w następujący sposób:

```rust
    #[proc_macro]
    pub fn sql(input: TokenStream) -> TokenStream {
```

Ta definicja jest podobna do sygnatury niestandardowego makra `derive`: otrzymujemy tokeny znajdujące się w nawiasach i zwracamy kod, który chcieliśmy wygenerować.