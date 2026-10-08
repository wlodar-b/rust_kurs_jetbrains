## Opanowywanie IDE: zarządzanie warunkami

Czy potrafisz szybko zrozumieć, co wyświetla kod z pliku `main.rs`?

%IDE_NAME% oferuje wiele przydatnych funkcji, które mogą pomóc Ci uczynić kod bardziej czytelnym.

Na przykład zastosujmy [prawo De Morgana](https://pl.wikipedia.org/wiki/Prawa_De_Morgana) za pomocą opcji **szybkiej poprawki**.

Umieść kursor na `&&` i użyj skrótu &shortcut:ShowIntentionActions;, aby zobaczyć dostępne akcje kontekstowe.

![](image.png)

Czy teraz potrafisz szybko zrozumieć, co wyświetla kod z pliku `main.rs`?

<div class="hint">
  Wynikowy warunek `if` powinien wyglądać następująco: <br>
  <code>(number > 4 && number <= 9) || (number > 0 && number < 10)</code>
</div>

<div class="hint">
  Kod wyświetla "If Branch"
</div>

*Uwaga 1*: `&&` i `||` to [operatorzy logiczni z krótkim obwodem](https://pl.wikipedia.org/wiki/Ewaluacja_z_kr%C3%B3tkim_obwodem).

*Uwaga 2*: Spróbuj użyć &shortcut:ShowIntentionActions; w różnych miejscach w swoim kodzie z poprzednich zadań. %IDE_NAME% często proponuje użyteczne sugestie poprawiające Twój kod.