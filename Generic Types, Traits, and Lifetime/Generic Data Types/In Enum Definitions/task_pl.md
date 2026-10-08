### W definicjach enumów

Tak jak w przypadku struktur, możemy definiować enumy, które przechowują generyczne typy danych w ich wariantach. Przyjrzyjmy się raz jeszcze enumowi `Option<T>`, który jest dostarczany przez bibliotekę standardową i którego używaliśmy w rozdziale „Enumy”:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Ta definicja powinna teraz być dla Ciebie bardziej zrozumiała. Jak możesz zauważyć, `Option<T>` to enum generyczny w stosunku do typu `T` i ma dwa warianty: `Some`, który przechowuje jedną wartość typu `T`, oraz wariant `None`, który nie przechowuje żadnej wartości. Korzystając z enumeracji `Option<T>`, możemy wyrazić abstrakcyjny koncept posiadania opcjonalnej wartości, a ponieważ `Option<T>` jest generyczne, możemy korzystać z tej abstrakcji bez względu na to, jaki typ ma opcjonalna wartość.

Enumy mogą także używać wielu generycznych typów. Definicja enuma `Result`, którego używaliśmy w rozdziale „Błędy odzyskiwalne i nieodzyskiwalne”, jest tego przykładem:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Enum `Result` jest generyczny w stosunku do dwóch typów, `T` i `E`, i ma dwa warianty: `Ok`, który przechowuje wartość typu `T`, oraz `Err`, który przechowuje wartość typu `E`. Ta definicja sprawia, że używanie enuma `Result` jest wygodne wszędzie tam, gdzie działanie może zakończyć się sukcesem (zwracając wartość jakiegoś typu `T`) lub porażką (zwracając błąd jakiegoś typu `E`). W rzeczywistości właśnie tego używaliśmy, otwierając plik w fragmencie kodu „Otwieranie pliku” (sekcja „Błędy odzyskiwalne z Result” w „Obsłudze błędów”), gdzie `T` oznaczał typ `std::fs::File`, gdy plik został poprawnie otwarty, a `E` oznaczał typ `std::io::Error`, gdy wystąpiły problemy z otwarciem pliku.

Rozpoznając sytuacje w swoim kodzie, w których istnieje wiele definicji struktur lub enumów różniących się jedynie typami wartości, które przechowują, możesz uniknąć duplikacji, korzystając z typów generycznych.