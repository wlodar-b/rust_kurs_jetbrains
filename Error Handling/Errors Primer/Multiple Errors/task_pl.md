## Wiele błędów

Ten program wykorzystuje ukończoną wersję kodu z poprzedniego zadania.  
Teraz nie będzie się kompilował! Dlaczego? Skorzystaj z podpowiedzi, jeśli utkniesz.

<div class="hint">
W funkcji <code>main()</code> występują dwa różne możliwe typy <code>Result</code>, które są propagowane za pomocą operatorów <code>?</code>. Jak zadeklarować typ zwracany z funkcji <code>main()</code>, aby obsługiwał oba?</div>

<div class="hint">Kolejna podpowiedź: w tle operator <code>?</code> wywołuje <code>From::from</code> na wartości błędu, aby przekonwertować ją na obiekt typu boxed trait, czyli <code>Box&lt;dyn error::Error></code>, który jest polimorficzny &mdash; oznacza to, że z tej samej funkcji może być zwróconych wiele różnych rodzajów błędów, ponieważ wszystkie błędy działają w ten sam sposób, implementując trait <code>error::Error</code>.  
Zapoznaj się z sekcją "Skrót do propagowania błędów: operator ?" w tym <a href="course://Error Handling/Error Handling/Propagating Errors Limitations">zadaniu</a>.
</div>

<div class="hint">To ćwiczenie wykorzystuje niektóre koncepcje, do których dojdziemy dopiero później w kursie, takie jak <code>Box</code> i trait <code>From</code>. W tej chwili nie jest istotne, aby rozumieć je szczegółowo, ale możesz przeczytać o nich wcześniej, jeśli masz ochotę.  
Więcej na temat opakowywania błędów:  
<a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/boxing_errors.html">tutaj</a>.</div>