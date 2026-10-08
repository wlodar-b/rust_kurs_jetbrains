### Dodawanie użyteczności dzięki dziedziczeniu cech

Byłoby miło móc wydrukować instancję `Rectangle` (prostokąt) podczas 
debugowania programu i zobaczyć wartości wszystkich jego pól. Kod poniżej próbuje
używać makra `println!`, jak to robiliśmy w poprzednich rozdziałach. Jednak to nie zadziała.

<span class="filename">Plik: src/main.rs</span>

```rust,ignore,does_not_compile
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);
}
```

#### Próba wydrukowania instancji `Rectangle`

Podczas kompilacji tego kodu otrzymujemy błąd z następującym kluczowym komunikatem:

```text
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```

Makro `println!` może obsługiwać wiele rodzajów formatowania, a domyślnie nawiasy klamrowe 
mówią `println!`, aby użyć formatowania znanego jako `Display`: wyjścia przeznaczonego 
dla bezpośredniego użytku przez użytkownika końcowego. Widziane do tej pory typy prymitywne 
domyślnie implementują `Display`, ponieważ istnieje tylko jeden sposób, w jaki chcielibyśmy 
pokazać użytkownikowi `1` lub inny typ prymitywny. Jednak w przypadku struktur sposób, w jaki 
`println!` powinien sformatować wyjście, nie jest tak oczywisty, ponieważ istnieje wiele 
możliwości wyświetlania: Czy chcesz używać przecinków czy nie? Czy nawiasy klamrowe mają być 
wydrukowane? Czy wszystkie pola mają być pokazane? Ze względu na tę niejednoznaczność Rust nie 
próbuje zgadywać, czego chcemy, i struktury nie mają dostarczonej implementacji `Display`.

Jeśli przeczytamy dalszy ciąg błędów, znajdziemy tę przydatną notatkę:

```text
   = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

Spróbujmy! Wywołanie makra `println!` będzie teraz wyglądać tak: `println!("rect1 is {:?}", rect1);`. 
Wstawienie specyfikatora `:?` wewnątrz nawiasów klamrowych mówi `println!`, że chcemy użyć 
formatowania wyjścia o nazwie `Debug`. Cechę `Debug` można wykorzystać do wydrukowania naszej 
struktury w sposób użyteczny dla programistów, aby zobaczyć jej wartość podczas debugowania kodu.

Skompiluj kod z tą zmianą. No nie! Nadal otrzymujemy błąd:

```text
error[E0277]: `Rectangle` doesn't implement `Debug`
```

Jednak ponownie kompilator daje nam pomocną notatkę:

```text
    = help: the trait `Debug` is not implemented for `Rectangle`
    = note: add `#[derive(Debug)]` or manually implement `Debug`
```

Rust *rzeczywiście* zawiera funkcjonalność do drukowania informacji do debugowania, ale musimy 
jawnie z niej skorzystać, aby była dostępna dla naszej struktury. Aby to zrobić, dodajemy 
adnotację `#[derive(Debug)]` tuż przed definicją struktury, jak pokazano poniżej.

<span class="filename">Plik: src/main.rs</span>

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
```

#### Dodanie adnotacji, aby uzyskać cechę `Debug`, i drukowanie instancji `Rectangle` z użyciem formatowania debugowania

Teraz, gdy uruchomimy program, nie otrzymamy żadnych błędów, a zobaczymy następujące wyjście:

```console
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/structs`
rect1 is Rectangle { width: 30, height: 50 }
```

Świetnie! To może nie być najładniejsze wyjście, ale pokazuje wartości wszystkich pól w tej instancji, co z pewnością pomaga podczas debugowania. Gdy mamy większe struktury, warto mieć wyjście, które jest nieco łatwiejsze do odczytania; w takich przypadkach możemy użyć `{:#?}` zamiast `{:?}` w ciągu formatowania dla `println!`. Gdy użyjemy stylu `{:#?}` w naszym przykładzie, wyjście będzie wyglądać tak:

```console
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/structs`
rect1 is Rectangle {
    width: 30,
    height: 50,
}
```

Rust udostępnia szereg cech, z których możemy skorzystać za pomocą adnotacji `derive`, aby dodać użyteczne zachowanie do naszych typów własnych. Te cechy i ich zachowania zostały wymienione w [Aneksie C][app3] Książki o Ruście. Omówimy, jak implementować te cechy z własnym zachowaniem, a także jak tworzyć własne cechy, w rozdziale "Typy generyczne, cechy i czasy życia".

Nasza funkcja `area` (pole powierzchni) jest bardzo szczegółowa: oblicza jedynie pole prostokątów. Warto jednak powiązać to zachowanie bliżej z naszą strukturą `Rectangle`, ponieważ nie działa ona w przypadku innych typów. Zobaczmy, jak możemy dalej refaktoryzować ten kod, przekształcając funkcję `area` w *metodę* zdefiniowaną dla naszego typu `Rectangle`.

[app3]: https://github.com/rust-lang/book/blob/master/src/appendix-03-derivable-traits.md