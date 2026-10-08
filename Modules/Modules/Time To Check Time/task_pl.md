## Czas na sprawdzenie czasu

Możesz użyć słowa kluczowego `use`, aby wprowadzić ścieżki modułów z dowolnych modułów, a szczególnie z biblioteki standardowej Rust, do swojego zakresu. Wprowadź `SystemTime` i `UNIX_EPOCH`  
z modułu `std::time`. Bonusowe punkty za styl, jeśli uda Ci się to zrobić w jednej linii!

Spraw, by kod się kompilował! 

<div class="hint">

`UNIX_EPOCH` i `SystemTime` są zadeklarowane w module `std::time`. Dodaj instrukcję `use`  
dla tych dwóch, aby wprowadzić je do zakresu. Możesz użyć zagnieżdżonych ścieżek lub operatora `glob`, aby wprowadzić je przy użyciu tylko jednej linii.

</div>