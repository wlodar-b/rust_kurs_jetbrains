## Chcesz lody?

Mamy funkcję `maybe_ice_cream`, która zwraca, ile lodów zostało w lodówce.  
Jeśli jest przed 22:00, zostaje 5 sztuk. O godzinie 22:00 ktoś zjada je wszystkie, więc nic już nie zostaje :(

Twoim zadaniem jest ukończenie implementacji funkcji `maybe_ice_cream` i poprawienie testu za nią.

Aby dowiedzieć się więcej o typie Option<T>, sprawdź te linki:

- [Format Enum Option](https://doc.rust-lang.org/stable/book/ch10-01-syntax.html#in-enum-definitions)  
- [Dokumentacja modułu Option](https://doc.rust-lang.org/std/option/)  
- [Dokumentacja enumu Option](https://doc.rust-lang.org/std/option/enum.Option.html)  

<div class="hint">Opcje mogą mieć wartość <code>Some</code> z wartością wewnętrzną lub wartość <code>None</code> bez wartości wewnętrznej.  
Istnieje wiele sposobów, aby uzyskać wewnętrzną wartość. Możesz użyć funkcji <code>unwrap</code> lub dopasowania wzorca. Rozpakowanie 
jest najprostsze, ale jak zrobić to bezpiecznie, aby później nie powodowało paniki?</div>