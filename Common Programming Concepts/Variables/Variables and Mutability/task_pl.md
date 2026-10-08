## Zmienne i mutowalność

Porozmawiajmy o prostych zmiennych.

```rust
let x = 5;
```

To jest polecenie `let`, które służy do tworzenia *zmiennej*. Oto kolejny przykład:

```rust
let foo = bar;
```

Ta linia tworzy nową zmienną o nazwie `foo` i przypisuje jej wartość zmiennej `bar`.

W Rust zmienne domyślnie są niemutowalne. Jest to jedno z wielu podejść, dzięki którym Rust zachęca do pisania kodu wykorzystującego bezpieczeństwo i łatwą współbieżność, jakie oferuje. Jednak nadal masz możliwość tworzenia zmiennych mutowalnych. Zbadamy, w jaki sposób i dlaczego Rust zachęca do preferowania niemutowalności oraz dlaczego czasami możesz chcieć z tego zrezygnować.

Jeśli zmienna jest niemutowalna, to raz przypisana do niej wartość nie może zostać zmieniona. Spójrz na plik main.rs, którego kod nie skompiluje się w obecnym stanie:

```rust
fn main() {
       let x = 5;
       println!("Wartość x wynosi: {}", x);
       x = 6;
       println!("Wartość x wynosi: {}", x);
   }
```

Uruchom program, klikając przycisk **Run**.

Powinieneś zobaczyć komunikat o błędzie, jak w poniższym wyniku:

```text
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         pierwsze przypisanie do `x`
  |         pomoc: uczynij to wiązanie mutowalnym: `mut x`
3 |     println!("Wartość x wynosi: {}", x);
4 |     x = 6;
  |     ^^^^^ nie można przypisać drugi raz do niemutowalnej zmiennej
```

Ten przykład pokazuje, jak kompilator pomaga znaleźć błędy w programie. Chociaż błędy kompilatora mogą być frustrujące, oznaczają jedynie, że Twój program jeszcze nie wykonuje się bezpiecznie zgodnie z zamierzeniami; nie oznacza to, że jesteś złym programistą! Nawet doświadczeni Rustaceans otrzymują błędy kompilatora.

Komunikat o błędzie wskazuje, że przyczyną jest to, że `cannot assign twice to immutable variable x`, ponieważ próbowałeś przypisać drugą wartość do niemutowalnej zmiennej `x`.

Ważne jest, aby otrzymywać błędy w czasie kompilacji, gdy próbujesz zmienić wartość wcześniej uznaną za niemutowalną, ponieważ taka sytuacja może prowadzić do błędów. Jeśli jedna część kodu opiera się na założeniu, że wartość nigdy się nie zmieni, a inna część kodu zmienia tę wartość, możliwe, że pierwsza część kodu nie będzie działała zgodnie z założeniami. Przyczyna tego rodzaju błędów może być trudna do zlokalizowania, zwłaszcza gdy druga część kodu zmienia wartość tylko czasami.

W Rust kompilator gwarantuje, że jeśli zadeklarujesz, że wartość się nie zmieni, to rzeczywiście tak się stanie. Oznacza to, że podczas czytania i pisania kodu nie musisz śledzić, gdzie i jak dana wartość może się zmienić. Dzięki temu Twój kod jest łatwiejszy do zrozumienia.

Jednak mutowalność może być bardzo użyteczna. Zmienne są niemutowalne tylko domyślnie; możesz uczynić je mutowalnymi, dodając `mut` przed nazwą zmiennej. Oprócz umożliwienia zmiany wartości `mut` wyraża zamiar wobec przyszłych czytelników kodu, wskazując, że inne części kodu będą zmieniać tę zmienną.

Na przykład zmieńmy _src/main.rs_ na poniższy kod:

```rust

fn main() {
    let mut x = 5;
    println!("Wartość x wynosi: {}", x);
    x = 6;
    println!("Wartość x wynosi: {}", x);
}
```

Po uruchomieniu programu teraz otrzymamy taki wynik:

```text
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/variables`
Wartość x wynosi: 5
Wartość x wynosi: 6
```

Teraz możemy zmienić wartość przypisaną do `x` z `5` na `6`, gdy użyto `mut`. W niektórych przypadkach warto uczynić zmienną mutowalną, ponieważ ułatwia to pisanie kodu w porównaniu z użyciem wyłącznie niemutowalnych zmiennych.

Istnieje wiele kompromisów do rozważenia, poza zapobieganiem błędom. Na przykład w sytuacjach, gdy używasz dużych struktur danych, mutowanie instancji na miejscu może być szybsze niż kopiowanie i zwracanie nowo przydzielonych instancji. W przypadku mniejszych struktur danych tworzenie nowych instancji i pisanie w bardziej funkcjonalnym stylu programowania może być łatwiejsze do zrozumienia, więc niższa wydajność może być akceptowalną ceną za uzyskanie tej jasności.

_Możesz zapoznać się z poniższym rozdziałem w książce o języku Rust: [Variables and Mutability](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html)_

Przejdźmy teraz do zadania praktycznego.