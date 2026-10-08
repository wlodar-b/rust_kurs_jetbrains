### Udostępnianie ścieżek za pomocą słowa kluczowego pub

Wróćmy do błędu w poprzednim przykładzie, który wskazuje, że moduł `hosting` jest prywatny. Chcemy, aby funkcja `eat_at_restaurant` w module nadrzędnym miała dostęp do funkcji `add_to_waitlist` w module podrzędnym, więc oznaczamy moduł `hosting` słowem kluczowym `pub`, jak pokazuje poniższy listing.

```rust
    mod front_of_house {
        pub mod hosting {
            fn add_to_waitlist() {}
        }
    }

    pub fn eat_at_restaurant() {
        // Ścieżka absolutna
        crate::front_of_house::hosting::add_to_waitlist();

        // Ścieżka względna
        front_of_house::hosting::add_to_waitlist();
    }
```

##### Deklarowanie modułu hosting jako pub, aby używać go w eat_at_restaurant

Niestety kod z powyższego fragmentu nadal powoduje błąd, jak pokazano poniżej.

```text
Compiling Test_Rust_Project v0.1.0
error[E0603]: function `add_to_waitlist` is private
 --> src/main.rs:9:37
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                                     ^^^^^^^^^^^^^^^

error[E0603]: function `add_to_waitlist` is private
  --> src/main.rs:12:30
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                              ^^^^^^^^^^^^^^^                            ^^^^^^^^^^^^^^^
```

##### Błędy kompilatora z budowania powyższego kodu

Co się stało? Dodanie słowa kluczowego `pub` przed `mod hosting` sprawia, że moduł staje się publiczny. W wyniku tej zmiany, jeśli możemy uzyskać dostęp do `front_of_house`, możemy również uzyskać dostęp do `hosting`. Jednak _zawartość_ modułu `hosting` nadal jest prywatna; oznaczenie modułu jako publicznego nie sprawia, że jego zawartość również staje się publiczna. Słowo kluczowe `pub` przy module pozwala jedynie kodowi w jego modułach przodkach odwoływać się do niego.

Błędy wskazują, że funkcja `add_to_waitlist` jest prywatna. Reguły prywatności dotyczą struktur, enumeracji, funkcji i metod, a także modułów.

Teraz także oznaczmy funkcję `add_to_waitlist` jako publiczną, dodając słowo kluczowe `pub` przed jej definicją, jak pokazano poniżej.

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    pub fn eat_at_restaurant() {
        // Ścieżka absolutna
        crate::front_of_house::hosting::add_to_waitlist();

        // Ścieżka względna
        front_of_house::hosting::add_to_waitlist();
    }
```

##### Dodanie słowa kluczowego pub do mod hosting i fn add_to_waitlist pozwala wywołać funkcję z eat_at_restaurant

Teraz kod skompiluje się! Przyjrzyjmy się ścieżce absolutnej i względnej oraz sprawdźmy, dlaczego dodanie słowa kluczowego `pub` pozwala nam używać tych ścieżek w `add_to_waitlist` w kontekście reguł prywatności.

W ścieżce absolutnej zaczynamy od `crate`, korzenia drzewa modułów naszego crate'a. Następnie moduł `front_of_house` jest zdefiniowany w korzeniu crate'a. Moduł `front_of_house` nie jest publiczny, ale ponieważ funkcja `eat_at_restaurant` jest zdefiniowana w tym samym module co `front_of_house` (to znaczy, `eat_at_restaurant` i `front_of_house` są na tym samym poziomie), możemy odnosić się do `front_of_house` z `eat_at_restaurant`. Następny jest moduł `hosting`, oznaczony jako `pub`. Możemy uzyskać dostęp do modułu nadrzędnego `hosting`, więc możemy również uzyskać dostęp do samego `hosting`. Na końcu funkcja `add_to_waitlist` jest oznaczona `pub`, co pozwala nam uzyskać dostęp do niej przez jej moduł nadrzędny, dzięki czemu wywołanie tej funkcji działa!

W ścieżce względnej logika jest taka sama jak w ścieżce absolutnej, z wyjątkiem pierwszego kroku: zamiast zaczynać od korzenia crate'a, ścieżka zaczyna się od `front_of_house`. Moduł `front_of_house` jest zdefiniowany w tym samym module co `eat_at_restaurant`, więc ścieżka względna zaczynająca się od modułu, w którym jest zdefiniowany `eat_at_restaurant`, działa. Następnie, ponieważ `hosting` i `add_to_waitlist` są oznaczone jako `pub`, reszta ścieżki działa i wywołanie tej funkcji jest ważne!