### Składnia adnotacji czasów życia

Adnotacje czasów życia nie zmieniają, jak długo żyją dane referencje. Podobnie jak funkcje mogą akceptować dowolny typ, gdy w ich sygnaturze określony jest parametr typu generycznego, tak samo funkcje mogą akceptować referencje z dowolnym czasem życia, określając generyczny parametr czasu życia. Adnotacje czasów życia opisują relacje między czasami życia wielu referencji, nie wpływając na same czasy życia.

Adnotacje czasów życia mają nieco nietypową składnię: nazwy parametrów czasów życia muszą zaczynać się od apostrofu (`'`), są zwykle zapisane małymi literami i są bardzo krótkie, podobnie jak typy generyczne. Większość osób używa nazwy `'a`. Adnotacje parametrów czasów życia umieszczamy po `&` referencji, stosując spację do oddzielenia adnotacji od typu referencji.

Oto kilka przykładów: referencja do `i32` bez parametru czasu życia, referencja do `i32` z parametrem czasu życia o nazwie `'a` oraz mutowalna referencja do `i32` również z czasem życia `'a`.

```rust,ignore
&i32        // referencja
&'a i32     // referencja z jawnie określonym czasem życia
&'a mut i32 // mutowalna referencja z jawnie określonym czasem życia
```

Jedna adnotacja czasu życia sama w sobie nie ma dużego znaczenia, ponieważ adnotacje służą do informowania Rust o relacjach między generycznymi parametrami czasów życia wielu referencji. Na przykład załóżmy, że mamy funkcję z parametrem `first`, który jest referencją do `i32` z czasem życia `'a`. Funkcja ma również inny parametr o nazwie `second`, który jest kolejną referencją do `i32` także z czasem życia `'a`. Adnotacje czasów życia wskazują, że referencje `first` i `second` muszą żyć tak długo, jak ten generyczny czas życia.