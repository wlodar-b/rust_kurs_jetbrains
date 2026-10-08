## Typ logiczny Boolean

Jak w większości innych języków programowania, typ logiczny Boolean w języku Rust może przyjmować dwie wartości: `true` i `false`. Typ logiczny w Rust zajmuje jeden bajt pamięci. Typ Boolean w Rust jest oznaczany jako `bool`. Na przykład:

```rust
fn main() {
    let t = true;

    let f: bool = false; // z jawną adnotacją typu
}
```

Głównym sposobem użycia wartości logicznych jest stosowanie ich w warunkach, takich jak wyrażenie `if`. Omówimy, jak działają wyrażenia `if` w języku Rust w lekcji ["Warunki"](course://Common Programming Concepts/Conditions/Intro) tej sekcji.