### Implementowanie cechy na typie

Teraz, gdy zdefiniowaliśmy pożądane zachowanie za pomocą cechy `Summary`, możemy zaimplementować ją dla typów w naszym agregatorze mediów. Poniższy fragment kodu pokazuje implementację cechy `Summary` dla struktury `NewsArticle`, która wykorzystuje nagłówek, autora oraz lokalizację do stworzenia zwracanej wartości w metodzie `summarize`. Dla struktury `Tweet` definiujemy `summarize` jako nazwę użytkownika, a następnie całą treść tweeta, przyjmując, że treść tweeta jest już ograniczona do 280 znaków.

```rust,noplayground
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

#### Implementowanie cechy `Summary` na typach `NewsArticle` i `Tweet`.

Implementowanie cechy na typie jest podobne do implementowania zwykłych metod. Różnica polega na tym, że po `impl` umieszczamy nazwę cechy, którą chcemy zaimplementować, następnie używamy słowa kluczowego `for`, a potem określamy nazwę typu, dla którego chcemy zaimplementować cechę. W bloku `impl` umieszczamy sygnatury metod, które zostały zdefiniowane w definicji cechy. Zamiast dodawać średnik po każdej sygnaturze, używamy nawiasów klamrowych i wypełniamy ciało metody specyficznym zachowaniem, które chcemy przypisać metodom cechy dla konkretnego typu.

Po zaimplementowaniu cechy możemy wywoływać metody na instancjach `NewsArticle` i `Tweet` w taki sam sposób, jak w przypadku zwykłych metod, na przykład:

```rust,ignore
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
        "of course, as you probably already know, people",
    ),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());
```

Ten kod wyświetla `1 new tweet: horse_ebooks: of course, as you probably already know, people`.

Zwróć uwagę, że ponieważ zdefiniowaliśmy cechę `Summary`, a także typy `NewsArticle` i `Tweet` w tym samym pliku *lib.rs* w sekcji „Implementowanie cechy `Summary` na typach `NewsArticle` i `Tweet`”, znajdują się one w tym samym zakresie. Załóżmy, że ten plik *lib.rs* jest częścią biblioteki, którą nazwaliśmy `aggregator`, a ktoś inny chce użyć funkcjonalności naszej biblioteki, aby zaimplementować cechę `Summary` na strukturze zdefiniowanej w jego własnym zakresie biblioteki. Musiałby najpierw wprowadzić cechę do swojego zakresu. Zrobiłby to, używając `use aggregator::Summary;`, co umożliwiłoby mu zaimplementowanie `Summary` dla swojego typu. Cechę `Summary` należy również oznaczyć jako publiczną, aby inna biblioteka mogła ją zaimplementować, co zapewniamy, dodając słowo kluczowe `pub` przed `trait` w liście 10-12.

Ograniczenie, o którym warto pamiętać przy implementacji cech, polega na tym, że możemy zaimplementować cechę na typie tylko wtedy, gdy albo cecha, albo typ jest lokalny dla naszej biblioteki. Na przykład możemy zaimplementować cechy standardowej biblioteki, takie jak `Display`, dla niestandardowego typu, takiego jak `Tweet`, w ramach funkcjonalności naszej biblioteki `aggregator`, ponieważ typ `Tweet` jest lokalny dla naszej biblioteki `aggregator`. Możemy również zaimplementować `Summary` dla `Vec<T>` w naszej bibliotece `aggregator`, ponieważ cecha `Summary` jest lokalna dla naszej biblioteki `aggregator`.

Ale nie możemy zaimplementować zewnętrznych cech na zewnętrznych typach. Na przykład nie możemy zaimplementować cechy `Display` dla `Vec<T>` w ramach naszej biblioteki `aggregator`, ponieważ zarówno `Display`, jak i `Vec<T>` są zdefiniowane w standardowej bibliotece i nie są lokalne dla naszej biblioteki `aggregator`. To ograniczenie jest częścią właściwości programów zwanej *spójnością* (*coherence*), a dokładniej *reguły sieroty* (*orphan rule*), nazwanej tak dlatego, że typ nadrzędny nie jest obecny. Reguła ta zapewnia, że kod innych osób nie może zepsuć twojego kodu i odwrotnie. Bez tej reguły dwie biblioteki mogłyby zaimplementować tę samą cechę dla tego samego typu, a Rust nie wiedziałby, której implementacji użyć.