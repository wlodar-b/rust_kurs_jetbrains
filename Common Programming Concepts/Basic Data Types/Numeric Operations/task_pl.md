## Operacje numeryczne

Rust obsługuje podstawowe operacje matematyczne, jakich można się spodziewać dla wszystkich typów liczbowych: dodawanie, odejmowanie, mnożenie, dzielenie oraz reszta z dzielenia. Poniższy kod pokazuje, jak używać każdej z tych operacji w instrukcji `let`:

```rust
fn main() {
    // dodawanie
    let sum = 5 + 10;

    // odejmowanie
    let difference = 95.5 - 4.3;

    // mnożenie
    let product = 4 * 30;

    // dzielenie
    let quotient = 56.7 / 32.2;

    // reszta z dzielenia
    let remainder = 43 % 5;
}
```

Każde wyrażenie w tych instrukcjach wykorzystuje operator matematyczny i jest obliczane do pojedynczej wartości, która następnie zostaje przypisana do zmiennej. [Aneks B](https://doc.rust-lang.org/stable/book/appendix-02-operators.html) zawiera listę wszystkich operatorów dostępnych w języku Rust.