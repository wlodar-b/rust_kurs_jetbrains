## Definiowanie modułów dla kontrolowania zakresu i prywatności

_Modules_ (moduły) umożliwiają organizowanie kodu w ramach crate w grupy dla lepszej czytelności i łatwego ponownego użycia. Moduły kontrolują również _prywatność_ elementów, czyli to, czy dany element może być używany przez kod zewnętrzny (_publiczny_), czy jest wewnętrznym szczegółem implementacyjnym i niedostępnym dla kodu zewnętrznego (_prywatny_).

Na przykład napiszmy crate biblioteczny, który zapewnia funkcjonalność restauracji. Zdefiniujemy sygnatury funkcji, ale pozostawimy ich ciała puste, aby skupić się na organizacji kodu, a nie na rzeczywistej implementacji restauracji w kodzie.

W branży restauracyjnej niektóre części restauracji określane są jako _front of house_ (część frontowa) i inne jako _back of house_ (część zaplecza). Front of house to miejsce, gdzie przebywają klienci; tam gospodarze (hosts) przyjmują gości, kelnerzy (servers) przyjmują zamówienia i płatności, a barmani (bartenders) przygotowują napoje. Back of house to miejsce, gdzie szefowie kuchni (chefs) i kucharze (cooks) pracują w kuchni, zmywarki (dishwashers) sprzątają, a menedżerowie zajmują się pracami administracyjnymi.

Aby zorganizować nasz crate w sposób podobny do tego, jak działa rzeczywista restauracja, możemy zorganizować funkcje w zagnieżdżone moduły. Stwórz nową bibliotekę o nazwie `restaurant`, używając polecenia `cargo new --lib restaurant`, a następnie wstaw poniższy kod do _src/lib.rs_, aby zdefiniować kilka modułów i sygnatur funkcji.

```rust
    mod front_of_house {
        mod hosting {
            fn add_to_waitlist() {}

            fn seat_at_table() {}
        }

        mod serving {
            fn take_order() {}

            fn serve_order() {}

            fn take_payment() {}
        }
    }
```

##### Moduł front_of_house zawierający inne moduły, które z kolei zawierają funkcje

Definiujemy moduł, zaczynając od słowa kluczowego `mod`, a następnie podajemy nazwę modułu (w tym przypadku `front_of_house`) i zamykamy ciało modułu w nawiasy klamrowe. W ramach modułów możemy mieć inne moduły, jak w tym przypadku z modułami `hosting` i `serving`. Moduły mogą również zawierać definicje innych elementów, takich jak struktury, wyliczenia, stałe, cechy, lub – jak w powyższym fragmencie kodu – funkcje.

Używając modułów, możemy grupować powiązane definicje razem i nazwać powód, dlaczego są powiązane. Programiści używający tego kodu będą mieli łatwiejszy czas na znalezienie potrzebnych definicji, ponieważ mogą nawigować po kodzie na podstawie grup, zamiast musieć przeszukiwać wszystkie definicje. Programiści dodający nową funkcjonalność do tego kodu będą wiedzieli, gdzie umieścić kod, aby utrzymać porządek w programie.

Wcześniej wspomnieliśmy, że _src/main.rs_ i _src/lib.rs_ są nazywane _crate roots_ (korzeniami crate). Powodem tej nazwy jest to, że zawartość któregokolwiek z tych plików tworzy moduł o nazwie `crate` w korzeniu struktury modułów crate, znanej jako _drzewo modułów_.

Poniżej znajduje się drzewo modułów dla struktury z powyższego kodu.

    crate
     └── front_of_house
         ├── hosting
         │   ├── add_to_waitlist
         │   └── seat_at_table
         └── serving
             ├── take_order
             ├── serve_order
             └── take_payment

##### Drzewo modułów

To drzewo pokazuje, jak niektóre moduły są zagnieżdżone w innych (na przykład `hosting` zagnieżdża się w `front_of_house`). Drzewo pokazuje również, że niektóre moduły są _rodzeństwem_ względem siebie, co oznacza, że są zdefiniowane w tym samym module (`hosting` i `serving` są zdefiniowane w `front_of_house`). Aby kontynuować rodzinne porównanie, jeśli moduł A jest zawarty w module B, mówimy, że moduł A jest _dzieckiem_ modułu B, a moduł B jest _rodzicem_ modułu A. Zauważ, że całe drzewo modułów jest zakorzenione w domyślnym module o nazwie `crate`.

Drzewo modułów może przypominać ci drzewo katalogów w systemie plików twojego komputera; to bardzo trafne porównanie! Tak jak katalogi w systemie plików, używasz modułów do organizowania swojego kodu. I tak jak pliki w katalogu, potrzebujemy sposobu na odnajdywanie naszych modułów.

_Możesz odnieść się do następującego rozdziału w książce o Rust: [Definiowanie modułów dla kontrolowania zakresu i prywatności](https://doc.rust-lang.org/stable/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)_