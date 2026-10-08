### Parametry funkcji

Funkcje mogą być również definiowane z _parametrami_, które są specjalnymi zmiennymi będącymi częścią sygnatury funkcji. Kiedy funkcja posiada parametry, możesz przekazać jej konkretne wartości dla tych parametrów. Technicznie rzecz biorąc, te konkretne wartości nazywane są _argumentami_, ale w potocznej rozmowie ludzie zazwyczaj używają słów _parametr_ i _argument_ wymiennie – zarówno w odniesieniu do zmiennych w definicji funkcji, jak i do konkretnych wartości przekazywanych podczas wywołania funkcji.

Poniższa zmodyfikowana wersja `another_function` pokazuje, jak wyglądają parametry w Rust:

```rust
   fn main() {
       another_function(5);
   }
   
   fn another_function(x: i32) {
       println!("Wartość x to: {}", x);
   }
```

Spróbuj uruchomić ten program; powinieneś otrzymać następujący wynik:

```text
   $ cargo run
      Compiling functions v0.1.0 (file:///projects/functions)
       Finished dev [unoptimized + debuginfo] target(s) in 1.21 secs
        Running `target/debug/functions`
   Wartość x to: 5
```

Deklaracja `another_function` zawiera jeden parametr o nazwie `x`. Typ `x` został określony jako `i32`. Kiedy `5` jest przekazywane do `another_function`, makro `println!` umieszcza `5` w miejscu, gdzie w łańcuchu formatowania znajdują się nawiasy klamrowe.

W sygnaturach funkcji _musisz_ zadeklarować typ każdego parametru. Jest to celowe rozwiązanie w konstrukcji Rust: wymaganie anotacji typów w definicjach funkcji oznacza, że kompilator prawie nigdy nie potrzebuje ich używać w pozostałej części kodu, aby zrozumieć, co masz na myśli.

Gdy chcesz, aby funkcja miała wiele parametrów, oddziel deklaracje parametrów przecinkami, tak jak w tym przykładzie:

```rust
   fn main() {
       another_function(5, 6);
   }
   
   fn another_function(x: i32, y: i32) {
       println!("Wartość x to: {}", x);
       println!("Wartość y to: {}", y);
   }
```

Ten przykład tworzy funkcję z dwoma parametrami, z których oba są typu `i32`. Następnie funkcja wypisuje wartości obu swoich parametrów. Zwróć uwagę, że parametry funkcji nie muszą mieć tego samego typu – po prostu w tym przykładzie obydwa są identyczne.

Spróbujmy uruchomić ten kod. Zastąp aktualny program w pliku src/main.rs swojego projektu `functions` powyższym przykładem i uruchom go:

```text
   $ cargo run
      Compiling functions v0.1.0 (file:///projects/functions)
       Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
        Running `target/debug/functions`
   Wartość x to: 5
   Wartość y to: 6
```

Ponieważ wywołaliśmy funkcję, przekazując `5` jako wartość dla `x` i `6` jako wartość dla `y`, dwa ciągi znaków zostały wypisane z tymi wartościami.

_Możesz odwołać się do kolejnego rozdziału w książce The Rust Programming Language: [How Functions Work](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html)_