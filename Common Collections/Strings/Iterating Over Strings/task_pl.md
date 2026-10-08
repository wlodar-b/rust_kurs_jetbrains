### Metody iteracji po stringach

Na szczęście, możesz uzyskać dostęp do elementów stringa na inne sposoby.

Jeśli potrzebujesz wykonywać operacje na pojedynczych wartościach Unicode scalar, najlepszym sposobem jest użycie metody `chars`. Wywołanie `chars` na „नमस्ते” oddziela i zwraca sześć wartości typu `char`, a następnie możesz iterować po wyniku, aby uzyskać dostęp do każdego elementu:

```rust
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
```

Ten kod wyświetli następujące rezultaty:

```text
    न
    म
    स
    ्
    त
    े
```

Metoda `bytes` zwraca każdy surowy bajt, co może być odpowiednie w Twoim przypadku:

```rust
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
```

Ten kod wyświetli 18 bajtów, które składają się na ten `String`:

```text
    224
    164
    // --snip--
    165
    135
```

Jednak pamiętaj, że prawidłowe wartości Unicode scalar mogą składać się z więcej niż jednego bajtu.

Uzyskanie klastrów graphemów ze stringów jest skomplikowane, dlatego ta funkcjonalność nie jest dostarczana przez bibliotekę standardową. Dostępne są jednak crate’y na [crates.io](https://crates.io), jeśli potrzebujesz takiej funkcjonalności.