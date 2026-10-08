### Enum `Option` i jego zalety w porównaniu z pustymi wartościami

W poprzednim rozdziale omówiliśmy, jak enum `IpAddr` pozwala używać systemu typów Rust, aby zakodować w naszym programie więcej informacji niż tylko dane. W tej części przyjrzymy się `Option`, czyli kolejnemu enumowi zdefiniowanemu w bibliotece standardowej. Typ `Option` używany jest w wielu miejscach, ponieważ reprezentuje bardzo powszechny scenariusz, w którym wartość może istnieć lub jej nie być. Wyrażenie tej koncepcji w systemie typów pozwala kompilatorowi sprawdzić, czy obsłużyliśmy wszystkie przypadki, które powinniśmy obsłużyć; ta funkcjonalność pomaga zapobiec błędom, które są niezwykle częste w innych językach programowania.

Projektowanie języka programowania często rozpatrywane jest pod kątem funkcji, które są dostępne, ale równie ważne są te funkcje, które zostały wykluczone. Rust nie posiada cechy, którą mają inne języki – wartości null. *Null* to wartość oznaczająca brak wartości. W językach wspierających null, zmienne zawsze mogą znajdować się w jednym z dwóch stanów: null lub nie-null.

W swojej prezentacji z 2009 roku „Null References: The Billion Dollar Mistake” Tony Hoare, wynalazca null, powiedział:

> Nazywam to moim błędem wartym miliard dolarów. W tamtym czasie projektowałem pierwszy kompletny system typów dla referencji w języku obiektowym. Moim celem było zapewnienie, aby wszystkie użycia referencji były całkowicie bezpieczne, z automatycznym sprawdzaniem wykonywanym przez kompilator. Ale nie mogłem oprzeć się pokusie dodania referencji null, po prostu dlatego, że było to tak łatwe do zaimplementowania. Doprowadziło to do niezliczonych błędów, luk w zabezpieczeniach i awarii systemów, które prawdopodobnie spowodowały cierpienia i straty finansowe na miliard dolarów w ciągu ostatnich czterdziestu lat.

Problem z pustymi wartościami polega na tym, że jeśli spróbujesz użyć null jako wartości nie-null, napotkasz jakiś błąd. Ponieważ cecha null lub nie-null jest wszechobecna, bardzo łatwo jest popełnić taki błąd.

Jednak sama koncepcja, którą null próbuje reprezentować, jest nadal użyteczna: null to wartość, która jest obecnie nieważna lub nie istnieje z jakiegoś powodu.

Problem nie leży w samej idei, lecz w konkretnej implementacji. Z tego powodu Rust nie posiada null, ale ma enum, który może zakodować koncepcję wartości istniejącej lub nieobecnej. Ten enum to `Option<T>`, a w bibliotece standardowej jest [zdefiniowany][option]<!-- ignore --> w następujący sposób:

[option]: https://doc.rust-lang.org/std/option/enum.Option.html

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Enum `Option<T>` jest tak użyteczny, że znajduje się w prelude; nie musisz go jawnie wprowadzać do zakresu. Dotyczy to także jego wariantów: możesz używać `Some` i `None` bez prefiksu `Option::`. Enum `Option<T>` to nadal zwykły enum, a `Some(T)` i `None` to nadal warianty o typie `Option<T>`.

Składnia `<T>` to funkcja w Rust, o której jeszcze nie rozmawialiśmy. To parametr typu generycznego i o generykach opowiemy szerzej w rozdziale "Typy generyczne, cechy i czas życia". Na razie wystarczy wiedzieć, że `<T>` oznacza, iż wariant `Some` enuma `Option` może przechowywać jedną wartość dowolnego typu. Oto kilka przykładów użycia wartości `Option` do przechowywania typów liczbowych i tekstowych:

