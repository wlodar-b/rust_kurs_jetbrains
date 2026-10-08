### Metody z większą liczbą parametrów

Przećwiczmy korzystanie z metod, implementując drugą metodę dla struktury `Rectangle`. Tym razem chcemy, aby instancja `Rectangle` przyjmowała inną instancję `Rectangle` i zwracała `true`, jeśli drugi `Rectangle` całkowicie mieści się w `self`; w przeciwnym razie powinna zwrócić `false`. Innymi słowy, chcemy móc napisać program pokazany poniżej po zdefiniowaniu metody `can_hold`.

```rust,ignore
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

#### Używanie jeszcze niegotowej metody `can_hold`

Oczekiwany wynik programu wygląda następująco, ponieważ oba wymiary `rect2` są mniejsze od wymiarów `rect1`, ale `rect3` jest szerszy niż `rect1`:

```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

Wiemy, że chcemy zdefiniować metodę, więc umieścimy ją w bloku `impl Rectangle`. Nazwa metody to `can_hold`, a jej parametrem będzie niemutowalne wypożyczenie innego `Rectangle`. Możemy określić typ parametru, patrząc na kod, który wywołuje tę metodę: `rect1.can_hold(&rect2)` przekazuje `&rect2`, co jest niemutowalnym wypożyczeniem `rect2`, instancji `Rectangle`. To jest logiczne, ponieważ potrzebujemy jedynie odczytać `rect2` (zamiast zapisu, co wymagałoby mutowalnego wypożyczenia), a chcemy, by `main` zachował własność `rect2`, aby móc ponownie go użyć po wywołaniu metody `can_hold`. Wartość zwracana przez `can_hold` będzie wartością logiczną, a implementacja sprawdzi, czy szerokość i wysokość `self` są jednocześnie większe niż szerokość i wysokość drugiego `Rectangle`, odpowiednio. Dodajmy nową metodę `can_hold` do bloku `impl` z pierwszego przykładu w tej sekcji, pokazanym poniżej.

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

#### Implementacja metody `can_hold` w strukturze `Rectangle`, która przyjmuje inną instancję `Rectangle` jako parametr

Po uruchomieniu tego kodu wraz z funkcją `main` z poprzedniego fragmentu kodu, otrzymamy oczekiwany wynik. Metody mogą przyjmować wiele parametrów, które dodajemy do sygnatury za parametrem `self`, a te parametry działają tak samo, jak parametry w funkcjach.