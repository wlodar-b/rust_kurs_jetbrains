## Ograniczenia dotyczące zmiennych referencji

Zmienna referencja ma jedno istotne ograniczenie: w danym zakresie można mieć tylko jedną zmienną referencję do konkretnego fragmentu danych. Ten kod spowoduje błąd:

```rust
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
```

Poniżej błąd:

```text
    error[E0499]: cannot borrow `s` as mutable more than once at a time
     --> src/main.rs:5:14
      |
    4 |     let r1 = &mut s;
      |              ------ pierwsze pożyczenie zmiennej następuje tutaj
    5 |     let r2 = &mut s;
      |              ^^^^^^ drugie pożyczenie zmiennej następuje tutaj
    6 |
    7 |     println!("{}, {}", r1, r2);
      |                        -- pierwsze pożyczenie jest później używane tutaj
```

To ograniczenie pozwala na modyfikowanie danych w bardzo kontrolowany sposób. Jest to coś, z czym nowi użytkownicy Rust (Rustaceans) często mają trudności, ponieważ większość języków pozwala na modyfikowanie danych w dowolnym momencie.

Zaletą tego ograniczenia jest to, że Rust może zapobiec wyścigom danych już na etapie kompilacji. _Wyścig danych_ jest zbliżony do warunku wyścigu i występuje, gdy mają miejsce następujące trzy zdarzenia:

*   Dwa lub więcej wskaźników uzyskuje dostęp do tych samych danych w tym samym czasie.
*   Co najmniej jeden z tych wskaźników jest używany do zapisu danych.
*   Nie ma żadnego mechanizmu synchronizującego dostęp do danych.

Wyścigi danych powodują nieokreślone działanie i mogą być trudne do zdiagnozowania i naprawienia, gdy próbujemy je znaleźć w czasie wykonania programu. Rust całkowicie zapobiega temu problemowi, ponieważ kod zawierający takie wyścigi danych nie zostanie skompilowany!

Jak zawsze, możemy użyć nawiasów klamrowych, aby stworzyć nowy zakres pozwalający na wiele zmiennych referencji, ale nie _jednocześnie_:

```rust
    let mut s = String::from("hello");

    {
        let r1 = &mut s;

    } // r1 wychodzi poza zakres tutaj, więc możemy utworzyć nową referencję bez problemów.

    let r2 = &mut s;
```

Podobna zasada dotyczy łączenia zmiennych i niezmiennych referencji. Ten kod skutkuje błędem:

```rust
    let mut s = String::from("hello");

    let r1 = &s; // bez problemu
    let r2 = &s; // bez problemu
    let r3 = &mut s; // DUŻY PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
```

Poniżej błąd:

```text
    error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
     --> src/main.rs:6:14
      |
    4 |     let r1 = &s; // bez problemu
      |              -- niezmienne pożyczenie następuje tutaj
    5 |     let r2 = &s; // bez problemu
    6 |     let r3 = &mut s; // DUŻY PROBLEM
      |              ^^^^^^ zmienne pożyczenie następuje tutaj
    7 |
    8 |     println!("{}, {}, and {}", r1, r2, r3);
      |                                -- niezmienne pożyczenie jest później używane tutaj
```

Uf! _Nie możemy_ również mieć zmiennej referencji, gdy mamy niezmienną. Użytkownicy niezmiennych referencji nie oczekują, że wartość nagle zostanie zmieniona w trakcie ich użycia! Jednak wiele niezmiennych referencji jest dozwolonych, ponieważ osoby tylko odczytujące dane nie mogą wpływać na to, jak inni te dane odczytują.