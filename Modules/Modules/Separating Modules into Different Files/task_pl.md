## Rozdzielanie modułów na różne pliki

Do tej pory, wszystkie przykłady w tym rozdziale definiowały wiele modułów w jednym pliku. Gdy moduły stają się duże, można przenieść ich definicje do osobnych plików, aby ułatwić nawigację w kodzie.

Na przykład, zacznijmy od kodu z jednego z poprzednich zadań i przenieśmy moduł `front_of_house` do jego własnego pliku _src/front_of_house.rs_, zmieniając plik główny crate tak, aby zawierał poniższy kod. W tym przypadku plikiem głównym crate jest _src/lib.rs_, ale ta procedura działa również w crate'ach binarnych, gdzie plikiem głównym crate jest _src/main.rs_.

```rust
    mod front_of_house;

    pub use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
```

##### Deklarowanie modułu front_of_house, którego zawartość znajdzie się w _src/front_of_house.rs_

Plik _src/front_of_house.rs_ przejmuje definicje z zawartości modułu `front_of_house`, jak pokazano poniżej.

<span class="filename">Nazwa pliku: src/front_of_house.rs</span>

```rust
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
```

##### Definicje wewnątrz modułu front_of_house w _src/front_of_house.rs_

Zastosowanie średnika po `mod front_of_house` zamiast użycia bloku informuje Rust, że zawartość modułu ma zostać załadowana z innego pliku o tej samej nazwie co moduł. Aby kontynuować nasz przykład i wyodrębnić również moduł `hosting` do jego własnego pliku, zmieniamy _src/front_of_house.rs_, aby zawierał jedynie deklarację modułu `hosting`:

```rust
    pub mod hosting;
```

Następnie tworzymy katalog _src/front_of_house_ i plik _src/front_of_house/hosting.rs_, który zawiera definicje zapisane w module `hosting`:

```rust
    pub fn add_to_waitlist() {}
```

Drzewo modułów pozostaje takie samo, a wywołania funkcji w `eat_at_restaurant` będą działały bez żadnych modyfikacji, nawet jeśli definicje znajdują się w różnych plikach. Ta technika pozwala przenosić moduły do nowych plików, gdy ich rozmiar rośnie.

Należy zauważyć, że instrukcja `pub use crate::front_of_house::hosting` w _src/lib.rs_ również nie została zmieniona, a `use` nie wpływa na to, które pliki są kompilowane jako część crate. Słowo kluczowe `mod` deklaruje moduły, a Rust szuka w pliku o tej samej nazwie co moduł kodu, który należy do tego modułu.