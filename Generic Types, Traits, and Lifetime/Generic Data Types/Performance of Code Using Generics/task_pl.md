### Wydajność kodu wykorzystującego generyki

Możesz się zastanawiać, czy użycie parametrów generycznych wiąże się z dodatkowym kosztem w czasie wykonywania programu. Dobra wiadomość jest taka, że Rust implementuje generyki w taki sposób, że Twój kod nie działa wolniej, korzystając z typów generycznych, niż gdyby używał typów konkretnych.

Rust osiąga to dzięki procesowi monomorfizacji kodu korzystającego z generyków na etapie kompilacji. *Monomorfizacja* to proces przekształcania kodu generycznego w kod konkretny poprzez wypełnienie go konkretnymi typami używanymi podczas kompilacji.

W tym procesie kompilator wykonuje kroki odwrotne do tych, które zastosowaliśmy do utworzenia funkcji generycznej w trzecim fragmencie kodu w tej sekcji: kompilator sprawdza wszystkie miejsca, gdzie kod generyczny jest wywoływany, i generuje kod dla konkretnych typów, z którymi kod generyczny jest używany.

Spójrzmy, jak to działa na przykładzie wykorzystującym enum `Option<T>` z biblioteki standardowej:

```rust
let integer = Some(5);
let float = Some(5.0);
```

Kiedy Rust kompiluje ten kod, przeprowadza monomorfizację. Podczas tego procesu kompilator analizuje wartości użyte w instancjach `Option<T>` i identyfikuje dwa rodzaje `Option<T>`: jednym z nich jest `i32`, a drugim `f64`. W związku z tym rozszerza generyczną definicję `Option<T>` do `Option_i32` i `Option_f64`, zastępując definicję generyczną konkretnymi.

Zmonomorfizowana wersja kodu wygląda następująco. Generyczne `Option<T>` zostaje zastąpione przez konkretne definicje stworzone przez kompilator:

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

Ponieważ Rust kompiluje kod generyczny do kodu, który w każdej instancji określa typ, nie ponosimy żadnych dodatkowych kosztów w czasie wykonywania programu za korzystanie z generyków. Gdy kod jest uruchamiany, działa równie szybko, jak gdybyśmy ręcznie duplikowali każdą definicję. Proces monomorfizacji sprawia, że generyki w Rust są wyjątkowo wydajne podczas wykonywania programu.