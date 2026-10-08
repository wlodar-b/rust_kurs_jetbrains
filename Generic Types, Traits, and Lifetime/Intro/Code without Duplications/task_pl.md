## Kod bez duplikacji

W poniższym fragmencie kodu wyodrębniliśmy kod, który znajduje największą liczbę, do funkcji o nazwie `largest`. W przeciwieństwie do kodu z pierwszego przykładu w tej sekcji, który potrafi znaleźć największą liczbę jedynie w konkretnej liście, ten program potrafi znaleźć największą liczbę w dwóch różnych listach.

```rust
fn largest(list: &[i32]) -> &i32 {
  let mut largest = &list[0];

  for item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];
  let result = largest(&number_list);
  println!("Największa liczba to {}", result);

  let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
  let result = largest(&number_list);
  println!("Największa liczba to {}", result);
}
```

#### Wyabstrahowany kod do znalezienia największej liczby w dwóch listach

Funkcja `largest` ma parametr o nazwie `list`, który reprezentuje dowolny konkretny wycinek wartości typu `i32`, jakie możemy przekazać do funkcji. W rezultacie, gdy wywołujemy tę funkcję, kod działa na konkretnych wartościach, które przekazujemy.

Podsumowując, oto kroki, jakie podjęliśmy, aby zmienić kod od drugiego do trzeciego przykładu:

1. Zidentyfikowanie zduplikowanego kodu.
2. Wyodrębnienie zduplikowanego kodu do ciała funkcji i określenie wejściowych danych oraz wartości zwracanych przez ten kod w sygnaturze funkcji.
3. Zaktualizowanie dwóch wystąpień zduplikowanego kodu, aby wywoływały funkcję.

Następnie zastosujemy te same kroki z typami generycznymi, aby zmniejszyć duplikację kodu w inny sposób. W ten sam sposób, w jaki ciało funkcji może działać na abstrakcyjnej `liście` zamiast na konkretnych wartościach, typy generyczne pozwalają kodowi działać na abstrakcyjnych typach danych.

Na przykład załóżmy, że mamy dwie funkcje: jedną, która znajduje największy element w wycinku wartości typu `i32`, oraz drugą, która znajduje największy element w wycinku wartości typu `char`. Jak moglibyśmy wyeliminować tę duplikację? Dowiedzmy się!