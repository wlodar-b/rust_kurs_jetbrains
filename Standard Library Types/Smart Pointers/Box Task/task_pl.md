## Użycie typu Box

Podczas kompilacji Rust musi wiedzieć, ile miejsca zajmuje dany typ. Staje się to problematyczne 
dla typów rekurencyjnych, gdzie wartość może zawierać w sobie inną wartość tego samego typu. 
Aby obejść ten problem, możemy użyć `Box` - inteligentnego wskaźnika, który przechowuje dane na stercie 
i pozwala również na opakowanie typu rekurencyjnego.

Typem rekurencyjnym, który implementujemy w tym ćwiczeniu, jest `lista kons` - struktura danych 
często występująca w językach programowania funkcyjnego. Każdy element listy kons zawiera dwa elementy: 
wartość bieżącego elementu oraz kolejny element. Ostatni element to wartość o nazwie `Nil`.

**Krok 1**: użyj `Box` w definicji enumeracji, aby kod mógł się skompilować

**Krok 2**: utwórz zarówno puste, jak i niepuste listy kons, zastępując `unimplemented!()`

Sprawdź podpowiedzi, jeśli utkniesz! :) 

<div class="hint">Krok 1:
Komunikat kompilatora powinien pomóc: ponieważ nie możemy przechowywać wartości faktycznego typu 
podczas pracy z typami rekurencyjnymi, musimy przechować odwołanie (wskaźnik) do jego wartości. 
Dlatego powinniśmy umieścić naszą <code>List</code> wewnątrz <code>Box</code>. Więcej szczegółów 
znajdziesz w książce tutaj: 
<a href="https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes">https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes</a>.
</div>

<div class="hint">
Krok 2:
Utworzenie pustej listy powinno być dość proste.
Dla listy niepustej pamiętaj, że chcemy użyć naszej "konstruktora list" Cons. 
Chociaż obecna lista dotyczy liczb całkowitych (i32), możesz zmienić definicję 
i spróbować użyć innych typów!</div>