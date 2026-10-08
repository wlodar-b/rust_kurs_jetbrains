## Typ String

Aby zilustrować zasady własności, potrzebujemy typu danych bardziej złożonego niż te, które omówiliśmy w sekcji [„Typy danych”](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#data-types) w Rozdziale „Podstawowe pojęcia programistyczne”. Typy omówione wcześniej są przechowywane na stosie i usuwane ze stosu, gdy ich zakres się kończy, ale chcemy przyjrzeć się danym przechowywanym na stercie i zbadać, w jaki sposób Rust wie, kiedy posprzątać te dane.

Użyjemy tu jako przykładu `String` i skupimy się na tych częściach `String`, które odnoszą się do własności. Te aspekty dotyczą również innych złożonych typów danych, niezależnie od tego, czy są one dostarczane przez bibliotekę standardową, czy tworzone przez użytkownika. Omówimy typ `String` bardziej szczegółowo w Rozdziale „Wspólne kolekcje”.

Już wcześniej widzieliśmy literały znakowe, w których wartość tekstu jest zakodowana na sztywno w naszym programie. Literały znakowe są wygodne, ale nie nadają się do każdej sytuacji, w której możemy potrzebować użycia tekstu. Jednym z powodów jest to, że są niezmienne. Innym jest to, że nie każda wartość tekstowa może być znana w momencie pisania kodu: na przykład, co jeśli chcemy przyjąć dane od użytkownika i je przechowywać? W takich sytuacjach Rust posiada drugi typ stringa - `String`. Ten typ jest alokowany na stercie, a zatem jest w stanie przechowywać ilość tekstu, która jest dla nas nieznana w czasie kompilacji. Możemy utworzyć `String` z literału znakowego za pomocą funkcji `from` w następujący sposób:

```rust
let s = String::from("hello");
```

Podwójny dwukropek (`::`) to operator, który pozwala nam na zagnieżdżenie tej konkretnej funkcji `from` w ramach typu `String`, zamiast stosowania jakiejś nazwy w stylu `string_from`. Omówimy tę składnię bardziej szczegółowo w sekcji [„Składnia metod”](https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html#method-syntax) w Rozdziale 5 oraz podczas omawiania przestrzeni nazw w modułach w [„Ścieżki do odwoływania się do elementów drzewa modułów”](https://doc.rust-lang.org/stable/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html) w Rozdziale 7.

Tego rodzaju string _może_ być modyfikowany:

```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() dodaje literał do String

println!("{}", s); // To wyświetli `hello, world!`
```

Więc jaka jest tutaj różnica? Dlaczego `String` można modyfikować, a literałów nie? Różnica polega na tym, w jaki sposób te dwa typy zarządzają pamięcią.

### Pamięć i alokacja

W przypadku literału znakowego jego treść jest znana w czasie kompilacji, więc tekst jest zakodowany bezpośrednio w końcowym pliku wykonywalnym. To właśnie dlatego literały znakowe są szybkie i wydajne. Ale ta efektywność wynika z faktu, że literały znakowe są niezmienne. Niestety, nie możemy wczytać bloku pamięci do pliku binarnego dla każdego tekstu, którego rozmiar jest nieznany w czasie kompilacji i który może zmieniać się podczas działania programu.

W przypadku typu `String`, aby obsłużyć zmienny i rozrastający się fragment tekstu, musimy alokować pewną ilość pamięci na stercie, nieznaną w czasie kompilacji, aby przechowywać zawartość. Oznacza to:

*   Pamięć musi zostać zażądana od alokatora pamięci w czasie działania.
*   Musimy mieć sposób na zwrócenie tej pamięci do alokatora, kiedy skończymy używać naszego `String`.

Pierwszą część wykonujemy my: kiedy wywołujemy `String::from`, jego implementacja żąda potrzebnej pamięci. To jest niemalże uniwersalne w językach programowania.

Druga część jest jednak inna. W językach z _zarządcą pamięci (GC)_, GC śledzi i oczyszcza pamięć, która nie jest już używana, dzięki czemu nie musimy o tym myśleć. Bez GC to nasza odpowiedzialność polega na zidentyfikowaniu, kiedy pamięć nie jest już używana, i wywołaniu kodu, aby ją jawnie zwolnić, tak jak to robiliśmy, aby ją zażądać. Wykonywanie tego poprawnie było historycznie trudnym problemem programistycznym. Jeśli zapomnimy, zmarnujemy pamięć. Jeśli zrobimy to zbyt wcześnie, będziemy mieć nieprawidłową zmienną. Jeśli zrobimy to dwa razy, pojawi się błąd. Musimy dokładnie parować jedno `allocate` z jednym `free`.

Rust podchodzi do tego w inny sposób: pamięć jest automatycznie zwracana, gdy zmienna, która ją posiada, wychodzi z zakresu. Oto wersja naszego przykładowego kodu zakresu z użyciem `String`, a nie literału znakowego:

```rust
{
let s = String::from("hello"); // s jest ważny od tego punktu

// wykonujemy operacje na s
}                                  // ten zakres się kończy, a s już nie jest ważny
```

Istnieje naturalny punkt, w którym możemy zwrócić pamięć potrzebną naszemu `String` do alokatora: kiedy `s` wychodzi z zakresu. Gdy zmienna wychodzi z zakresu, Rust wywołuje dla nas specjalną funkcję. Funkcja ta nazywa się `drop` i to właśnie tam autor `String` może umieścić kod zwracający pamięć. Rust wywołuje automatycznie funkcję `drop` na zamykającym nawiasie klamrowym.

> Uwaga: W C++ ten wzorzec zwalniania zasobów na końcu życia elementu czasami nazywany jest _Resource Acquisition Is Initialization (RAII)_. Funkcja `drop` w Rust jest Ci zapewne znana, jeśli używałeś wzorców RAII.

Ten wzorzec ma ogromny wpływ na sposób, w jaki pisany jest kod w Rust. Może wydawać się to teraz proste, ale zachowanie kodu może być nieoczekiwane w bardziej skomplikowanych sytuacjach, gdy chcemy, aby wiele zmiennych korzystało z danych, które zaalokowaliśmy na stercie. Przyjrzyjmy się teraz niektórym z tych sytuacji.