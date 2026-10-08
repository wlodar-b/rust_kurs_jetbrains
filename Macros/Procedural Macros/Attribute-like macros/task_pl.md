### Makra podobne do atrybutów

Makra podobne do atrybutów są podobne do makr typu derive, ale zamiast generowania kodu dla atrybutu `derive`, pozwalają tworzyć nowe atrybuty. Są również bardziej elastyczne: `derive` działa tylko dla struktur i enumów, natomiast atrybuty mogą być stosowane także do innych elementów, takich jak funkcje. Oto przykład użycia makra podobnego do atrybutu: załóżmy, że mamy atrybut o nazwie `route`, który oznacza funkcje w ramach korzystania z frameworka aplikacji webowej:

```rust
    #[route(GET, "/")]
    fn index() {
```

Ten atrybut `#[route]` zostałby zdefiniowany przez framework jako makro proceduralne. Sygnatura funkcji definiującej to makro wyglądałaby następująco:

```rust
    #[proc_macro_attribute]
    pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

Tutaj mamy dwa parametry typu `TokenStream`. Pierwszy zawiera treść atrybutu: część `GET, "/"`. Drugi to ciało elementu, do którego atrybut jest przypisany: w tym przypadku `fn index() {}` oraz reszta ciała funkcji.

Poza tym makra podobne do atrybutów działają w ten sam sposób co makra typu derive: tworzysz pakiet z typem `proc-macro` i implementujesz funkcję, która generuje pożądany kod!