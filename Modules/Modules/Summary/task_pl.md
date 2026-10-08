## Podsumowanie

Rust pozwala podzielić pakiet na wiele crate'ów, a crate na moduły, dzięki czemu można odwoływać się do elementów zdefiniowanych w jednym module z innego modułu. Można to zrobić, określając ścieżki absolutne lub względne. Te ścieżki można wprowadzić w zakres przy użyciu instrukcji `use`, co pozwala na korzystanie z krótszej ścieżki przy wielokrotnym użyciu elementu w tym zakresie. Kod modułu jest domyślnie prywatny, ale można uczynić definicje publicznymi, dodając słowo kluczowe `pub`.

_Można odnieść się do następującego rozdziału w książce "The Rust Programming Language": [Rozdzielanie modułów na różne pliki](https://doc.rust-lang.org/stable/book/ch07-05-separating-modules-into-different-files.html#separating-modules-into-different-files)_