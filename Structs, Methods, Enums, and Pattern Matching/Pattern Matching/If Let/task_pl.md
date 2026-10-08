## Zwarta kontrola przepływu z użyciem `if let`

Składnia `if let` pozwala połączyć `if` i `let` w mniej rozbudowany sposób, aby obsłużyć wartości, które pasują do jednego wzorca, ignorując pozostałe. Rozważ poniższy program, który dopasowuje się do wartości `Option<u8>`, lecz chce wykonać kod tylko wtedy, gdy wartość wynosi 3.

```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
```

Chcemy zrobić coś w przypadku dopasowania do `Some(3)`, natomiast nie podejmować żadnych działań w przypadku innych wartości `Some<u8>` lub wartości `None`. Aby spełnić wymagania wyrażenia `match`, musimy dodać `_ => ()` po obsłużeniu tylko jednej warianty, co wiąże się z dodaniem sporej ilości kodu szablonowego.

Zamiast tego możemy napisać to w bardziej zwięzły sposób, używając `if let`. Poniższy kod działa identycznie jak `match` w poprzednim przykładzie:

```rust
let some_u8_value = Some(0u8);
if let Some(3) = some_u8_value {
    println!("three");
}
```

Składnia `if let` przyjmuje wzorzec i wyrażenie oddzielone znakiem równości. Działa tak samo jak `match`, gdzie wyrażenie jest przekazywane do `match`, a wzorzec odpowiada jego pierwszemu ramieniu.

Korzystanie z `if let` oznacza mniej pisania, mniejsze wcięcia i mniej kodu szablonowego. Jednak tracisz sprawdzanie kompletności, które wymusza `match`. Wybór między `match` a `if let` zależy od tego, co robisz w danej sytuacji i czy skrócenie kodu jest odpowiednim kompromisem względem utraty sprawdzania kompletności.

Innymi słowy, można myśleć o `if let` jako o "cukrze składniowym" dla `match`, który uruchamia kod, gdy wartość pasuje do jednego wzorca, a następnie ignoruje pozostałe wartości.

Możemy dołączyć `else` do `if let`. Blok kodu powiązany z `else` jest taki sam, jak blok kodu odpowiadający przypadkowi `_` w wyrażeniu `match`, które jest równoważne `if let` oraz `else`. Przypomnijmy definicję wyliczenia `Coin` z poprzedniej sekcji, gdzie wariant `Quarter` również zawierał wartość `UsState`. Jeśli chcielibyśmy zliczać wszystkie monety inne niż ćwierćdolary, ogłaszając jednocześnie stan ćwierćdolarów, moglibyśmy to zrobić za pomocą wyrażenia `match`, jak poniżej:

```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```

Lub moglibyśmy użyć wyrażenia `if let` z `else`, jak poniżej:

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

Jeśli w twoim programie znajduje się logika zbyt rozbudowana, aby wyrazić ją za pomocą `match`, pamiętaj, że `if let` również jest dostępne w twoim zestawie narzędzi Rust.