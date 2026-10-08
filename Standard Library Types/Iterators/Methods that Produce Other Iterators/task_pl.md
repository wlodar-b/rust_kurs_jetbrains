### Metody tworzące inne iteratory

Inne metody zdefiniowane w trait `Iterator`, znane jako _adaptory iteratorów_, pozwalają na przekształcenie iteratorów w różne ich rodzaje. Możesz łączyć wiele wywołań adaptorów iteratorów, aby wykonywać skomplikowane operacje w czytelny sposób. Jednak ponieważ wszystkie iteratory są leniwe, musisz wywołać jedną z metod konsumujących, aby uzyskać wyniki z wywołań adaptorów iteratorów.

Poniższy fragment kodu pokazuje przykład wywołania metody adaptera iteratora `map`, która przyjmuje klamrę (closure) do wywołania na każdym elemencie w celu utworzenia nowego iteratora. Klamra w tym przypadku tworzy nowy iterator, w którym każdy element wektora został zwiększony o 1. Jednak ten kod generuje ostrzeżenie:

```rust
    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter().map(|x| x + 1);
```

##### Wywołanie adaptera iteratora map w celu utworzenia nowego iteratora

Ostrzeżenie, które otrzymujemy, wygląda następująco:

```text
warning: unused `Map` that must be used
 --> src/main.rs:4:5
  |
4 |     v1.iter().map(|x| x + 1);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_must_use)]` domyślnie włączone
  = note: iteratory są leniwe i nie robią nic, dopóki nie zostaną skonsumowane
```

Kod w ostatnim przykładzie niczego nie robi; klamra, którą określiliśmy, nigdy nie jest wywoływana. Ostrzeżenie przypomina nam dlaczego: adaptory iteratorów są leniwe i musimy tutaj skonsumować iterator.

Aby temu zaradzić i skonsumować iterator, użyjemy metody `collect`, która jest omówiona w **[Rozdziale 12](https://doc.rust-lang.org/stable/book/ch12-01-accepting-command-line-arguments.html)** książki o Rust, wraz z `env::args`. Metoda ta konsumuje iterator i zbiera wynikowe wartości w strukturę danych kolekcji.

W poniższym przykładzie zbieramy wyniki iteracji przez iterator zwrócony z wywołania `map` do wektora (vector). Ten wektor będzie zawierał każdy element z oryginalnego wektora, zwiększony o 1.

```rust
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
```

##### Wywołanie metody map w celu utworzenia nowego iteratora, a następnie wywołanie metody collect w celu skonsumowania nowego iteratora i stworzenia wektora

Ponieważ `map` przyjmuje klamrę, możemy określić dowolną operację, którą chcemy wykonać na każdym elemencie. To świetny przykład na to, jak klamry pozwalają dostosować pewne zachowania przy jednoczesnym ponownym użyciu zachowania iteracji dostarczanego przez trait `Iterator`.