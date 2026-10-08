## Implementacja TryFrom

`TryFrom` to prosty i bezpieczny sposób konwersji typów, który może zakończyć się niepowodzeniem w kontrolowany sposób w pewnych okolicznościach. 
Zasadniczo jest to to samo co `From`. Główna różnica polega na tym, że w tym przypadku powinno być zwracane `Result`, 
zamiast docelowego typu jako takiego.
Więcej informacji można znaleźć pod adresem: https://doc.rust-lang.org/std/convert/trait.TryFrom.html

Twoim zadaniem jest tutaj dokończenie tej implementacji 
i zwrócenie wyniku `Ok` z wewnętrznym typem `Color`.
Musisz utworzyć implementację dla krotki składającej się z trzech liczb całkowitych, 
tablicy trzech liczb całkowitych oraz wycinka (slice) liczb całkowitych.

Zwróć uwagę, że implementacje dla krotki i tablicy zostaną sprawdzone podczas kompilacji, 
ale implementacja dla wycinka wymaga sprawdzenia jego długości! 
Zauważ również, że poprawne wartości RGB muszą być liczbami całkowitymi w przedziale 0..=255.

<div class="hint">Postępuj zgodnie z krokami przewidzianymi w zadaniu.
Możesz także skorzystać z tego <a href="https://doc.rust-lang.org/std/convert/trait.TryFrom.html">przykładu</a>.</div>

<div class="hint">Podpowiedź: Czy istnieje implementacja <code>TryFrom</code> w standardowej bibliotece, 
która może zarówno przeprowadzić wymaganą konwersję liczby całkowitej, jak i sprawdzić zakres wejścia?</div>

<div class="hint">Spójrz na przypadki testowe, aby zobaczyć, które warianty błędów należy zwrócić.</div>

<div class="hint">Możesz użyć metod <code>map_err</code> lub <code>or</code> w `Result`, aby 
przekształcić błędy.</div>

<div class="hint">Jeśli chciałbyś propagować błędy za pomocą operatora <code>?</code> w swoim rozwiązaniu, 
możesz chcieć spojrzeć na ten <a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/reenter_question_mark.html">artykuł</a>.</div>

**Wyzwanie**: Czy potrafisz sprawić, aby implementacje `TryFrom` były generyczne dla wielu typów liczbowych?