## Zaawansowane błędy 2

To ćwiczenie demonstruje kilka podejść, które są przydatne przy tworzeniu własnych typów błędów, zwłaszcza aby inne fragmenty kodu mogły bardziej zrozumiale i efektywnie korzystać z niestandardowych typów błędów.

Spraw, aby ten kod się kompilował i aby testy przechodziły pomyślnie! Zapoznaj się z podpowiedziami, jeśli utkniesz.

Kroki:
1. Upewnij się, że `ParseClimateError` nadaje się do propagacji błędów w funkcji `main()`.
2. Uzupełnij niekompletną implementację `From` dla `ParseClimateError`.
3. Obsłuż brakujące przypadki błędów w implementacji `FromStr` dla `Climate` zgodnie z wytycznymi dotyczącymi parsowania poniżej.
4. Uzupełnij niekompletną implementację `Display` dla `ParseClimateError`.

Parser dla `Climate`:
1. Podziel ciąg wejściowy na 3 pola: `city`, `year`, `temp`.
2. Zwróć błąd, jeśli ciąg jest pusty lub zawiera nieprawidłową liczbę pól.
3. Zwróć błąd, jeśli nazwa miasta jest pusta.
4. Sparsuj rok jako `u32` i zwróć błąd, jeśli to się nie powiedzie.
5. Sparsuj temperaturę jako `f32` i zwróć błąd, jeśli to się nie powiedzie.
6. Zwróć wartość `Ok` zawierającą ukończoną wartość `Climate`.

Dodaj lub uzupełnij brakujące kroki. 
Nie zapomnij przeczytać komentarzy w kodzie.

<div class="hint">

Spójrzmy na plik `main.rs`. Typ wyniku funkcji `main` to `Result<(), Box<dyn Error>>`. Oznacza to, że możemy zwrócić wartość `()`, lub *dowolny błąd*. Dalej w funkcji `main` używamy propagacji błędu:

```rust
  "Hong Kong,1999,25.7".parse::<Climate>()?
```

Ten kod propaguje niestandardowy błąd `ParseClimateError`. Aby uczynić go odpowiednim do przekazywania jako *dowolny błąd*, musimy go uczynić `Error` w następujący sposób:
```rust
impl Error for ParseClimateError {}
```

`Error` (`std::error::Error`) to cecha (trait), która reprezentuje podstawowe oczekiwania wobec wartości błędów. Szczegóły dotyczące cech omówimy [później w tym kursie](course://Generic%20Types,%20Traits,%20and Lifetime/Traits/Traits).
</div>

<div class="hint">
Nie ma potrzeby implementowania żadnych metod w ramach implementacji <code>Error</code>. (Niektóre metody mają standardowe domyślne implementacje.)
</div>

<div class="hint">Zapoznaj się z testami, aby określić, które warianty błędów (i jakie teksty komunikatów o błędach) mają zostać wygenerowane dla określonych warunków błędów.</div>

<div class="hint">
Poniższe strony mogą być przydatnymi odniesieniami:
<a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/define_error_type.html">1</a>,
<a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/boxing_errors.html">2</a>, 
<a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/wrap_error.html">3</a>.
</div>

**Wyzwanie**: Jeden z testów oznaczony jest jako `#[ignore]`. Czy możesz uzupełnić brakujący kod, aby test przeszedł? Możesz skonsultować dokumentację standardowej biblioteki dotyczącej konkretnej cechy, aby uzyskać więcej wskazówek.