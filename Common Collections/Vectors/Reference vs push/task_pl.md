### Odniesienie vs `push`

Kiedy program ma prawidłowe odniesienie, kompilator (borrow checker) wymusza zasady
własności i pożyczania (omówione w części "Zrozumienie własności"), aby upewnić się, że to odniesienie
i jakiekolwiek inne odniesienia do zawartości wektora pozostaną ważne. Przypomnijmy
zasadę, która mówi, że nie można mieć jednocześnie mutowalnych i niemutowalnych odniesień w tym samym zakresie.
Ta zasada znajduje zastosowanie w poniższym kodzie, gdzie posiadamy niemutowalne odniesienie
do pierwszego elementu wektora i próbujemy dodać element na końcu, co się nie powiedzie,
jeśli spróbujemy później w funkcji użyć odniesienia do tego elementu:

```rust,ignore,does_not_compile
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("Pierwszy element to: {}", first);
```

#### Próba dodania elementu do wektora przy jednoczesnym posiadaniu odniesienia do elementu

Kompilacja tego kodu zakończy się błędem:

```text
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let first = &v[0];
  |                  - tutaj występuje niemutowalne pożyczanie
5 | 
6 |     v.push(6);
  |     ^^^^^^^^^ tutaj występuje mutowalne pożyczanie
7 | 
8 |     println!("Pierwszy element to: {}", first);
  |                                          ----- niemutowalne pożyczanie używane tutaj później
```

Powyższy kod może wyglądać tak, jakby powinien działać: dlaczego odniesienie
do pierwszego elementu miałoby się przejmować zmianami na końcu wektora? Ten błąd wynika ze sposobu działania wektorów: dodanie nowego elementu na końcu
może wymagać rezerwacji nowej pamięci i skopiowania starych elementów do nowego miejsca, jeśli nie ma wystarczająco dużo miejsca,
aby umieścić wszystkie elementy obok siebie w aktualnym miejscu wektora. W takim przypadku odniesienie do pierwszego
elementu wskazywałoby na zdealokowany obszar pamięci. Zasady pożyczania zapobiegają
sytuacjom, w których program może skończyć w takim niebezpiecznym stanie.