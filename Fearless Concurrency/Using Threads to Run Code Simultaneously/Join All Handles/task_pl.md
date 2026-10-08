## Dołącz wszystkie uchwyty

Jednym z wyzwań w aplikacjach wielowątkowych jest to, że główny wątek może zakończyć się przed ukończeniem wątków potomnych. Funkcja `thread::spawn` zwraca strukturę `JoinHandle`. Zbieraj uchwyty typu JoinHandle i zaczekaj, aż się zakończą.

<div class="hint">
Dodaj wartość, którą otrzymałeś z funkcji <code>thread::spawn</code>, do wektora <code>handles</code>.
</div>

<div class="hint">
Aby dołączyć wątek, musisz wywołać metodę <code>join</code> na odpowiadającym mu 
<code>JoinHandle</code>, a następnie użyć <code>unwrap</code> na wyniku.
</div>