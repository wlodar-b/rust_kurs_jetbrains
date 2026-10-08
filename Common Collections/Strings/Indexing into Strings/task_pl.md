### Indeksowanie w ciągach znaków

W wielu innych językach programowania dostęp do pojedynczych znaków w ciągu za pomocą odniesienia do nich przez indeks jest poprawną i powszechną operacją. Jednak w języku Rust, jeśli spróbujesz uzyskać dostęp do części `String` za pomocą składni indeksowania, otrzymasz błąd. Rozważ poniższy niepoprawny kod.

```rust
    let s1 = String::from("hello");
    let h = s1[0];
```

##### Próba użycia składni indeksowania z ciągiem `String`

Ten kod wygeneruje następujący błąd:

```text
error[E0277]: the type `String` cannot be indexed by `{integer}`
 --> src/main.rs:3:13
  |
3 |     let h = s1[0];
  |             ^^^^^ `String` cannot be indexed by `{integer}`
  |
  = help: the trait `Index<{integer}>` is not implemented for `String`
```

Błąd i uwaga wyjaśniają sytuację: ciągi w języku Rust nie obsługują indeksowania. Ale dlaczego? Aby odpowiedzieć na to pytanie, musimy omówić sposób, w jaki Rust przechowuje ciągi w pamięci.  
Więcej szczegółów znajdziesz w [The Book](https://doc.rust-lang.org/stable/book/ch08-02-strings.html#indexing-into-strings).