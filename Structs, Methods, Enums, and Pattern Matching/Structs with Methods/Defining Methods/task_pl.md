### Definiowanie metod

Zmieńmy funkcję `area`, która przyjmuje instancję `Rectangle` jako parametr, aby zamiast tego była metodą `area` zdefiniowaną na strukturze `Rectangle`, jak pokazano poniżej.

<span class="filename">Nazwa pliku: src/main.rs</span>

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "Pole prostokąta wynosi {} pikseli kwadratowych.",
        rect1.area()
    );
}
```

#### Definiowanie metody `area` na strukturze `Rectangle`

Aby zdefiniować funkcję w kontekście `Rectangle`, rozpoczynamy blok `impl` (implementacja). Następnie przenosimy funkcję `area` do nawiasów klamrowych `impl` i zmieniamy pierwszy (a w tym przypadku jedyny) parametr na `self` w sygnaturze oraz wszędzie w ciele funkcji. W `main`, gdzie wcześniej wywoływaliśmy funkcję `area`, przekazując `rect1` jako argument, teraz możemy użyć *składni metody*, aby wywołać metodę `area` na naszej instancji `Rectangle`. Składnia metody pojawia się po instancji: dodajemy kropkę, a po niej nazwę metody, nawiasy i ewentualne argumenty.

W sygnaturze `area` używamy `&self` zamiast `rectangle: &Rectangle`, ponieważ Rust wie, że typ `self` to `Rectangle`, z uwagi na to, że metoda ta znajduje się w kontekście `impl Rectangle`. Zauważ, że nadal musimy użyć `&` przed `self`, tak jak zrobiliśmy to w `&Rectangle`. Metody mogą przejmować własność `self`, pożyczać `self` w sposób niemutowalny, jak zrobiliśmy to tutaj, lub pożyczać `self` w sposób mutowalny, tak jak w przypadku każdego innego parametru.

Wybraliśmy tutaj `&self` z tego samego powodu, dla którego używaliśmy `&Rectangle` w wersji funkcji: nie chcemy przejmować własności, a jedynie odczytać dane w strukturze, bez ich modyfikacji. Jeśli chcielibyśmy zmienić instancję, na której wywołaliśmy metodę, w ramach działania metody, użylibyśmy `&mut self` jako pierwszego parametru. Metoda, która przejmuje własność instancji, używając tylko `self` jako pierwszego parametru, jest rzadko spotykana; technika ta jest zwykle stosowana, gdy metoda przekształca `self` w coś innego i chcemy uniemożliwić wywołującemu użycie oryginalnej instancji po transformacji.

Główną korzyścią ze stosowania metod zamiast funkcji, oprócz możliwości użycia składni metody i braku konieczności powtarzania typu `self` w każdej sygnaturze metody, jest organizacja. Umieszczamy wszystkie rzeczy, które można zrobić z instancją danego typu, w jednym bloku `impl` zamiast zmuszać przyszłych użytkowników naszego kodu do szukania możliwości struktury `Rectangle` w różnych miejscach biblioteki, którą udostępniamy.

> ### Gdzie jest operator `->`?
>
> W językach C i C++ używa się dwóch różnych operatorów do wywoływania metod: korzystamy z `.`, gdy wywołujemy metodę bezpośrednio na obiekcie, oraz `->`, gdy wywołujemy metodę na wskaźniku do obiektu i najpierw musimy dokonać dereferencji tego wskaźnika. Innymi słowy, jeśli `object` jest wskaźnikiem, `object->something()` jest podobne do `(*object).something()`.
>
> Rust nie posiada odpowiednika operatora `->`; zamiast tego, Rust oferuje funkcję zwaną *automatycznym referencjonowaniem i dereferencjonowaniem*. Wywoływanie metod jest jednym z niewielu miejsc w Rust, gdzie używana jest ta funkcjonalność.
>
> Oto jak to działa: kiedy wywołujesz metodę z `object.something()`, Rust automatycznie dodaje odpowiednie `&`, `&mut` lub `*`, tak aby `object` pasował do sygnatury metody. Innymi słowy, poniższe są równoważne:
>
> ```rust
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
>
> Pierwszy zapis wygląda o wiele czytelniej. To automatyczne referencjonowanie działa, ponieważ metody mają jednoznacznego odbiorcę – typ `self`. Znając odbiorcę i nazwę metody, Rust może jednoznacznie ustalić, czy metoda odczytuje (`&self`), modyfikuje (`&mut self`), czy konsumuje (`self`). Fakt, że Rust sprawia, iż pożyczanie jest domyślne dla odbiorcy metody, stanowi ważny element ergonomii zarządzania własnością w praktyce.