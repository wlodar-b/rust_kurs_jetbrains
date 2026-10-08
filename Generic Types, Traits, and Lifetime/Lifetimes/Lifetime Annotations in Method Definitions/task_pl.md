### Adnotacje czasu życia w definicjach metod

Podczas implementacji metod dla struktury z czasami życia, używamy tej samej składni co dla ogólnych parametrów typu przedstawionej w sekcji "Typy danych generycznych" w podrozdziale "Metoda używająca różnych typów generycznych w definicji swojej struktury". To, gdzie deklarujemy i używamy parametrów czasu życia, zależy od tego, czy są one powiązane z polami struktury, czy z parametrami i wartościami zwracanymi metody.

Nazwy czasów życia dla pól struktury zawsze muszą być zadeklarowane po słowie kluczowym `impl` i następnie używane po nazwie struktury, ponieważ te czasy życia są częścią typu struktury.

W podpisach metod w bloku `impl` referencje mogą być powiązane z czasem życia referencji w polach struktury, lub mogą być od nich niezależne. Dodatkowo, reguły pomijania czasu życia często sprawiają, że adnotacje czasu życia w podpisach metod nie są konieczne. Przyjrzyjmy się kilku przykładom z użyciem struktury o nazwie `ImportantExcerpt`, którą zdefiniowaliśmy wcześniej w tej sekcji.

Najpierw użyjemy metody o nazwie `level`, której jedynym parametrem jest referencja do `self`, a zwracaną wartością jest `i32`, co nie jest referencją do niczego:

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

Deklaracja parametru czasu życia po `impl` i jego użycie po nazwie typu są wymagane, ale nie musimy dodawać adnotacji czasu życia dla referencji do `self` ze względu na pierwszą regułę pomijania.

Oto przykład, w którym ma zastosowanie trzecia reguła pomijania czasu życia:

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Uwaga, proszę: {}", announcement);
        self.part
    }
}
```

Są tu dwa czasy życia wejściowe, więc Rust stosuje pierwszą regułę pomijania czasu życia i przypisuje oddzielne czasy życia zarówno `&self`, jak i `announcement`. Następnie, ponieważ jednym z parametrów jest `&self`, typ zwracany otrzymuje czas życia `&self`, i wszystkie czasy życia zostały uwzględnione.