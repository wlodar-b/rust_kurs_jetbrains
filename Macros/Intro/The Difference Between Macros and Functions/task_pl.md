### Różnica między makrami a funkcjami

Zasadniczo makra są sposobem pisania kodu, który generuje inny kod, co jest znane jako _metaprogramowanie_. W Dodatku C omawiamy atrybut `derive`, który generuje implementację różnych cech (traits) dla Ciebie. Korzystaliśmy również z makr `println!` i `vec!` w całej książce. Wszystkie te makra _rozszerzają się_, aby wygenerować więcej kodu niż ten, który napisałeś ręcznie.

Metaprogramowanie jest przydatne, ponieważ zmniejsza ilość kodu, który musisz napisać i utrzymać, co jest również jedną z funkcji, jakie pełnią funkcje. Jednak makra mają pewne dodatkowe możliwości, których funkcje nie posiadają.

Sygnatura funkcji musi deklarować liczbę i typ parametrów, jakie funkcja przyjmuje. Makra natomiast mogą przyjmować zmienną liczbę parametrów: możemy wywołać `println!("hello")` z jednym argumentem lub `println!("hello {}", name)` z dwoma argumentami. Ponadto makra są rozszerzane przed tym, jak kompilator interpretuje znaczenie kodu, więc makro może na przykład zaimplementować cechę (trait) dla danego typu. Funkcja nie może tego zrobić, ponieważ jest wywoływana w czasie wykonywania, a cecha musi zostać zaimplementowana w czasie kompilacji.

Minusem implementowania makra zamiast funkcji jest to, że definicje makr są bardziej złożone niż definicje funkcji, ponieważ piszesz kod w Rust, który generuje kod w Rust. Ze względu na tę złożoność definicje makr są zazwyczaj trudniejsze do czytania, zrozumienia i utrzymania niż definicje funkcji.

Kolejną istotną różnicą między makrami a funkcjami jest to, że musisz zdefiniować makra lub wprowadzić je w zakres _przed_ ich wywołaniem w pliku, w przeciwieństwie do funkcji, które możesz zdefiniować i wywołać w dowolnym miejscu.