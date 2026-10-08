## Literały łańcuchów znaków to wycinki

Przypomnij sobie, że mówiliśmy o tym, że literały łańcuchów znaków są przechowywane wewnątrz pliku binarnego. Teraz, gdy wiemy o wycinkach, możemy właściwie zrozumieć literały łańcuchów znaków:

```rust
    let s = "Hello, world!";
```

Typ `s` tutaj to `&str`: jest to wycinek wskazujący na konkretny punkt w pliku binarnym. To także powód, dla którego literały łańcuchów znaków są niemutowalne; `&str` jest niemutowalnym odwołaniem.