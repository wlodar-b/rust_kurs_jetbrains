## Iteracja po tablicy za pomocą for

Możesz użyć konstrukcji `while`, aby iterować po elementach kolekcji, takiej jak tablica. Na przykład spójrzmy na poniższy kod:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("wartość to: {}", a[index]);

        index += 1;
    }
}
```
##### Przykład iteracji po każdym elemencie kolekcji przy użyciu pętli while

Tutaj kod przechodzi przez elementy tablicy, zaczynając od indeksu `0` i pętląc się, aż dotrze do ostatniego indeksu w tablicy (tj. gdy `index < 5` przestaje być prawdziwe). Uruchomienie tego kodu wydrukuje każdy element tablicy:

```text
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/loops`
wartość to: 10
wartość to: 20
wartość to: 30
wartość to: 40
wartość to: 50
```

Wszystkie pięć wartości tablicy pojawia się w terminalu, zgodnie z oczekiwaniami. Mimo że `index` w pewnym momencie osiąga wartość `5`, pętla przestaje się wykonywać przed próbą pobrania szóstego elementu z tablicy.

Jednak to podejście jest podatne na błędy; moglibyśmy spowodować awarię programu (panic), jeśli długość indeksu byłaby nieprawidłowa. Jest ono również wolne, ponieważ kompilator dodaje kod wykonywany w czasie działania programu, aby sprawdzać warunki dla każdego elementu podczas każdej iteracji w pętli.

Jako bardziej zwięzłą alternatywę, możesz użyć pętli `for` i wykonywać pewien kod dla każdego elementu w kolekcji. Pętla for wygląda następująco:

```rust
fn main() {
    let arr = [10, 20, 30, 40, 50];

    for element in arr {
        println!("wartość to: {}", element);
    }
}
```
##### Przykład iteracji po każdym elemencie kolekcji przy użyciu pętli for

Po uruchomieniu tego kodu zobaczymy ten sam wynik, co w poprzednim fragmencie kodu. Co ważniejsze, zwiększyliśmy teraz bezpieczeństwo kodu i wyeliminowaliśmy ryzyko błędów wynikających z wychodzenia poza koniec tablicy lub niepełnego jej przejścia, przez co moglibyśmy pominąć niektóre elementy.

Na przykład, w poprzednim fragmencie kodu, jeśli zmienisz definicję tablicy `a` na cztery elementy, ale zapomnisz zaktualizować warunek na `while index < 4`, kod spowoduje awarię programu (panic). Używając pętli `for`, nie musisz pamiętać o zmianie żadnego innego kodu, gdy zmienisz liczbę wartości w tablicy.