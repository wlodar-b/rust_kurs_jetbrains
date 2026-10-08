## Pracownicy w silniku

Spraw, aby kod się kompilował!

Chociaż warto najpierw przeczytać wyjaśnienia.

`Engine` zawiera wektor `Worker`s. Początkowo ten wektor jest pusty, ale możemy łatwo dodać dowolną liczbę pracowników. Gdy silnik ma wystarczającą liczbę pracowników, możemy go uruchomić. Silnik uruchamia swoich pracowników. Pracownicy wykonują swoją pracę i zapisują do `log`. Czym jest ten `log` i skąd się bierze? Cóż, to po prostu wektor `String` w silniku. Widzisz problem?

Pracownicy potrzebują możliwości zapisu do loga, ale Rust nigdy nie pozwoli nam mieć kilku zmiennych referencji do loga. Dodanie `mut` w różnych miejscach niestety nie zadziała. Potrzebujemy czegoś, co nazywa się [interior mutability](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#interior-mutability-a-mutable-borrow-to-an-immutable-value) (wewnętrzna mutowalność) oraz [`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html), aby dać pracownikom tymczasowy zapisowy dostęp do loga silnika.

Jest jeszcze jeden problem. Struktura `Worker` przechowuje referencję do loga. Wiemy, że referencje w `struct` wymagają określenia czasów życia (lifetimes), ale nie próbuj podążać za sugestiami kompilatora. One donikąd Cię nie zaprowadzą. Prawdziwym problemem jest to, że pracownicy i silnik mają różne czasy życia. Nie dałoby się przekonać borrow checkera, że wszystko jest w porządku, określając czasy życia. Zamiast tego powinniśmy użyć [inteligentnych wskaźników Rc&lt;T>](https://doc.rust-lang.org/book/ch15-04-rc.html).

Edytuj moduły `worker` i `engine`, aby program się kompilował, działał i przechodził testy. Nie edytuj pliku `main.rs`. Powinien działać tak, jak jest.

<div class="hint">
Zamiast podążać za sugestiami kompilatora, Twoim pierwszym krokiem może być zmiana typu loga na <code>Rc&lt;RefCell&lt;Vec&lt;String>>></code> (inteligentny wskaźnik do <code>RefCell</code> z wektorem ciągów tekstowych) w modułach <code>worker</code> i <code>engine</code>. Następnie musisz poprawić resztę kodu.
</div>

<div class="hint">
Mogą Ci się przydać następujące metody:

- `Rc::new()`, `Rc::clone()`;
- `RefCell::new()`, `RefCell::borrow()`, i `RefCell::borrow_mut()`.
</div>

<div class="hint">
Uważaj, aby nie zniszczyć loga po jego wypisaniu. Upewnij się, że odwołujesz się do wpisów loga przez referencje, kiedy wypisujesz je w <code>Engine::print_log()</code>.
</div>