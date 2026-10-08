## Korzystanie z wątków do równoczesnego wykonywania kodu

W większości współczesnych systemów operacyjnych kod wykonywanego programu działa w _procesie_, a system operacyjny zarządza jednocześnie wieloma procesami. W obrębie swojego programu możesz również mieć niezależne części, które wykonują się jednocześnie. Funkcje pozwalające na wykonywanie takich niezależnych części nazywane są _wątkami_.

Podzielenie obliczeń w swoim programie na wiele wątków może poprawić wydajność, ponieważ program realizuje kilka zadań jednocześnie, ale dodaje to również złożoności. Ponieważ wątki mogą działać równocześnie, nie ma wbudowanej gwarancji co do kolejności, w jakiej części twojego kodu działające na różnych wątkach zostaną wykonane. Może to prowadzić do problemów, takich jak:

*   Warunki wyścigu (ang. _race conditions_), gdzie wątki uzyskują dostęp do danych lub zasobów w niespójnej kolejności
*   Zakleszczenia (ang. _deadlocks_), gdzie dwa wątki czekają na zakończenie korzystania z zasobu przez drugi wątek, co uniemożliwia kontynuację obu wątków
*   Błędy występujące tylko w określonych sytuacjach, które są trudne do odtworzenia i naprawienia w niezawodny sposób

Rust stara się minimalizować negatywne skutki używania wątków, ale programowanie w kontekście wielowątkowym nadal wymaga przemyślanego podejścia i wymusza strukturę kodu różniącą się od tej stosowanej w programach działających w pojedynczym wątku.

Języki programowania implementują wątki na różne sposoby. Wiele systemów operacyjnych oferuje API do tworzenia nowych wątków. Ten model, w którym język korzysta z systemowego API do tworzenia wątków, czasami nazywany jest modelem _1:1_, co oznacza jeden wątek systemu operacyjnego na jeden wątek języka programowania.

Wiele języków programowania oferuje własną specjalną implementację wątków. Wątki dostarczane przez język programowania są znane jako _zielone_ wątki (ang. _green threads_), a języki korzystające z takich wątków wykonują je w kontekście określonej liczby wątków systemu operacyjnego. Z tego powodu model z zielonymi wątkami określany jest jako model _M:N_: istnieje `M` zielonych wątków na `N` wątków systemu operacyjnego, gdzie `M` i `N` nie muszą być takimi samymi liczbami.

Każdy model ma swoje zalety i kompromisy, a kompromis najważniejszy dla Rusta wiąże się z obsługą środowiska uruchomieniowego. _Środowisko uruchomieniowe_ (ang. _runtime_) to mylący termin, który w różnych kontekstach może mieć różne znaczenia.

W tym kontekście przez _środowisko uruchomieniowe_ rozumiemy kod, który język dodaje do każdego pliku wykonywalnego. Kod ten może być większy lub mniejszy w zależności od języka, ale każdy język inny niż asembler będzie miał pewną ilość kodu środowiska uruchomieniowego. Dlatego potocznie, gdy ludzie mówią, że język ma „brak środowiska uruchomieniowego”, często mają na myśli „małe środowisko uruchomieniowe”. Mniejsze środowiska mają mniej funkcji, ale ich zaletą są mniejsze pliki wynikowe, co ułatwia łączenie języka z innymi językami w różnych kontekstach. Chociaż wiele języków zgadza się na zwiększenie rozmiaru środowiska uruchomieniowego w zamian za dodatkowe funkcje, Rust musi mieć minimalne środowisko uruchomieniowe i nie może iść na kompromis w zakresie możliwości wywoływania C w celu zachowania wydajności.

Model z zielonymi wątkami M:N wymaga większego środowiska uruchomieniowego języka do zarządzania wątkami. Z tego powodu biblioteka standardowa Rusta oferuje jedynie implementację wątków 1:1. Ponieważ Rust jest językiem niskopoziomowym, istnieją paczki (ang. _crates_), które implementują wątki M:N, jeśli wolisz zrezygnować z wydajności na rzecz większej kontroli nad tym, które wątki działają w danym momencie, oraz niższych kosztów przełączania kontekstu, na przykład.

Teraz, gdy zdefiniowaliśmy wątki w Ruście, przyjrzyjmy się, jak korzystać z API związanych z wątkami, które dostarcza biblioteka standardowa.