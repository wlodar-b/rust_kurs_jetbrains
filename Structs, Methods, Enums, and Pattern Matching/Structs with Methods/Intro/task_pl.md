## Składnia metod

*Metody* są podobne do funkcji: są deklarowane za pomocą słowa kluczowego `fn` i nazwy, mogą mieć parametry oraz zwracać wartość, a także zawierają kod, który jest wykonywany, gdy zostaną wywołane z innego miejsca. Jednak metody różnią się od funkcji tym, że są zdefiniowane w kontekście struktury (lub enumera lub obiektu typu trait, które są omówione w rozdziale "Enumy" oraz w [Rozdziale 17][ch17] "Książki o Ruście", odpowiednio), a ich pierwszym parametrem zawsze jest `self`, który reprezentuje instancję struktury, na której metoda jest wywoływana.

[ch17]: https://github.com/rust-lang/book/blob/master/src/ch17-00-oop.md