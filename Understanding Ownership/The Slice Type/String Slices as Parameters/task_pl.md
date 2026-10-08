## Wycinki łańcuchów znaków jako parametry

Wiedząc, że możesz tworzyć wycinki z literałów oraz wartości typu `String`, prowadzi nas to do kolejnej ulepszonej wersji funkcji `first_word` i jej sygnatury:

```rust
    fn first_word(s: &String) -> &str {
```

Bardziej doświadczony użytkownik języka Rust napisałby sygnaturę, jak pokazano w poniższym przykładzie, ponieważ pozwala to użyć tej samej funkcji zarówno na wartościach typu `String`, jak i `&str`.

```rust
    fn first_word(s: &str) -> &str {
```

##### Ulepszenie funkcji first_word przez użycie wycinka łańcucha znaków jako typu parametru s

Jeśli mamy wycinek łańcucha znaków, możemy przekazać go bezpośrednio. Jeśli mamy zmienną typu `String`, możemy przekazać wycinek obejmujący całą zawartość `String`. Zdefiniowanie funkcji przyjmującej wycinek łańcucha znaków zamiast referencji do `String` sprawia, że nasze API staje się bardziej ogólne i użyteczne, nie tracąc przy tym żadnej funkcjonalności:

```rust
fn main() {
    let my_string = String::from("hello world");

    // first_word działa na wycinkach `String`
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word działa na wycinkach literałów łańcuchów znaków
    let word = first_word(&my_string_literal[..]);

    // Ponieważ literały łańcuchów znaków *są* już wycinkami,
    // to działa również bez składni wycinka!
    let word = first_word(my_string_literal);
}