## Wprowadzanie ścieżek do zakresu za pomocą słowa kluczowego use

Może się wydawać, że ścieżki, które dotychczas pisaliśmy, aby wywołać funkcje, są niewygodnie długie i powtarzalne. Na przykład, w jednym z przykładów z rozdziału "Ścieżki do odnoszenia się do elementów w drzewie modułów", niezależnie od tego, czy wybraliśmy ścieżkę absolutną, czy względną do funkcji `add_to_waitlist`, za każdym razem, gdy chcieliśmy wywołać `add_to_waitlist`, musieliśmy również określić `front_of_house` i `hosting`. Na szczęście jest sposób, aby uprościć ten proces. Możemy wprowadzić ścieżkę do zakresu raz, a następnie odwoływać się do elementów w tej ścieżce, jakby były lokalnymi elementami, używając słowa kluczowego `use`.

W poniższym przykładzie wprowadzamy moduł `crate::front_of_house::hosting` do zakresu funkcji `eat_at_restaurant`, dzięki czemu musimy tylko określić `hosting::add_to_waitlist`, aby wywołać funkcję `add_to_waitlist` w `eat_at_restaurant`.

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
```

##### Wprowadzanie modułu do zakresu za pomocą use

Dodanie `use` i ścieżki do zakresu jest podobne do tworzenia linku symbolicznego w systemie plików. Dodając `use crate::front_of_house::hosting` w korzeniu paczki (crate root), `hosting` staje się teraz poprawną nazwą w tym zakresie, podobnie jak gdyby moduł `hosting` był zdefiniowany w korzeniu paczki. Ścieżki wprowadzane do zakresu za pomocą `use` również podlegają zasadom prywatności, tak samo jak inne ścieżki.

Możesz również wprowadzić element do zakresu za pomocą `use` i ścieżki względnej. Poniższy przykład pokazuje, jak określić ścieżkę względną, aby osiągnąć ten sam efekt, co w powyższym kodzie.

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    use self::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
```

##### Wprowadzanie modułu do zakresu za pomocą use i ścieżki względnej zaczynającej się od self

Pamiętaj, że używanie `self` w ten sposób może nie być konieczne w przyszłości; jest to niekonsekwencja w języku, którą deweloperzy języka Rust starają się wyeliminować.

### Tworzenie idiomatycznych ścieżek use

W pierwszym fragmencie kodu mogłeś się zastanawiać, dlaczego określiliśmy `use crate::front_of_house::hosting`, a następnie wywołaliśmy `hosting::add_to_waitlist` w `eat_at_restaurant`, zamiast określenia ścieżki `use` wychodzącej wprost do funkcji `add_to_waitlist`, aby osiągnąć ten sam rezultat, jak w poniższym fragmencie.

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    use crate::front_of_house::hosting::add_to_waitlist;

    pub fn eat_at_restaurant() {
        add_to_waitlist();
        add_to_waitlist();
        add_to_waitlist();
    }
```

##### Wprowadzanie funkcji add_to_waitlist do zakresu za pomocą use, co jest nieidiomatyczne

Chociaż oba fragmenty realizują to samo zadanie, pierwszy sposób jest idiomatycznym sposobem wprowadzania funkcji do zakresu za pomocą `use`. Wprowadzenie modułu nadrzędnego funkcji do zakresu za pomocą `use`, tak abyśmy musieli określać moduł nadrzędny podczas wywoływania funkcji, jasno wskazuje, że funkcja nie jest zdefiniowana lokalnie, jednocześnie minimalizując powtarzanie pełnej ścieżki. Kod w ostatnim fragmencie jest niejasny w kwestii tego, gdzie `add_to_waitlist` jest zdefiniowana.

Z drugiej strony, podczas wprowadzania struktur, wyliczeń i innych elementów za pomocą `use`, idiomatyczne jest określenie pełnej ścieżki. Poniższy przykład pokazuje idiomatyczny sposób wprowadzania struktury `HashMap` ze standardowej biblioteki do zakresu programu.

```rust
    use std::collections::HashMap;

    fn main() {
        let mut map = HashMap::new();
        map.insert(1, 2);
    }
```

##### Idiomatyczne wprowadzanie HashMap do zakresu

Nie istnieje żaden silny powód stojący za tą konwencją: jest to po prostu przyzwyczajenie, które wykształciło się wraz z czytaniem i pisaniem kodu w Rust.

Wyjątkiem od tej konwencji jest sytuacja, gdy wprowadzamy dwie rzeczy o tej samej nazwie do zakresu za pomocą wyrażeń `use`, ponieważ Rust na to nie pozwala. W przykładzie poniżej pokazano, jak wprowadzać dwa typy `Result` o tej samej nazwie, ale różnych modułach nadrzędnych, oraz jak się do nich odnosić.

```rust
    use std::fmt;
    use std::io;

    fn function1() -> fmt::Result {
    }

    fn function2() -> io::Result<()> {
    }
```

##### Wprowadzanie dwóch typów o tej samej nazwie do tego samego zakresu wymaga użycia ich modułów nadrzędnych

Jak widać, użycie modułów nadrzędnych rozróżnia dwa typy `Result`. Gdybyśmy zamiast tego określili `use std::fmt::Result` i `use std::io::Result`, mielibyśmy dwa typy `Result` w tym samym zakresie, a Rust nie wiedziałby, o który typ chodzi, gdy używalibyśmy `Result`. Spróbuj i zobacz, jaki błąd zwróci kompilator!