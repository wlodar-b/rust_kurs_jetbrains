### Tworzenie instancji na podstawie innych instancji z użyciem składni aktualizacji struktury

Często przydatne jest tworzenie nowej instancji struktury, która wykorzystuje większość wartości z istniejącej instancji, ale zmienia niektóre z nich. Możesz to zrobić, używając *składni aktualizacji struktury*.

Na początek, poniższy przykład pokazuje, jak stworzyć nowy obiekt `User` w `user2` bez użycia składni aktualizacji. Ustawiamy nowe wartości dla `email` i `username`, natomiast pozostałe wartości są używane z obiektu `user1`, który utworzyliśmy w drugim fragmencie kodu w tej sekcji.

```rust
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
```

#### Tworzenie nowej instancji `User` z wykorzystaniem niektórych wartości z `user1`

Korzystając ze składni aktualizacji struktury, możemy osiągnąć ten sam efekt przy użyciu mniejszej ilości kodu, jak pokazano poniżej. Składnia `..` wskazuje, że pozostałe pola, które nie zostały jawnie określone, powinny mieć takie same wartości jak pola z przekazanej instancji.

```rust
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
```

#### Użycie składni aktualizacji struktury do ustawienia nowych wartości `email` i `username` dla instancji `User`, przy jednoczesnym zachowaniu pozostałych wartości z pól instancji w zmiennej `user1`

Powyższy kod również tworzy instancję w `user2`, która ma inne wartości dla pól `email` i `username`, ale takie same wartości dla pól `active` i `sign_in_count` co obiekt `user1`.