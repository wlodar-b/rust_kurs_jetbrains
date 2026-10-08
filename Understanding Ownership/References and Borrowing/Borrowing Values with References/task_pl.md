## Pożyczanie wartości za pomocą referencji

Problem z kodem krotki w poprzednim zadaniu polegał na tym, że musieliśmy zwrócić `String` do wywołującej funkcji, aby można go było nadal używać po wywołaniu `calculate_length`, ponieważ `String` został przeniesiony do `calculate_length`.

Oto jak można zdefiniować i użyć funkcji `calculate_length`, która jako parametr przyjmuje referencję do obiektu zamiast przejmowania własności wartości:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("Długość '{}' wynosi {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

Po pierwsze, zauważ, że cały kod związany z krotką w deklaracji zmiennej i zwracanej wartości funkcji został usunięty. Po drugie, zauważ, że przekazujemy `&s1` do `calculate_length`, a w jej definicji przyjmujemy `&String` zamiast `String`.

Te znaki ampersand to _referencje_. Pozwalają one odwoływać się do pewnej wartości bez przejmowania nad nią własności. Rysunek 5 przedstawia diagram.

<img alt="Referencja &amp;String s wskazuje na String s1" src="https://doc.rust-lang.org/stable/book/img/trpl04-05.svg" class="center">

##### Rysunek 5: Diagram referencji &String s wskazującej na String s1

> Uwaga: Przeciwieństwem odwoływania się za pomocą `&` jest _odwoływanie przez dereferencję_, które realizuje operator `*`. Zobaczymy zastosowania operatora dereferencji w rozdziale 8 i omówimy szczegóły dereferencji w rozdziale 15.

Przyjrzyjmy się dokładniej wywołaniu funkcji:

```rust
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
```

Składnia `&s1` pozwala nam utworzyć referencję, która _odnosi się_ do wartości `s1`, ale jej nie przejmuje. Ponieważ nie przejmuje własności, wartość, na którą wskazuje, nie zostanie usunięta, gdy referencja wyjdzie poza zakres.

Podobnie, sygnatura funkcji używa `&`, aby wskazać, że typ parametru `s` to referencja. Dodajmy kilka wyjaśniających adnotacji:

```rust
fn calculate_length(s: &String) -> usize { // s jest referencją do String
    s.len()
} // Tutaj, s wychodzi poza zakres. Ale ponieważ nie przejmuje własności
  // tego, na co wskazuje, nic się nie dzieje.
```

Zakres, w którym zmienna `s` jest ważna, jest taki sam jak zakres każdego parametru funkcji, ale nie usuwamy tego, na co wskazuje referencja, gdy wychodzi ona poza zakres, ponieważ nie posiadamy własności. Gdy funkcje mają referencje jako parametry zamiast rzeczywistych wartości, nie musimy zwracać wartości, aby przekazać z powrotem własność, ponieważ nigdy jej nie posiadaliśmy.

Określamy sytuację, gdy funkcje mają referencje jako parametry, jako _pożyczanie_. Jak w prawdziwym życiu, jeśli ktoś coś posiada, możesz to od niego pożyczyć. Kiedy skończysz, musisz to zwrócić.