```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

Jeśli używamy `None` zamiast `Some`, musimy poinformować Rust o typie `Option<T>`, ponieważ kompilator nie może wywnioskować, jaki typ przechowuje wariant `Some`, patrząc jedynie na wartość `None`.

Gdy mamy wartość `Some`, wiemy, że wartość istnieje i jest przechowywana wewnątrz `Some`. Gdy mamy wartość `None`, w pewnym sensie oznacza to to samo co null: nie mamy prawidłowej wartości. Dlaczego więc `Option<T>` jest lepszy niż null?

Krótko mówiąc, ponieważ `Option<T>` i `T` (gdzie `T` może być dowolnym typem) to różne typy, kompilator nie pozwoli nam używać wartości `Option<T>` tak, jakby była zdecydowanie prawidłową wartością. Na przykład ten kod nie skompiluje się, ponieważ próbuje dodać `i8` do `Option<i8>`:

```rust,ignore,does_not_compile
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

Jeśli uruchomimy ten kod, otrzymamy komunikat o błędzie podobny do tego:

```console
error[E0277]: cannot add `Option<i8>` to `i8`
 --> src/main.rs:5:17
  |
5 |     let sum = x + y;
  |                 ^ brak implementacji dla `i8 + Option<i8>`
  |
  = help: cecha `Add<Option<i8>>` nie jest zaimplementowana dla `i8`
```

Zdecydowane! W efekcie ten komunikat o błędzie oznacza, że Rust nie wie, jak dodać `i8` i `Option<i8>`, ponieważ to różne typy. Gdy mamy wartość typu `i8` w Rust, kompilator upewnia się, że zawsze mamy prawidłową wartość. Możemy działać pewnie, bez konieczności sprawdzania, czy wartość jest null przed jej użyciem. Tylko gdy mamy `Option<i8>` (lub dowolny typ wartości, z którą pracujemy), musimy martwić się o możliwość braku wartości, a kompilator upewni się, że ten przypadek został obsłużony przed użyciem wartości.

Innymi słowy, trzeba zamienić `Option<T>` na `T`, zanim będzie można wykonywać operacje na `T`. Generalnie pomaga to wychwycić jeden z najczęstszych problemów związanych z null: zakładanie, że coś nie jest null, podczas gdy rzeczywiście jest.

Brak konieczności martwienia się o błędne założenie, że wartość nie jest null, pomaga zwiększyć pewność w naszym kodzie. Aby mieć wartość, która może być null, trzeba wyraźnie to zaznaczyć, deklarując typ takiej wartości jako `Option<T>`. Następnie, podczas używania tej wartości, należy jawnie obsłużyć przypadek, w którym wartość jest null. Wszędzie tam, gdzie wartość ma typ inny niż `Option<T>`, możemy bezpiecznie założyć, że wartość nie jest null. Była to świadoma decyzja projektowa Rust, aby ograniczyć wszechobecność null i zwiększyć bezpieczeństwo kodu w tym języku.

Jak więc wydobyć wartość `T` z wariantu `Some`, mając wartość typu `Option<T>`, aby można było jej użyć? Enum `Option<T>` ma wiele metod, które są przydatne w różnych sytuacjach; możesz je przejrzeć w [dokumentacji][docs]<!-- ignore -->. Poznanie metod dla `Option<T>` będzie niezwykle przydatne w twojej podróży z Rust.

[docs]: https://doc.rust-lang.org/std/option/enum.Option.html

Ogólnie rzecz biorąc, aby korzystać z wartości `Option<T>`, kod musi obsługiwać każdy wariant. Powinniśmy mieć kod, który zostanie wykonany tylko wtedy, gdy mamy wartość `Some(T)` i który może korzystać z wewnętrznego `T`. Powinniśmy mieć inny kod, który zostanie wykonany, jeśli mamy wartość `None`, i ten kod nie będzie miał dostępu do wartości `T`. Wyrażenie `match` to konstrukcja sterowania przepływem, która robi właśnie to podczas pracy z enumami: uruchamia różny kod w zależności od tego, który wariant enuma jest dostępny, a ten kod może korzystać z danych wewnątrz dopasowanej wartości.