### Re-eksportowanie nazw za pomocą `pub use`

Kiedy wprowadzamy nazwę do zakresu za pomocą słowa kluczowego `use`, nazwa dostępna w nowym zakresie jest prywatna. Aby móc odwoływać się do tej nazwy z innego kodu (jakby była zdefiniowana w zakresie tego kodu), możemy połączyć `pub` i `use`.  
Ta technika nazywa się _re-eksportowaniem_, ponieważ wprowadzamy element do zakresu, ale jednocześnie udostępniamy go innym, aby mogli wprowadzić go do swojego zakresu.

Kolejny przykład pokazuje kod z początku zadania, w którym `use` w module głównym zostało zmienione na `pub use`.

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    pub use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
```

##### Udostępnianie nazwy w nowym zakresie dla dowolnego kodu za pomocą pub use

Dzięki użyciu `pub use` kod zewnętrzny może teraz wywoływać funkcję `add_to_waitlist` przy użyciu `hosting::add_to_waitlist`. Gdybyśmy nie użyli `pub use`, funkcja `eat_at_restaurant` mogłaby wywoływać `hosting::add_to_waitlist` w swoim zakresie, ale kod zewnętrzny nie mógłby skorzystać z tej nowej ścieżki.

Re-eksportowanie jest przydatne, gdy wewnętrzna struktura twojego kodu różni się od sposobu, w jaki programiści używający twojego kodu myślą o danym problemie. Na przykład w tej metaforze restauracji osoby prowadzące restaurację myślą o „części frontowej” i „części kuchennej”. Jednak klienci odwiedzający restaurację prawdopodobnie nie będą postrzegać jej przez pryzmat tych terminów. Za pomocą `pub use` możemy napisać nasz kod z jedną strukturą, ale ujawnić inną strukturę. Dzięki temu nasza biblioteka jest dobrze zorganizowana zarówno dla programistów pracujących nad biblioteką, jak i dla tych, którzy ją wykorzystują.