## Używanie Clippy jako zewnętrznego lintera

Narzędzie `Clippy` to zbiór lintów służących do analizy Twojego kodu, 
aby pomóc Ci wykrywać typowe błędy i ulepszać kod w języku Rust. 
Aby użyć `Clippy` jako zewnętrznego lintera, postępuj zgodnie z instrukcjami z [wcześniejszego zadania](course://Introduction/Getting started/External Linter), ale 
tym razem wybierz **Clippy** zamiast **Cargo Check**.

Sprawdź sugestie `Clippy` i zastosuj je, aby rozwiązać zadanie. 
Zwróć uwagę, że przycisk **Check** w tym zadaniu faktycznie niczego nie sprawdza. 
Tutaj musisz polegać na własnym osądzie!

<div class="hint">
Rust przechowuje wersje matematycznych stałych o najwyższej precyzji, 
w tym stałych długich lub o nieskończonej precyzji, w <a href="https://doc.rust-lang.org/stable/std/f32/consts/index.html">standardowej bibliotece języka Rust</a>.

Możemy być kuszeni, aby używać własnych przybliżeń dla niektórych stałych matematycznych, 
ale `Clippy` rozpoznaje te nieprecyzyjne stałe matematyczne jako potencjalne źródło błędów. 
Zapoznaj się z sugestiami ostrzeżeń `Clippy` w wynikach kompilacji i użyj 
odpowiedniego zamiennika stałej z `std::f32::consts`.
</div>