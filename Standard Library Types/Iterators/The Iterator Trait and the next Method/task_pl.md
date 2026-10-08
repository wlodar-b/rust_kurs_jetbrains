### Cecha iteratora i metoda next

Wszystkie iteratory implementują cechę o nazwie `Iterator`, która jest zdefiniowana w bibliotece standardowej. Definicja tej cechy wygląda następująco:

```rust
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;

        // pominięto metody z domyślną implementacją
    }
```

Zwróć uwagę, że ta definicja używa nowej składni: `type Item` i `Self::Item`, co oznacza definiowanie _związanych typów_ w tej cesze. Omówimy związane typy bardziej szczegółowo w Rozdziale 19. Na razie wystarczy wiedzieć, że ten kod mówi, iż implementowanie cechy `Iterator` wymaga również zdefiniowania typu `Item`, a ten typ `Item` jest używany w typie zwracanym przez metodę `next`. Innymi słowy, typ `Item` to typ zwracany z iteratora.

Cechy `Iterator` wymagają od implementatorów zdefiniowania tylko jednej metody: metody `next`, która zwraca jeden element iteratora na raz, opakowany w `Some`, a gdy iteracja się kończy, zwraca `None`.

Możemy wywołać metodę `next` bezpośrednio na iteratorach; poniższy przykład demonstruje, jakie wartości są zwracane z kolejnych wywołań metody `next` na iteratorze utworzonym z wektora.

```rust
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
```

##### Wywoływanie metody next na iteratorze

Zauważ, że musieliśmy uczynić `v1_iter` zmienną mutowalną: wywoływanie metody `next` na iteratorze zmienia jego wewnętrzny stan, który służy do śledzenia miejsca w sekwencji. Innymi słowy, ten kod _konsumuje_ lub zużywa iterator. Każde wywołanie `next` pobiera jeden element z iteratora. Nie musieliśmy uczynić `v1_iter` mutowalnym, kiedy użyliśmy pętli `for`, ponieważ pętla przejmuje własność nad `v1_iter` i czyni go mutowalnym w tle.

Zwróć także uwagę, że wartości otrzymywane podczas wywołań metody `next` to niemutowalne referencje do wartości w wektorze. Metoda `iter` tworzy iterator nad niemutowalnymi referencjami. Jeśli chcemy utworzyć iterator, który przejmuje własność nad `v1` i zwraca posiadane wartości, możemy wywołać `into_iter` zamiast `iter`. Podobnie, jeśli chcemy iterować po mutowalnych referencjach, możemy wywołać `iter_mut` zamiast `iter`.