### Myślenie w kategoriach czasów życia

Sposób, w jaki musisz określić parametry związane z czasami życia, zależy od tego, co robi Twoja funkcja. Na przykład, jeśli zmienimy implementację funkcji `longest`, aby zawsze zwracała pierwszy parametr zamiast najdłuższego wycinka tekstu, nie będziemy musieli określać czasu życia dla parametru `y`. Poniższy kod skompiluje się:

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

W tym przykładzie określiliśmy parametr czasu życia `'a` dla parametru `x` oraz dla typu zwracanego, ale nie dla parametru `y`, ponieważ czas życia `y` nie ma żadnego związku z czasem życia `x` ani ze zwracaną wartością.

Podczas zwracania referencji z funkcji parametr czasu życia dla typu zwracanego musi odpowiadać parametrowi czasu życia jednego z parametrów. Jeśli zwracana referencja *nie* odnosi się do jednego z parametrów, musi odnosić się do wartości utworzonej wewnątrz tej funkcji, co oznaczałoby wiszącą referencję, ponieważ wartość ta przestanie istnieć wraz z zakończeniem funkcji. Rozważmy tę próbę implementacji funkcji `longest`, która się nie skompiluje:

```rust,ignore,does_not_compile
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

Tutaj, mimo że określiliśmy parametr czasu życia `'a` dla typu zwracanego, implementacja ta nie skompiluje się, ponieważ czas życia zwracanej wartości nie ma żadnego związku z czasem życia parametrów. Oto komunikat o błędzie, który otrzymujemy:

```console
error[E0515]: cannot return value referencing local variable `result`
  --> src/main.rs:11:5
   |
11 |     result.as_str()
   |     ------^^^^^^^^^
   |     |
   |     returns a value referencing data owned by the current function
   |     `result` is borrowed here
```

Problem polega na tym, że `result` wychodzi spoza zakresu (scope) i zostaje usunięty po zakończeniu funkcji `longest`. Próbowaliśmy również zwrócić referencję do `result` z funkcji. Nie ma możliwości, abyśmy mogli określić parametry czasu życia, które zmieniłyby tę wiszącą referencję, a Rust nie pozwoli nam stworzyć wiszącej referencji. W tym przypadku najlepszym rozwiązaniem byłoby zwrócenie typu danych będącego własnością funkcji zamiast referencji, tak aby funkcja wywołująca była odpowiedzialna za usunięcie tej wartości.

Ostatecznie, składnia czasów życia ma na celu powiązanie czasów życia różnych parametrów i wartości zwracanych funkcji. Gdy zostaną one powiązane, Rust posiada wystarczające informacje, aby umożliwić bezpieczne operacje na pamięci i zablokować operacje, które mogłyby stworzyć wiszące wskaźniki lub w inny sposób naruszyć bezpieczeństwo pamięci.