## Przykład: Praca z prostokątami

Aby zrozumieć, kiedy warto użyć struktur (structs), napiszemy program, który
oblicza pole prostokąta. Zaczniemy od użycia pojedynczych zmiennych, a następnie
zrefaktoryzujemy program, aby ostatecznie korzystał ze struktur.

Utwórzmy nowy projekt binarny w Cargo o nazwie *rectangles*, który przyjmie
szerokość i wysokość prostokąta podane w pikselach i obliczy pole tego
prostokąta. Poniższy listing pokazuje krótki program, który ukazuje jeden ze
sposobów realizacji tego zadania w pliku naszego projektu *src/main.rs*.

<span class="filename">Plik: src/main.rs</span>

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "Pole prostokąta wynosi {} pikseli kwadratowych.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

#### Obliczanie pola prostokąta określonego przez oddzielne zmienne szerokości i wysokości

Teraz uruchom ten program za pomocą `cargo run`:

```console
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/structs`
Pole prostokąta wynosi 1500 pikseli kwadratowych.
```

Chociaż powyższy program działa i oblicza pole prostokąta, wywołując funkcję `area` 
z każdą z jego wymiarów, możemy zrobić to lepiej. Szerokość i wysokość są ze sobą
powiązane, ponieważ razem opisują jeden prostokąt.

Problem z tym kodem jest widoczny w sygnaturze funkcji `area`:

```rust
fn area(width: u32, height: u32) -> u32 {
```

Funkcja `area` powinna obliczać pole jednego prostokąta, ale napisana przez nas funkcja
ma dwa parametry. Parametry są ze sobą powiązane, ale nie jest to wyrażone w żaden sposób
w naszym programie. Lepszym rozwiązaniem, bardziej czytelnym i łatwiejszym do zarządzania,
byłoby połączenie szerokości i wysokości w jedną całość. Omawialiśmy już jeden
ze sposobów, jak można to zrobić, w sekcji „Krotki” lekcji "Podstawowe koncepcje programowania/Typy danych": używając krotek.