### Ogólne parametry życia w funkcjach

Napiszmy funkcję, która zwraca dłuższy z dwóch wycinków tekstowych (ang. string slices). 
Ta funkcja przyjmie dwa wycinki tekstowe i zwróci jeden z nich. Po zaimplementowaniu funkcji `longest`, poniższy kod powinien wyświetlić `The longest string is abcd`.

```rust,ignore
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

#### Funkcja `main`, która wywołuje funkcję `longest`, aby znaleźć dłuższy z dwóch wycinków tekstowych

Zwróć uwagę, że chcemy, aby funkcja przyjmowała wycinki tekstowe, które są referencjami, ponieważ nie chcemy, aby funkcja `longest` przejmowała własność swoich parametrów. Odwołaj się do rozdziału "Wycinki tekstowe jako parametry" w "Zrozumienie własności" (ang. "Understanding Ownership") po więcej szczegółów na temat tego, dlaczego parametry użyte w powyższym fragmencie kodu są tymi, na których nam zależy.

Jeżeli spróbujemy zaimplementować funkcję `longest` tak, jak to opisano poniżej, kod nie będzie się kompilował.

```rust,ignore,does_not_compile
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

#### Implementacja funkcji `longest`, która zwraca dłuższy z dwóch wycinków tekstowych, ale nie kompiluje się

Zamiast tego otrzymamy następujący błąd dotyczący czasu życia zmiennych:

```console
error[E0106]: missing lifetime specifier
 --> src/main.rs:9:33
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ^^^^    ^^^^^^^     ^^^^^^^     ^^^
```

Tekst pomocy wskazuje, że typ zwracany przez funkcję potrzebuje ogólnego parametru życia (ang. generic lifetime parameter), ponieważ Rust nie może określić, czy zwracana referencja odnosi się do `x`, czy do `y`. W rzeczywistości my również tego nie wiemy, ponieważ blok `if` w ciele tej funkcji zwraca referencję do `x`, a blok `else` zwraca referencję do `y`!

Definiując tę funkcję, nie znamy konkretnych wartości, które zostaną przekazane do tej funkcji, więc nie możemy wiedzieć, czy wykonany zostanie przypadek `if` czy `else`. Nie znamy również konkretnych długości życia referencji, które zostaną przekazane, więc nie możemy spojrzeć na zakresy, tak jak to zrobiliśmy w drugim i trzecim fragmencie kodu w tej sekcji, aby określić, czy zwracana referencja zawsze będzie ważna. Kontroler pożyczek (ang. borrow checker) również tego nie potrafi, ponieważ nie wie, jak długości życia `x` i `y` mają się do długości życia wartości zwracanej. Aby rozwiązać ten błąd, dodamy ogólne parametry życia, które zdefiniują związek między referencjami, aby kontroler pożyczek mógł przeprowadzić swoją analizę.