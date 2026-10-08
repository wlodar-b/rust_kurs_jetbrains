### Zwracanie typów implementujących cechy

Możemy również użyć składni `impl Trait` w miejscu zwracania, aby zwrócić wartość jakiegoś typu, który implementuje daną cechę, jak pokazano tutaj:

```rust,ignore
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "oczywiście, jak zapewne już wiesz, ludzie",
        ),
        reply: false,
        retweet: false,
    }
}
```

Korzystając z `impl Summary` dla typu zwracanego, określamy, że funkcja `returns_summarizable` zwraca pewien typ, który implementuje cechę `Summary` bez podawania konkretnego typu. W tym przypadku `returns_summarizable` zwraca `Tweet`, ale kod wywołujący tę funkcję nie wie o tym.

Możliwość zwrócenia typu określonego jedynie przez implementowaną cechę jest szczególnie przydatna w kontekście domknięć i iteratorów, które omówimy w Rozdziale 13. Domknięcia i iteratory tworzą typy znane tylko kompilatorowi lub bardzo długie do zdefiniowania. Składnia `impl Trait` pozwala w zwięzły sposób określić, że funkcja zwraca jakiś typ implementujący cechę `Iterator` bez konieczności wypisywania bardzo długiej definicji typu.

Należy jednak pamiętać, że można użyć `impl Trait` tylko, jeśli zwracany jest jeden typ. Na przykład, poniższy kod, który zwraca albo `NewsArticle`, albo `Tweet` ze wskazanym typem zwracanym jako `impl Summary`, nie zadziała:

```rust,ignore,does_not_compile
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Pingwiny wygrywają Mistrzostwa Pucharu Stanleya!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "Pittsburgh Penguins ponownie są najlepszą \
                 drużyną hokejową w NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "oczywiście, jak zapewne już wiesz, ludzie",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

Zwracanie albo `NewsArticle`, albo `Tweet` nie jest dozwolone z powodu ograniczeń związanych z implementacją składni `impl Trait` w kompilatorze. Dowiemy się, jak napisać funkcję o takim zachowaniu, w sekcji [„Używanie cech jako obiektów umożliwiających wartości różnych typów”][using-trait-objects-that-allow-for-values-of-different-types]<!-- ignore --> Rozdziału 17 książki o Rust.

[using-trait-objects-that-allow-for-values-of-different-types]: https://doc.rust-lang.org/book/ch17-02-trait-objects.html