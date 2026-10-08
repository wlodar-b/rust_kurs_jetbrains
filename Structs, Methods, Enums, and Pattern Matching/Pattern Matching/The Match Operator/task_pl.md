## Operator sterowania przepływem `match`

Rust posiada niezwykle potężny operator sterowania przepływem o nazwie `match`, który pozwala porównać wartość z serią wzorców, a następnie wykonać kod na podstawie dopasowanego wzorca. Wzorce mogą składać się z wartości literałów, nazw zmiennych, symboli wieloznacznych i wielu innych elementów; Rozdział 18 [Książki o Rust][book]<!-- ignore --> omawia wszystkie rodzaje wzorców i ich zastosowanie. Moc `match` wynika z wyrazistości wzorców oraz faktu, że kompilator potwierdza obsłużenie wszystkich możliwych przypadków.

[book]: https://github.com/rust-lang/book/tree/master/src

Pomyśl o wyrażeniu `match` jak o maszynie sortującej monety: monety przesuwają się po ścieżce z otworami o różnych rozmiarach, a każda moneta wpada do pierwszego napotkanego otworu, który pasuje do jej rozmiaru. W podobny sposób wartości są sprawdzane pod kątem dopasowania do poszczególnych wzorców w `match`, a gdy wartość „pasuje” do pierwszego wzorca, trafia do odpowiadającego mu bloku kodu, który zostanie wykonany.

Ponieważ właśnie wspomnieliśmy o monetach, użyjmy ich jako przykładu w `match`! Możemy napisać funkcję, która przyjmuje nieznaną monetę ze Stanów Zjednoczonych i podobnie jak maszyna sortująca, określa, jaka to moneta, a następnie zwraca jej wartość w centach, jak pokazano poniżej.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

#### Enum i wyrażenie `match`, które wykorzystuje warianty enum jako swoje wzorce

Przeanalizujmy `match` w funkcji `value_in_cents`. Najpierw zapisuje się słowo kluczowe `match`, a następnie wyrażenie, które w tym przypadku to wartość `coin`. Wyrażenie to wygląda bardzo podobnie do wyrażenia używanego z `if`, ale istnieje zasadnicza różnica: w `if` wyrażenie musi zwracać wartość logiczną, natomiast tutaj może to być dowolny typ. Typ `coin` w tym przykładzie to enum `Coin`, który został zdefiniowany w linii 1.

Następnie mamy ramiona (`arms`) `match`. Ramiona składają się z dwóch części: wzorca i kodu. Pierwsze ramię w tym przypadku ma wzorzec `Coin::Penny`, a następnie operator `=>`, który oddziela wzorzec od kodu do wykonania. Kod w tym przypadku to po prostu wartość `1`. Każde ramię jest oddzielone od kolejnego przecinkiem.

Gdy wyrażenie `match` jest wykonywane, porównuje wynikową wartość z wzorcem każdego ramienia, po kolei. Jeśli wzorzec pasuje do wartości, kod związany z tym wzorcem jest wykonywany. Jeśli wzorzec nie pasuje, wykonywanie przechodzi do kolejnego ramienia, podobnie jak w maszynie sortującej monety. Możemy mieć tyle ramion, ile potrzeba: w przedstawionym powyżej kodzie nasz `match` ma cztery ramiona.

Kod przypisany do każdego ramienia to wyrażenie, a wynikowa wartość tego wyrażenia w dopasowanym ramieniu jest wartością zwracaną dla całego wyrażenia `match`.

Klauzury nawiasowe zwykle nie są używane, jeśli kod w ramieniu `match` jest krótki, jak w powyższym przykładzie, gdzie każde ramię zwraca tylko jedną wartość. Jeśli chcesz wykonać wiele linii kodu w ramieniu `match`, możesz użyć klauzur nawiasowych. Na przykład poniższy kod będzie wypisywał „Lucky penny!” za każdym razem, gdy metoda zostanie wywołana z `Coin::Penny`, ale nadal zwróci ostatnią wartość w bloku, czyli `1`:

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}