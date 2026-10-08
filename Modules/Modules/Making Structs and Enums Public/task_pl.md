### Udostępnianie struktur i enumeracji

Możemy również używać `pub`, aby oznaczyć struktury (structs) i enumeracje (enums) jako publiczne, jednak istnieje kilka dodatkowych szczegółów. Jeśli użyjemy `pub` przed definicją struktury, uczynimy strukturę publiczną, ale jej pola nadal pozostaną prywatne. Możemy decydować o publiczności każdego pola indywidualnie. W poniższym przykładzie zdefiniowaliśmy publiczną strukturę `back_of_house::Breakfast` z publicznym polem `toast`, ale prywatnym polem `seasonal_fruit`. Ten przypadek odzwierciedla sytuację w restauracji, gdzie klient może wybrać rodzaj chleba do posiłku, ale szef kuchni decyduje o owocu podanym z posiłkiem w zależności od sezonu i dostępności. Dostępny owoc zmienia się szybko, więc klienci nie mogą wybrać owocu ani nawet zobaczyć, jaki owoc otrzymają.

```rust
    mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }

        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("brzoskwinie"),
                }
            }
        }
    }

    pub fn eat_at_restaurant() {
        // Zamów śniadanie latem z tostami z żytniego chleba
        let mut meal = back_of_house::Breakfast::summer("Żytni");
        // Zmieniamy zdanie co do rodzaju chleba
        meal.toast = String::from("Pszeniczny");
        println!("Poproszę tosty z {} chleba", meal.toast);

        // Następna linia nie skompiluje się, jeśli ją odkomentujemy; nie możemy
        // zobaczyć ani zmodyfikować sezonowego owocu, który jest dołączony do posiłku
        // meal.seasonal_fruit = String::from("jagody");
    }
```

##### Struktura z kilkoma publicznymi i kilkoma prywatnymi polami

Ponieważ pole `toast` w strukturze `back_of_house::Breakfast` jest publiczne, w funkcji `eat_at_restaurant` możemy odczytywać oraz zapisywać to pole za pomocą notacji kropkowej. Zauważ, że nie możemy używać pola `seasonal_fruit` w `eat_at_restaurant`, ponieważ `seasonal_fruit` jest prywatne. Spróbuj odkomentować linię zmieniającą wartość pola `seasonal_fruit`, aby zobaczyć, jaki błąd się pojawi!

Zwróć również uwagę, że ponieważ `back_of_house::Breakfast` zawiera prywatne pole, struktura musi dostarczyć publiczną funkcję stowarzyszoną, która tworzy instancję `Breakfast` (nazwaliśmy ją tutaj `summer`). Gdyby `Breakfast` nie miała takiej funkcji, nie moglibyśmy utworzyć instancji `Breakfast` w `eat_at_restaurant`, ponieważ nie moglibyśmy ustawić wartości prywatnego pola `seasonal_fruit` w `eat_at_restaurant`.

Z kolei, jeśli oznaczymy enumerację jako publiczną, wszystkie jej warianty również staną się publiczne. Potrzebujemy jedynie `pub` przed słowem kluczowym `enum`, jak pokazano poniżej.

```rust
    mod back_of_house {
        pub enum Appetizer {
            Soup,
            Salad,
        }
    }

    pub fn eat_at_restaurant() {
        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }
```

##### Oznaczenie enumeracji jako publicznej czyni jej warianty publicznymi

Ponieważ oznaczyliśmy enumerację `Appetizer` jako publiczną, możemy używać wariantów `Soup` i `Salad` w `eat_at_restaurant`. Enumeracje nie byłyby zbyt użyteczne, gdyby ich warianty nie były publiczne; mogłoby być uciążliwe oznaczanie wszystkich wariantów enumeracji za pomocą `pub` w każdym przypadku, dlatego domyślnie warianty enumeracji są publiczne. Struktury są natomiast często użyteczne, nawet jeśli ich pola nie są publiczne, dlatego pola struktur stosują ogólną zasadę — wszystko jest domyślnie prywatne, chyba że oznaczone jako `pub`.

Jest jeszcze jedna sytuacja związana z `pub`, której nie omówiliśmy, a dotyczy ona ostatniej funkcji systemu modułów: słowa kluczowego `use`. Najpierw omówimy `use` samodzielnie, a następnie pokażemy, jak łączyć `pub` i `use`.

_Możesz zapoznać się z następnym rozdziałem w książce „The Rust Programming Language": [Ścieżki odnoszące się do elementu w drzewie modułów](https://doc.rust-lang.org/stable/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#paths-for-referring-to-an-item-in-the-module-tree)_