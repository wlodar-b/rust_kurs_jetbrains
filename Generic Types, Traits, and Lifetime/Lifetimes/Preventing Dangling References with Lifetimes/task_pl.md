### Zapobieganie wiszącym referencjom za pomocą czasu życia

Głównym celem czasu życia (ang. lifetimes) jest zapobieganie wiszącym referencjom, które powodują, że program odnosi się do danych innych niż te, które miał na celu odwoływać. Rozważmy poniższy program, który zawiera zewnętrzny zakres i wewnętrzny zakres.

```rust,ignore,does_not_compile
    {
        let r;

        {
            let x = 5;
            r = &x;
        }

        println!("r: {}", r);
    }
```

#### Próba użycia referencji, której wartość wyszła poza zakres

> Uwaga: Przykłady w tym oraz w kilku następnych listowaniach deklarują zmienne bez przypisywania im początkowych wartości, dzięki czemu nazwa zmiennej istnieje w zewnętrznym zakresie. Na pierwszy rzut oka może to wydawać się sprzeczne z brakiem wartości null w Rust. Jednakże, jeśli spróbujemy użyć zmiennej przed przypisaniem jej wartości, otrzymamy błąd w czasie kompilacji, co pokazuje, że Rust rzeczywiście nie dopuszcza wartości null.

Zewnętrzny zakres deklaruje zmienną o nazwie `r` bez wartości początkowej, a wewnętrzny zakres deklaruje zmienną o nazwie `x` z początkową wartością 5. W wewnętrznym zakresie próbujemy przypisać do `r` referencję do `x`. Następnie wewnętrzny zakres się kończy, a my próbujemy wypisać wartość zmiennej `r`. Kod ten nie skompiluje się, ponieważ wartość, do której odnosi się `r`, wyszła poza zakres, zanim spróbowaliśmy jej użyć. Oto komunikat błędu:

```console
error[E0597]: `x` does not live long enough
  --> src/main.rs:7:17
   |
7  |             r = &x;
   |                 ^^ wartość odwołana nie żyje wystarczająco długo
8  |         }
   |         - `x` został zwolniony tutaj, wciąż będąc odwoływanym
9  | 
10 |         println!("r: {}", r);
   |                           - odwołanie użyte później tutaj
```

Zmienna `x` „nie żyje wystarczająco długo.” Powodem jest to, że `x` będzie poza zakresem, gdy wewnętrzny zakres zakończy się na linii 7. Jednakże zmienna `r` pozostaje ważna w zewnętrznym zakresie; ponieważ jej zakres jest większy, mówimy, że „żyje dłużej.” Gdyby Rust pozwolił na działanie tego kodu, `r` odnosiłoby się do pamięci, która została zdealokowana, gdy `x` wyszedł z zakresu, a jakiekolwiek operacje na `r` nie działałyby poprawnie. Więc jak Rust określa, że ten kod jest nieważny? Wykorzystuje do tego kontroler pożyczek (ang. borrow checker).