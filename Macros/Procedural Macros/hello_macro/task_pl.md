### Krok 1: Tworzenie biblioteki typu crate

Pierwszym krokiem jest utworzenie nowej biblioteki typu crate. Tutaj już to zrobiliśmy,  
ale jeśli chcesz spróbować samodzielnie, możesz to zrobić w następujący sposób:

```text
$ cargo new hello_macro --lib
```

Następnie zdefiniujemy cechę (trait) `HelloMacro` i jej powiązaną funkcję w pliku `lib.rs`:

```rust
pub trait HelloMacro {
    fn hello_macro();
}
```

Mamy cechę i jej funkcję. Na tym etapie użytkownik naszego crate’a mógłby zaimplementować tę cechę w pliku `main.rs`, aby uzyskać oczekiwaną funkcjonalność, na przykład w ten sposób:

```rust
use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
```

Jednak musiałby napisać blok implementacji dla każdego typu, z którym chciałby używać `hello_macro`; chcemy oszczędzić im tego wysiłku.

Dodatkowo nie możemy jeszcze zapewnić funkcji `hello_macro` domyślnej implementacji, która wypisywałaby nazwę typu, na którym cecha jest zaimplementowana: Rust nie posiada możliwości refleksji, więc nie może odczytać nazwy typu w czasie wykonania. Potrzebujemy makra, które wygeneruje kod w czasie kompilacji.