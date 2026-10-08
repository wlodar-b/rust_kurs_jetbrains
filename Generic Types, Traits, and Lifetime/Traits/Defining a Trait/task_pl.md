### Definiowanie cechy (traitu)

Zachowanie typu określa zestaw metod, które można wywołać na tym typie. Różne typy mają to samo zachowanie, jeśli można na wszystkich z nich wywołać te same metody. Definicje cech (traitów) pozwalają na grupowanie sygnatur metod w celu zdefiniowania zestawu zachowań niezbędnych do realizacji określonego celu.

Na przykład załóżmy, że mamy kilka struktur (struct), które przechowują różne rodzaje i ilości tekstu: strukturę `NewsArticle`, która przechowuje artykuł prasowy z określoną lokalizacją, oraz strukturę `Tweet`, która może mieć maksymalnie 280 znaków wraz z metadanymi wskazującymi, czy jest to nowy tweet, retweet lub odpowiedź na inny tweet.

Chcemy stworzyć bibliotekę agregatora mediów, która potrafi wyświetlać podsumowania danych przechowywanych w instancjach `NewsArticle` lub `Tweet`. Aby to osiągnąć, potrzebujemy podsumowania z każdego typu, które będziemy mogli uzyskać, wywołując metodę `summarize` na danej instancji. Poniższy fragment kodu pokazuje definicję cechy `Summary`, która opisuje to zachowanie.

```rust,noplayground
pub trait Summary {
    fn summarize(&self) -> String;
}
```

#### Cechą `Summary` jest zachowanie dostarczane przez metodę `summarize`

Tutaj definiujemy cechę przy użyciu słowa kluczowego `trait` oraz nazwy cechy, w tym przypadku `Summary`. Wewnątrz nawiasów klamrowych deklarujemy sygnatury metod, które opisują zachowania typów implementujących tę cechę, w tym przypadku `fn summarize(&self) -> String`.

Po sygnaturze metody, zamiast podawać implementację w nawiasach klamrowych, używamy średnika. Każdy typ implementujący tę cechę musi dostarczyć własne, niestandardowe zachowanie w ciele tej metody. Kompilator wymusi, aby każdy typ posiadający cechę `Summary` miał zdefiniowaną metodę `summarize` dokładnie z taką sygnaturą.

Cechy mogą zawierać wiele metod w swoim ciele: sygnatury metod są wypisywane jedna pod drugą, a każda linia kończy się średnikiem.