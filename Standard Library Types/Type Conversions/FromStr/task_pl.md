## Implementacja FromStr

To działa praktycznie tak samo, jak `TryFrom<&str>`.
Dodatkowo, po zaimplementowaniu `FromStr`, możesz użyć metody `parse`
na ciągach znaków, aby wygenerować obiekt typu implementującego.
Więcej informacji znajdziesz na https://doc.rust-lang.org/std/str/trait.FromStr.html

Kroki:
1. Jeśli długość podanego ciągu znaków wynosi 0, zwróć błąd.
2. Podziel podany ciąg znaków na części oddzielone przecinkami.
3. Operacja podziału powinna zwrócić dokładnie 2 elementy, w przeciwnym razie zwróć błąd.
4. Wyciągnij pierwszy element z operacji podziału i użyj go jako imię.
5. Wyciągnij drugi element z operacji podziału i przeparsuj go do `usize` jako wiek,
korzystając z czegoś takiego jak `"4".parse::<usize>()`.
6. Jeśli podczas parsowania wieku wystąpi błąd, powinien zostać zwrócony błąd.
   Jeśli wszystko pójdzie dobrze, zwróć obiekt typu `Result` z obiektem `Person`.

<div class="hint">Implementacja <code>FromStr</code> powinna zwrócić <code>Ok</code> z obiektem <code>Person</code>,
lub <code>Err</code> z ciągiem znaków, jeśli ciąg znaków jest nieprawidłowy. </div>

<div class="hint">To jest prawie jak ćwiczenie from_into, ale zamiast zwracania wartości domyślnej,
zwracane są błędy.</div>

<div class="hint">Wskazówka: Spójrz na przypadki testowe, aby zobaczyć, jakie warianty błędów należy zwrócić.</div>

<div class="hint">Inna wskazówka: Możesz użyć metody <code>map_err</code> z rodzaju <code>Result</code> z funkcją
lub zamknięciem, aby opakować błąd z <code>parse::<usize></code>.</div>

<div class="hint">Jeszcze jedna wskazówka: Jeśli chciałbyś propagować błędy za pomocą operatora <code>?</code>
w swoim rozwiązaniu, możesz sprawdzić <a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/reenter_question_mark.html">ten</a> artykuł.</div>