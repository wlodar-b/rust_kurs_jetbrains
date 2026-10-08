## Niestandardowe błędy

Używanie ogólnych typów błędów, takich jak `Box<dyn error::Error>`, nie jest zalecane w bibliotekach, gdzie użytkownicy mogą chcieć podejmować decyzje na podstawie zawartości błędu, zamiast tylko go wypisywać lub propagować dalej. Tutaj definiujemy niestandardowy typ błędu, aby umożliwić użytkownikom decydowanie, co zrobić dalej, gdy nasza funkcja zwróci błąd.

Zrób, aby testy przechodziły poprawnie! Przeczytaj wskazówki, jeśli utkniesz.

<div class="hint">To zadanie wykorzystuje gotową wersję <code>PositiveNonzeroInteger</code> z
<a href="course://Error Handling/Errors Primer/Positive Nonzero Integer">poprzedniego zadania</a>.</div>

<div class="hint">Możesz stworzyć kolejną funkcję wewnątrz <code>impl ParsePosNonzeroError</code>.

Na przykład, warto byłoby stworzyć `ParsePosNonzeroError` bazując na `ParseIntError`, który może być
zwracany przez funkcję `parse`.
</div>

<div class="hint">Poniżej linii, którą należy zmienić według komentarza, znajduje się przykład użycia
metody <code>map_err()</code> na obiekcie <code>Result</code>, aby przekształcić jeden typ błędu w
inny. Spróbuj użyć czegoś podobnego na <code>Result</code> z funkcji <code>parse()</code>. Możesz
użyć operatora <code>?</code>, aby wcześnie wyjść z funkcji, lub użyć wyrażenia <code>match</code>,
albo może znajdzie się jakiś inny sposób!</div>

<div class="hint">Przeczytaj więcej o <code>map_err()</code> w dokumentacji <code>std::result</code> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err">tutaj</a>.
</div>