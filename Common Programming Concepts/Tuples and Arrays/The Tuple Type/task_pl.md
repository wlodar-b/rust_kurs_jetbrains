## Typ krotki

Krotka to uniwersalny sposób grupowania kilku wartości o różnych typach w jeden złożony typ. Krotki mają stałą długość: po zadeklarowaniu nie można ich powiększać ani zmniejszać.

Tworzymy krotkę, zapisując listę wartości oddzielonych przecinkami w nawiasach. Każda pozycja w krotce ma swój typ, a typy różnych wartości w krotce nie muszą być takie same. Dodaliśmy opcjonalne adnotacje typów w tym przykładzie:

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

Zmienna `tup` wiąże się z całą krotką, ponieważ krotka jest uważana za pojedynczy złożony element. Aby pobrać poszczególne wartości z krotki, możemy użyć dopasowania wzorca, aby rozpakować wartość krotki, jak w tym przykładzie:

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("Wartość y to: {}", y);
}
```

Ten program najpierw tworzy krotkę i przypisuje ją do zmiennej `tup`. Następnie używa wzorca z `let`, aby wziąć `tup` i podzielić ją na trzy oddzielne zmienne: `x`, `y` i `z`. To nazywa się _destrukturyzacją_, ponieważ dzieli pojedynczą krotkę na trzy części. Na końcu program wypisuje wartość `y`, która wynosi `6.4`.

Oprócz destrukturyzacji przez dopasowanie wzorca możemy bezpośrednio uzyskać dostęp do elementu krotki za pomocą kropki `(.)`, po której następuje indeks wartości, do której chcemy uzyskać dostęp. Na przykład:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

Ten program tworzy krotkę `x`, a następnie przypisuje nowe zmienne dla każdego elementu, używając odpowiednich indeksów. Podobnie jak w większości języków programowania, pierwszy indeks w krotce to 0.