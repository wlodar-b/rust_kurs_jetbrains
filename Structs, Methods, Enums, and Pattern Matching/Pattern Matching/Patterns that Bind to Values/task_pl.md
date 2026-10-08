### Wzorce przypisujące wartości

Kolejną przydatną cechą ramion wyrażenia `match` jest możliwość przypisania elementów wartości, które pasują do wzorca. Dzięki temu możemy wyodrębnić wartości z wariantów wyliczeniowych (`enum`).

Na przykład zmieńmy jeden z naszych wariantów wyliczenia, aby przechowywał dane w sobie. Od 1999 do 2008 roku Stany Zjednoczone wybijały ćwierćdolarówki z różnymi wzorami dla każdego z 50 stanów na jednej stronie monety. Żadne inne monety nie miały wzorów stanowych, więc tylko ćwierćdolarówki mają tę dodatkową wartość. Możemy dodać te informacje do naszego `enum`, zmieniając wariant `Quarter`, aby zawierał wartość `UsState` przechowywaną w nim, co pokazano w poniższym fragmencie kodu.

```rust
#[derive(Debug)] // aby za chwilę móc przyjrzeć się stanowi
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

#### Wyliczenie `Coin`, w którym wariant `Quarter` przechowuje także wartość `UsState`

Wyobraźmy sobie, że nasz znajomy próbuje zebrać wszystkie 50 ćwierćdolarówek stanowych. Podczas sortowania naszej drobnej reszty według rodzaju monety będziemy także wywoływać nazwę stanu powiązanego z każdą ćwierćdolarówką, aby jeśli to moneta, której znajomy jeszcze nie ma, mógł dodać ją do swojej kolekcji.

W wyrażeniu `match` dla tego kodu dodajemy zmienną o nazwie `state` do wzorca, który pasuje do wartości wariantu `Coin::Quarter`. Gdy `Coin::Quarter` pasuje, zmienna `state` przypisuje wartość stanu tej ćwierćdolarówki. Następnie możemy użyć `state` w kodzie tego ramienia, jak poniżej:

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Ćwierćdolarówka ze stanu {:?}!", state);
            25
        }
    }
}
```

Jeśli wywołamy `value_in_cents(Coin::Quarter(UsState::Alaska))`, zmienna `coin` przyjmie wartość `Coin::Quarter(UsState::Alaska)`. Gdy porównamy tę wartość z ramionami wyrażenia `match`, żadne z nich nie pasuje, dopóki nie dotrzemy do `Coin::Quarter(state)`. W tym momencie do `state` przypisujemy `UsState::Alaska`. Możemy wtedy użyć tego przypisania w wyrażeniu `println!`, wyodrębniając wewnętrzną wartość stanu z wariantu `Quarter` w wyliczeniu `Coin`.