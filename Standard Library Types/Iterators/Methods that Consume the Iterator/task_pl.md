### Metody zużywające iterator

Cechą (`trait`) `Iterator` jest dostępność wielu różnych metod z domyślnymi implementacjami dostarczonymi przez bibliotekę standardową; informacje o tych metodach można znaleźć w dokumentacji API biblioteki standardowej dotyczącej cechy `Iterator`. Niektóre z tych metod korzystają z metody `next` w swojej definicji, co jest powodem, dla którego musisz zaimplementować metodę `next` podczas implementacji cechy `Iterator`.

Metody, które wywołują `next`, nazywane są _adaptatorami zużywającymi_, ponieważ ich wywołanie powoduje zużycie iteratora. Jednym z przykładów jest metoda `sum`, która przejmuje własność iteratora i przechodzi przez jego elementy, wielokrotnie wywołując `next`, co skutkuje zużyciem iteratora. Podczas iteracji metoda ta dodaje każdy element do sumy całkowitej i zwraca wynik po zakończeniu iteracji. Poniższy fragment kodu zawiera test ilustrujący użycie metody `sum`:

```rust
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
```

##### Wywołanie metody sum w celu uzyskania sumy wszystkich elementów w iteratorze

Nie możemy użyć `v1_iter` po wywołaniu metody `sum`, ponieważ `sum` przejmuje własność iteratora, na którym została wywołana.