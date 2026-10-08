### Użycie backtrace dla `panic!`

Przyjrzyjmy się kolejnemu przykładowi, aby zobaczyć, jak wygląda sytuacja, gdy wywołanie `panic!` pochodzi z biblioteki z powodu błędu w naszym kodzie, zamiast z naszego kodu wywołującego makro bezpośrednio. Poniższy fragment kodu zawiera przykład próby uzyskania dostępu do elementu w wektorze za pomocą indeksu.

```rust
    fn main() {
        let v = vec![1, 2, 3];

        v[99];
    }
```

##### Próba uzyskania dostępu do elementu poza końcem wektora, co spowoduje wywołanie `panic!`

Tutaj próbujemy uzyskać dostęp do setnego elementu naszego wektora (który ma indeks 99, ponieważ indeksowanie zaczyna się od zera), ale wektor zawiera tylko 3 elementy. W tej sytuacji Rust zgłosi panikę. Użycie `[]` ma zwrócić element, ale jeśli podasz nieprawidłowy indeks, nie ma elementu, który Rust mógłby tutaj zwrócić.

Inne języki, takie jak C, w takiej sytuacji próbują zwrócić dokładnie to, o co prosisz, nawet jeśli nie jest to to, czego chcesz: otrzymasz jakąkolwiek zawartość pamięci znajdującej się w miejscu, które odpowiadałoby temu elementowi wektora, nawet jeśli ta pamięć nie należy do wektora. Nazywa się to _przekroczeniem bufora_ (ang. _buffer overread_) i może prowadzić do luk w zabezpieczeniach, jeśli atakujący jest w stanie manipulować indeksem w taki sposób, aby odczytać dane, do których nie powinien mieć dostępu, a które znajdują się za tablicą.

Aby chronić program przed tego rodzaju podatnościami, jeśli spróbujesz odczytać element pod nieistniejącym indeksem, Rust zatrzyma wykonanie i odmówi jego kontynuacji. Spróbujmy to zobaczyć w praktyce:

```text
   Compiling test_rust_project v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/test_rust_project`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', /rustc/3c235d5600393dfe6c36eeed34042efad8d4f26e/src/libcore/slice/mod.rs:2686:10
```
Ten błąd wskazuje na plik, którego nie napisaliśmy, _libcore/slice/mod.rs_. Jest to implementacja `slice` w kodzie źródłowym Rusta. Kod, który wykonuje się, gdy używamy `[]` na naszym wektorze `v`, znajduje się w _libcore/slice/mod.rs_, i to tam faktycznie dochodzi do wywołania `panic!`.

Kolejna linia wskazuje, że możemy ustawić zmienną środowiskową `RUST_BACKTRACE`, aby uzyskać backtrace z dokładnymi informacjami o tym, co spowodowało błąd. _Backtrace_ to lista wszystkich funkcji, które zostały wywołane, aby dojść do punktu, w którym wystąpił błąd. Backtrace w Rust działa jak w innych językach: klucz do jego czytania polega na tym, by zacząć od góry i czytać, aż zobaczysz pliki, które sam napisałeś. To jest miejsce, gdzie problem się zaczyna. Linie powyżej linii odnoszących się do twoich plików przedstawiają kod, który twój kod wywołał; linie poniżej pokazują kod, który wywołał twój kod. Te linie mogą obejmować kod rdzenia Rusta, biblioteki standardowej lub używane przez ciebie pakiety (crates). Spróbujmy uzyskać backtrace, ustawiając zmienną środowiskową `RUST_BACKTRACE` na dowolną wartość różną od 0. Poniższy fragment kodu pokazuje wyjście podobne do tego, które zobaczysz.

```console
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:483
   1: core::panicking::panic_fmt
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:85
   2: core::panicking::panic_bounds_check
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:62
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:255
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:15
   5: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/alloc/src/vec.rs:1982
   6: panic::main
             at ./src/main.rs:4
   7: core::ops::function::FnOnce::call_once
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/ops/function.rs:227
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

##### Backtrace wygenerowany przez wywołanie `panic!`, wyświetlany, gdy zmienna środowiskowa `RUST_BACKTRACE` jest ustawiona

To dużo wyjścia! Dokładne wyjście, jakie zobaczysz, może się różnić w zależności od systemu operacyjnego i wersji Rust. Aby uzyskać backtrace z tymi informacjami, konieczne jest włączenie symboli debugowania. Symbole debugowania są domyślnie włączone podczas używania `cargo build` lub `cargo run` bez flagi `--release`, jak w naszym przypadku.

W wyjściu w przedstawionym fragmencie kodu linia 6 backtrace'u wskazuje linię w naszym projekcie, która powoduje problem: linia 4 pliku _src/main.rs_. Jeśli nie chcemy, aby nasz program zgłaszał panikę, miejsce wskazane przez pierwszą linię odnoszącą się do pliku, który sami napisaliśmy, jest miejscem, gdzie powinniśmy rozpocząć dochodzenie. W pierwszym fragmencie kodu, w którym celowo napisaliśmy kod powodujący panikę w celu demonstracji użycia backtrace'ów, rozwiązaniem problemu jest nieżądanie elementu o indeksie 99 z wektora, który zawiera tylko 3 elementy. Gdy twój kod w przyszłości zgłosi panikę, będziesz musiał ustalić, jakie działanie z jakimi wartościami spowodowało panikę i co kod powinien robić zamiast tego.

Powrócimy do `panic!` oraz tego, kiedy powinniśmy, a kiedy nie powinniśmy używać `panic!` do obsługi warunków błędu, w sekcji [„To `panic!` or Not to `panic!`”](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#to-panic-or-not-to-panic) później w tym rozdziale. Następnie przyjrzymy się, jak radzić sobie z błędami za pomocą `Result`.

_Możesz odwołać się do następującego rozdziału z książki Rust Programming Language:
[Unrecoverable Errors with panic!](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unrecoverable-errors-with-panic)_