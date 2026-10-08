### Domyślne implementacje

Czasami przydatne jest posiadanie domyślnego zachowania dla niektórych lub wszystkich metod w trzecie, zamiast wymagać implementacji wszystkich metod dla każdego typu. Wtedy, implementując tę cechę dla konkretnego typu, możemy zachować albo nadpisać domyślne zachowanie każdej metody.

Poniższy kod pokazuje, jak określić domyślny ciąg znaków dla metody `summarize` w trzecie `Summary`, zamiast definiować jedynie sygnaturę metody, jak to zrobiliśmy w pierwszym przykładzie w tej sekcji.

```rust,noplayground
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Czytaj dalej...)")
    }
}
```

#### Definicja cechy `Summary` z domyślną implementacją metody `summarize`

Aby użyć domyślnej implementacji do podsumowania instancji `NewsArticle` zamiast definiowania niestandardowej implementacji, określamy pusty blok `impl` z `impl Summary for NewsArticle {}`.

Mimo że nie definiujemy już bezpośrednio metody `summarize` dla `NewsArticle`, dostarczamy domyślnej implementacji i określamy, że `NewsArticle` implementuje cechę `Summary`. Dzięki temu nadal możemy wywołać metodę `summarize` na instancji `NewsArticle`, jak w poniższym przykładzie:

```rust,ignore
let article = NewsArticle {
    headline: String::from("Pingwiny wygrywają mistrzostwo Pucharu Stanleya!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
        "Pingwiny z Pittsburgha ponownie są najlepszą \
        drużyną hokejową w NHL.",
    ),
};

println!("Nowy artykuł dostępny! {}", article.summarize());
```

Ten kod drukuje: `Nowy artykuł dostępny! (Czytaj dalej...)`.

Stworzenie domyślnej implementacji dla metody `summarize` nie wymaga zmiany czegokolwiek w implementacji cechy `Summary` dla `Tweet` w drugim przykładzie w tej sekcji. Powodem jest to, że składnia do nadpisywania domyślnej implementacji jest taka sama jak składnia do implementacji metody cechy, która nie posiada domyślnej implementacji.