### Mieszanie typów generycznych

Parametry typów generycznych w definicji struktury nie zawsze są takie same jak te, które są używane w sygnaturach metod tej struktury. Na przykład, w poniższym przykładzie zdefiniowano metodę `mixup` dla struktury `Point<T, U>` przedstawionej wcześniej w tej sekcji. Metoda przyjmuje inny `Point` jako parametr, który może mieć inne typy niż `Point` wywołujący metodę `mixup`. Metoda tworzy nową instancję `Point` z wartością `x` z `Point` wywołującego metodę `self` (o typie `T`) oraz wartością `y` z przekazanego `Point` (o typie `W`).

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

#### Metoda wykorzystująca różne typy generyczne niż te zdefiniowane w strukturze

W funkcji `main` zdefiniowaliśmy `Point`, który ma `i32` jako `x` (o wartości `5`) oraz `f64` jako `y` (o wartości `10.4`). Zmienna `p2` to struktura `Point`, która ma ciąg znaków (string slice) jako `x` (o wartości `"Hello"`) oraz `char` jako `y` (o wartości `c`). Wywołanie metody `mixup` na `p1` z argumentem `p2` tworzy `p3`, który będzie miał `i32` dla `x`, ponieważ `x` pochodzi z `p1`. Zmienna `p3` będzie miała `char` dla `y`, ponieważ `y` pochodzi z `p2`. Wywołanie makra `println!` wyświetli `p3.x = 5, p3.y = c`.

Celem tego przykładu jest pokazanie sytuacji, w której niektóre parametry generyczne są zadeklarowane w `impl`, a inne w definicji metody. Tutaj parametry generyczne `T` i `U` są zadeklarowane po `impl`, ponieważ są związane z definicją struktury. Parametry generyczne `V` i `W` są zadeklarowane po `fn mixup`, ponieważ są istotne tylko w kontekście tej metody.