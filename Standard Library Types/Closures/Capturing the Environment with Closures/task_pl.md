### Przechwytywanie środowiska za pomocą domknięć

Pierwszym aspektem domknięć, który zbadamy, jest możliwość przechwytywania wartości z otoczenia, w którym zostały zdefiniowane, do późniejszego wykorzystania. Oto scenariusz: Firma produkująca koszulki co jakiś czas rozdaje darmową koszulkę komuś z listy mailingowej. Osoby będące na liście mailingowej mogą opcjonalnie dodać swój ulubiony kolor do swojego profilu. Jeśli osoba wybrana do otrzymania darmowej koszulki ma w swoim profilu określony ulubiony kolor, otrzymuje koszulkę w tym kolorze. Jeśli osoba nie określiła ulubionego koloru, otrzymuje koszulkę w kolorze, którego firma ma obecnie najwięcej.

Istnieje wiele sposobów na zaimplementowanie tego scenariusza. W tym przykładzie użyjemy typu wyliczeniowego `ShirtColor`, który ma warianty `Red` i `Blue`. Stan magazynowy firmy jest reprezentowany przez strukturę `Inventory`, która zawiera pole nazwane `shirts`, będące wektorem `Vec<ShirtColor>` reprezentującym koszulki aktualnie dostępne w magazynie. Metoda `giveaway` zdefiniowana w `Inventory` pobiera opcjonalną preferencję koloru koszulki osoby, która ma otrzymać darmową koszulkę, i zwraca kolor koszulki, którą ta osoba dostanie. Przykład jest pokazany na poniższym listingu:

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
```

##### Rozdanie koszulek przez firmę

Obiekt `store` zdefiniowany w `main` ma w magazynie dwie niebieskie koszulki i jedną czerwoną. Następnie wywołuje metodę `giveaway` dla użytkownika z preferencją czerwoną koszulką oraz dla użytkownika bez żadnych preferencji. Uruchomienie tego kodu wyświetla następujący wynik:

```console
$ cargo run
   Compiling shirt-company v0.1.0 (file:///projects/shirt-company)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/shirt-company`
The user with preference Some(Red) gets Red
The user with preference None gets Blue
```

Ten kod mógłby zostać zaimplementowany na wiele sposobów, ale tutaj użyto koncepcji, które już poznaliśmy, z wyjątkiem ciała metody `giveaway`, która wykorzystuje domknięcie. Metoda `giveaway` pobiera preferencje użytkownika `Option<ShirtColor>` i wywołuje na nich metodę `unwrap_or_else`. Metoda [`unwrap_or_else` w `Option<T>`](https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.unwrap_or_else) jest zdefiniowana w bibliotece standardowej. Przyjmuje jeden argument: domknięcie bez argumentów, które zwraca wartość `T` (ten sam typ, który jest przechowywany w wariancie `Some` typu `Option<T>`, w tym przypadku `ShirtColor`). Jeśli `Option<T>` to wariant `Some`, `unwrap_or_else` zwraca wartość z wnętrza `Some`. Jeśli `Option<T>` to wariant `None`, `unwrap_or_else` wywołuje domknięcie i zwraca jego wynik.

To jest interesujące, ponieważ przekazaliśmy domknięcie, które wywołuje metodę `self.most_stocked()` na bieżącej instancji `Inventory`. Biblioteka standardowa nie musiała wiedzieć nic o typach `Inventory` czy `ShirtColor`, ani o logice, którą chcieliśmy zastosować w tym scenariuszu. Domknięcie przechwyciło niezmienną referencję do instancji `self` `Inventory` i przekazało ją wraz z naszym kodem do metody `unwrap_or_else`. Funkcje nie są w stanie przechwytywać swojego środowiska w ten sposób.