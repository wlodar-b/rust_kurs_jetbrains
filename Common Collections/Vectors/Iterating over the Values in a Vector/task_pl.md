### Iteracja po wartościach w wektorze

Jeśli chcemy uzyskać dostęp do każdego elementu w wektorze po kolei, możemy iterować przez wszystkie elementy, zamiast korzystać z indeksów, aby uzyskiwać dostęp do jednego na raz. Kod poniżej pokazuje, jak użyć pętli `for`, aby uzyskać niemutowalne referencje do każdego elementu w wektorze wartości typu `i32` i je wydrukować.

```rust
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
```

#### Drukowanie każdego elementu w wektorze przez iterację po elementach za pomocą pętli for

Możemy również iterować po mutowalnych referencjach do każdego elementu w mutowalnym wektorze, aby wprowadzać zmiany we wszystkich elementach. Pętla `for` w poniższym przykładzie doda `50` do każdego elementu.

```rust
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
```

#### Iteracja po mutowalnych referencjach do elementów w wektorze

Aby zmienić wartość, do której odnosi się mutowalna referencja, musimy użyć operatora dereferencji (`*`), aby uzyskać dostęp do wartości w `i`, zanim będziemy mogli użyć operatora `+=`. Możesz przeczytać więcej na temat operatora dereferencji w rozdziale „Podążanie za wskaźnikiem do wartości za pomocą operatora dereferencji” w rozdziale 15 [The Rust Programming Language][book].

[book]: https://doc.rust-lang.org/stable/book/ch15-02-deref.html?highlight=dereference#following-the-pointer-to-the-value