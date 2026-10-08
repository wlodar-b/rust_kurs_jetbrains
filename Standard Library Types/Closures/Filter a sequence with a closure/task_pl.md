## Zadanie: Filtrowanie sekwencji za pomocą domknięcia

Masz tablicę z elementami typu `i32`, wartość `cap` oraz licznik `left_behind`. Twoim zadaniem
jest uzupełnienie domknięcia przekazanego jako parametr do metody `filter`. Domknięcie
powinno zachować tylko te elementy, które są ściśle mniejsze od `cap`. Powinno ono
również zliczyć wszystkie elementy pominięte.

<div class="hint">
Przeczytaj dokumentację metody <code>filter</code>, aby zrozumieć, o co chodzi z 
tymi <code>&&</code> w argumencie domknięcia.
</div>

<div class="hint">
Możesz skorzystać z faktu, że prawa strona wyrażenia <code> warunek || warunek </code> jest
ewaluowana jedynie wówczas, gdy lewa strona jest <code>false</code>.
</div>

<div class="hint">
Użyj <code>{...; warunek}</code>, aby wykonać akcję podczas ewaluacji wyrażenia logicznego.
</div>