### Korzystanie z ograniczeń w traitach do warunkowego implementowania metod

Używając ograniczenia traitu w bloku `impl`, który wykorzystuje parametry typów generycznych, możemy implementować metody warunkowo dla typów, które implementują określone traity. Na przykład typ `Pair<T>` w poniższym przykładzie zawsze implementuje funkcję `new`. Jednak `Pair<T>` implementuje metodę `cmp_display` tylko wtedy, gdy jego wewnętrzny typ `T` implementuje zarówno trait `PartialOrd`, który umożliwia porównywanie, *jak i* trait `Display`, który umożliwia wyświetlanie.

```rust,noplayground
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("Największy element to x = {}", self.x);
        } else {
            println!("Największy element to y = {}", self.y);
        }
    }
}
```

#### Warunkowa implementacja metod dla typu generycznego w zależności od ograniczeń traitów

Możemy także warunkowo implementować trait dla dowolnego typu, który implementuje inny trait. Implementacje traitów dla dowolnego typu spełniającego ograniczenia danego traitu nazywane są *implementacjami ogólnymi* (*blanket implementations*) i są szeroko stosowane w standardowej bibliotece Rust. Na przykład biblioteka standardowa implementuje trait `ToString` dla każdego typu, który implementuje trait `Display`. Blok `impl` w bibliotece standardowej wygląda podobnie do tego kodu:

```rust,ignore
impl<T: Display> ToString for T {
    // --snip--
}
```

Dzięki tej ogólnej implementacji w bibliotece standardowej możemy wywoływać metodę `to_string` zdefiniowaną przez trait `ToString` na każdym typie, który implementuje trait `Display`. Na przykład możemy przekształcić liczby całkowite w odpowiadające im wartości `String` w następujący sposób, ponieważ liczby całkowite implementują `Display`:

```rust
let s = 3.to_string();
```

Ogólne implementacje pojawiają się w dokumentacji traitów w sekcji „Implementors”.

Traity i ograniczenia traitów pozwalają nam pisać kod wykorzystujący parametry typów generycznych, aby zmniejszyć duplikację, ale również określić dla kompilatora, że chcemy, aby typ generyczny miał określone zachowanie. Kompilator może następnie wykorzystać informacje z ograniczeń traitów, aby sprawdzić, czy wszystkie konkretne typy używane w naszym kodzie zapewniają poprawne zachowanie. W językach dynamicznie typowanych otrzymalibyśmy błąd w czasie wykonywania, jeśli wywołalibyśmy metodę na typie, który nie definiuje tej metody. Jednak w Rust takie błędy są zgłaszane w czasie kompilacji, dzięki czemu jesteśmy zmuszeni poprawić problem przed uruchomieniem kodu. Dodatkowo nie musimy pisać kodu sprawdzającego zachowanie w czasie wykonywania, ponieważ sprawdziliśmy je już w czasie kompilacji. Podejście to poprawia wydajność bez konieczności rezygnacji z elastyczności generyków.