## Zwracanie wartości

Spraw, by kod się skompilował!

<div class="hint">
  To bardzo powszechny błąd, który można naprawić usuwając jeden znak.

  Dzieje się tak, ponieważ Rust rozróżnia wyrażenia i instrukcje: wyrażenia zwracają wartość na podstawie swojego operandum, a instrukcje zwracają po prostu typ `()`, który działa podobnie jak `void` w językach C/C++.

  Chcemy zwrócić wartość typu `i32` z funkcji `square`, ale obecnie zwraca ona typ `()`...

  To nie to samo. Istnieją dwa rozwiązania:
  1. Dodaj `return` przed `num * num;`
  2. usuń `;`, aby uzyskać `num * num`
</div>