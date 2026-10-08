## Definiowanie i tworzenie struktur

Struktury są podobne do krotek, które omawialiśmy w rozdziale „Podstawowe zagadnienia programowania”. Podobnie jak w przypadku krotek, elementy struktury mogą być różnych typów. W przeciwieństwie do krotek, każdy element danych w strukturze ma nazwę, co pozwala jednoznacznie zrozumieć, co oznaczają wartości. Dzięki nadaniu tych nazw struktury są bardziej elastyczne niż krotki: nie trzeba polegać na kolejności danych, aby określić lub uzyskać dostęp do wartości instancji.

Aby zdefiniować strukturę, używamy słowa kluczowego `struct` i nadajemy nazwę całej strukturze. Nazwa struktury powinna opisywać znaczenie grupowanych elementów danych. Następnie, w nawiasach klamrowych, definiujemy nazwy i typy elementów danych, które nazywamy *polami*. Na przykład poniższy listing przedstawia strukturę przechowującą informacje o koncie użytkownika.

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

#### Definicja struktury `User`

Aby użyć struktury po jej zdefiniowaniu, tworzymy *instancję* tej struktury, przypisując konkretne wartości do każdego z pól. Instancję tworzymy, podając nazwę struktury, a następnie nawiasy klamrowe zawierające pary `klucz: wartość`, gdzie klucze to nazwy pól, a wartości to dane, które chcemy w tych polach przechowywać. Nie musimy podawać pól w tej samej kolejności, w jakiej zostały zadeklarowane w definicji struktury. Innymi słowy, definicja struktury jest ogólnym szablonem dla danego typu, a instancje wypełniają ten szablon określonymi danymi, tworząc wartości tego typu. Przykładowo, możemy zadeklarować konkretnego użytkownika, jak pokazano poniżej.

```rust
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
```

#### Tworzenie instancji struktury `User`

Aby uzyskać konkretną wartość z struktury, możemy użyć notacji kropkowej. Jeżeli chcemy pobrać tylko adres e-mail danego użytkownika, możemy użyć `user1.email` w dowolnym miejscu, w którym potrzebujemy tej wartości. Jeżeli instancja jest zmienna, możemy zmienić wartość, używając notacji kropkowej i przypisując wartość do określonego pola. Kod poniżej pokazuje, jak zmienić wartość w polu `email` zmiennej instancji `User`.

```rust
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
```

#### Zmiana wartości w polu `email` instancji `User`

Zauważ, że cała instancja musi być zmienna; Rust nie pozwala oznaczyć jako zmienne tylko określonych pól. Jak w przypadku każdej instrukcji, możemy stworzyć nową instancję struktury jako ostatnią wyrażoną wartość w ciele funkcji, aby niejawnie zwrócić tę instancję.

Poniższy kod przedstawia funkcję `build_user`, która zwraca instancję `User` na podstawie podanego adresu e-mail i nazwy użytkownika. Pole `active` przyjmuje wartość `true`, a `sign_in_count` wartość `1`.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

#### Funkcja `build_user`, która przyjmuje e-mail i nazwę użytkownika, zwracając instancję `User`

Sensowne jest, aby nazwy parametrów funkcji odpowiadały nazwom pól w strukturze, jednak ciągłe powtarzanie nazw pól, takich jak `email` czy `username`, może być uciążliwe. Gdyby struktura miała więcej pól, powtarzanie każdej nazwy stałoby się jeszcze bardziej irytujące. Na szczęście istnieje wygodne skrócone rozwiązanie!