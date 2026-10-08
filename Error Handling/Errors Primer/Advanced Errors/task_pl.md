## Zaawansowane błędy

Pamiętasz, jak w poprzednim ćwiczeniu mieliśmy wiele funkcji mapujących, aby przekształcać błędy niższego poziomu w nasz własny typ błędu za pomocą `map_err()`? A co, jeśli moglibyśmy używać bezpośrednio operatora `?`?

Spraw, aby ten kod się skompilował! Uzupełnij kod w taki sposób, aby w każdej z przypadków w `main()` został zwrócony odpowiedni błąd. Zwróć uwagę na komentarze i zapoznaj się ze wskazówkami, jeśli utkniesz.

<div class="hint">
Kod parsujący znajduje się teraz w implementacji cechy <code>FromStr</code>. Zauważ, że kod parsujący korzysta bezpośrednio z <code>?</code>, bez żadnych wywołań <code>map_err()</code>. Jest jeden częściowy przykład implementacji cechy <code>From</code>, który powinieneś uzupełnić.
</div>

<div class="hint">
Szczegóły: Operator <code>?</code> wywołuje <code>From::from()</code> na typie błędu, aby przekonwertować go na typ błędu zwracany przez otaczającą funkcję.
</div>
<div class="hint">
Będziesz musiał napisać kolejną implementację <code>From</code>, która przyjmuje inny typ wejściowy.
</div>