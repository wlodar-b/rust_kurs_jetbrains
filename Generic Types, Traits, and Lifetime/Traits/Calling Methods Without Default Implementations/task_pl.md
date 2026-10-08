### Wywoływanie metod bez domyślnych implementacji

Domyślne implementacje mogą wywoływać inne metody w tym samym traitcie, nawet jeśli te inne metody nie mają domyślnej implementacji. W ten sposób trait może dostarczać wiele użytecznych funkcjonalności, wymagając od implementatorów określenia jedynie niewielkiej części. Na przykład, możemy zdefiniować trait `Summary`, który wymaga implementacji metody `summarize_author`, a następnie zdefiniować metodę `summarize` z domyślną implementacją wywołującą metodę `summarize_author`:

```rust,noplayground
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Przeczytaj więcej od {}...)", self.summarize_author())
    }
}
```

Aby używać tej wersji `Summary`, wystarczy zdefiniować `summarize_author` podczas implementacji traitu dla typu:

```rust,ignore
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

Po zdefiniowaniu `summarize_author` możemy wywoływać `summarize` na instancjach struktury `Tweet`. Domyślna implementacja `summarize` wywoła zdefiniowaną przez nas metodę `summarize_author`. Dzięki temu, że zaimplementowaliśmy `summarize_author`, trait `Summary` dostarcza nam zachowanie metody `summarize` bez konieczności pisania dodatkowego kodu.

```rust,ignore
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
        "oczywiście, jak prawdopodobnie już wiesz, ludzie",
    ),
    reply: false,
    retweet: false,
};

println!("1 nowy tweet: {}", tweet.summarize());
```

Ten kod wypisuje: `1 nowy tweet: (Przeczytaj więcej od @horse_ebooks...)`.

Należy zauważyć, że nie jest możliwe wywołanie domyślnej implementacji z nadpisującej implementacji tej samej metody.