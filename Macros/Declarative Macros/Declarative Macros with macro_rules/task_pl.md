### Deklaratywne makra z `macro_rules!` do ogólnego metaprogramowania

Najczęściej używaną formą makr w Rust są _deklaratywne makra_. Czasami nazywa się je również „makra przez przykład” ("macros by example"), „makra `macro_rules!`” lub po prostu „makra”. W swojej istocie deklaratywne makra pozwalają pisać coś podobnego do wyrażenia `match` w Rust. Jak omawiano w Rozdziale 6, wyrażenia `match` to struktury sterujące, które przyjmują wyrażenie, porównują wartość wynikającą z tego wyrażenia z wzorcami, a następnie uruchamiają kod powiązany z pasującym wzorcem. Makra również porównują wartość z wzorcami powiązanymi z określonym kodem: w tym przypadku wartością jest dosłowny kod źródłowy Rust przekazany do makra; wzorce są porównywane ze strukturą tego kodu źródłowego; a kod powiązany z każdym pasującym wzorcem zastępuje kod przekazany do makra. Wszystko to odbywa się podczas kompilacji.

Aby zdefiniować makro, używamy konstrukcji `macro_rules!`. Przyjrzyjmy się, jak używać `macro_rules!`, analizując sposób, w jaki zdefiniowano makro `vec!`. Rozdział 8 omawiał, jak można używać makra `vec!` do tworzenia nowego wektora z określonymi wartościami. Na przykład poniższe makro tworzy nowy wektor zawierający trzy liczby całkowite:

```rust
    let v: Vec<u32> = vec![1, 2, 3];
```

Możemy także użyć makra `vec!`, aby utworzyć wektor z dwóch liczb całkowitych lub wektor z pięcioma fragmentami tekstowymi. Nie moglibyśmy użyć funkcji do wykonania tego samego, ponieważ nie znalibyśmy z góry liczby lub typu wartości.

Poniższy fragment kodu prezentuje nieco uproszczoną definicję makra `vec!`.

```rust
    #[macro_export]
    macro_rules! vec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }
```

##### Uproszczona wersja definicji makra vec!

> Uwaga: Faktyczna definicja makra `vec!` w bibliotece standardowej zawiera kod do uprzedniego prealokowania odpowiedniej ilości pamięci. Kod ten jest optymalizacją, której tutaj nie zamieszczamy, aby przykład był prostszy.

Adnotacja `#[macro_export]` wskazuje, że to makro powinno być dostępne zawsze, gdy moduł, w którym zostało zdefiniowane makro, zostanie wprowadzony do zakresu. Bez tej adnotacji nie można by było wprowadzić makra do zakresu.

Następnie rozpoczynamy definicję makra za pomocą `macro_rules!` i nazwy makra, które definiujemy, _bez_ wykrzyknika. Nazwa, w tym przypadku `vec`, jest po niej następująca w nawiasach klamrowych oznaczających ciało definicji makra.

Struktura w treści `vec!` jest podobna do struktury wyrażenia `match`. Mamy tu jedno ramię z wzorcem `( $( $x:expr ),* )`, po którym następuje `=>` i blok kodu powiązany z tym wzorcem. Jeśli wzorzec pasuje, emitowany będzie przypisany blok kodu. Ponieważ w tym makrze jest to jedyny wzorzec, istnieje tylko jeden prawidłowy sposób dopasowania; każde inne dopasowanie zakończy się błędem. Bardziej złożone makra będą mieć więcej niż jedno ramię.

Poprawna składnia wzorców w definicjach makr różni się od składni wzorców omówionej w Rozdziale 18, ponieważ wzorce makr są dopasowywane do struktury kodu Rust, a nie wartości. Przeanalizujmy, co oznaczają elementy wzorca w powyższym fragmencie kodu; pełną składnię wzorców makr można znaleźć w [dokumentacji](https://doc.rust-lang.org/1.30.0/book/first-edition/macros.html).

Najpierw zestaw nawiasów obejmuje cały wzorzec. Następnie występuje znak dolara (`$`), po którym następują nawiasy obejmujące wartości, które pasują do wzorca wewnątrz nawiasów, w celu użycia ich w kodzie zastępczym. Wewnątrz `$()` znajduje się `$x:expr`, który dopasowuje każde wyrażenie w Rust i przypisuje temu wyrażeniu nazwę `$x`.

Przecinek po `$()` wskazuje, że po kodzie pasującym do `$()` w wzorcu opcjonalnie może pojawić się dosłowny znak przecinka. Gwiazdka `*` określa, że wzorzec pasuje zero lub więcej razy do elementów poprzedzających `*`.

Gdy wywołujemy to makro za pomocą `vec![1, 2, 3];`, wzorzec `$x` pasuje trzy razy do trzech wyrażeń `1`, `2` i `3`.

Teraz przyjrzyjmy się wzorcowi w treści kodu powiązanego z tym ramieniem: `temp_vec.push()` wewnątrz `$()*` jest generowane dla każdej części, która pasuje do `$()` w wzorcu zero lub więcej razy w zależności od liczby dopasowań. `$x` zostaje zastąpione każdą dopasowaną wartością. Gdy wywołujemy to makro za pomocą `vec![1, 2, 3];`, kod wygenerowany, który zastępuje to wywołanie makra, będzie następujący:

```rust
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
```

Zdefiniowaliśmy makro, które może przyjmować dowolną liczbę argumentów dowolnego typu i generować kod do utworzenia wektora zawierającego określone elementy.

Istnieją pewne dziwne przypadki brzegowe związane z `macro_rules!`. W przyszłości Rust będzie zawierał drugi rodzaj deklaratywnego makra, który będzie działał w podobny sposób, ale rozwiąże niektóre z tych przypadków brzegowych. Po wprowadzeniu tej aktualizacji `macro_rules!` zostanie w praktyce wycofany. Mając to na uwadze, a także fakt, że większość programistów Rust częściej _używa_ makr niż je _pisze_, nie będziemy dalej omawiać `macro_rules!`. Aby dowiedzieć się więcej o tym, jak pisać makra, zapoznaj się z dokumentacją online lub innymi zasobami, takimi jak [„The Little Book of Rust Macros”](https://danielkeep.github.io/tlborm/book/index.html).