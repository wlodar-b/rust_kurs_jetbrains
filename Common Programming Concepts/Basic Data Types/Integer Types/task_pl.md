## Typy całkowitoliczbowe

_Liczba całkowita_ to liczba pozbawiona części ułamkowej. W Lekcji 2 używaliśmy jednego typu całkowitoliczbowego, `u32`. To oznaczenie typu wskazuje, że wartość, z którą jest powiązany, powinna być liczbą całkowitą bez znaku (typy liczb całkowitych ze znakiem zaczynają się od `i`, zamiast `u`), zajmującą 32 bity pamięci. Poniższa tabela przedstawia wbudowane typy całkowitoliczbowe w języku Rust. Każdy wariant z kolumn Znakowane i Bezznakowe (na przykład, i16) może być używany do deklarowania typu wartości całkowitoliczbowej.

| Długość | Znakowane | Bezznakowe |
|---------|-----------|------------|
| 8-bit   | i8        | u8         |
| 16-bit  | i16       | u16        |
| 32-bit  | i32       | u32        |
| 64-bit  | i64       | u64        |
| 128-bit | i128      | u128       |
| arch    | isize     | usize      |

##### Tabela: Typy całkowitoliczbowe w Rust

Każdy wariant może być albo znakowany, albo bezznakowy i ma określoną wielkość. _Znakowane_ i _bezznakowe_ odnoszą się do tego, czy liczba może być ujemna lub dodatnia—innymi słowy, czy liczba musi mieć znak (znakowana) lub czy będzie zawsze dodatnia i dlatego może być przedstawiana bez znaku (bezznakowa). To jak pisanie liczb na papierze: kiedy znak ma znaczenie, liczba jest pokazywana z plusem lub minusem; jednak gdy można bezpiecznie założyć, że liczba jest dodatnia, jest ona przedstawiana bez znaku. Liczby znakowane są przechowywane przy użyciu [reprezentacji uzupełnienia do dwóch](https://pl.wikipedia.org/wiki/Uzupe%C5%82nienie_dw%C3%B3jkowe).

Każdy znakowany wariant może przechowywać liczby w zakresie od -($2^{n-1}$) do $2^{n-1}$-1 włącznie, gdzie _n_ to liczba bitów używanych przez ten wariant. Tak więc `i8` może przechowywać liczby od -($2^7$) do $2^7$-1, co daje zakres od -128 do 127. Bezznakowe warianty mogą przechowywać liczby od 0 do $2^n$-1, więc `u8` może przechowywać liczby od 0 do $2^8$-1, czyli od 0 do 255.

Dodatkowo typy `isize` i `usize` zależą od rodzaju komputera, na którym działa Twój program: 64 bity w przypadku architektury 64-bitowej i 32 bity w przypadku architektury 32-bitowej.

Możesz zapisać literały całkowitoliczbowe w dowolnej z form pokazanych w tabeli poniżej. Należy zauważyć, że wszystkie literały liczbowe z wyjątkiem literału bajtowego pozwalają na dodanie sufiksu określającego typ, takiego jak `57u8`, oraz `_` jako separatora wizualnego, na przykład `1_000`.

| Literały liczbowe | Przykład        |
|--------------------|-----------------|
| Dziesiętny        | 98_222         |
| Szesnastkowy      | 0xff           |
| Ósemkowy          | 0o77           |
| Binarny           | 0b1111_0000    |
| Bajtowy (tylko u8) | b'A'           |

##### Tabela: Literały liczbowe w Rust

Jak więc wybrać odpowiedni typ całkowitoliczbowy? Jeśli nie masz pewności, domyślne ustawienia Rust są zazwyczaj odpowiednim wyborem, a typy całkowitoliczbowe domyślnie ustawione są na `i32`: ten typ jest generalnie najszybszy, nawet na systemach 64-bitowych. Podstawową sytuacją, w której używałbyś `isize` lub `usize`, jest indeksowanie jakiegoś rodzaju kolekcji.