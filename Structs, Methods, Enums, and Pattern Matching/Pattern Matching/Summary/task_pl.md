## Podsumowanie

Omówiliśmy, jak używać enumów do tworzenia niestandardowych typów, które mogą przyjmować jedną z enumerowanych wartości. Pokazaliśmy, jak typ `Option<T>` z biblioteki standardowej pomaga wykorzystać system typów do zapobiegania błędom. Gdy wartości enuma zawierają dane, możesz użyć `match` lub `if let`, aby wyodrębnić i użyć tych wartości, w zależności od liczby przypadków, które musisz obsłużyć.

Twoje programy w języku Rust mogą teraz wyrażać koncepcje z twojej dziedziny za pomocą struktur i enumów. Tworzenie niestandardowych typów do wykorzystania w twoim API zapewnia bezpieczeństwo typów: kompilator upewni się, że twoje funkcje otrzymują tylko wartości typu, którego każda funkcja oczekuje.

Aby dostarczyć użytkownikom dobrze zorganizowane API, które jest proste w użyciu i udostępnia dokładnie to, czego użytkownicy będą potrzebować, przejdźmy teraz do modułów w języku Rust.