### Krok 2: Definiowanie makra proceduralnego

Kolejnym krokiem jest zdefiniowanie makra proceduralnego. W momencie pisania tego tekstu, makra proceduralne muszą znajdować się w osobnym crate. W przyszłości może dojść do zniesienia tego ograniczenia. Zgodnie z konwencją, struktura crate’ów i crate’ów makr proceduralnych wygląda następująco: dla crate o nazwie `foo`, crate makra proceduralnego dla niestandardowego mechanizmu derive nazywa się `foo_derive`.

Tutaj mamy nasz nowy crate `hello_macro_derive` zdefiniowany w pliku `Cargo.toml`.

Nasze dwa crate’y są ściśle powiązane, więc tworzymy crate makra proceduralnego wewnątrz katalogu crate’u `hello_macro`. Jeśli zmienimy definicję cechy (trait) w `hello_macro`, będziemy musieli również zmienić implementację makra proceduralnego w `hello_macro_derive`. Oba crate’y muszą być publikowane oddzielnie, a programiści używający tych crate’ów będą musieli dodać oba jako zależności i wprowadzić je do zakresu (scope). Możemy jednak również zdecydować, aby crate `hello_macro` używał `hello_macro_derive` jako zależności i eksportował kod makra proceduralnego. Struktura projektu, którą przyjęliśmy, umożliwia jednak używanie crate’u `hello_macro` nawet bez funkcjonalności `derive`.

Musimy zadeklarować crate `hello_macro_derive` jako crate makra proceduralnego. Będziemy także potrzebować funkcjonalności z crate’ów `syn` i `quote`, co zobaczysz za chwilę, więc dodajmy je jako zależności. Dodaj następujące wpisy do pliku _Cargo.toml_ dla `hello_macro_derive`:

```toml
    [lib]
    proc-macro = true

    [dependencies]
    syn = "1.0"
    quote = "1.0"
```

Aby rozpocząć definiowanie makra proceduralnego, umieść kod z poniższego fragmentu w pliku _src/lib.rs_ crate’u `hello_macro_derive`. Zwróć uwagę, że kod nie będzie się kompilował, dopóki nie dodamy definicji dla funkcji `impl_hello_macro`.

```rust
    extern crate proc_macro;

    use crate::proc_macro::TokenStream;
    use quote::quote;
    use syn;

    #[proc_macro_derive(HelloMacro)]
    pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
        // Tworzymy reprezentację kodu Rust w postaci drzewa składniowego,
        // które możemy manipulować
        let ast = syn::parse(input).unwrap();

        // Budowanie implementacji cechy (trait)
        impl_hello_macro(&ast)
    }
```

##### Kod wymagany przez większość crate’ów makr proceduralnych w celu przetworzenia kodu Rust

Zauważ, że rozdzieliliśmy kod na funkcję `hello_macro_derive`, która odpowiada za parsowanie `TokenStream`, oraz funkcję `impl_hello_macro`, która odpowiada za transformację drzewa składniowego. Takie podejście zwiększa wygodę pisania makr proceduralnych. Kod w zewnętrznej funkcji (w tym przypadku `hello_macro_derive`) będzie taki sam w niemal każdym crate makra proceduralnego. Kod umieszczony w ciele wewnętrznej funkcji (tu: `impl_hello_macro`) będzie się różnił w zależności od celu konkretnego makra proceduralnego.

Wprowadziliśmy trzy nowe crate’y: `proc_macro`, [`syn`](https://crates.io/crates/syn) oraz [`quote`](https://crates.io/crates/quote). Crate `proc_macro` jest dostarczany z językiem Rust, więc nie musieliśmy dodawać go do zależności w _Cargo.toml_. Crate `proc_macro` to API kompilatora umożliwiające czytanie i manipulowanie kodem Rust z poziomu naszego kodu.

Crate `syn` analizuje kod Rust z ciągu znaków (string) do struktury danych, na której można wykonać operacje. Crate `quote` zamienia struktury danych `syn` z powrotem na kod Rust. Dzięki tym crate’om znacznie uproszczone jest parsowanie dowolnego kodu Rust, który chcemy obsłużyć — napisanie pełnego parsera dla kodu Rust nie jest prostym zadaniem.

Funkcja `hello_macro_derive` zostanie wywołana, gdy użytkownik naszej biblioteki określi `#[derive(HelloMacro)]` na typie. Jest to możliwe, ponieważ oznaczyliśmy tutaj funkcję `hello_macro_derive` za pomocą `proc_macro_derive` i podaliśmy nazwę `HelloMacro`, która odpowiada nazwie naszego traitu; taka konwencja jest stosowana w większości makr proceduralnych.

Funkcja `hello_macro_derive` najpierw konwertuje `input` z `TokenStream` do struktury danych, którą możemy następnie interpretować i wykonywać na niej operacje. Tutaj właśnie wchodzi w grę `syn`. Funkcja `parse` w crate `syn` pobiera `TokenStream` i zwraca strukturę `DeriveInput`, reprezentującą sparsowany kod Rust. Poniżej przykład odpowiednich części struktury `DeriveInput`, którą otrzymujemy, parsując ciąg znaków `struct Pancakes;`:

```rust
    DeriveInput {
        // --obcięte--

        ident: Ident {
            ident: "Pancakes",
            span: #0 bytes(95..103)
        },
        data: Struct(
            DataStruct {
                struct_token: Struct,
                fields: Unit,
                semi_token: Some(
                    Semi
                )
            }
        )
    }
```

##### Obiekt DeriveInput uzyskany podczas parsowania kodu z atrybutem naszego makra w sekcji „Kod, który użytkownik naszego crate może napisać, używając naszego makra proceduralnego”

Pola tej struktury pokazują, że sparsowany kod Rust to struktura jednostkowa (unit struct) z `ident` (identyfikatorem, czyli nazwą) `Pancakes`. Struktura ta zawiera więcej pól do opisu różnych aspektów kodu Rust; szczegółowe informacje znajdziesz w [dokumentacji crate `syn` dla `DeriveInput`](https://docs.rs/syn/0.14.4/syn/struct.DeriveInput.html).

Wkrótce zdefiniujemy funkcję `impl_hello_macro`, w której zbudujemy nowy kod Rust, który chcemy uwzględnić. Zauważ jednak, że wyjście naszego makra derive również jest `TokenStream`. Zwrócony `TokenStream` jest dodawany do kodu pisanego przez użytkowników naszego crate, dzięki czemu podczas kompilacji ich crate uzyskują dodatkową funkcjonalność, którą zapewniamy w zmodyfikowanym `TokenStream`.

Być może zauważyłeś, że wywołujemy `unwrap`, aby spowodować panikę funkcji `hello_macro_derive`, jeśli wywołanie `syn::parse` zakończy się błędem. Makro proceduralne musi się przerwać w przypadku błędów, ponieważ funkcje `proc_macro_derive` muszą zwracać `Token