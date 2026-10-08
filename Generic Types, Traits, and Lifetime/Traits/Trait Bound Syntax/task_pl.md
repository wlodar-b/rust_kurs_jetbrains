## Składnia Trait Bound

Wyimaginowana magiczna szkoła ma nowy system generowania świadectw napisany w języku Rust!  
Obecnie system obsługuje tworzenie świadectw, w których ocena ucznia jest przedstawiona numerycznie (np. 1.0 -> 5.5).  
Jednakże, szkoła wystawia również oceny alfabetyczne (A+ -> F-) i potrzebuje możliwości drukowania obu typów świadectw!

Wprowadź odpowiednie zmiany w kodzie w strukturze `ReportCard` oraz w bloku `impl`, aby obsługiwać alfabetyczne świadectwa.

<div class="hint">Aby znaleźć najlepsze rozwiązanie tego zadania, będziesz musiał odwołać się do swojej wiedzy o cechach (traits), szczególnie o składni Trait Bound. Może być Ci również potrzebne to: <code>use std::fmt::Display;</code></div>

<div class="hint">To zadanie jest zdecydowanie trudniejsze niż dwa poprzednie! Musisz pomyśleć nie tylko o tym, aby struktura <code>ReportCard</code> była ogólna (generic), ale także o poprawnej właściwości - konieczna będzie również niewielka zmiana implementacji tej struktury... dasz radę!</div>