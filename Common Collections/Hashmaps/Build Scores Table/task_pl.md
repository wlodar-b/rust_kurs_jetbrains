## Tworzenie tabeli wyników

Dane to lista wyników (jeden na linię) meczu piłkarskiego. Każda linia ma format:
```
<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>
```

Przykład: `England,France,4,2` (Anglia strzeliła 4 gole, Francja 2).

Musisz stworzyć tabelę wyników zawierającą nazwę drużyny, gole 
strzelone przez drużynę oraz gole przez nią stracone. Jednym ze sposobów na stworzenie 
tabeli wyników jest użycie Hashmapy. Rozwiązanie jest częściowo napisane 
z użyciem Hashmapy, dokończ je, aby przejść testy.

Spraw, abym zdał testy!

<div class="hint">
Użyj metod <code>entry()</code> i <code>or_insert()</code> z <code>HashMap</code>, aby wstawić wpisy odpowiadające każdej drużynie do tabeli wyników.

Dowiedz się więcej w [Książce](https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html#only-inserting-a-value-if-the-key-has-no-value).
</div>

<div class="hint">
Jeśli dla danego klucza istnieje już wpis, zwróconą wartość metody <code>entry()</code> można zaktualizować na podstawie istniejącej wartości.

Dowiedz się więcej w [Książce](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#updating-a-value-based-on-the-old-value).
</div>