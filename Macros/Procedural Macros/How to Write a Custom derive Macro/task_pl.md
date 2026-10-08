### Jak napisać własny makro `derive`

Stwórzmy crate o nazwie `hello_macro`, który definiuje trait o nazwie `HelloMacro` z jedną powiązaną funkcją o nazwie `hello_macro`. Zamiast zmuszać użytkowników naszego crate'a do implementowania traitu `HelloMacro` dla każdego ze swoich typów, dostarczymy makro proceduralne, aby użytkownicy mogli oznaczyć swój typ za pomocą `#[derive(HelloMacro)]` i uzyskać domyślną implementację funkcji `hello_macro`. Domyślna implementacja będzie wyświetlać `Hello, Macro! My name is TypeName!`, gdzie `TypeName` to nazwa typu, dla którego ten trait został zdefiniowany. Innymi słowy, napiszemy crate, który pozwoli innemu programiście napisać kod podobny do poniższego przy użyciu naszego crate'a.

```rust
    use hello_macro::HelloMacro;
    use hello_macro_derive::HelloMacro;

    #[derive(HelloMacro)]
    struct Pancakes;

    fn main() {
        Pancakes::hello_macro();
    }
```

##### Kod, który użytkownik naszego crate'a będzie mógł napisać, korzystając z naszego makra proceduralnego

Ten kod wyświetli `Hello, Macro! My name is Pancakes!`, gdy nasza praca zostanie ukończona.