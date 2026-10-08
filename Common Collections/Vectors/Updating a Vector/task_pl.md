### Aktualizowanie wektora

Aby utworzyć wektor, a następnie dodać do niego elementy, możemy użyć metody `push`, jak pokazano w poniższym fragmencie kodu.

```rust
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
```

#### Używanie metody `push` do dodawania wartości do wektora

Jak w przypadku każdej zmiennej, jeśli chcemy mieć możliwość zmieniania jej wartości, musimy uczynić ją zmienną za pomocą słowa kluczowego `mut`, co zostało omówione w rozdziale "Podstawowe pojęcia programistyczne". Liczby, które umieszczamy wewnątrz, są wszystkimi typami `i32`, a Rust wywnioskuje to z danych, więc nie potrzebujemy adnotacji `Vec<i32>`.