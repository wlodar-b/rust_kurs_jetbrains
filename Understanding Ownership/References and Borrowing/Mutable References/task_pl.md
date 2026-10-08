## Zmienne referencje

Co się stanie, jeśli spróbujemy zmodyfikować coś, co pożyczyliśmy? Wypróbuj kod w poniższym fragmencie. Spoiler: to nie zadziała!

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

##### Próba modyfikacji pożyczonej wartości

Oto błąd:

```text
    error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- help: consider changing this to be a mutable reference: `&mut String`
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
```

Tak samo jak zmienne domyślnie są niemutowalne, tak samo jest z referencjami. Nie możemy modyfikować czegoś, do czego mamy referencję.

Możemy naprawić błąd w kodzie z powyższego fragmentu poprzez niewielką korektę:

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

Najpierw musieliśmy zmienić `s` na `mut`. Następnie musieliśmy utworzyć zmienną referencję za pomocą `&mut s` i zaakceptować zmienną referencję argumentem `some_string: &mut String`.