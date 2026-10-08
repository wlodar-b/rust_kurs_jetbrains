## Rozwiązywanie dzielenia

To jest większe ćwiczenie niż większość pozostałych!
Dasz radę!

Oto Twoja misja, jeśli zdecydujesz się ją przyjąć:

1. Uzupełnij funkcję `divide`, aby cztery pierwsze testy przechodziły pomyślnie.
2. Upewnij się, że pozostałe testy przechodzą, uzupełniając funkcje `result_with_list` oraz
`list_of_results`.

Zobacz rozdział [Iteratory](https://doc.rust-lang.org/stable/book/ch13-02-iterators.html) w książce Rust oraz [dokumentację dotyczącą iteratorów](https://doc.rust-lang.org/stable/std/iter/).

Przewiń w dół, aby znaleźć niewielką wskazówkę dotyczącą części 2, a jeszcze dalej, aby uzyskać większą wskazówkę.
Powodzenia :-)

<div class="hint">
  Funkcja <code>divide</code> musi zwrócić odpowiedni błąd, gdy dzielenie na równe części nie jest możliwe.
</div>

<div class="hint">
Zmienna <code>division_results</code> musi być zebrana w typ kolekcji.

Funkcja `result_with_list` musi zwrócić pojedynczy `Result`, gdzie przypadek sukcesu to wektor liczb całkowitych, a przypadek błędu to `DivisionError`.

Funkcja `list_of_results` musi zwrócić wektor wyników.
</div>