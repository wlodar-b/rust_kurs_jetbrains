### Adnotacje czasu życia w definicjach struktur

Do tej pory definiowaliśmy struktury przechowujące wyłącznie typy posiadające właściciela. Możliwe jest jednak, aby struktury przechowywały referencje, ale w takim przypadku musimy dodać adnotację czasu życia do każdej referencji w definicji struktury. Poniższy przykład przedstawia strukturę o nazwie `ImportantExcerpt`, która przechowuje fragment ciągu znaków (string slice).

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

#### Struktura przechowująca referencję, wymagająca adnotacji czasu życia

Ta struktura ma jedno pole, `part`, które przechowuje fragment ciągu znaków (string slice), będący referencją. Podobnie jak w przypadku generycznych typów danych, nazwę generycznego parametru czasu życia deklarujemy w nawiasach ostrych po nazwie struktury, abyśmy mogli użyć parametru czasu życia w definicji ciała struktury. Ta adnotacja oznacza, że instancja `ImportantExcerpt` nie może przetrwać dłużej niż referencja przechowywana w jej polu `part`.

Funkcja `main` w tym przypadku tworzy instancję struktury `ImportantExcerpt`, która przechowuje referencję do pierwszego zdania zawartego w obiekcie `String` należącym do zmiennej `novel`. Dane w `novel` istnieją zanim zostanie utworzona instancja `ImportantExcerpt`. Dodatkowo `novel` nie wychodzi poza swój zasięg przed zakończeniem zasięgu `ImportantExcerpt`, co sprawia, że referencja w instancji `ImportantExcerpt` jest poprawna.