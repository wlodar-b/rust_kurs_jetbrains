## Krytyczne błędy z `panic!`

Czasami w Twoim kodzie zdarzają się poważne problemy i nie ma sposobu, aby sobie z nimi poradzić. W takich przypadkach Rust udostępnia makro `panic!`. Gdy makro `panic!` zostanie wywołane, Twój program wypisze komunikat o błędzie, cofnie stos oraz posprząta dane, a następnie zakończy działanie. Najczęściej ma to miejsce wtedy, gdy wykryto błąd, którego programista nie wie, jak obsłużyć.

> ### Cofanie stosu lub przerwanie w odpowiedzi na `panic`
>
> Domyślnie, gdy nastąpi `panic`, program zaczyna proces _cofania stosu_, co oznacza, że Rust przechodzi wstecz przez stos i sprząta dane po każdej napotkanej funkcji. Jednak taki proces porządkowania wymaga sporo pracy. Alternatywą jest natychmiastowe _przerwanie_ działania programu, co kończy jego działanie bez sprzątania. Pamięć używana przez program musi wtedy zostać posprzątana przez system operacyjny. Jeśli w Twoim projekcie ważne jest, aby wynikowy plik binarny był jak najmniejszy, możesz zamienić cofanie stosu na przerwanie w przypadku wystąpienia `panic`, dodając `panic = 'abort'` do odpowiednich sekcji `[profile]` w pliku _Cargo.toml_. Na przykład, jeśli chcesz, aby w trybie release wystąpiło przerwanie zamiast cofania stosu, dodaj:

>     [profile.release]
>     panic = 'abort'

Spróbujmy wywołać `panic!` w prostym programie:

```rust
    fn main() {
        panic!("crash and burn");
    }
```

Po uruchomieniu programu zobaczysz coś takiego:

```text
  Compiling test_rust_project v0.1.0
      Finished dev [unoptimized + debuginfo] target(s) in 0.42s
       Running `target/debug/test_rust_project`
  thread 'main' panicked at 'crash and burn', src/main.rs:2:5
```

Wywołanie `panic!` powoduje powstanie komunikatu błędu widocznego w dwóch ostatnich liniach. Pierwsza linia zawiera naszą wiadomość `panic` oraz miejsce w kodzie źródłowym, gdzie wystąpiło `panic`: _src/main.rs:2:5_ wskazuje, że jest to druga linia, piąty znak w pliku _src/main.rs_.

W tym przypadku wskazana linia znajduje się w naszym kodzie, a po przejściu do niej widzimy wywołanie makra `panic!`. W innych przypadkach wywołanie `panic!` może znajdować się w kodzie, który nasz kod wywołuje, a nazwa pliku oraz numer linii podane w komunikacie o błędzie będą dotyczyć kodu innego autora, gdzie makro `panic!` zostało wywołane – nie naszego kodu, który ostatecznie doprowadził do wywołania `panic!`. Możemy użyć śladu stosu (backtrace) funkcji, z których pochodzi wywołanie `panic!`, aby ustalić, która część naszego kodu powoduje problem. Szczegółowo omówimy, czym jest ślad stosu, w kolejnych rozdziałach.