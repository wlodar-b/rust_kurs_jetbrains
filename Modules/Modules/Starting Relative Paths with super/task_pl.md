### Rozpoczynanie ścieżek względnych za pomocą `super`

Możemy tworzyć ścieżki względne, które rozpoczynają się w nadrzędnym module, zamiast w bieżącym module lub w korzeniu pakietu (crate), używając `super` na początku ścieżki. Jest to podobne do rozpoczynania ścieżki systemu plików składnią `..`. Użycie `super` pozwala odwoływać się do elementu, o którym wiemy, że znajduje się w nadrzędnym module, co może ułatwić reorganizację drzewa modułów, gdy moduł jest ściśle powiązany z nadrzędnym, ale nadrzędny może być w przyszłości przeniesiony w inne miejsce w drzewie modułów.

Rozważmy poniższy kod, który modeluje sytuację, w której szef kuchni naprawia błędne zamówienie i osobiście zanosi je do klienta. Funkcja `fix_incorrect_order` zdefiniowana w module `back_of_house` wywołuje funkcję `deliver_order`, zdefiniowaną w nadrzędnym module, określając ścieżkę do `deliver_order`, rozpoczynając ją od `super`:

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

##### Wywoływanie funkcji za pomocą ścieżki względnej rozpoczynającej się od `super`

Funkcja `fix_incorrect_order` znajduje się w module `back_of_house`, więc możemy użyć `super`, aby przejść do nadrzędnego modułu `back_of_house`, którym w tym przypadku jest `crate`, czyli korzeń. Stamtąd szukamy funkcji `deliver_order` i znajdujemy ją. Sukces! Uważamy, że moduł `back_of_house` oraz funkcja `deliver_order` prawdopodobnie pozostaną w takim samym związku względem siebie i zostaną przeniesione razem, jeśli zdecydujemy się zreorganizować drzewo modułów pakietu. Dlatego użyliśmy `super`, aby w przyszłości było mniej miejsc do aktualizacji kodu, jeśli ten fragment zostanie przeniesiony do innego modułu.