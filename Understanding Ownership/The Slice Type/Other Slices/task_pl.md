## Inne wycinki

Wycinki ciągów znaków, jak można się domyślić, są specyficzne dla ciągów znaków. Istnieje jednak również bardziej ogólny typ wycinka. Rozważmy tę tablicę:

```rust
    let a = [1, 2, 3, 4, 5];
```

Tak jak możemy chcieć odwołać się do części ciągu znaków, możemy również chcieć odwołać się do części tablicy. Możemy to zrobić w ten sposób:

```rust
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
```

Ten wycinek ma typ `&[i32]`. Działa on w taki sam sposób, jak wycinki ciągów znaków, przechowując odniesienie do pierwszego elementu i długość. Użyjesz tego rodzaju wycinka dla różnych innych kolekcji. Omówimy te kolekcje szczegółowo [później](course://Common Collections/Vectors/Intro).