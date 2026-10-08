## Dopasowywanie wyniku

Załóżmy, że piszemy grę, w której możesz kupować przedmioty za tokeny.  
Wszystkie przedmioty kosztują 5 tokenów, a za każdym razem, gdy kupujesz przedmioty, naliczana jest opłata manipulacyjna w wysokości 1 tokena.  
Gracz gry wpisze, ile przedmiotów chce kupić, a funkcja `total_cost` obliczy całkowitą liczbę tokenów.  
Jednakże, ponieważ gracz wpisuje ilość, otrzymujemy ją jako ciąg znaków (string) – i może wpisać cokolwiek, a nie tylko liczby!

W tej chwili funkcja ta w ogóle nie obsługuje przypadków błędu (i również nie radzi sobie właściwie z przypadkami sukcesu).  
Chcemy zrobić tak: jeśli wywołamy funkcję `parse` na ciągu znaków, który nie jest liczbą, funkcja ta zwróci `ParseIntError`, a w takim przypadku chcemy natychmiast zwrócić ten błąd z naszej funkcji i nie próbować wykonywać mnożenia i dodawania.

Istnieją co najmniej dwa sposoby na zaimplementowanie tego, które są oba poprawne – jednak jeden jest znacznie krótszy!  
Przewiń w dół, aby zobaczyć wskazówki dla obu podejść.

<div class="hint">
  Jednym ze sposobów obsługi tego jest użycie instrukcji <code>match</code> na <code>item_quantity.parse::&lt;i32&gt()</code>, gdzie przypadki to <code>Ok(something)</code> i <code>Err(something)</code>.  
  Ten wzorzec jest bardzo powszechny w Ruście, dlatego istnieje operator <code>?</code>, który robi właściwie to, co zrobiłbyś za pomocą tej instrukcji match!  
  Rzuć okiem na <a href="https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator">tę sekcję w rozdziale o obsłudze błędów</a> i spróbuj z niego skorzystać!
</div>