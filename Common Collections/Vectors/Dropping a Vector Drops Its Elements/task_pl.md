### Upuszczenie wektora upuszcza jego elementy

Podobnie jak w przypadku każdej innej `struct`, wektor jest zwalniany, gdy wychodzi poza zakres, co zostało zilustrowane poniżej.

```rust
    {
        let v = vec![1, 2, 3, 4];

        // wykonaj operacje na v
    } // <- v wychodzi poza zakres i zostaje tutaj zwolniony
```

#### Pokazywanie, gdzie wektor i jego elementy są zwalniane

Kiedy wektor jest zwalniany, wszystkie jego zawartości również są zwalniane, co oznacza, że przechowywane w nim liczby całkowite zostaną usunięte. Może się to wydawać prostą kwestią, ale sprawa staje się bardziej skomplikowana, gdy zacznie się wprowadzać odwołania do elementów wektora. Przyjrzyjmy się temu bliżej w następnej części!