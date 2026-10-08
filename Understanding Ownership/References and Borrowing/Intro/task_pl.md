## Wprowadzenie

Przekazywanie własności, a następnie jej zwracanie przy każdej funkcji jest dość uciążliwe. Co, jeśli chcemy pozwolić funkcji na użycie wartości, ale bez przejmowania jej własności? To dość irytujące, że wszystko, co przekazujemy, musi być również zwrócone, jeśli chcemy użyć tego ponownie, a także uzyskać dowolne dane wynikowe z ciała funkcji, które możemy chcieć zwrócić.

Możliwe jest zwracanie wielu wartości za pomocą krotki, jak pokazano w poniższym fragmencie kodu.

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("Długość '{}' wynosi {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() zwraca długość typu String

    (s, length)
}
```

##### Zwracanie własności parametrów

Ale to jest zbyt czasochłonne i wymaga dużo pracy dla pojęcia, które powinno być powszechne. Na szczęście dla nas Rust ma funkcję dla tego konceptu, która nazywa się _referencjami_.