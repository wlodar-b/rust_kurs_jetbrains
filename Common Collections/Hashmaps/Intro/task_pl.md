## Przechowywanie kluczy z powiązanymi wartościami w mapach haszujących

Ostatnią z naszych popularnych kolekcji jest *mapa haszująca*. Typ `HashMap<K, V>`
przechowuje odwzorowanie kluczy typu `K` na wartości typu `V`. Robi to za pomocą
*funkcji haszującej*, która decyduje, jak umieszcza te klucze i wartości w pamięci. 
Wiele języków programowania obsługuje tego typu strukturę danych, lecz często używają 
innych nazw, takich jak hash, map, obiekt, tabela haszująca, słownik lub tablica 
asocjacyjna, by wymienić tylko kilka przykładów.

Mapy haszujące są przydatne, gdy chcesz odszukać dane nie używając indeksu, jak w przypadku wektorów,
ale za pomocą klucza, który może być dowolnego typu. Na przykład w grze możesz śledzić wynik
każdej drużyny w mapie haszującej, w której kluczem jest nazwa drużyny, a wartościami są wyniki
danej drużyny. Mając nazwę drużyny, możesz uzyskać jej wynik.

Przejdziemy przez podstawowe API map haszujących w tej sekcji, ale wiele ciekawych 
funkcjonalności kryje się w funkcjach zdefiniowanych na `HashMap<K, V>` w bibliotece 
standardowej. Jak zawsze, zapoznaj się z dokumentacją biblioteki standardowej, aby uzyskać 
więcej informacji.