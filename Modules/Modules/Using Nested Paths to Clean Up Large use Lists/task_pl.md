### Używanie zagnieżdżonych ścieżek, aby uporządkować duże listy use

Jeśli korzystamy z wielu elementów zdefiniowanych w tym samym crate lub module, wymienianie każdego elementu w osobnej linii może zajmować dużo miejsca w pionie w naszych plikach. Na przykład, te dwa polecenia `use` wprowadzają elementy z `std` do zasięgu:

```rust
    use std::cmp::Ordering;
    use std::io;
    // ---snip---
```

Zamiast tego możemy użyć zagnieżdżonych ścieżek, aby wprowadzić te same elementy do zasięgu w jednej linii. Robimy to, określając wspólną część ścieżki, po której stawiamy dwa dwukropki oraz listę elementów ścieżek, które się różnią, umieszczonych w nawiasach klamrowych, jak pokazano poniżej:

```rust
    use std::{cmp::Ordering, io};
    // ---snip---
```

##### Określanie zagnieżdżonej ścieżki, aby wprowadzić wiele elementów z tym samym prefiksem do zasięgu

W większych programach wprowadzanie wielu elementów z tego samego crate lub modułu za pomocą zagnieżdżonych ścieżek może znacznie zmniejszyć liczbę potrzebnych osobnych poleceń `use`!

Możemy użyć zagnieżdżonej ścieżki na dowolnym poziomie w ścieżce, co jest przydatne przy łączeniu dwóch poleceń `use` mających wspólny podkatalog. Na przykład poniższy fragment kodu pokazuje dwa polecenia `use`: jedno wprowadza `std::io` do zasięgu, a drugie wprowadza `std::io::Write`.

```rust
    use std::io;
    use std::io::Write;
```

##### Dwa polecenia use, gdzie jedno jest podścieżką drugiego

Wspólną częścią tych dwóch ścieżek jest `std::io`, i to jest kompletna pierwsza ścieżka. Aby połączyć te dwie ścieżki w jedno polecenie `use`, możemy użyć `self` w zagnieżdżonej ścieżce, jak pokazano w poniższym przykładzie.

```rust
    use std::io::{self, Write};
```

##### Łączenie ścieżek w jedno polecenie use

Ta linia wprowadza `std::io` oraz `std::io::Write` do zasięgu.