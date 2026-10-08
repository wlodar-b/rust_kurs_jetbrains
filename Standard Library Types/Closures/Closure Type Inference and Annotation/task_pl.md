### Inference typów i adnotacje w zamknięciach

Istnieje więcej różnic między funkcjami a zamknięciami. Zamknięcia zazwyczaj nie wymagają od Ciebie adnotacji typów parametrów lub wartości zwracanej, jak w przypadku funkcji `fn`. Adnotacje są wymagane w funkcjach, ponieważ stanowią one element jawnego interfejsu udostępnianego użytkownikom. Sztywne określenie tego interfejsu jest ważne, aby zapewnić, że wszyscy zgadzają się co do typów wartości, które funkcja wykorzystuje i zwraca. Natomiast zamknięcia nie są używane w taki sposób – są przechowywane w zmiennych i wykorzystywane bez konieczności ich nazywania czy udostępniania użytkownikom naszej biblioteki.

Zamknięcia są zazwyczaj krótkie i mają zastosowanie wyłącznie w wąskim kontekście, a nie w dowolnym scenariuszu. W tych ograniczonych kontekstach kompilator może wywnioskować typy parametrów i typ zwracanej wartości, podobnie jak w większości przypadków potrafi wywnioskować typy zmiennych (chociaż istnieją rzadkie przypadki, kiedy kompilator również potrzebuje adnotacji typów dla zamknięć).

Podobnie jak w przypadku zmiennych, możemy dodać adnotacje typów, jeśli chcemy zwiększyć jawność i klarowność, kosztem większej szczegółowości, która nie zawsze jest konieczna. Adnotowanie typów dla zamknięcia może wyglądać jak poniższa definicja:

```rust
    let expensive_closure = |num: u32| -> u32 {
        println!("obliczanie powoli...");
        thread::sleep(Duration::from_secs(2));
        num
    };
```

##### Przykład opcjonalnych adnotacji typów parametrów i wartości zwracanej w zamknięciu

Po dodaniu adnotacji typów składnia zamknięć bardziej przypomina składnię funkcji. Poniżej znajduje się porównanie pionowe składni definicji funkcji, która dodaje 1 do swojego parametru, oraz zamknięcia, które działa w ten sam sposób. Dodaliśmy kilka spacji, aby wyrównać odpowiednie części. Ilustruje to, jak składnia zamknięć jest podobna do składni funkcji, z wyjątkiem użycia pionowych kresek (pipes) oraz tego, że część składni jest opcjonalna:

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

Pierwsza linia pokazuje definicję funkcji, natomiast druga linia pokazuje w pełni adnotowaną definicję zamknięcia. Trzecia linia usuwa adnotacje typów z definicji zamknięcia, a czwarta usuwa nawiasy klamrowe, które są opcjonalne, ponieważ ciało zamknięcia zawiera tylko jedno wyrażenie. Wszystkie te definicje są poprawne i będą działały w ten sam sposób przy wywołaniu. Aby `add_one_v3` i `add_one_v4` mogły zostać skompilowane, konieczne jest ich wywołanie, ponieważ typy zostaną wywnioskowane na podstawie ich użycia.

Definicje zamknięć będą miały jeden konkretny typ wywnioskowany dla każdego z parametrów oraz dla wartości zwracanej. Na przykład poniżej pokazano definicję krótkiego zamknięcia, które po prostu zwraca wartość, jaką otrzyma jako parametr. To zamknięcie nie jest zbyt użyteczne, poza celami tego przykładu. Zwróć uwagę, że nie dodano żadnych adnotacji typów w definicji: jeśli następnie spróbujemy wywołać zamknięcie dwukrotnie, raz przekazując `String` jako argument, a drugi raz `u32`, otrzymamy błąd.

```rust
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);
```

##### Przykład próby wywołania zamknięcia, którego typy zostały wywnioskowane, z wykorzystaniem dwóch różnych typów

Kompilator zwróci nam następujący błąd:

```console
$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
error[E0308]: mismatched types
 --> src/main.rs:5:29
  |
5 |     let n = example_closure(5);
  |                             ^- help: spróbuj użyć metody konwersji: `.to_string()`
  |                             |
  |                             oczekiwano struktury `String`, znaleziono liczbę całkowitą

Aby uzyskać więcej informacji o tym błędzie, użyj `rustc --explain E0308`.
error: could not compile `closure-example` due to previous error
```

Za pierwszym razem, gdy wywołujemy `example_closure` z wartością `String`, kompilator wywnioskował typ `x` oraz typ zwracany przez zamknięcie jako `String`. Te typy zostały następnie przypisane do zamknięcia w `example_closure`, więc otrzymujemy błąd typów, jeśli próbujemy użyć innego typu z tym samym zamknięciem.