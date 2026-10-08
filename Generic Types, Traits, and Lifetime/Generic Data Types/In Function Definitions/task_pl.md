### W definicjach funkcji

Podczas definiowania funkcji używającej typów generycznych umieszczamy te typy w podpisie funkcji, tam gdzie normalnie określamy typy danych parametrów i wartości zwracanej. Dzięki temu nasz kod staje się bardziej elastyczny i oferuje większe możliwości użytkownikom naszej funkcji, jednocześnie zapobiegając duplikacji kodu.

Kontynuując pracę z funkcją `largest`, poniżej widzimy dwie funkcje, które obie znajdują największą wartość w przekroju (slice).

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("Największa liczba to {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("Największy znak to {}", result);
}
```

#### Dwie funkcje różniące się tylko nazwami i typami w podpisach

Funkcja `largest_i32` to ta, którą wyodrębniliśmy w ostatnim fragmencie kodu poprzedniej sekcji, znajdująca największe `i32` w przekroju. Funkcja `largest_char` znajduje największy `char` w przekroju. Ciała obu funkcji zawierają identyczny kod, więc wyeliminujemy tę duplikację, wprowadzając parametr typu generycznego w jednej funkcji.

Aby sparametryzować typy w nowej funkcji, którą zdefiniujemy, musimy nadać parametrów typom, tak jak nadajemy nazwę parametrom wartości w funkcji. Możemy użyć dowolnego identyfikatora jako nazwy parametru typu. My jednak użyjemy `T`, ponieważ zgodnie z konwencją w języku Rust, nazwy parametrów są krótkie, często składają się tylko z jednej litery, a konwencja nazw typów w języku Rust to CamelCase. Skrót od „type” – „typ” – `T` jest domyślnym wyborem większości programistów Rust.

Gdy używamy parametru w ciele funkcji, musimy zadeklarować nazwę parametru w podpisie funkcji, tak aby kompilator wiedział, co oznacza dana nazwa. Podobnie, gdy używamy nazwy parametru typu w podpisie funkcji, musimy zadeklarować nazwę parametru typu przed jego użyciem. Aby zdefiniować generyczną funkcję `largest`, umieszczamy deklarację nazwy typu wewnątrz nawiasów ostrokątnych `<>`, pomiędzy nazwą funkcji a listą parametrów, jak poniżej:

```rust,ignore
fn largest<T>(list: &[T]) -> &T {
```

Ten zapis odczytujemy jako: funkcja `largest` jest generyczna dla jakiegoś typu `T`. Ta funkcja ma jeden parametr nazwany `list`, który jest przekrojem wartości typu `T`. Funkcja `largest` zwraca referencję do wartości tego samego typu `T`.

Poniższy fragment kodu przedstawia zdefiniowaną funkcję `largest`, która używa typu generycznego w swoim podpisie. Fragment pokazuje również, jak możemy wywołać funkcję zarówno dla przekroju wartości typu `i32`, jak i dla wartości typu `char`. Zauważ jednak, że ten kod jeszcze nie będzie się kompilował. Naprawimy to później w tym rozdziale.

```rust,ignore,does_not_compile
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("Największa liczba to {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("Największy znak to {}", result);
}
```

#### Definicja funkcji `largest`, która używa generycznych parametrów typu, ale jeszcze się nie kompiluje

Jeśli spróbujemy skompilować ten kod, otrzymamy następujący błąd:

```console
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  |             ^^^^^^^^^^^^^^^^^^^^^^
```

Informacja o błędzie wspomina o `std::cmp::PartialOrd`, który jest *cechą* (trait). Omówimy cechy w następnej sekcji. Na razie ten błąd oznacza, że ciało funkcji `largest` nie działa dla wszystkich możliwych typów, które `T` może reprezentować. Ponieważ chcemy porównywać wartości typu `T` w ciele funkcji, możemy używać tylko typów, których wartości mogą być porównywane. Aby umożliwić porównania, biblioteka standardowa posiada cechę `std::cmp::PartialOrd`, którą można zaimplementować dla typów (zobacz Dodatek C, aby dowiedzieć się więcej o tej cesze). Nauczysz się, jak określić, że typ generyczny posiada konkretne cechy, w części „Cechy jako parametry” w kolejnej lekcji, ale najpierw przyjrzyjmy się innym sposobom korzystania z generycznych parametrów typu.