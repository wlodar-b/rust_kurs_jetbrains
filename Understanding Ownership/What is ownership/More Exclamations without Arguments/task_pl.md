## Więcej okrzyków bez argumentów

Zrefaktoryzuj ten kod tak, aby zamiast tworzyć ciąg znaków `Hello` w `fn main`, utworzyć go wewnątrz `fn with_exclamation` i przenieść świeżo utworzony ciąg znaków z `fn with_exclamation` do funkcji, która go wywołała.

<div class="hint">
  Przestań czytać w momencie, kiedy uznasz, że masz wystarczające wskazówki :)
  Albo spróbuj wykonać jeden krok i naprawić błędy kompilatora, które się pojawią!

Procedura przedstawia się następująco:
- usuń pierwszy wiersz w `main`, który tworzy nowy ciąg znaków
- ponieważ `Hello` teraz nie istnieje, nie możemy go przekazać do `with_exclamation`
- ponieważ nie chcemy niczego przekazywać do `with_exclamation`, jego sygnatura powinna odzwierciedlać, że nie przyjmuje żadnych argumentów
- ponieważ nie tworzymy już nowego ciągu znaków w `main`, musimy utworzyć nowy ciąg w `with_exclamation` w sposób podobny do tego, jak robiliśmy to w `main`
</div>