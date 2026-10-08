### Przykład: `sort_by_key`

Przyjrzyjmy się teraz metodzie `sort_by_key` z biblioteki standardowej, zdefiniowanej dla fragmentów tablic (slices), aby zobaczyć, jak się różni. Przyjmuje ona zamknięcie (closure), które implementuje `FnMut`. To zamknięcie otrzymuje jeden argument - referencję do obecnie rozpatrywanego elementu w tablicy - i zwraca wartość typu `K`, która może być uporządkowana. Ta funkcja jest przydatna, gdy chcemy posortować fragment tablicy według konkretnego atrybutu każdego elementu. W poniższym listingu mamy listę instancji `Rectangle` i używamy `sort_by_key`, aby uporządkować je według atrybutu `width` od najmniejszej do największej wartości:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}
```

##### Przykład użycia `sort_by_key` i zamknięcia do sortowania listy instancji `Rectangle` według wartości atrybutu `width`

Ten kod wypisuje:

```console
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/rectangles`
[
    Rectangle {
        width: 3,
        height: 5,
    },
    Rectangle {
        width: 7,
        height: 12,
    },
    Rectangle {
        width: 10,
        height: 1,
    },
]
```

Powodem, dla którego `sort_by_key` wymaga zamknięcia implementującego `FnMut`, jest to, że wywołuje zamknięcie wielokrotnie: raz dla każdego elementu w tablicy. Zamknięcie `|r| r.width` nie przechwytuje, nie zmienia ani nie przenosi niczego ze swojego otoczenia, więc spełnia wymagania narzucone przez cechy (`trait bound`).

Dla porównania, poniższy listing pokazuje przykład zamknięcia, które implementuje jedynie `FnOnce`, ponieważ przenosi wartość z otoczenia. Kompilator nie pozwoli nam użyć takiego zamknięcia z `sort_by_key`:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut sort_operations = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
    println!("{:#?}", list);
}
```

##### Przykład próby użycia zamknięcia `FnOnce` z `sort_by_key`

Jest to złożony, nienaturalny sposób (który nie działa), aby próbować zliczać liczbę wywołań funkcji `sort_by_key` podczas sortowania listy. Ten kod usiłuje to zrobić, dodając `value`, czyli `String` z otoczenia zamknięcia, do wektora `sort_operations`. Zamknięcie przechwytuje `value`, a następnie przenosi `value` z zamknięcia poprzez przekazanie własności `value` do wektora `sort_operations`. Takie zamknięcie można wywołać tylko raz; próba ponownego wywołania nie zadziała, ponieważ `value` nie będzie już dostępna w otoczeniu, aby można było ją ponownie dodać do `sort_operations`! Dlatego takie zamknięcie implementuje jedynie `FnOnce`. Gdy próbujemy skompilować ten kod, otrzymujemy błąd mówiący, że `value` nie może być przeniesiona z zamknięcia, ponieważ zamknięcie musi implementować `FnMut`:

```console
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
error[E0507]: cannot move out of `value`, a captured variable in an `FnMut` closure
  --> src/main.rs:27:30
   |
24 |       let value = String::from("by key called");
   |           ----- captured outer variable
25 | 
26 |       list.sort_by_key(|r| {
   |  ______________________-
27 | |         sort_operations.push(value);
   | |                              ^^^^^ move occurs because `value` has type `String`, which does not implement the `Copy` trait
28 | |         r.width
29 | |     });
   | |_____- captured by this `FnMut` closure

For more information about this error, try `rustc --explain E0507`.
error: could not compile `rectangles` due to previous error
```

Błąd wskazuje linię w ciele zamknięcia, która przenosi `value` z otoczenia. Aby naprawić ten błąd, musimy zmodyfikować ciało zamknięcia tak, aby nie przenosiło wartości z otoczenia. Jeśli chcemy znać liczbę wywołań `sort_by_key`, prostszym sposobem jest przechowywanie licznika w otoczeniu i zwiększanie jego wartości w ciele zamknięcia. Zamknięcie z poniższego listingu działa z `sort_by_key`, ponieważ przechwytuje jedynie mutowalną referencję do licznika `num_sort_operations` i może być wywoływane wielokrotnie:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}
```

##### Przykład użycia zamknięcia `FnMut` z `sort_by_key`, które jest dozwolone

Cechy (`traits`) `Fn` są istotne podczas definiowania lub używania funkcji czy typów, które korzystają z zamknięć. W następnej sekcji omówione zostaną iteratory, a wiele metod iteratorów przyjmuje zamknięcia jako argumenty. Miej te szczegóły dotyczące zamknięć na uwadze podczas zgłębiania tematu iteratorów!