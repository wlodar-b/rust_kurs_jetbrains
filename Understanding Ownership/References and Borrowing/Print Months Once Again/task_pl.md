## Wyświetlanie miesięcy jeszcze raz

Dzieje się tutaj coś dziwnego. Program kompiluje się bez żadnych problemów. Mamy `fn print_months`, która pożycza tablicę i wypisuje jej zawartość. To w porządku. A co z `fn print_months_reversed`? Czy nie przejmuje ona własności tablicy `months`? Dlaczego więc możemy ponownie użyć tej samej tablicy `months` po jej wywołaniu? Otóż, własność tutaj nie jest przenoszona. Wartość tablicy jest kopiowana. Jeśli uruchomisz program, zobaczysz to od razu.

Zrefaktoruj ten program, aby uniknąć kopiowania tablicy, poprzez zastosowanie zmiennej referencji w `fn print_months_reversed` i napraw błędy kompilatora.

<div class="hint">
Pierwszym krokiem będzie zmiana typu argumentu w <code>fn print_months_reversed</code>.
</div>

<div class="hint">
Jak powinniśmy wywołać funkcję, która pożycza swój argument w sposób zmienny?
</div>