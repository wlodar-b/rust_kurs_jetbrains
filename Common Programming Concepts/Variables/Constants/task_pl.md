### Stałe

Podobnie jak niezmienne zmienne, _stałe_ to wartości przypisane do nazwy, które nie mogą być zmieniane, ale istnieje kilka różnic między stałymi a zmiennymi.

Po pierwsze, nie można używać `mut` w przypadku stałych. Stałe są nie tylko domyślnie niezmienne – są zawsze niezmienne. Deklaruje się je za pomocą słowa kluczowego `const` zamiast słowa kluczowego `let`, a typ wartości _musi_ być podany w adnotacjach. Typy i adnotacje typów omówimy w następnej lekcji, [Podstawowe typy danych](course://Common Programming Concepts/Basic Data Types), więc na razie nie przejmuj się szczegółami. Po prostu wiedz, że zawsze musisz podać typ.

Stałe można deklarować w dowolnym zakresie, włączając w to zakres globalny, co czyni je przydatnymi dla wartości, które wiele części kodu musi znać.

Ostatnią różnicą jest to, że stałe mogą być ustawione tylko na wyrażenia stałe, a nie na wynik wartości, która mogłaby zostać obliczona dopiero w czasie wykonywania programu.

Oto przykład deklaracji stałej:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

Nazwa stałej to `THREE_HOURS_IN_SECONDS`, a jej wartość jest ustawiona na wynik pomnożenia 60 (liczby sekund w minucie) przez 60 (liczby minut w godzinie) oraz przez 3 (liczbę godzin, które chcemy policzyć w tym programie). Konwencja nazewnicza w Rust dla stałych polega na używaniu wyłącznie wielkich liter oddzielonych podkreśleniami. Kompilator jest w stanie ocenić ograniczony zestaw operacji w czasie kompilacji, co pozwala nam zapisać tę wartość w sposób łatwiejszy do zrozumienia i weryfikacji, zamiast przypisywać tej stałej wartość 10 800. Zajrzyj do [sekcji oceny stałych w dokumentacji języka Rust](https://doc.rust-lang.org/stable/reference/const_eval.html), aby dowiedzieć się więcej o tym, jakie operacje można używać podczas deklaracji stałych.

Stałe są ważne przez cały czas działania programu, w zakresie, w którym je zdeklarowano. Ta właściwość sprawia, że są one przydatne dla wartości w Twojej dziedzinie aplikacji, które mogą być potrzebne w różnych częściach programu, takich jak maksymalna liczba punktów, jakie może zdobyć gracz w grze, lub prędkość światła.

Nazywanie zakodowanych na sztywno wartości używanych w całym programie jako stałych jest przydatne, aby przekazać znaczenie tej wartości przyszłym osobom konserwującym kod. Pomaga to również mieć tylko jedno miejsce w kodzie, które trzeba zmienić, jeśli ta zakodowana wartość musiałaby zostać zaktualizowana w przyszłości.

_Możesz odwołać się do następującego rozdziału w książce o języku Rust: [Constants](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html#constants)_