### Traits jako parametry

Teraz, gdy wiesz, jak definiować i implementować cechy (traits), możemy zgłębić, jak wykorzystać cechy do definiowania funkcji, które akceptują wiele różnych typów.

Na przykład, w drugim przykładzie w tej sekcji zaimplementowaliśmy cechę `Summary` dla typów `NewsArticle` i `Tweet`. Możemy zdefiniować funkcję `notify`, która wywołuje metodę `summarize` na swoim parametrze `item`, który jest pewnym typem implementującym cechę `Summary`. Aby to zrobić, możemy użyć składni `impl Trait`, jak poniżej:

```rust,ignore
pub fn notify(item: &impl Summary) {
    println!("Przełomowa wiadomość! {}", item.summarize());
}
```

Zamiast konkretnego typu dla parametru `item`, określamy słowo kluczowe `impl` oraz nazwę cechy. Ten parametr akceptuje każdy typ, który implementuje daną cechę. W ciele funkcji `notify` możemy wywołać dowolne metody na `item`, które pochodzą z cechy `Summary`, takie jak `summarize`. Możemy wywołać `notify` i przekazać dowolną instancję `NewsArticle` lub `Tweet`. Kod, który wywołuje tę funkcję z innym typem, takim jak `String` lub `i32`, nie skompiluje się, ponieważ te typy nie implementują `Summary`.

#### Składnia ograniczenia cechy (Trait Bound Syntax)

Składnia `impl Trait` działa w prostych przypadkach, ale jest faktycznie uproszczoną formą dłuższej składni nazywanej *trait bound*; wygląda ona tak:

```rust,ignore
pub fn notify<T: Summary>(item: &T) {
    println!("Przełomowa wiadomość! {}", item.summarize());
}
```

Ta dłuższa forma jest równoważna przykładom z poprzedniej sekcji, ale jest bardziej szczegółowa. Ograniczenia cechy umieszczamy przy deklaracji ogólnego parametru typu po dwukropku i wewnątrz nawiasów ostrokątnych.

Składnia `impl Trait` jest wygodna i pozwala na bardziej zwięzły kod w prostych przypadkach. Składnia ograniczeń cechy może natomiast wyrażać większą złożoność w innych sytuacjach. Na przykład możemy mieć dwa parametry, które implementują `Summary`. Korzystając ze składni `impl Trait`, wyglądałoby to tak:

```rust,ignore
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

Jeśli chcielibyśmy, aby ta funkcja pozwalała parametrom `item1` i `item2` mieć różne typy, użycie `impl Trait` byłoby odpowiednie (o ile oba typy implementują `Summary`). Jeśli natomiast chcielibyśmy wymusić na obu parametrach, aby miały ten sam typ, można to wyrazić tylko za pomocą ograniczenia cechy, jak poniżej:

```rust,ignore
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

Ogólny typ `T` określony jako typ parametrów `item1` i `item2` ogranicza funkcję w taki sposób, że konkretny typ wartości przekazanych jako argumenty dla `item1` i `item2` musi być taki sam.

#### Określanie wielu ograniczeń cech przy użyciu składni `+`

Możemy również określać więcej niż jedno ograniczenie cechy. Załóżmy, że chcielibyśmy, aby funkcja `notify` używała formatowania wyświetlania (`Display`) na `item`, a także metody `summarize`; możemy to określić w definicji `notify`, wymagając zaimplementowania zarówno `Display`, jak i `Summary`. Możemy to zrobić, używając składni `+`:

```rust,ignore
pub fn notify(item: &(impl Summary + Display)) {
```

Składnia `+` jest również prawidłowa w przypadku ograniczeń cech dla ogólnych typów:

```rust,ignore
pub fn notify<T: Summary + Display>(item: &T) {
```

Przy określeniu obu ograniczeń cech, ciało funkcji `notify` może wywoływać metodę `summarize` i używać `{}` do formatowania `item`.

#### Czytelniejsze ograniczenia cech z klauzulą `where`

Zbyt wiele ograniczeń cech może być problematyczne. Każdy typ ogólny ma swoje ograniczenia cech, dlatego funkcje z wieloma ogólnymi parametrami typu mogą zawierać mnóstwo informacji o ograniczeniach cech między nazwą funkcji a jej listą parametrów, co może utrudniać czytanie sygnatury funkcji. Z tego powodu Rust oferuje alternatywną składnię do specyfikowania ograniczeń cech za pomocą klauzuli `where`, która znajduje się po sygnaturze funkcji. Zamiast pisać tak:

```rust,ignore
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

możemy użyć klauzuli `where`, jak poniżej:

```rust,ignore
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

Sygnatura tej funkcji jest mniej zagracona: nazwa funkcji, lista parametrów i typ zwracany są blisko siebie, podobnie jak w funkcji bez wielu ograniczeń cech.