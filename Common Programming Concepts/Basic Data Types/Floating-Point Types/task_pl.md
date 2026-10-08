## Typy zmiennoprzecinkowe

Rust posiada również dwa prymitywne typy dla `liczb zmiennoprzecinkowych`, czyli liczb z punktami dziesiętnymi. Typy zmiennoprzecinkowe w Rust to `f32` i `f64`, które mają odpowiednio 32 bity i 64 bity. Domyślnym typem jest `f64`, ponieważ na nowoczesnych procesorach działa zbliżoną szybkością co `f32`, ale oferuje większą precyzję.

Oto przykład pokazujący liczby zmiennoprzecinkowe w użyciu:

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    let z = 4.0f32; // f32
}
```

Liczby zmiennoprzecinkowe są reprezentowane zgodnie ze standardem IEEE-754. Typ `f32` to liczba zmiennoprzecinkowa o pojedynczej precyzji, a `f64` ma podwójną precyzję.