### Pętle warunkowe z while

Często przydaje się, aby program oceniał warunek w obrębie pętli. Dopóki warunek jest prawdziwy, pętla działa. Gdy warunek przestaje być spełniony, program wywołuje `break`, zatrzymując pętlę. Tego typu pętlę można zaimplementować za pomocą kombinacji `loop`, `if`, `else` i `break`; możesz spróbować to teraz w swoim programie, jeśli chcesz.

Jednak ten wzorzec jest tak powszechny, że Rust ma wbudowaną konstrukcję językową dla tego przypadku, zwaną pętlą `while`. Przykład poniżej przedstawia użycie `while`: program wykonuje pętlę trzy razy, za każdym razem zmniejszając licznik, a następnie, po zakończeniu pętli, wyświetla inny komunikat i kończy działanie.

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```
##### Przykład użycia pętli while do wykonywania kodu, gdy warunek jest prawdziwy

Ta konstrukcja eliminuje wiele zagnieżdżeń, które byłyby konieczne przy użyciu `loop`, `if`, `else` i `break`, a także jest bardziej przejrzysta. Dopóki warunek jest prawdziwy, kod jest wykonywany; w przeciwnym razie pętla zostaje zakończona.