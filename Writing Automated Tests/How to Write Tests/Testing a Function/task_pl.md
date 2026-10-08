## Przetestuj funkcję

Ten test nie testuje naszej funkcji — zmień to tak, aby test działał poprawnie. 
Następnie napisz drugą funkcję testującą o nazwie `is_false_when_odd`, która sprawdza, czy otrzymujemy oczekiwany wynik, gdy wywołamy `is_even(5)`.

<div class="hint">
  Możesz wywołać funkcję bezpośrednio w miejscu, gdzie przekazujesz argumenty do <code>assert!</code> — więc możesz zrobić coś takiego:

```rust
assert!(having_fun());
```

  Jeśli chcesz sprawdzić, czy rzeczywiście zwracana jest wartość false, możesz zanegować wynik tego, co robisz, używając `!`, na przykład:
  
```rust
assert!(!having_fun());
```
</div>