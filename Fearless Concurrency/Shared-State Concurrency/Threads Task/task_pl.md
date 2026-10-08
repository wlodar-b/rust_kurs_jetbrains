## Zlicz ukończone zadania

Spraw, aby ten kod się kompilował!

Idea jest taka, że wątek uruchomiony w linii 12 wykonuje zadania, podczas gdy główny wątek monitoruje postęp, aż 10 zadań zostanie ukończonych. Ze względu na różnicę w czasie uśpienia między uruchomionymi wątkami a wątkami oczekującymi, jeśli zobaczysz 6 linii "czekam..." i program zakończy się bez przekroczenia limitu czasu w playground, zrobiłeś to poprawnie :)

<div class="hint">
  <code>Arc</code> to wskaźnik liczony atomowo, który umożliwia bezpieczny współdzielony dostęp do danych <b>niemutowalnych</b>. Ale w naszym przypadku chcemy zmieniać wartość <code>jobs_completed</code>, więc musimy dodatkowo użyć innego typu, który pozwoli tylko jednemu wątkowi na modyfikowanie danych w danym momencie.
  Spójrz na <a href ="https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html#atomic-reference-counting-with-arct">ten fragment książki</a>
  i przewiń dalej, jeśli chcesz więcej wskazówek :)
</div>

<div class="hint">
  Czy na początku funkcji main masz teraz <code>Arc</code> <code>Mutex</code> <code>JobStatus</code>? Na przykład:

  
  <code>let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));</code>
  
  Podobnie jak w kodzie z przykładu w książce, który pojawia się po tekście: "Możemy użyć Arc\<T\>, aby to naprawić.". Jeśli nie, spróbuj to zrobić!
  Jeśli tak i nadal chcesz więcej wskazówek, przewiń dalej!!
</div>

<div class="hint">
  Upewnij się, że żaden z twoich wątków nie trzyma blokady mutexa podczas uśpienia, ponieważ to uniemożliwi innemu wątkowi zdobycie tej blokady. Blokady są automatycznie zwalniane, gdy wychodzą poza zakres.
</div>

<div class="hint">
  Dobra, szczerze mówiąc, to było dla <i>mnie</i> również trudne do zrobienia.
  I mogłem zauważyć wiele różnych problemów, na które możesz się natknąć, więc na tym etapie nie jestem pewien, z którym się zmagasz :)
  Zobacz kilka możliwych <a href="https://github.com/carols10cents/rustlings/issues/3 ">odpowiedzi</a> -- moja jest trochę bardziej skomplikowana, ponieważ zdecydowałem, że chcę zobaczyć liczbę zadań, które zostały aktualnie wykonane, podczas gdy sprawdzam status.

  Otwórz zgłoszenie, jeśli nadal napotykasz problem, z którym te wskazówki ci nie pomagają, lub jeśli przejrzałeś przykładowe odpowiedzi i nie rozumiesz, dlaczego one działają, a twoje nie.

  Jeśli nauczyłeś się czegoś z przykładowych rozwiązań, zachęcam cię, abyś wrócił do tego ćwiczenia za kilka dni i spróbował ponownie, aby wzmocnić zdobyte umiejętności :)
</div>