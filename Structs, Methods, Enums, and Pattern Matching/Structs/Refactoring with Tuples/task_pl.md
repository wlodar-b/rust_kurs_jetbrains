### Refaktoryzacja z wykorzystaniem krotek

Poniższy fragment kodu przedstawia inną wersję naszego programu, która używa krotek.

<span class="filename">Nazwa pliku: src/main.rs</span>

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "Powierzchnia prostokąta wynosi {} pikseli kwadratowych.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

#### Określanie szerokości i wysokości prostokąta za pomocą krotki

Pod pewnym względem ten program jest lepszy. Krotki pozwalają nam dodać trochę struktury, a teraz przekazujemy tylko jeden argument. Jednak w innym sensie ta wersja jest mniej czytelna: krotki nie nazywają swoich elementów, więc nasza kalkulacja stała się bardziej zagmatwana, ponieważ musimy odwoływać się do indeksów poszczególnych części krotki.

Nie miałoby to znaczenia, gdybyśmy pomieszali szerokość i wysokość w obliczeniach powierzchni, ale gdybyśmy chcieli narysować prostokąt na ekranie, miałoby to znaczenie! Musielibyśmy pamiętać, że `szerokość` to indeks `0` w krotce, a `wysokość` to indeks `1`. Jeśli ktoś inny pracowałby nad tym kodem, musiałby to zrozumieć i również o tym pamiętać. Łatwo byłoby zapomnieć lub pomylić te wartości i spowodować błędy, ponieważ nie wyraziliśmy znaczenia naszych danych w kodzie.