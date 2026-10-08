### Adnotacje dotyczące czasu życia w sygnaturach funkcji

Przyjrzyjmy się teraz adnotacjom dotyczącym czasu życia (ang. lifetime annotations) w kontekście funkcji `longest`. Podobnie jak w przypadku parametryzacji generycznymi typami, musimy zadeklarować generyczne parametry czasu życia w nawiasach kątowych, umieszczonych pomiędzy nazwą funkcji a listą parametrów. Ograniczenie, które chcemy wyrazić w tej sygnaturze, polega na tym, że wszystkie referencje w parametrach i w wartości zwracanej muszą mieć ten sam czas życia. Nazwiemy ten czas życia `'a`, a następnie dodamy go do każdej referencji, jak pokazano we fragmencie kodu poniżej.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

#### Definicja funkcji `longest` określająca, że wszystkie referencje w sygnaturze muszą mieć ten sam czas życia `'a`

Ten kod powinien się skompilować i zwrócić oczekiwany wynik, gdy użyjemy go w funkcji `main` w listingu, gdzie wywołujemy funkcję `longest`, aby znaleźć dłuższy z dwóch wycinków ciągów znaków (string slices).

Sygnatura funkcji teraz mówi kompilatorowi Rust, że dla pewnego czasu życia `'a`, funkcja przyjmuje dwa parametry, z których oba są wycinkami ciągów znaków, które żyją co najmniej tak długo, jak czas życia `'a`. Sygnatura funkcji również informuje Rust, że wycinek ciągu znaków zwrócony z funkcji będzie żył co najmniej tak długo, jak czas życia `'a`. W praktyce oznacza to, że czas życia referencji zwróconej przez funkcję `longest` jest taki sam, jak krótszy z czasów życia referencji przekazanych do tej funkcji. Takie ograniczenia są tym, co chcemy, aby Rust egzekwował. Pamiętajmy, że gdy określamy parametry czasu życia w sygnaturze funkcji, nie zmieniamy czasu życia żadnych wartości przekazanych lub zwróconych. Zamiast tego wskazujemy kompilatorowi Rust, że powinien odrzucać wartości, które nie spełniają tych ograniczeń. Warto zauważyć, że funkcja `longest` nie musi wiedzieć dokładnie, jak długo `x` i `y` będą żyły, wystarczy, że zostanie podstawiony jakiś zakres odpowiadający `'a`, który spełnia tę sygnaturę.

Podczas adnotacji dotyczących czasu życia w funkcjach, adnotacje te umieszczamy w sygnaturze funkcji, a nie w ciele funkcji. Rust jest w stanie przeanalizować kod wewnątrz funkcji bez żadnej dodatkowej pomocy. Natomiast gdy funkcja ma referencje do kodu spoza funkcji lub z innych części kodu, staje się prawie niemożliwe dla Rusta samodzielne określenie czasu życia parametrów lub wartości zwracanych. Czasy życia mogą się różnić przy każdym wywołaniu funkcji. Dlatego musimy ręcznie dodać te adnotacje.

Gdy przekazujemy konkretne referencje do funkcji `longest`, konkretny czas życia, który jest podstawiany dla `'a`, to część zakresu czasu życia `x`, która pokrywa się z zakresem czasu życia `y`. Innymi słowy, generyczny czas życia `'a` otrzyma konkretny czas życia równy krótszemu z czasów życia `x` i `y`. Ponieważ oznaczyliśmy zwracaną referencję tym samym parametrem czasu życia `'a`, zwracana referencja również będzie ważna przez długość krótszego z czasów życia `x` i `y`.

Przyjrzyjmy się, w jaki sposób adnotacje dotyczące czasu życia ograniczają funkcję `longest`, przekazując referencje o różnych konkretnych czasach życia. Kod poniżej jest prostym przykładem.

```rust
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```

#### Użycie funkcji `longest` z referencjami do wartości typu `String` mających różne konkretne czasy życia

W tym przykładzie `string1` jest ważny do końca zewnętrznego zakresu, `string2` jest ważny do końca wewnętrznego zakresu, a `result` referencjuje coś, co jest ważne do końca wewnętrznego zakresu. Uruchom ten kod, a zobaczysz, że kompilator Rust akceptuje ten kod; zostanie on skompilowany i wyświetli `The longest string is long string is long`.