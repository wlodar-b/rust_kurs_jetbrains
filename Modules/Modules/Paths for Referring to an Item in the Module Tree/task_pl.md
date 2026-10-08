## Ścieżki do odwoływania się do elementów w drzewie modułów

Aby pokazać Rust, gdzie znaleźć element w drzewie modułów, używamy _ścieżki_ w taki sam sposób, jak używamy ścieżki podczas nawigacji w systemie plików. Jeśli chcemy wywołać funkcję, musimy znać jej ścieżkę.

_Ścieżka_ może przyjąć dwie formy:

*   _Ścieżka absolutna_ zaczyna się od korzenia crate, używając nazwy crate lub literału `crate`.
*   _Ścieżka względna_ zaczyna się od bieżącego modułu i używa `self`, `super` lub identyfikatora w bieżącym module.

Zarówno ścieżki absolutne, jak i względne są kontynuowane przez jeden lub więcej identyfikatorów oddzielonych podwójnymi dwukropkami (`::`).

Wróćmy do przykładu z pierwszego fragmentu kodu w poprzednim zadaniu. Jak wywołać funkcję `add_to_waitlist`? To to samo pytanie, co: jaka jest ścieżka do funkcji `add_to_waitlist`? W poniższym fragmencie kodu nieco uprościliśmy nasz kod, usuwając niektóre moduły i funkcje. Pokażemy dwa sposoby wywołania funkcji `add_to_waitlist` z nowej funkcji `eat_at_restaurant` zdefiniowanej w korzeniu crate. Funkcja `eat_at_restaurant` jest częścią publicznego API naszej biblioteki, więc oznaczamy ją słowem kluczowym `pub`. W sekcji „Eksponowanie ścieżek za pomocą słowa kluczowego `pub`” omówimy `pub` bardziej szczegółowo. Zauważ, że ten przykład jeszcze się nie skompiluje; wkrótce wyjaśnimy dlaczego.

```rust
    mod front_of_house {
        mod hosting {
            fn add_to_waitlist() {}
        }
    }

    pub fn eat_at_restaurant() {
        // Ścieżka absolutna
        crate::front_of_house::hosting::add_to_waitlist();

        // Ścieżka względna
        front_of_house::hosting::add_to_waitlist();
    }
```

##### Wywoływanie funkcji add_to_waitlist za pomocą ścieżek absolutnych i względnych

Za pierwszym razem, gdy wywołujemy funkcję `add_to_waitlist` w `eat_at_restaurant`, używamy ścieżki absolutnej. Funkcja `add_to_waitlist` jest zdefiniowana w tym samym crate, co `eat_at_restaurant`, co oznacza, że możemy użyć słowa kluczowego `crate`, aby rozpocząć ścieżkę absolutną.

Po `crate` dodajemy kolejne moduły, aż dotrzemy do `add_to_waitlist`. Możesz wyobrazić sobie system plików o podobnej strukturze, a my wskazalibyśmy ścieżkę `/front_of_house/hosting/add_to_waitlist`, aby uruchomić program `add_to_waitlist`; użycie nazwy `crate` do rozpoczęcia od korzenia crate jest jak użycie `/`, aby rozpocząć od korzenia systemu plików w terminalu.

Drugi raz, gdy wywołujemy `add_to_waitlist` w `eat_at_restaurant`, używamy ścieżki względnej. Ścieżka zaczyna się od `front_of_house`, nazwy modułu zdefiniowanego na tym samym poziomie drzewa modułów, co `eat_at_restaurant`. Odpowiednikiem w systemie plików byłoby użycie ścieżki `front_of_house/hosting/add_to_waitlist`. Rozpoczęcie od nazwy oznacza, że ścieżka jest względna.

Wybór między użyciem ścieżki względnej a absolutnej zależy od projektu. Decyzja powinna opierać się na tym, czy bardziej prawdopodobne jest, że \ kod definiujący element zostanie przeniesiony niezależnie od kodu, który używa tego elementu, czy razem z nim. Na przykład, jeśli przeniesiemy moduł `front_of_house` i funkcję `eat_at_restaurant` do modułu o nazwie `customer_experience`, musimy zaktualizować ścieżkę absolutną do `add_to_waitlist`, ale ścieżka względna pozostanie ważna. Jeśli jednak przeniesiemy funkcję `eat_at_restaurant` osobno do modułu `dining`, ścieżka absolutna do `add_to_waitlist` pozostanie taka sama, ale ścieżka względna będzie wymagała aktualizacji. Preferujemy określanie ścieżek absolutnych, ponieważ bardziej prawdopodobne jest, że definicje kodu i wywołania elementów będą przenoszone niezależnie od siebie.

Spróbujmy skompilować powyższy fragment kodu i zobaczmy, dlaczego jeszcze się nie kompiluje! Błąd, który otrzymujemy, pokazano w poniższym fragmencie.

```text
Compiling Test_Rust_Project v0.1.0
error[E0603]: module `hosting` is private
 --> src/main.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^

error[E0603]: module `hosting` is private
  --> src/main.rs:12:21
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^
```

##### Błędy kompilatora podczas budowania kodu z powyższego przykładu

Komunikaty błędów mówią, że moduł `hosting` jest prywatny. Innymi słowy, mamy poprawne ścieżki dla modułu `hosting` i funkcji `add_to_waitlist`, ale Rust nie pozwoli nam ich używać, ponieważ nie ma dostępu do prywatnych sekcji.

Moduły nie służą tylko do organizowania kodu; definiują również _granice prywatności_ w Rust: granicę, która kapsułkuje szczegóły implementacji, do których kod zewnętrzny nie ma prawa dostępu, wywoływania ani polegania na nich. Więc jeśli chcesz, aby element, taki jak funkcja lub struktura, był prywatny, umieszczasz go w module.

Prywatność w Rust działa w ten sposób, że wszystkie elementy (funkcje, metody, struktury, wyliczenia, moduły i stałe) są domyślnie prywatne. Elementy w module nadrzędnym nie mogą używać prywatnych elementów w modułach potomnych, ale elementy w modułach potomnych mogą używać elementów w swoich modułach przodków. Wynika to z faktu, że moduły potomne ukrywają szczegóły swojej implementacji, ale mogą widzieć kontekst, w którym są definiowane. Kontynuując analogię z restauracją, zasady prywatności można porównać do zaplecza restauracji: to, co się tam dzieje, jest prywatne dla klientów restauracji, ale menedżerowie biura mogą widzieć i robić wszystko w restauracji, w której działają.

Rust wybrał takie funkcjonowanie systemu modułów, aby ukrywanie szczegółów wewnętrznej implementacji było domyślne. Dzięki temu wiadomo, które części wewnętrznego kodu można zmienić bez łamania zewnętrznego kodu. Ale możesz udostępnić wewnętrzne części kodu modułów potomnych modułom nadrzędnym, używając słowa kluczowego `pub`, aby uczynić element publicznym.