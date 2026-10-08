## From Into

Trait `From` jest używany do konwersji wartości na inną wartość.  
Jeśli `From` jest poprawnie zaimplementowany dla danego typu, to `Into` powinien działać odwrotnie.  
Możesz przeczytać więcej o tym na stronie: https://doc.rust-lang.org/std/convert/trait.From.html  

Implementujemy trait `Default`, aby użyć go jako rozwiązania zapasowego,  
kiedy podany ciąg znaków nie może zostać skonwertowany na obiekt typu `Person`.  
Twoim zadaniem jest dokończenie tej implementacji w taki sposób,  
aby linia `let p = Person::from("Mark,20")` mogła się kompilować.  
Pamiętaj, że musisz sparsować komponent wieku do wartości `usize`  
używając czegoś w rodzaju `"4".parse::<usize>()`. Wynik tego działania musi być odpowiednio obsłużony.  

Kroki:  
1. Jeśli długość podanego ciągu znaków wynosi 0, zwróć wartość domyślną `Person`.  
2. Podziel dany ciąg znaków, korzystając z przecinków jako separatorów.  
3. Wyciągnij pierwszy element z wyniku operacji podziału i użyj go jako imienia.  
4. Jeśli imię jest puste, zwróć wartość domyślną `Person`.  
5. Wyciągnij drugi element z wyniku operacji podziału i sparsuj go  
   jako `usize`, aby uzyskać wiek. Jeśli podczas parsowania wieku wystąpi błąd,  
   zwróć wartość domyślną `Person`. W przeciwnym razie zwróć zainicjalizowany obiekt  
   `Person` na podstawie uzyskanych wyników.  

<div class="hint">Postępuj zgodnie z krokami opisanymi tuż przed implementacją <code>TryFrom</code>.  
Możesz również skorzystać z tego <a href="https://doc.rust-lang.org/std/convert/trait.TryFrom.html">przykładu</a>.</div>