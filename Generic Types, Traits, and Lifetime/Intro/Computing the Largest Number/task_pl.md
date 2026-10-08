## Obliczanie największej liczby

Rozważmy krótki program, który znajduje największą liczbę w liście, jak pokazano w poniższym fragmencie kodu.

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
}
```

#### Kod do znalezienia największej liczby w liście liczb.

Ten kod przechowuje listę liczb całkowitych w zmiennej `number_list` i umieszcza pierwszą liczbę z listy w zmiennej o nazwie `largest`. Następnie iteruje przez wszystkie liczby w liście, a jeśli bieżąca liczba jest większa od tej zapisanej w `largest`, zastępuje nią liczbę w tej zmiennej. Jednakże, jeśli bieżąca liczba jest mniejsza lub równa największej liczbie napotkanej do tej pory, zmienna pozostaje bez zmian, a kod przechodzi do kolejnej liczby na liście. Po rozważeniu wszystkich liczb z listy, `largest` powinna zawierać największą liczbę, która w tym przypadku wynosi 100.