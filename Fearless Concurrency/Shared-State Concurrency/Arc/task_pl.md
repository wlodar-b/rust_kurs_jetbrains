## Użyj `Arc`

Spraw, by ten kod się skompilował, uzupełniając wartość dla `shared_numbers` i tworząc początkowe wiązanie dla `child_numbers` w odpowiednim miejscu. 
Postaraj się nie tworzyć żadnych kopii wektora `numbers`!

Zapoznaj się z rozdziałem [Współbieżność ze współdzielonym stanem](https://doc.rust-lang.org/book/2018-edition/ch16-03-shared-state.html) z Książki Rust.

<div class="hint">

  Uczyń `shared_numbers` typem `Arc` z wektora liczb.
  Następnie, aby uniknąć tworzenia kopii `numbers`, musisz utworzyć `child_numbers` wewnątrz pętli, ale nadal w głównym wątku.

  `child_numbers` powinno być klonem obiektu Arc reprezentującego `numbers`, zamiast lokalną dla wątku kopią `numbers`.
</div>

<div class="hint">To proste ćwiczenie, jeśli rozumiesz podstawowe pojęcia, ale jeśli jest ono zbyt trudne, rozważ przeczytanie (lub ukończenie) sekcji "Bezpieczna współbieżność" najpierw. Alternatywnie, przeczytaj cały <a href="https://doc.rust-lang.org/stable/book/ch16-00-concurrency.html">Rozdział 16</a> w Książce.
</div>