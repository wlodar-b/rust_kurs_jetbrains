## Typy generyczne, cechy (traits) i okresy życia (lifetimes)

Każdy język programowania posiada narzędzia umożliwiające efektywne radzenie sobie z powielaniem koncepcji. W Rust jednym z takich narzędzi są *generyki*. Generyki to abstrakcyjne zastępstwa dla konkretnych typów lub innych właściwości. Pisząc kod, możemy określić, jak generyki zachowują się lub jak odnoszą się do innych generyków, bez znajomości ich konkretnych wartości w momencie kompilacji i uruchamiania programu.

Podobnie jak funkcja przyjmuje parametry z nieznanymi wartościami, aby uruchomić ten sam kod na różnych konkretnych wartościach, funkcje mogą przyjmować parametry o pewnym generycznym typie zamiast konkretnego typu, takiego jak `i32` czy `String`. W rzeczywistości już korzystaliśmy z generyków w rozdziale „Wyliczenia” (Enums) z `Option<T>`, w „Powszechnych Kolekcjach” (Common Collections) z `Vec<T>` i `HashMap<K, V>`, a także w „Obsłudze Błędów” (Recoverable and Unrecoverable Errors) z `Result<T, E>`. W tym rozdziale nauczysz się, jak definiować własne typy, funkcje i metody wykorzystujące generyki!

Najpierw omówimy, jak wydzielić funkcję w celu zredukowania powielania kodu. Następnie zastosujemy tę samą technikę, aby stworzyć funkcję generyczną z dwóch funkcji, które różnią się jedynie typami swoich parametrów. Wyjaśnimy również, jak używać typów generycznych w definicjach struktur (struct) i wyliczeń (enum).

Następnie dowiesz się, jak korzystać z *cech* (traits), aby definiować zachowanie w sposób generyczny. Możesz łączyć cechy z typami generycznymi, aby ograniczyć typy generyczne tylko do tych, które mają określone zachowanie, zamiast dopuszczać dowolny typ.

Na koniec omówimy *okresy życia* (lifetimes), jeden z rodzajów generyków, który dostarcza kompilatorowi informacji o tym, jak referencje odnoszą się do siebie nawzajem. Okresy życia pozwalają nam na wypożyczanie wartości w wielu sytuacjach przy jednoczesnym umożliwieniu kompilatorowi sprawdzenia poprawności referencji.