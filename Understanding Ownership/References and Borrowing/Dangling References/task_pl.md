## Odwołania wiszące

W językach używających wskaźników łatwo jest przez pomyłkę utworzyć _wiszący wskaźnik_ — wskaźnik odwołujący się do miejsca w pamięci, które mogło zostać przydzielone komuś innemu, ponieważ pamięć została zwolniona, ale wskaźnik do niej został zachowany. W przeciwieństwie do tego, w Rust kompilator gwarantuje, że odwołania nigdy nie staną się wiszącymi odwołaniami: jeśli posiadasz odwołanie do jakichś danych, kompilator upewni się, że dane te nie zostaną usunięte z zakresu przed tym, jak samo odwołanie zostanie usunięte.

Spróbujmy stworzyć wiszące odwołanie, jednak Rust zapobiegnie temu dzięki błędowi na etapie kompilacji:

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

Oto błąd:

```text
    error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
5 | fn dangle() -> &'static String {
  |                ^^^^^^^^

```

Ta wiadomość referuje do funkcji językowej, której jeszcze nie omówiliśmy: "czasy życia" (_lifetimes_). Szczegółowo omówimy je w Rozdziale 10. Jednak jeśli pominiemy części dotyczące "czasów życia", w komunikacie znajduje się kluczowe wyjaśnienie, dlaczego ten kod stanowi problem:

```text
    this function's return type contains a borrowed value, but there is no value
    for it to be borrowed from.
```

Przyjrzyjmy się bliżej, co dokładnie dzieje się na każdym etapie kodu `dangle`:

```rust
fn dangle() -> &String { // dangle zwraca odwołanie do String

    let s = String::from("hello"); // s jest nowym String

    &s // zwracamy odwołanie do String, czyli s
} // Tutaj s wychodzi z zakresu i jest wyczyszczone. Jego pamięć jest zwalniana.
  // Niebezpieczeństwo!
```

Ponieważ `s` jest utworzony w ciele funkcji `dangle`, kiedy kod `dangle` zostanie wykonany, `s` zostanie zwolniony. Jednak próbowaliśmy zwrócić do niego odwołanie. Oznacza to, że odwołanie wskazywałoby na nieprawidłowy obiekt `String`. To niedopuszczalne! Rust nie pozwoli nam tego zrobić.

Rozwiązaniem w tym przypadku jest bezpośrednie zwrócenie `String`:

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

Ten kod działa bez problemów. Własność (_ownership_) jest przekazywana dalej, a żadna pamięć nie jest zwalniana.