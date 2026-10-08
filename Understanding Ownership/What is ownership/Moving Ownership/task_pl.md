## Sposoby interakcji zmiennych i danych: Przeniesienie

W języku Rust zmienne mogą w różny sposób współdzielić dane. Przyjrzyjmy się przykładowi z użyciem liczby całkowitej w poniższym fragmencie kodu.

```rust
let x = 5;
let y = x;
```

##### Przypisanie wartości całkowitej zmiennej x do y

Prawdopodobnie możemy odgadnąć, co się tutaj dzieje: „przypisz wartość `5` do `x`; następnie utwórz kopię wartości z `x` i przypisz ją do `y`.” Teraz mamy dwie zmienne, `x` i `y`, i obie mają wartość `5`. Tak właśnie się dzieje, ponieważ liczby całkowite to proste wartości o znanym, stałym rozmiarze, i te dwie wartości `5` są umieszczane na stosie.

Teraz przyjrzyjmy się wersji z użyciem `String`:

```rust
let s1 = String::from("hello");
let s2 = s1;
```

Kod wygląda bardzo podobnie do poprzedniego, więc moglibyśmy przypuszczać, że działa on w ten sam sposób: linia druga stworzy kopię wartości z `s1` i przypisze ją do `s2`. Ale w rzeczywistości dzieje się coś innego.

Spójrz na Rysunek 1, aby zobaczyć, jak `String` jest reprezentowany w pamięci. `String` składa się z trzech części, ukazanych po lewej stronie: wskaźnika do pamięci przechowującej zawartość tekstu, długości oraz pojemności. Ten zestaw danych jest przechowywany na stosie. Po prawej stronie znajduje się pamięć na stercie przechowująca zawartość.

<img alt="String w pamięci" src="https://doc.rust-lang.org/stable/book/img/trpl04-01.svg" class="center" style="width: 50%;">

##### Rysunek 1: Reprezentacja w pamięci łańcucha String przechowującego wartość "hello" przypisaną do s1

Długość określa, ile bajtów pamięci obecnie wykorzystuje zawartość `String`. Pojemność to całkowita ilość pamięci, w bajtach, jaką `String` otrzymał od alokatora. Różnica między długością a pojemnością ma znaczenie, ale w tym kontekście można ją pominąć.

Kiedy przypisujemy `s1` do `s2`, kopiowane są dane `String`, czyli wskaźnik, długość i pojemność, które znajdują się na stosie. Dane na stercie, do których odnosi się wskaźnik, nie są kopiowane. Innymi słowy, reprezentacja danych w pamięci jest taka jak na Rysunku 2.

<img alt="s1 i s2 wskazujące na tę samą wartość" src="https://doc.rust-lang.org/stable/book/img/trpl04-02.svg" class="center" style="width: 50%;">

##### Rysunek 2: Reprezentacja w pamięci zmiennej s2 posiadającej kopię wskaźnika, długości i pojemności z s1

Reprezentacja _nie_ wygląda jak na Rysunku 3, który pokazuje, jak wyglądałaby pamięć, gdyby Rust kopiował również dane na stercie. Gdyby Rust tak postępował, operacja `s2 = s1` mogłaby być bardzo kosztowna pod względem wydajności, zwłaszcza gdyby dane na stercie były duże.

<img alt="s1 i s2 wskazujące na dwa miejsca" src="https://doc.rust-lang.org/stable/book/img/trpl04-03.svg" class="center" style="width: 50%;">

##### Rysunek 3: Inna możliwość działania `s2 = s1`, gdyby Rust również kopiował dane na stercie

Wcześniej wspomnieliśmy, że gdy zmienna wychodzi z zakresu, Rust automatycznie wywołuje funkcję `drop` i usuwa pamięć na stercie przypisaną tej zmiennej. Ale Rysunek 2 pokazuje, że oba wskaźniki danych wskazują na to samo miejsce. To problem: kiedy `s2` i `s1` wychodzą z zakresu, obie spróbują zwolnić tę samą pamięć. To błąd znany jako _podwójne zwolnienie pamięci_ i jest to jeden z błędów bezpieczeństwa pamięci, o których wspomnieliśmy wcześniej. Zwolnienie tej samej pamięci dwukrotnie może prowadzić do uszkodzenia pamięci, co potencjalnie stwarza zagrożenia bezpieczeństwa.

Aby zapewnić bezpieczeństwo pamięci, Rust rozwiązuje tę sytuację w inny sposób. Zamiast kopiować przydzieloną pamięć, Rust uznaje `s1` za nieważną i w rezultacie nie próbuje niczego zwalniać po wyjściu `s1` z zakresu. Sprawdź, co się stanie, jeśli spróbujesz użyć `s1`, gdy `s2` już istnieje; operacja nie będzie działać:

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, świat!", s1);
```

Otrzymasz błąd, ponieważ Rust nie pozwala na użycie unieważnionego odniesienia:

```text
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 | 
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move
```

Jeśli słyszałeś terminy _płytka kopia_ i _głęboka kopia_ w kontekście innych języków programowania, to kopiowanie wskaźnika, długości i pojemności bez kopiowania danych można porównać do tworzenia płytkiej kopii. Jednak ponieważ Rust unieważnia pierwszą zmienną, zamiast nazywać to płytką kopią, proces ten określa się jako _przeniesienie_. W tym przykładzie możemy powiedzieć, że `s1` zostało _przeniesione_ do `s2`. Tak naprawdę, to co się dzieje, pokazuje Rysunek 4.

<img alt="s1 przeniesione do s2" src="https://doc.rust-lang.org/stable/book/img/trpl04-04.svg" class="center" style="width: 50%;">

##### Rysunek 4: Reprezentacja w pamięci po unieważnieniu `s1`

Problem rozwiązany! Tylko `s2` jest ważne, więc kiedy wychodzi ono z zakresu, samo zwalnia pamięć, i na tym kończy się proces.

Ponadto taki sposób projektowania implikuje, że Rust nigdy automatycznie nie tworzy „głębokich” kopii Twoich danych. Oznacza to, że każda _automatyczna_ kopia jest stosunkowo tania pod względem wydajności w czasie działania programu.