## Przetwarzanie serii elementów za pomocą iteratorów

Wzorzec iteratora pozwala wykonywać zadania na sekwencji elementów jeden po drugim. Iterator odpowiada za logikę iteracji po każdym elemencie i określanie, kiedy sekwencja została zakończona. Używając iteratorów, nie musisz samodzielnie implementować tej logiki.

W Rust iteratory są _leniwe_, co oznacza, że nie mają wpływu, dopóki nie wywołasz metod, które konsumują iterator, aby go użyć. Na przykład kod w poniższym fragmencie tworzy iterator po elementach wektora `v1`, wywołując metodę `iter` zdefiniowaną dla `Vec<T>`. Sam ten kod nie wykonuje niczego użytecznego.

```rust
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
```

##### Tworzenie iteratora

Kiedy już utworzymy iterator, możemy go używać na różne sposoby. W rozdziale "Podstawowe koncepty programowania/If" używaliśmy iteratorów w pętlach `for`, aby wykonać kod dla każdego elementu, chociaż wcześniej pominęliśmy wyjaśnienie, co dokładnie robi wywołanie `iter`.

Przykład w następnym fragmencie oddziela tworzenie iteratora od jego użycia w pętli `for`. Iterator jest przechowywany w zmiennej `v1_iter`, a iteracja nie odbywa się w tym momencie. Kiedy pętla `for` zostaje wywołana z użyciem iteratora w `v1_iter`, każdy element iteratora jest wykorzystywany w jednej iteracji pętli, co skutkuje wypisaniem każdej wartości.

```rust
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
```

##### Użycie iteratora w pętli for

W językach, które nie mają iteratorów dostarczanych przez ich standardowe biblioteki, prawdopodobnie napisałbyś tę samą funkcjonalność, zaczynając od zmiennej z wartością 0 jako indeksu, używając tej zmiennej do indeksowania wektora, aby uzyskać wartość, i zwiększając wartość zmiennej w pętli, aż osiągnęłaby całkowitą liczbę elementów w wektorze.

Iteratory zajmują się całą tą logiką za Ciebie, eliminując powtarzalny kod, w którym mógłbyś popełnić błąd. Dają Ci również większą elastyczność, pozwalając używać tej samej logiki z wieloma różnymi rodzajami sekwencji, nie tylko strukturami danych, do których możesz się odwoływać za pomocą indeksów, takimi jak wektory. Przyjrzyjmy się, jak iteratory to robią.