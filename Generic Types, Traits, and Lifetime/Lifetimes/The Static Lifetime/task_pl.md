### Statyczny czas życia

Jednym z wyjątkowych czasów życia, który musimy omówić, jest `'static`, co oznacza, że ta referencja *może* istnieć przez cały czas trwania programu. Wszystkie literały łańcuchów znaków mają czas życia `'static`, który możemy oznaczyć w następujący sposób:

```rust
let s: &'static str = "Mam statyczny czas życia.";
```

Treść tego łańcucha znaków jest przechowywana bezpośrednio w kodzie binarnym programu, który zawsze jest dostępny. Dlatego czas życia wszystkich literałów łańcuchów znaków to `'static`.

Możesz zauważyć sugestie użycia czasu życia `'static` w komunikatach o błędach. Jednak zanim określisz `'static` jako czas życia referencji, zastanów się, czy referencja, którą masz, rzeczywiście istnieje przez cały czas trwania programu, czy nie. Możesz także rozważyć, czy rzeczywiście chcesz, aby była tak długo żywa, nawet jeśli mogłaby być. W większości przypadków problem wynika z próby utworzenia wiszącej referencji lub niedopasowania dostępnych czasów życia. W takich sytuacjach rozwiązaniem jest naprawienie tych problemów, a nie określenie czasu życia `'static`.