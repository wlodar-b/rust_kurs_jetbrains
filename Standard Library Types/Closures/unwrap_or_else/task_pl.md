#### Przykład: `unwrap_or_else`

Przyjrzyjmy się definicji metody `unwrap_or_else` na `Option<T>`, której użyliśmy [wcześniej](course://Standard Library Types/Closures/Capturing the Environment with Closures):

```rust
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

Przypomnijmy, że `T` to typ generyczny reprezentujący typ wartości w wariancie `Some` z `Option`. Ten typ `T` jest także typem zwracanym funkcji `unwrap_or_else`: kod, który wywołuje `unwrap_or_else` na `Option<String>`, na przykład, otrzyma `String`.

Następnie zauważmy, że funkcja `unwrap_or_else` ma dodatkowy parametr typu generycznego, `F`. Typ `F` to typ parametru nazwanego `f`, który jest domknięciem przekazywanym podczas wywoływania `unwrap_or_else`.

Ograniczenie nałożone na typ generyczny `F` to `FnOnce() -> T`, co oznacza, że `F` musi mieć możliwość bycia wywołanym przynajmniej raz, nie przyjmować żadnych argumentów i zwracać `T`. Użycie `FnOnce` w ograniczeniu wyraża warunek, że `unwrap_or_else` wywoła `f` co najwyżej raz. W ciele `unwrap_or_else` widzimy, że jeśli `Option` to `Some`, `f` nie zostanie wywołane. Jeśli `Option` to `None`, `f` zostanie wywołane raz. Ponieważ wszystkie domknięcia implementują `FnOnce`, `unwrap_or_else` akceptuje najróżniejsze rodzaje domknięć i jest maksymalnie elastyczne.

> Uwaga: Funkcje również mogą implementować wszystkie trzy cechy `Fn`. Jeśli to, co chcemy osiągnąć, nie wymaga przechwytywania wartości ze środowiska, możemy użyć nazwy funkcji zamiast domknięcia w miejscach, gdzie potrzebujemy czegoś, co implementuje jedną z cech `Fn`. Na przykład, na wartości typu `Option<Vec<T>>`, możemy wywołać `unwrap_or_else(Vec::new)`, aby otrzymać nowy, pusty wektor, jeśli wartość jest `None`.