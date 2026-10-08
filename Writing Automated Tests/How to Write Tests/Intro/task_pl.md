## Wprowadzenie

W swoim eseju z 1972 roku „The Humble Programmer” Edsger W. Dijkstra napisał:  
„Testowanie programów może być bardzo skutecznym sposobem na wykrycie błędów, ale jest zupełnie bezużyteczne, jeśli chodzi o udowodnienie ich braku.” To jednak nie oznacza, że nie powinniśmy testować naszego kodu tak bardzo, jak to tylko możliwe!

Poprawność w naszych programach to stopień, w jakim nasz kod robi to, co zamierzamy, aby robił. Rust został zaprojektowany z dużą troską o poprawność programów, ale poprawność to kwestia złożona i trudna do udowodnienia. System typów w Rust bierze na siebie dużą część tego ciężaru, ale nie jest w stanie wykryć każdego rodzaju niepoprawności. W związku z tym, Rust zawiera wsparcie dla pisania automatycznych testów oprogramowania w samej językowej strukturze.

Załóżmy na przykład, że napiszemy funkcję nazwaną `add_two`, która dodaje 2 do liczby przekazanej do niej jako argument. Sygnatura tej funkcji przyjmuje liczbę całkowitą jako parametr i zwraca liczbę całkowitą jako wynik. Podczas implementacji i kompilacji tej funkcji Rust sprawdzi za pomocą weryfikacji typów i pożyczek (ang. borrow checking), z którymi już się zapoznaliście, czy na przykład nie przekazujemy wartości `String` lub nieprawidłowego odwołania. Jednak Rust *nie może* sprawdzić, czy funkcja zrobi dokładnie to, co zamierzamy, czyli zwróci przekazany parametr powiększony o 2, a nie na przykład parametr powiększony o 10 lub zmniejszony o 50! Właśnie tutaj wkraczają testy.

Możemy napisać testy, które sprawdzą, na przykład, czy gdy przekażemy `3` do funkcji `add_two`, zwrócona wartość będzie równa `5`. Możemy uruchamiać te testy zawsze, gdy wprowadzamy zmiany w naszym kodzie, aby upewnić się, że istniejące poprawne działanie nie zostało zmienione.

Testowanie to złożona umiejętność: chociaż nie możemy omówić wszystkich szczegółów dotyczących tworzenia dobrych testów w jednym rozdziale, omówimy podstawy dotyczące mechanizmów obsługi testów w Rust. Porozmawiamy o adnotacjach i makrach, które są dostępne podczas pisania testów, o domyślnym zachowaniu i opcjach uruchamiania testów oraz o tym, jak organizować testy w testy jednostkowe i testy integracyjne.