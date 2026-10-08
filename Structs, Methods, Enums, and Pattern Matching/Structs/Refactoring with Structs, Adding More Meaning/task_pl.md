### Refaktoryzacja z użyciem struktur: dodawanie więcej znaczenia

Używamy struktur (structs), aby dodać znaczenie poprzez etykietowanie danych. Możemy przekształcić krotkę, którą używamy, w typ danych z nazwą dla całości, a także nazwami dla poszczególnych części, jak pokazano poniżej.

<span class="filename">Nazwa pliku: src/main.rs</span>

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "Pole prostokąta wynosi {} pikseli kwadratowych.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

#### Definiowanie struktury `Rectangle`

Tutaj zdefiniowaliśmy strukturę i nazwaliśmy ją `Rectangle`. Wewnątrz nawiasów klamrowych zdefiniowaliśmy pola jako `width` i `height`, z których oba mają typ `u32`. Następnie w funkcji `main` utworzyliśmy konkretną instancję `Rectangle`, która ma szerokość równą 30 i wysokość równą 50.

Nasza funkcja `area` jest teraz zdefiniowana z jednym parametrem, który nazwaliśmy `rectangle` i którego typem jest niezmienne wypożyczenie (immutable borrow) instancji struktury `Rectangle`. Jak wspomniano w rozdziale "Zrozumienie własności (Ownership)", chcemy wypożyczyć strukturę zamiast przejmować jej własność. Dzięki temu `main` zachowuje własność i może nadal korzystać z `rect1`, co jest powodem zastosowania `&` w sygnaturze funkcji oraz tam, gdzie wywołujemy funkcję.

Funkcja `area` uzyskuje dostęp do pól `width` i `height` instancji `Rectangle`. Nasza sygnatura funkcji `area` teraz dokładnie oddaje to, co mamy na myśli: oblicz pole prostokąta (`Rectangle`), używając jego pól `width` i `height`. To wskazuje, że szerokość i wysokość są ze sobą powiązane, a także nadaje wartościom opisowe nazwy, zamiast używać indeksów krotki `0` i `1`. To duży krok w kierunku większej przejrzystości.