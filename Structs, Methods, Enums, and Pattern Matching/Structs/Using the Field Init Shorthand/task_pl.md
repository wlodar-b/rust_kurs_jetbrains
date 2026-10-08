### Używanie skrótu inicjalizacji pól, gdy zmienne i pola mają te same nazwy

Ponieważ nazwy parametrów i nazwy pól struktury są dokładnie takie same w powyższym kodzie, możemy użyć składni *skrótu inicjalizacji pól* (ang. field init shorthand), aby przepisać funkcję `build_user` w taki sposób, że działa dokładnie tak samo, ale nie wymaga powtarzania `email` i `username`, jak pokazano poniżej.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

#### Funkcja `build_user` używająca skrótu inicjalizacji pól, ponieważ parametry `email` i `username` mają te same nazwy co pola struktury

Tutaj tworzymy nową instancję struktury `User`, która ma pole o nazwie `email`. Chcemy ustawić wartość pola `email` na wartość parametru `email` funkcji `build_user`. Ponieważ pole `email` i parametr `email` mają tę samą nazwę, musimy napisać tylko `email`, zamiast `email: email`.