#### API Mutex<T>

Jako przykład użycia mutexu, zacznijmy od zastosowania go w jednobieżnym kontekście, jak pokazano poniżej:

```rust
    use std::sync::Mutex;

    fn main() {
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }

        println!("m = {:?}", m);
    }
```

##### Eksploracja API Mutex<T> w jednobieżnym kontekście dla uproszczenia

Podobnie jak w przypadku wielu typów, tworzymy `Mutex<T>` za pomocą powiązanej funkcji `new`. Aby uzyskać dostęp do danych wewnątrz mutexu, używamy metody `lock`, aby zdobyć blokadę. To wywołanie zablokuje bieżący wątek, uniemożliwiając mu wykonywanie jakiejkolwiek pracy, dopóki nie nadejdzie nasza kolej na uzyskanie blokady.

Wywołanie `lock` zakończyłoby się niepowodzeniem, gdyby inny wątek trzymający blokadę zakończył się paniką. W takim przypadku nikt nie byłby w stanie zdobyć blokady, więc zdecydowaliśmy się na `unwrap`, co spowoduje panikę tego wątku w takiej sytuacji.

Po zdobyciu blokady możemy traktować wartość zwróconą, w tym przypadku nazwaną `num`, jako zmienne odwołanie do danych wewnątrz mutexu. System typów zapewnia, że zdobywamy blokadę przed wykorzystaniem wartości z `m`: `Mutex<i32>` nie jest `i32`, więc _musimy_ zdobyć blokadę, aby móc użyć wartości `i32`. Nie możemy o tym zapomnieć; system typów nie pozwoli nam uzyskać dostępu do wewnętrznego `i32` w inny sposób.

Jak zapewne się domyślasz, `Mutex<T>` jest wskaźnikiem inteligentnym. Dokładniej rzecz biorąc, wywołanie `lock` _zwraca_ wskaźnik inteligentny o nazwie `MutexGuard`, opakowany w `LockResult`, który obsługujemy przy pomocy wywołania `unwrap`. Wskaźnik inteligentny `MutexGuard` implementuje `Deref`, wskazując na nasze wewnętrzne dane; ma także implementację `Drop`, która automatycznie zwalnia blokadę, gdy `MutexGuard` wychodzi poza zakres, co następuje na końcu wewnętrznego zakresu w powyższym fragmencie kodu. W rezultacie nie ryzykujemy zapomnienia o zwolnieniu blokady i zablokowania możliwości korzystania z mutexu przez inne wątki, ponieważ zwolnienie blokady następuje automatycznie.

Po zwolnieniu blokady możemy wydrukować wartość mutexu i zobaczyć, że udało nam się zmienić wewnętrzne `i32` na 6.