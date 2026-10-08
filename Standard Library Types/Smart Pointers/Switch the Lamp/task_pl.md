## Przełącz Lampę

W tym zadaniu potrzebujemy jednej lampy i kilku zewnętrznych przełączników, które mogą obsługiwać lampę
jak w poniższym scenariuszu:

* utwórz lampę
* utwórz przełącznik dla lampy
* utwórz kolejny przełącznik dla tej samej lampy
* obsługuj różne przełączniki i upewnij się, że lampa jest włączona, a potem znowu wyłączona

Niestety, Rust nie pozwala nam mieć kilku modyfikowalnych odwołań do lampy. Jeśli spróbujesz to zrobić 
w pliku `main.rs`, zauważysz to od razu.

Twoim celem jest edytowanie modułów `lamp` i `switchers` tak, aby `main` kompilował się i działał poprawnie. 
Jednym ze sposobów na to jest wprowadzenie *wewnętrznej zmienności* w `Lamp` za pomocą typu [`std::cell::Cell`](https://doc.rust-lang.org/std/cell/struct.Cell.html).

Nie powinieneś modyfikować kodu w `main.rs`. Powinien działać tak, jak jest.

<div class="hint">
Usuwaj modyfikatory <code>mut</code> w trakcie pracy. W Rust wszystko powinno być niemodyfikowalne, aby umożliwić nam posiadanie kilku odwołań do tej samej wartości.
</div>

<div class="hint">Konsultuj się z <a href="https://doc.rust-lang.org/std/cell/struct.Cell.html">dokumentacją</a>, aby dowiedzieć się, jak uzyskać dostęp do wartości w <code>Cell</code> i ją zmodyfikować.</div>