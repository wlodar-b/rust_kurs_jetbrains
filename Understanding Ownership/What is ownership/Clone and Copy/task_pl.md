## Sposoby interakcji zmiennych i danych: Clone

Jeżeli _chcemy_ dokonać głębokiej kopii danych znajdujących się na stercie, a nie tylko danych na stosie, możemy użyć powszechnie stosowanej metody o nazwie `clone`. Omówimy składnię metod w rozdziale "Struktury", ale ponieważ metody są częstą cechą wielu języków programowania, prawdopodobnie widziałeś je już wcześniej.

Oto przykład działania metody `clone`:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

To działa bez problemu i wyraźnie pokazuje zachowanie zilustrowane na Rysunku 3, gdzie dane znajdujące się na stercie _zostają_ skopiowane.

Kiedy widzisz wywołanie metody `clone`, wiesz, że wykonywany jest pewien arbitralny kod i ten kod może być kosztowny. Jest to wizualna wskazówka, że dzieje się coś innego.

## Dane tylko na stosie: Copy

Istnieje inny aspekt, którego jeszcze nie omówiliśmy. Kod wykorzystujący liczby całkowite, częściowo przedstawiony we fragmencie "Przypisywanie wartości całkowitej zmiennej x do y", działa i jest poprawny:

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

Jednak ten kod wydaje się przeczyć temu, czego właśnie się nauczyliśmy: nie wywołujemy `clone`, ale `x` nadal jest ważne i nie zostało przeniesione do `y`.

Powód jest taki, że typy, takie jak liczby całkowite, które mają znany rozmiar w czasie kompilacji, są przechowywane w całości na stosie, więc kopiowanie ich wartości jest szybkie. Oznacza to, że nie ma powodu, aby uniemożliwiać zmiennej `x` dalsze istnienie po utworzeniu zmiennej `y`. Innymi słowy, nie ma tutaj różnicy pomiędzy głębokim a płytkim kopiowaniem, więc wywołanie `clone` nie zrobiłoby nic innego niż zwykłe płytkie kopiowanie i możemy je pominąć.

Rust ma specjalną adnotację o nazwie `Copy`, którą możemy zastosować do typów takich jak liczby całkowite, przechowywanych na stosie (więcej o cechach, tzw. "traits", omówimy w Rozdziale 10). Jeżeli typ implementuje cechę `Copy`, starsza zmienna nadal może być używana po przypisaniu. Rust nie pozwoli nam jednak oznaczyć typu cechą `Copy`, jeżeli ten typ lub którąkolwiek z jego części oznaczono jako implementującą cechę `Drop`. Jeżeli typ wymaga szczególnych działań podczas usuwania wartości z zakresu, a my dodamy mu adnotację `Copy`, otrzymamy błąd kompilacji. Aby dowiedzieć się, jak dodać adnotację `Copy` do własnego typu w celu implementacji tej cechy, zobacz [“Cechy pochodne”](https://doc.rust-lang.org/stable/book/appendix-03-derivable-traits.html) w Załączniku C.

A jakie typy implementują cechę `Copy`? Możesz sprawdzić dokumentację dla konkretnego typu, aby się upewnić, ale generalnie każda grupa prostych wartości skalarnych może implementować `Copy`, a nic, co wymaga alokacji lub jest formą zasobów, nie może tego zrobić. Oto niektóre z typów, które implementują `Copy`:

*   Wszystkie typy całkowite, takie jak `u32`.
*   Typ logiczny, `bool`, z wartościami `true` i `false`.
*   Wszystkie typy zmiennoprzecinkowe, takie jak `f64`.
*   Typ znakowy, `char`.
*   Krotki, pod warunkiem, że zawierają tylko typy, które również implementują `Copy`. Na przykład `(i32, i32)` implementuje `Copy`, ale `(i32, String)` już nie.