## Ciała funkcji zawierają instrukcje i wyrażenia

Ciała funkcji składają się z sekwencji instrukcji, opcjonalnie kończących się wyrażeniem. Jak dotąd omawialiśmy funkcje bez kończących wyrażeń, ale zauważyłeś wyrażenia jako część instrukcji. Ponieważ Rust jest językiem opartym na wyrażeniach, zrozumienie tej różnicy jest kluczowe. Inne języki nie zawsze rozróżniają te pojęcia w ten sposób, dlatego spójrzmy, czym są instrukcje i wyrażenia oraz jak ich różnice wpływają na ciała funkcji.

W rzeczywistości używaliśmy już zarówno instrukcji, jak i wyrażeń. _Instrukcje_ to polecenia, które wykonują jakąś akcję i nie zwracają wartości. _Wyrażenia_ są elementami kodu, które obliczają i zwracają wynikową wartość. Spójrzmy na kilka przykładów.

Tworzenie zmiennej i przypisywanie do niej wartości za pomocą słowa kluczowego `let` jest instrukcją. Na poniższym przykładzie `let y = 6;` to instrukcja.

```rust
fn main() {
    let y = 6;
}
```
##### Przykład deklaracji funkcji main zawierającej jedną instrukcję

Definicje funkcji również są instrukcjami; cały poprzedni przykład jest sam w sobie instrukcją.

Instrukcje nie zwracają wartości. Dlatego nie możesz przypisać instrukcji `let` do innej zmiennej, jak w pokazanym poniżej kodzie; program zwróci błąd:

```rust
fn main() {
    let x = (let y = 6);
}
```

Po uruchomieniu tego programu uzyskasz następujący błąd:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: expected expression, found statement (`let`)
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^
  |
  = note: variable declaration using `let` is a statement
```

Instrukcja `let y = 6` nie zwraca wartości, więc `x` nie może niczego związać. To różni się od innych języków, takich jak C czy Ruby, gdzie przypisanie zwraca wartość przypisywaną. W tych językach możesz napisać `x = y = 6` i zarówno `x`, jak i `y` będą miały wartość `6`; w Rust tak nie jest.

Wyrażenia obliczają wartość i stanowią większość kodu, który będziesz pisać w Rust. Rozważ prostą operację matematyczną, na przykład `5 + 6`, która jest wyrażeniem zwracającym wartość `11`. Wyrażenia mogą być częścią instrukcji: w Przykładzie 3-1, `6` w instrukcji `let y = 6;` jest wyrażeniem, które zwraca wartość `6`. Wywołanie funkcji jest wyrażeniem. Wywołanie makra jest wyrażeniem. Blok, którego używamy do tworzenia nowych zakresów za pomocą `{}`, również jest wyrażeniem, na przykład:

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("Wartość y to: {}", y);
}
```

To wyrażenie:

```rust
{
    let x = 3;
    x + 1
}
```

jest blokiem, który, w tym przypadku, zwraca `4`. Ta wartość jest wiązana do zmiennej `y` jako część instrukcji `let`. Zauważ linię `x + 1` bez średnika na końcu, co jest inne od większości linii, które widziałeś do tej pory. Wyrażenia nie kończą się średnikami. Jeśli dodasz średnik na końcu wyrażenia, przekształcisz je w instrukcję, która wówczas nie zwróci wartości. Zapamiętaj to, gdy będziesz badać wartości zwracane przez funkcje i wyrażenia w kolejnych krokach.

_Możesz zapoznać się z następną sekcją w książce o języku Rust: [Function Bodies Contain Statements and Expressions](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html#function-bodies-contain-statements-and-expressions)._

Przejdźmy teraz do praktyki.