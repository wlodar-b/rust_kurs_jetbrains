## Powtarzające się fragmenty kodu

Zanim przejdziemy do składni generyków, najpierw przyjrzyjmy się, jak usunąć 
powtórzenia, które nie dotyczą typów generycznych, poprzez wyodrębnienie funkcji. Następnie 
zastosujemy tę technikę, aby wyodrębnić funkcję generyczną! W ten sam sposób, w jaki 
rozpoznajesz powtarzający się kod do wydzielenia w funkcję, zaczniesz rozpoznawać 
powtarzający się kod, który może używać generyków.

Aby znaleźć największą liczbę w dwóch różnych listach liczb, możemy powielić 
kod powyżej i użyć tej samej logiki w dwóch różnych miejscach 
programu, jak przedstawiono poniżej.

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("Największa liczba to {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("Największa liczba to {}", largest);
}
```

### Kod do znalezienia największej liczby w *dwóch* listach liczb

Chociaż ten kod działa, powielanie kodu jest żmudne i podatne na błędy. Musimy również 
aktualizować kod w kilku miejscach, jeśli chcemy go zmienić.