## Zrozumienie Własności

Własność (ang. ownership) to najbardziej unikalna cecha języka Rust, która pozwala mu zapewniać bezpieczeństwo pamięci bez potrzeby użycia mechanizmów odśmiecania pamięci (garbage collector). Dlatego kluczowe jest zrozumienie, jak działa własność w Rust. W tym rozdziale omówimy własność i kilka związanych z nią funkcji: pożyczanie (borrowing), wycinki (slices) oraz sposób rozmieszczania danych w pamięci przez Rust.

## Czym jest własność?

Centralną cechą Rusta jest _własność_. Chociaż koncepcja własności jest dość prosta do wyjaśnienia, wpływa ona znacząco na resztę języka.

Każdy program musi w trakcie pracy zarządzać pamięcią komputera. Niektóre języki używają mechanizmów odśmiecania pamięci, które na bieżąco wyszukują nieużywaną pamięć, inne wymagają od programisty jawnego przydzielania i zwalniania pamięci. Rust stosuje trzecie podejście: pamięć jest zarządzana poprzez system własności z zestawem reguł, które kompilator weryfikuje na etapie kompilacji. Żadna z funkcji związanych z własnością nie spowalnia działania programu.

Ponieważ własność to nowa koncepcja dla wielu programistów, jej opanowanie wymaga czasu. Dobra wiadomość jest taka, że im więcej masz doświadczenia z Rustem i jego zasadami własności, tym bardziej naturalne staje się tworzenie bezpiecznego i wydajnego kodu. Nie poddawaj się!

Kiedy zrozumiesz własność, zyskasz solidne podstawy do ogarnięcia funkcji, które wyróżniają Rust. W tym rozdziale poznasz własność poprzez analizę kilku przykładów skoncentrowanych na bardzo powszechnej strukturze danych: ciągach znaków (strings).

### Stos i sterta

W wielu językach programowania nie trzeba często rozważać kwestii stosu i sterty. Jednak w języku programowania systemowego, takim jak Rust, to, czy wartość znajduje się na stosie, czy na stercie, ma duży wpływ na sposób działania języka i decyzje, które musisz podejmować. Części koncepcji własności będą omawiane w odniesieniu do stosu i sterty później w tym rozdziale, więc oto krótkie wyjaśnienie w ramach przygotowania.

Zarówno stos, jak i sterta to części pamięci, które są dostępne dla twojego kodu podczas wykonywania, ale są one zorganizowane w różny sposób. Stos przechowuje wartości w kolejności ich dodania i usuwa je w odwrotnej kolejności. To podejście nazywa się _ostatni na wejściu, pierwszy na wyjściu_ (last in, first out). Można to porównać do stosu talerzy: dokładając więcej talerzy, kładziemy je na szczycie stosu, a kiedy potrzebujemy talerza, zdejmujemy go z góry. Dodawanie lub usuwanie talerzy ze środka albo spodu stosu byłoby trudniejsze! Dodawanie danych na stos nazywa się _wstawianiem na stos_ (pushing onto the stack), a usuwanie danych - _zdejmowaniem ze stosu_ (popping off the stack).

Wszystkie dane przechowywane na stosie muszą mieć znany, stały rozmiar. Dane o rozmiarze nieznanym podczas kompilacji lub rozmiarze, który może się zmieniać, muszą być przechowywane na stercie. Sterta jest mniej zorganizowana: gdy umieszczasz dane na stercie, żądasz określonej ilości miejsca. Alokator pamięci znajduje wolne miejsce na stercie, które jest wystarczająco duże, oznacza je jako zajęte i zwraca _wskaźnik_ — czyli adres tej lokalizacji. Ten proces nazywa się _alokacją na stercie_ (allocating on the heap) i czasami jest skracany do samej _alokacji_. Dodawanie danych na stos nie jest uznawane za alokację. Ponieważ wskaźnik ma znany, stały rozmiar, możesz przechowywać go na stosie, ale kiedy chcesz uzyskać właściwe dane, musisz podążyć za wskaźnikiem.

Pomyśl o sytuacji w restauracji. Gdy wchodzisz, podajesz liczbę osób w swojej grupie, a obsługa znajduje wolny stolik, który pomieści wszystkich, i prowadzi cię tam. Jeśli ktoś z twojej grupy przyjdzie później, może zapytać, gdzie zostaliście posadzeni, aby cię znaleźć.

Dodawanie danych na stosie jest szybsze niż alokacja na stercie, ponieważ alokator nie musi szukać miejsca na nowe dane; zawsze dodaje je na szczycie stosu. Alokowanie przestrzeni na stercie wymaga więcej pracy, ponieważ alokator musi najpierw znaleźć wystarczająco duże miejsce na dane, a następnie przeprowadzić działania organizacyjne przygotowujące do kolejnej alokacji.

Dostęp do danych na stercie jest wolniejszy niż dostęp do danych na stosie, ponieważ musisz podążyć za wskaźnikiem, aby się do nich dostać. Współczesne procesory działają szybciej, jeśli rzadziej przeskakują między różnymi adresami w pamięci. Porównując to do analogii z restauracją, kelnerowi bardziej opłaca się zebrać zamówienia ze wszystkich osób przy jednym stoliku, zanim przejdzie do następnego. Zbieranie zamówień od stolika A, potem od stolika B, a następnie znów od A i B zajmie znacznie więcej czasu. W podobny sposób procesor działa wydajniej, jeśli pracuje z danymi bliskimi sobie w pamięci (jak na stosie), zamiast z danymi oddalonymi od siebie (jak na stercie). Alokacja dużej ilości miejsca na stercie także może zająć czas.

Kiedy twój kod wywołuje funkcję, wartości przekazywane do funkcji (w tym, potencjalnie, wskaźniki do danych na stercie) oraz zmienne lokalne funkcji są wstawiane na stos. Po zakończeniu działania funkcji te wartości są zdejmowane ze stosu.

Śledzenie, które części kodu korzystają z jakich danych na stercie, minimalizowanie ilości duplikatów danych na stercie oraz sprzątanie nieużywanych danych z pamięci, aby jej nie zabrakło — to wszystko są problemy, które rozwiązuje system własności. Gdy zrozumiesz własność, rzadko będziesz musiał myśleć o stosie i stercie, ale świadomość, że zarządzanie danymi na stercie jest powodem istnienia systemu własności, pomoże ci zrozumieć, dlaczego działa on w taki sposób.

### Zasady własności

Najpierw przyjrzyjmy się zasadom własności. Miej je na uwadze podczas pracy z przykładami ilustrującymi te reguły:

*   Każda wartość w Rust ma zmienną nazywaną jej _właścicielem_.
*   W danym momencie może być tylko jeden właściciel.
*   Kiedy właściciel wychodzi z zakresu (scope), wartość zostaje usunięta.