### Zasięg zmiennych

Omówiliśmy już przykład programu w języku Rust w Rozdziale 2. Teraz, gdy znamy podstawową składnię, nie będziemy zawierać całego kodu `fn main() {` w przykładach. Jeśli podążasz za kursem, będziesz musiał ręcznie umieścić poniższe przykłady w funkcji `main`. Dzięki temu przykłady będą bardziej zwięzłe i pozwolą nam skupić się na szczegółach, a nie na kodzie szablonowym.

Jako pierwszy przykład własności (ang. _ownership_), przyjrzymy się _zasięgowi_ niektórych zmiennych. Zasięg to zakres w programie, w którym element jest ważny. Załóżmy, że mamy zmienną wyglądającą tak:

```rust
let s = "hello"
```

Zmienna `s` odnosi się do stałego napisu (ang. _string literal_), gdzie wartość napisu jest zakodowana na stałe w tekście naszego programu. Zmienna jest ważna od momentu jej zadeklarowania aż do końca bieżącego _zasięgu_. Poniższy fragment kodu zawiera komentarze opisujące, gdzie zmienna `s` jest ważna.

```rust
{                      // s nie jest tutaj ważne, ponieważ jeszcze nie zostało zadeklarowane
let s = "hello";   // s jest ważne od tego momentu

// operacje na s
}                      // ten zasięg się kończy, a zmienna s przestaje być ważna
```

##### Zmienna oraz zasięg, w którym jest ważna

Innymi słowy, mamy tutaj dwa istotne momenty:

*   Gdy `s` _wchodzi w zasięg_, staje się ważne.
*   Pozostaje ważne aż do momentu, gdy _wychodzi z zasięgu_.

Na tym etapie relacja między zasięgami a momentami, kiedy zmienne są ważne, jest podobna do tej w innych językach programowania. Teraz rozbudujemy to zrozumienie, wprowadzając typ `String`.