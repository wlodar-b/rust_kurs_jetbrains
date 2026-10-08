### Makra proceduralne do generowania kodu z atrybutów

Drugą formą makr są _makra proceduralne_, które działają bardziej jak funkcje (i są rodzajem procedury). Makra proceduralne przyjmują kod jako dane wejściowe, operują na tym kodzie, a następnie produkują kod jako dane wyjściowe, zamiast dopasowywać wzorce i zastępować kod innym kodem, jak robią to makra deklaratywne.

Trzy rodzaje makr proceduralnych (custom derive, podobne do atrybutów i podobne do funkcji) działają w podobny sposób.

Podczas tworzenia makr proceduralnych definicje muszą znajdować się w osobnej paczce (crate) ze specjalnym typem paczki. Jest to spowodowane złożonymi technicznymi względami, które mamy nadzieję wyeliminować w przyszłości. Używanie makr proceduralnych wygląda jak kod w poniższym przykładzie, gdzie `some_attribute` jest symbolem zastępczym dla konkretnego makra.

```rust
    use proc_macro;

    #[some_attribute]
    pub fn some_name(input: TokenStream) -> TokenStream {
    }
```

##### Przykład użycia makra proceduralnego

Funkcja definiująca makro proceduralne przyjmuje `TokenStream` jako dane wejściowe i zwraca `TokenStream` jako dane wyjściowe. Typ `TokenStream` jest zdefiniowany w paczce `proc_macro`, która jest dołączona do Rust i reprezentuje sekwencję tokenów. To jest istota makra: kod źródłowy, na którym działa makro, tworzy wejściowy `TokenStream`, a kod wygenerowany przez makro to wyjściowy `TokenStream`. Funkcja ma również przypisany atrybut, który określa, jaki rodzaj makra proceduralnego tworzymy. W ramach jednej paczki możemy mieć różne rodzaje makr proceduralnych.

Spójrzmy na różne rodzaje makr proceduralnych. Rozpoczniemy od makra custom derive, a następnie wyjaśnimy niewielkie różnice, które odróżniają inne formy.