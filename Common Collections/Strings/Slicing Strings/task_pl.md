### Przetwarzanie fragmentów ciągów tekstowych

Indeksowanie ciągów tekstowych często nie jest dobrym pomysłem, ponieważ nie jest jasne, jaki powinien być typ zwracany przez operację indeksowania ciągu: wartość bajtowa, znak, klaster znaków (grapheme cluster) czy fragment ciągu (string slice). Dlatego Rust wymaga, abyś był bardziej precyzyjny, jeśli rzeczywiście musisz używać indeksów do tworzenia fragmentów ciągów. Aby być bardziej precyzyjnym podczas indeksowania i wskazać, że chcesz utworzyć fragment ciągu, a nie używać samego indeksowania za pomocą `[]` z pojedynczą liczbą, możesz użyć `[]` z zakresem, aby stworzyć fragment zawierający określone bajty:

```rust
    let hello = "Здравствуйте";

    let s = &hello[0..4];
```

Tutaj `s` będzie `&str`, który zawiera pierwsze 4 bajty ciągu. Wcześniej wspomnieliśmy, że każdy z tych znaków zajmuje 2 bajty, co oznacza, że `s` będzie `Зд`.

Co by się stało, gdybyśmy użyli `&hello[0..1]`? Odpowiedź: Rust spowodowałby panic w czasie wykonania w ten sam sposób, jak przy próbie dostępu do nieprawidłowego indeksu w wektorze:

    thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`', src/libcore/str/mod.rs:2188:4

Powinieneś ostrożnie używać zakresów do tworzenia fragmentów ciągów, ponieważ może to spowodować awarię programu.