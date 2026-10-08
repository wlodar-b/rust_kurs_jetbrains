## Liczenie postępu

Postęp w nauce języka Rust jest modelowany za pomocą mapy mieszającej (hash map). Nazwa zadania jest kluczem, a postęp wartością. Zostały utworzone dwie funkcje liczące, które zliczają liczbę zadań o danym poziomie postępu. Te funkcje liczące korzystają z imperatywnych pętli for. Odtwórz tę funkcjonalność liczenia przy użyciu iteratorów.

Upewnij się, że kod się kompiluje i testy przechodzą.

<div class="hint">Krok 1:
Dokumentacja dla cechy <code>std::iter::Iterator</code> zawiera wiele metod, które mogą być tutaj przydatne.
</div>

<div class="hint">
Krok 2:
Zwróć 0 z funkcji <code>count_stack</code>, aby kod się kompilował i można było przetestować funkcję count.
</div>

<div class="hint">Zmienna <code>stack</code> w funkcji <code>count_stack</code> to wycinek (slice) HashMap. Musi zostać
przekształcona w iterator, aby można było używać metod iteratorów.
</div>

<div class="hint">Metoda <code>fold</code> może być przydatna w funkcji <code>count_stack</code>.</div>