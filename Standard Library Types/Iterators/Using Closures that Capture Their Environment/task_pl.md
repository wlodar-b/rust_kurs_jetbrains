### Używanie wyrażeń lambda przechwytujących swoje otoczenie

Teraz, gdy przedstawiliśmy iteratory, możemy zademonstrować typowe zastosowanie wyrażeń lambda (closures) przechwytujących swoje otoczenie za pomocą adaptatora iteratora `filter`. Metoda `filter` na iteratorze przyjmuje wyrażenie lambda, które przetwarza każdy element iteratora i zwraca wartość logiczną (Boolean). Jeśli wyrażenie lambda zwróci `true`, wartość zostanie uwzględniona w iteratorze produkowanym przez `filter`. Jeśli wyrażenie zwróci `false`, wartość nie zostanie uwzględniona w wynikowym iteratorze.

W poniższym fragmencie kodu używamy `filter` z wyrażeniem lambda, które przechwytuje zmienną `shoe_size` ze swojego otoczenia, aby iterować po kolekcji instancji struktury `Shoe`. Funkcja zwróci tylko buty o podanym rozmiarze.

```rust
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter()
            .filter(|s| s.size == shoe_size)
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn filters_by_size() {
            let shoes = vec![
                Shoe { size: 10, style: String::from("sneaker") },
                Shoe { size: 13, style: String::from("sandal") },
                Shoe { size: 10, style: String::from("boot") },
            ];

            let in_my_size = shoes_in_my_size(shoes, 10);

            assert_eq!(
                in_my_size,
                vec![
                    Shoe { size: 10, style: String::from("sneaker") },
                    Shoe { size: 10, style: String::from("boot") },
                ]
            );
        }
    }
```

##### Używanie metody filter z wyrażeniem lambda przechwytującym shoe_size

Funkcja `shoes_in_my_size` przyjmuje na własność wektor butów oraz rozmiar buta jako parametry. Zwraca wektor zawierający tylko buty o podanym rozmiarze.

W ciele funkcji `shoes_in_my_size` wywołujemy `into_iter`, aby stworzyć iterator, który przejmuje własność wektora. Następnie wywołujemy `filter`, aby przekształcić ten iterator w nowy iterator zawierający tylko elementy, dla których wyrażenie lambda zwróci `true`.

Wyrażenie lambda przechwytuje parametr `shoe_size` ze swojego otoczenia i porównuje tę wartość z rozmiarem każdego buta, zatrzymując tylko buty o określonym rozmiarze. Na końcu, wywołanie `collect` zbiera wartości zwrócone przez przekształcony iterator w wektor, który jest zwracany przez funkcję.

Test pokazuje, że kiedy wywołujemy `shoes_in_my_size`, otrzymujemy tylko buty o rozmiarze zgodnym z wartością, którą podaliśmy.