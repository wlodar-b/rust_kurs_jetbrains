### Wzorce ogólne i symbol zastępczy `_`

Używając enumów, możemy także podejmować szczególne działania dla kilku konkretnych wartości, ale dla wszystkich innych przyjmować jedną domyślną akcję. Wyobraźmy sobie, że tworzymy grę, w której, jeśli wyrzucisz 3 w rzucie kostką, gracz nie porusza się, ale zamiast tego dostaje nowy fantazyjny kapelusz. Jeśli wyrzucisz 7, gracz traci fantazyjny kapelusz. Dla wszystkich innych wartości gracz przesuwa się o liczbę pól równą wyrzuconej wartości. Oto `match`, który implementuje tę logikę, z wynikiem rzutu kostką ustawionym na stałą wartość zamiast losowej, a cała pozostała logika jest przedstawiona jako funkcje bez ciał, ponieważ ich faktyczna implementacja wykracza poza zakres tego przykładu:

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
```

Dla pierwszych dwóch gałęzi, wzorcami są dosłowne wartości 3 i 7. W ostatniej gałęzi, która obejmuje każdą inną możliwą wartość, wzorcem jest zmienna nazwana `other`. Kod wykonywany dla gałęzi `other` korzysta z tej zmiennej, przekazując ją do funkcji `move_player`.

Ten kod się kompiluje, mimo że nie wymieniliśmy wszystkich możliwych wartości typu u8, ponieważ ostatni wzorzec dopasowuje wszystkie wartości, które nie zostały wcześniej wymienione. Ten wzorzec ogólny spełnia wymaganie, że `match` musi być wyczerpujący. Zwróć uwagę, że musimy umieścić gałąź ogólną na końcu, ponieważ wzorce są oceniane w kolejności. Rust ostrzeże nas, jeśli dodamy gałęzie po wzorcu ogólnym, ponieważ te późniejsze gałęzie nigdy się nie dopasują!

Rust ma także wzorzec, którego możemy używać, gdy nie chcemy korzystać z wartości w wzorcu ogólnym: `_`, który jest specjalnym wzorcem dopasowującym dowolną wartość i nie przypisuje jej do zmiennej. Informuje on Rust, że nie zamierzamy używać tej wartości, więc Rust nie ostrzeże nas o nieużywanej zmiennej.

Zmieńmy zasady gry tak, aby w przypadku wyrzucenia wartości innej niż 3 lub 7 należało rzucić kostką jeszcze raz. W takim przypadku nie musimy używać wartości, więc możemy zmienić kod, używając `_` zamiast zmiennej o nazwie `other`:

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
```

Ten przykład także spełnia wymóg wyczerpującego dopasowania, ponieważ jawnie ignorujemy wszystkie inne wartości w ostatniej gałęzi; niczego nie pomijamy.

Jeśli ponownie zmienimy zasady gry, tak aby poza wyrzuceniem 3 lub 7 nic innego nie działo się podczas tury gracza, możemy to wyrazić, używając wartości jednostkowej (pustego typu krotki, wspomnianego w sekcji „Typ krotki”) jako kodu towarzyszącego gałęzi `_`:

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```

W tym przypadku jawnie informujemy Rust, że nie zamierzamy korzystać z żadnej wartości, która nie pasuje do wzorca w wcześniejszych gałęziach, i nie chcemy wykonywać żadnego kodu w tej sytuacji.

Więcej o wzorcach i dopasowywaniu omówimy w [Rozdziale 18][ch18-00-patterns] Książki o Ruście. Na razie przejdziemy do składni `if let`, która może być przydatna w sytuacjach, gdy wyrażenie `match` jest nieco rozwlekłe.

[ch18-00-patterns]: https://github.com/rust-lang/book/blob/master/src/ch18-00-patterns.md