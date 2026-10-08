## Wartości zwracane i zakres

Zwracanie wartości może również przenosić własność. Poniższy fragment kodu jest przykładem z podobnymi adnotacjami jak te w poprzednim fragmencie.

```rust
fn main() {
    let s1 = daje_własność();         // daje_własność przenosi swoją wartość
    // zwracaną do s1

    let s2 = String::from("hello");   // s2 pojawia się w zakresie

    let s3 = zabiera_i_zwraca(s2);    // s2 zostaje przeniesione do
    // zabiera_i_zwraca, które również
    // przenosi swoją wartość zwracaną do s3
} // Tutaj s3 wychodzi poza zakres i jest usuwane. s2 wychodzi poza zakres,
  // ale zostało przeniesione, więc nic się nie dzieje. s1 wychodzi poza zakres i jest usuwane.

fn daje_własność() -> String {             // daje_własność przeniesie swoją
    // wartość zwracaną do funkcji,
    // która ją wywołała

    let jakiś_tekst = String::from("hello"); // jakiś_tekst pojawia się w zakresie

    jakiś_tekst                              // jakiś_tekst zostaje zwrócony i
    // przeniesiony do funkcji wywołującej
}

// zabiera_i_zwraca przyjmie String i zwróci go
fn zabiera_i_zwraca(jakiś_string: String) -> String { // jakiś_string pojawia się
    // w zakresie

    jakiś_string  // jakiś_string zostaje zwrócony i przeniesiony do funkcji wywołującej
}
```

##### Przenoszenie własności wartości zwracanych

Własność zmiennej zawsze podąża tym samym wzorcem: przypisanie wartości do innej zmiennej powoduje przeniesienie jej. Gdy zmienna, która zawiera dane na stercie, wychodzi poza zakres, wartość zostanie wyczyszczona przez funkcję `drop`, chyba że dane zostały przeniesione i są teraz własnością innej zmiennej.

_Możesz odnieść się do następującego rozdziału w książce „Język programowania Rust”: [Własność](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html)_