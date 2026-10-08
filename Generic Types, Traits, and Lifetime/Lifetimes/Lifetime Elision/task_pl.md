### Pomijanie czasu życia

Nauczyłeś(-aś) się, że każde odniesienie (reference) ma czas życia (lifetime) i że musisz określać parametry czasu życia dla funkcji lub struktur, które używają odniesień. Jednakże, w sekcji "Slicing" w części "Zrozumienie własności" w przykładzie "Poprawienie funkcji `first_word` poprzez użycie cięcia tekstu jako typu parametru `s`" mieliśmy funkcję (zobacz poniżej), która kompilowała się bez adnotacji czasu życia.

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

#### Funkcja zdefiniowana w "Slicing", która kompilowała się bez adnotacji czasu życia, mimo że parametr i typ zwracany są odniesieniami

Powodem, dla którego ta funkcja kompiluje się bez adnotacji czasu życia, jest historia: w wczesnych wersjach (przed 1.0) Rust ten kod by się nie skompilował, ponieważ każde odniesienie wymagało jawnego określenia czasu życia. W tamtym czasie sygnatura funkcji wyglądałaby tak:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

Po napisaniu dużej ilości kodu w Rust zespół Rust zauważył, że programiści Rustu wielokrotnie określali te same adnotacje czasu życia w określonych sytuacjach. Te sytuacje były przewidywalne i podążały za kilkoma deterministycznymi wzorcami. Programiści zakodowali te wzorce w kompilatorze, aby mechanizm borrow checker mógł wywnioskować czasy życia w tych sytuacjach, nie wymagając jawnych adnotacji.

Ten element historii Rust jest istotny, ponieważ istnieje możliwość, że zostaną odkryte kolejne deterministyczne wzorce i dodane do kompilatora. W przyszłości być może będzie wymaganych jeszcze mniej adnotacji czasu życia.

Wzorce wprowadzone do analizy odniesień w Rust noszą nazwę *zasad pomijania czasu życia* (lifetime elision rules). Nie są to reguły, które programiści mają przestrzegać; to szczególne przypadki, które kompilator uwzględni, i jeśli twój kod pasuje do tych przypadków, nie musisz jawnie określać czasów życia.

Zasady pomijania czasu życia nie dostarczają pełnej inferencji. Jeśli Rust deterministycznie zastosuje zasady, ale nadal istnieje niejednoznaczność co do tego, jakie czasy życia mają odniesienia, kompilator nie zgadnie, jaki powinien być czas życia pozostałych odniesień. W takim przypadku, zamiast zgadywać, kompilator generuje błąd, który możesz rozwiązać, dodając adnotacje czasu życia określające relacje między odniesieniami.

Czasy życia funkcji lub parametrów metod nazywane są *wejściowymi czasami życia* (input lifetimes), a czasy życia wartości zwracanych - *wyjściowymi czasami życia* (output lifetimes).

Kompilator używa trzech zasad do określenia czasów życia odniesień, gdy nie podano jawnych adnotacji. Pierwsza zasada dotyczy wejściowych czasów życia, a druga i trzecia dotyczą wyjściowych czasów życia. Jeśli kompilator dojdzie do końca trzech zasad i nadal istnieją odniesienia, dla których nie może ustalić czasów życia, kompilator zakończy pracę z błędem. Te zasady odnoszą się zarówno do definicji funkcji (`fn`), jak i do bloków `impl`.

Pierwsza zasada mówi, że każdy parametr będący odniesieniem otrzymuje własny parametr czasu życia. Innymi słowy, funkcja z jednym parametrem otrzymuje jeden parametr czasu życia: `fn foo<'a>(x: &'a i32)`; funkcja z dwoma parametrami otrzymuje dwa oddzielne parametry czasu życia: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`; i tak dalej.

Druga zasada mówi, że jeśli istnieje dokładnie jeden parametr czasu życia wejściowego, ten czas życia jest przypisany do wszystkich parametrów czasu życia wyjściowego: `fn foo<'a>(x: &'a i32) -> &'a i32`.

Trzecia zasada mówi, że jeśli istnieje wiele parametrów czasu życia wejściowego, lecz jeden z nich jest `&self` lub `&mut self`, ponieważ jest to metoda, czas życia `self` zostaje przypisany do wszystkich parametrów czasu życia wyjściowego. Ta trzecia zasada sprawia, że metody są znacznie łatwiejsze do odczytu i pisania, ponieważ potrzeba mniej symboli.

Załóżmy, że jesteśmy kompilatorem. Zastosujemy te zasady, aby określić czasy życia odniesień w sygnaturze funkcji `first_word` w Listing 10-26. Sygnatura rozpoczyna się bez żadnych czasów życia związanych z odniesieniami:

```rust,ignore
fn first_word(s: &str) -> &str {
```

Następnie kompilator stosuje pierwszą zasadę, która określa, że każdy parametr otrzymuje swój własny czas życia. Nazwiemy go `'a`, jak zwykle, więc teraz sygnatura wygląda tak:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &str {
```

Zasada druga ma zastosowanie, ponieważ istnieje dokładnie jeden parametr czasu życia wejściowego. Zasada druga określa, że czas życia jedynego parametru wejściowego zostaje przypisany do czasu życia wyjściowego, więc sygnatura wygląda teraz tak:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

Teraz wszystkie odniesienia w tej sygnaturze funkcji mają przypisane czasy życia, i kompilator może kontynuować analizę bez potrzeby, aby programista sam dodawał adnotacje czasów życia w tej sygnaturze funkcji.

Spójrzmy na inny przykład, tym razem używając funkcji `longest`, która początkowo nie miała parametrów czasu życia w Listing 10-21:

```rust,ignore
fn longest(x: &str, y: &str) -> &str {
```

Zastosujmy pierwszą zasadę: każdy parametr otrzymuje swój własny czas życia. Tym razem mamy dwa parametry zamiast jednego, więc mamy dwa czasy życia:

```rust,ignore
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

Widać, że zasada druga nie ma zastosowania, ponieważ istnieje więcej niż jeden parametr czasu życia wejściowego. Zasada trzecia również nie ma zastosowania, ponieważ `longest` to funkcja, a nie metoda, więc żaden z parametrów nie jest `self`. Po przejściu przez wszystkie trzy zasady nadal nie ustaliliśmy, jaki jest czas życia typu zwracanego. Dlatego otrzymaliśmy błąd przy próbie skompilowania kodu w przykładzie, w którym po raz pierwszy próbowaliśmy zaimplementować funkcję `longest`: kompilator przeszedł przez zasady pomijania czasu życia, lecz nie zdołał określić wszystkich czasów życia odniesień w sygnaturze.

Ponieważ zasada trzecia faktycznie odnosi się tylko do sygnatur metod, przyjrzymy się następnie czasom życia w tym kontekście, aby zrozumieć, dlaczego trzecia zasada sprawia, że rzadko musimy adnotować czasy życia w sygnaturach metod.