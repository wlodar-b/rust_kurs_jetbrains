## Typ znakowy

Do tej pory pracowaliśmy tylko z liczbami, ale Rust obsługuje także litery. Typ `char` w języku Rust jest najbardziej prymitywnym typem alfabetycznym, a poniższy kod pokazuje jeden ze sposobów jego użycia. (Zwróć uwagę, że literały `char` są zapisywane w pojedynczych cudzysłowach, w przeciwieństwie do literałów tekstowych, które używają podwójnych cudzysłowów.)

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart = '❤';
}
```

Typ `char` w języku Rust zajmuje cztery bajty i reprezentuje wartość skalarową Unicode, co oznacza, że może przedstawiać znacznie więcej niż tylko znaki ASCII. Litery z akcentami; chińskie, japońskie i koreańskie znaki; emoji; a także spacje o zerowej szerokości są wszystkimi poprawnymi wartościami typu `char` w Rust. Wartości skalarowe Unicode mieszczą się w zakresie od `U+0000` do `U+D7FF` oraz od `U+E000` do `U+10FFFF` włącznie. Jednakże, "znak" tak naprawdę nie jest jasno zdefiniowanym pojęciem w Unicode, więc Twoje ludzkie wyobrażenie o tym, czym jest „znak”, może nie odpowiadać temu, czym jest `char` w Rust. Omówimy ten temat szczegółowo w rozdziale 8, w sekcji [„Przechowywanie tekstu kodowanego w UTF-8 przy użyciu ciągów znaków”](https://doc.rust-lang.org/stable/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings).