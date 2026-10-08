## Typ tablica

Innym sposobem przechowywania kolekcji wielu wartości jest _tablica_. W przeciwieństwie do krotek, każdy element tablicy musi mieć ten sam typ. Tablice w Rust różnią się od tablic w niektórych innych językach, ponieważ mają stałą długość, podobnie jak krotki.

W Rust wartości umieszczane w tablicy zapisywane są jako lista oddzielona przecinkami w nawiasach kwadratowych:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

Tablice są przydatne, gdy chcemy, aby dane były alokowane na stosie zamiast na stercie (omówimy stos i stertę bardziej szczegółowo w Rozdziale 4) lub gdy chcemy upewnić się, że zawsze mamy stałą liczbę elementów. Tablica nie jest jednak tak elastyczna jak typ wektor. Wektor to podobny typ kolekcji dostarczany przez bibliotekę standardową, który może dynamicznie zmieniać swój rozmiar. Jeśli nie jesteś pewien, czy użyć tablicy, czy wektora, prawdopodobnie powinieneś wybrać wektor. Wektory zostały omówione bardziej szczegółowo w Rozdziale 8.

Przykładem sytuacji, w której można użyć tablicy zamiast wektora, jest program, który musi znać nazwy miesięcy w roku. Jest mało prawdopodobne, aby taki program musiał dodawać lub usuwać miesiące, więc możesz użyć tablicy, ponieważ wiesz, że zawsze będzie zawierała dokładnie 12 elementów:

```rust
let months = ["Styczeń", "Luty", "Marzec", "Kwiecień", "Maj", "Czerwiec", "Lipiec",
              "Sierpień", "Wrzesień", "Październik", "Listopad", "Grudzień"];
```

Typ tablicy zapisujemy, używając nawiasów kwadratowych, w których określamy typ każdego elementu, średnik, a następnie liczbę elementów w tablicy, tak jak poniżej:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Tutaj `i32` to typ każdego elementu. Po średniku liczba `5` wskazuje, że tablica zawiera pięć elementów.

Zapisanie typu tablicy w ten sposób jest podobne do alternatywnej składni inicjalizowania tablicy: jeśli chcesz utworzyć tablicę, której każdy element ma tę samą wartość, możesz podać wartość początkową, po której następuje średnik i długość tablicy w nawiasach kwadratowych, jak pokazano tutaj:

```rust
let a = [3; 5];
```

Tablica `a` będzie zawierała `5` elementów, z których wszystkie początkowo będą miały wartość `3`. To jest to samo, co zapisanie `let a = [3, 3, 3, 3, 3];`, ale w bardziej zwięzły sposób.

#### Dostęp do elementów tablicy

Tablica to spójny blok pamięci alokowany na stosie. Możesz uzyskać dostęp do elementów tablicy za pomocą indeksowania, tak jak w tym przykładzie:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

W tym przykładzie zmienna `first` przyjmie wartość `1`, ponieważ jest to wartość pod indeksem `[0]` w tablicy. Zmienna `second` przyjmie wartość `2`, znajdującą się pod indeksem `[1]` w tablicy.

#### Nieprawidłowy dostęp do elementu tablicy

Co się stanie, jeśli spróbujesz uzyskać dostęp do elementu tablicy poza jej końcem? Załóżmy, że zmienisz przykład na następujący kod, który skompiluje się, ale zakończy się błędem podczas uruchamiania:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("Wartość elementu to: {}", element);
}
```

Uruchomienie tego kodu za pomocą cargo run spowoduje następujący wynik:

```text
   Finished dev [unoptimized + debuginfo] target(s) in 0.05s
   Running `target/debug/Test_Rust_Project`
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:5:19
```

Kompilacja nie wywołała żadnych błędów, ale program zakończył się _błędem podczas wykonywania_ i nie działał poprawnie. Kiedy próbujesz uzyskać dostęp do elementu za pomocą indeksowania, Rust sprawdza, czy podany indeks jest mniejszy niż długość tablicy. Jeśli indeks jest większy lub równy długości tablicy, Rust uruchomi panikę.

To jest pierwszy przykład działania zasad bezpieczeństwa Rust. W wielu językach niskiego poziomu tego rodzaju sprawdzenie nie jest wykonywane, a gdy podasz nieprawidłowy indeks, możesz uzyskać dostęp do nieprawidłowej pamięci. Rust chroni cię przed tego rodzaju błędem, natychmiast kończąc działanie zamiast kontynuowania z błędnym dostępem do pamięci. Więcej na temat obsługi błędów w Rust znajdziesz w Rozdziale 9.

_Możesz zapoznać się z następującym rozdziałem z książki o języku programowania Rust: [Typy złożone](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#compound-types)_