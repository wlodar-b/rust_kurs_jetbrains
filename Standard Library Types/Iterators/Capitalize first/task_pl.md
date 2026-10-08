## Capitalize first

W tym module nauczysz się niektórych unikalnych zalet, jakie oferują iteratory.

Krok 1. Uzupełnij funkcję `capitalize_first`, aby przejść dwa pierwsze testy. Powinna zwracać "Hello", gdy wejściem jest "hello" oraz pusty ciąg znaków, gdy wejściem jest pusty ciąg znaków.

Krok 2. Zastosuj funkcję `capitalize_first` do wycinka ciągów znaków.
Zwróć wektor napisów.
`["hello", "world"]` -> `["Hello", "World"]`

Krok 3. Ponownie zastosuj funkcję `capitalize_first` do wycinka ciągów znaków.
Zwróć jeden ciąg znaków.
`["hello", " ", "world"]` -> `"Hello World"`

Jak zawsze, dostępne są wskazówki!

<div class="hint">
Zmienna <code>first</code> jest typu <code>char</code>. Musi zostać zamieniona na wielką literę i dodana do 
pozostałych znaków w <code>c</code>, aby zwrócić poprawny <code>String</code>. 
Pozostałe znaki w <code>c</code> można potraktować jako wycinek ciągu znaków, używając metody 
<code>as_str</code>.
Dokumentacja dla <code>char</code> zawiera wiele przydatnych metod.
<a href="https://doc.rust-lang.org/std/primitive.char.html">https://doc.rust-lang.org/std/primitive.char.html</a>.</div>

<div class="hint">Utwórz iterator z wycinka. Przekształć iterowane wartości, stosując funkcję 
<code>capitalize_first</code>. Pamiętaj, aby zebrać iterator.</div>

<div class="hint">To jest zaskakująco podobne do poprzedniego rozwiązania. Collect jest bardzo potężny 
i bardzo uniwersalny. Rust po prostu musi wiedzieć, jaki typ jest wymagany.</div>