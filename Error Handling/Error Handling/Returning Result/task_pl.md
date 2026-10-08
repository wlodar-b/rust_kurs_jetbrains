## Zwracanie Result

Ta funkcja odmawia generowania tekstu, który ma być wydrukowany na identyfikatorze, jeśli przekażesz jej pusty ciąg znaków. Byłoby lepiej, gdyby wyjaśniała, na czym polega problem, zamiast po prostu czasami zwracać `None`.

<div class="hint">
Dlaczego nie użyć typu <code>Result</code> zamiast <code>Option</code>?
</div>

<div class="hint">
Aby wprowadzić tę zmianę, musisz:

- zaktualizować typ zwracany w sygnaturze funkcji na `Result<String, String>`, który  
  może przyjmować warianty `Ok(String)` i `Err(String)`
- zmienić ciało funkcji, aby zwracało `Ok(cośtam)` w miejscach, gdzie obecnie  
  zwraca `Some(cośtam)`
- zmienić ciało funkcji, aby zwracało `Err(komunikat błędu)` w miejscach, gdzie  
  obecnie zwraca `None`
</div>