### Tworzenie kanału

Najpierw, w poniższym przykładzie, utworzymy kanał, ale nic z nim jeszcze nie zrobimy. Zwróć uwagę, że ten kod nie będzie się kompilował, ponieważ Rust nie wie, jakiego typu wartości chcemy przesyłać przez kanał.

```rust
    use std::sync::mpsc;

    fn main() {
        let (tx, rx) = mpsc::channel();
    }
```

##### Tworzenie kanału i przypisywanie obu części do zmiennych tx i rx

Tworzymy nowy kanał za pomocą funkcji `mpsc::channel`; `mpsc` oznacza _wielu producentów, jeden konsument_. W skrócie, sposób implementacji kanałów w standardowej bibliotece Rusta pozwala na posiadanie wielu punktów _wysyłających_, które produkują wartości, ale tylko jednego punktu _odbierającego_, który je konsumuje. Możesz to sobie wyobrazić jako wiele małych strumieni łączących się w jedną dużą rzekę: wszystko, co zostanie przesłane którymkolwiek ze strumieni, ostatecznie dotrze do tej jednej rzeki. Na razie zaczniemy od jednego producenta, ale dodamy kilku producentów, gdy ten przykład zacznie działać.

Funkcja `mpsc::channel` zwraca krotkę, której pierwszym elementem jest koniec wysyłający, a drugim elementem jest koniec odbierający. Skróty `tx` i `rx` są tradycyjnie używane w wielu dziedzinach dla _nadajnika_ i _odbiornika_, dlatego nazwaliśmy nasze zmienne w taki sposób, aby wskazywały na każdy z końców. Używamy instrukcji `let` z wzorcem, który rozbija krotkę na jej składniki; omówimy użycie wzorców w instrukcjach `let` i destrukturyzację w rozdziale 18. Użycie instrukcji `let` w ten sposób to wygodne podejście do wyodrębnienia elementów krotki zwracanej przez `mpsc::channel`.