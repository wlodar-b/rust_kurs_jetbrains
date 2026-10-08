## Włącz zewnętrzny linter

Wtyczka IntelliJ Rust nie wykrywa wszystkich błędów. Polega na kompilatorze Rust, aby to zrobić. Podczas nauki Rusta warto widzieć błędy na bieżąco, podczas pisania kodu. Aby osiągnąć taką funkcjonalność, zalecamy włączenie zewnętrznego lintera w następujący sposób:

1. Przejdź do **Ustawienia / Preferencje | Języki i Frameworki | Rust | Zewnętrzne Lintery** (lub bezpośrednio **Ustawienia / Preferencje | Rust | Zewnętrzne Lintery**, jeśli używasz RustRover).
2. Ustaw parametry w następujący sposób:
    - Wybierz **Cargo Check** z listy w polu **Zewnętrzne narzędzie:**;
    - Zaznacz pole wyboru **Uruchom zewnętrzny linter, aby analizować kod w locie**.

![Zewnętrzne Lintery](images/external-linters.png)
3. Naciśnij **OK**.

Gdy to zrobisz, %IDE_NAME% będzie zgłaszać wszystkie błędy wykryte przez wtyczkę IntelliJ Rust lub kompilator Rust.