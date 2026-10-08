## Szybki przewodnik po modułach

Oto jak moduły, ścieżki, słowo kluczowe `use` oraz słowo kluczowe `pub` działają w kompilatorze i jak większość programistów organizuje swój kod. Przejdziemy przez przykłady każdego z tych reguł, ale jest to świetne miejsce do przypomnienia sobie przyszłościowo, jak działają moduły.

- **Zacznij od korzenia crate**: Podczas kompilowania crate kompilator najpierw sprawdza plik główny crate (zazwyczaj _src/lib.rs_ dla crate bibliotecznego lub _src/main.rs_ dla crate binarnego).  
- **Deklarowanie modułów**: W pliku głównym crate możesz zadeklarować nowy moduł nazwany np. „garden” za pomocą `mod garden;`. Kompilator będzie szukał kodu w module w tych miejscach:  
  - Inline, bezpośrednio po `mod garden`, wewnątrz klamer zamiast średnika  
  - W pliku _src/garden.rs_  
  - W pliku _src/garden/mod.rs_  
- **Deklarowanie podmodułów**: W dowolnym pliku innym niż plik główny crate, który jest kompilowany jako część crate (na przykład _src/garden.rs_), możesz deklarować podmoduły (na przykład `mod vegetables;`). Kompilator będzie szukał kodu wewnątrz podmodułów w tych miejscach w katalogu nazwanym od modułu nadrzędnego:  
  - Inline, bezpośrednio po `mod vegetables`, wewnątrz klamer zamiast średnika  
  - W pliku _src/garden/vegetables.rs_  
  - W pliku _src/garden/vegetables/mod.rs_  
- **Ścieżki do kodu w modułach**: Gdy moduł jest kompilowany jako część twojego crate, możesz odwoływać się do kodu w tym module (na przykład do typu `Asparagus` w module garden vegetables) z dowolnego miejsca w tym crate, używając ścieżki `crate::garden::vegetables::Asparagus`, o ile pozwalają na to zasady prywatności.  
- **Prywatny vs publiczny**: Kod wewnątrz modułu domyślnie jest prywatny względem modułów nadrzędnych. Aby uczynić moduł publicznym, zadeklaruj go jako `pub mod` zamiast `mod`. Aby elementy wewnątrz publicznego modułu również były publiczne, użyj `pub` przed ich deklaracjami.  
- **Słowo kluczowe `use`**: W zakresie można użyć słowa kluczowego `use` do tworzenia skrótów do elementów, aby zmniejszyć powtarzalność długich ścieżek. W każdym zakresie, który może odnosić się do `crate::garden::vegetables::Asparagus`, możesz utworzyć skrót za pomocą `use crate::garden::vegetables::Asparagus;`, po czym wystarczy pisać `Asparagus`, aby używać tego typu w tym zakresie.

Oto crate binarny o nazwie `backyard`, który ilustruje te zasady. Katalog crate, również nazwany `backyard`, zawiera te pliki i katalogi:

```text
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

Plik główny crate, w tym przypadku _src/main.rs_, zawiera:

Nazwa pliku: src/main.rs

```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("Hoduję {:?}!", plant);
}
```

`pub mod garden;` oznacza, że kompilator dołącza kod znaleziony w _src/garden.rs_, który wygląda tak:

Nazwa pliku: src/garden.rs

```rust
pub mod vegetables;
```

A `pub mod vegetables;` oznacza, że kod w _src/garden/vegetables.rs_ również jest dołączony:

```rust
#[derive(Debug)]
pub struct Asparagus {}
```

Przejdźmy teraz do szczegółów tych zasad i pokażmy je w działaniu!