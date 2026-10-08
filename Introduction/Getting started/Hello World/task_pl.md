## Witaj, świecie!

Teraz napiszmy Twój pierwszy program w języku Rust. Tradycyjnie, gdy uczymy się nowego języka programowania, piszemy mały program, który wyświetla tekst `Witaj, świecie!` na ekranie — zróbmy to również tutaj!

Widzisz okno **Edytora** po lewej stronie. To jest Twoje środowisko testowe, w którym możesz sprawdzać swój kod. Plik `main.rs` zawiera następujące linie:

```rust
fn main() {
    // umieść tutaj swój kod, aby go uruchomić
}
``` 
Teraz zamień linię wewnątrz funkcji main na kod:

```rust
    println!("Witaj, świecie!");
```
##### Przykład: Program, który wyświetla Witaj, świecie!

Aby uruchomić ten kod, kliknij ikonę **Run** w lewym górnym rogu edytora. Kliknij przycisk **Check** w oknie **Opis zadania**, aby uruchomić testy sprawdzające poprawność rozwiązania.

Jeśli `Witaj, świecie!` zostało wyświetlone, gratulacje! Oficjalnie napisałeś program w języku Rust. To czyni Cię programistą języka Rust — witaj na pokładzie!

### Anatomia programu w języku Rust

Przeanalizujmy teraz szczegółowo, co dokładnie wydarzyło się w Twoim programie Witaj, świecie! Oto pierwszy element układanki:

```rust

fn main() {

}
```

Te linie definiują funkcję w języku Rust. Funkcja `main` jest szczególna: zawsze jest pierwszym kodem, który uruchamia się w każdym wykonywalnym programie Rust. Pierwsza linia deklaruje funkcję o nazwie `main`, która nie przyjmuje żadnych parametrów i nic nie zwraca. Gdyby parametry były potrzebne, umieszczono by je w nawiasach, `()`.

Zwróć też uwagę na to, że ciało funkcji jest zamknięte w nawiasach klamrowych `{}`. Rust wymaga ich dla wszystkich ciał funkcji. Dobrą praktyką jest umieszczenie otwierającego nawiasu klamrowego w tej samej linii co deklaracja funkcji, dodając jedną spację pomiędzy nimi.

W momencie pisania tego tekstu narzędzie do automatycznego formatowania kodu, zwane `rustfmt`, jest w fazie rozwoju. Jeśli chcesz trzymać się standardowego stylu w projektach Rust, `rustfmt` sformatuje Twój kod w określony sposób. Zespół Rust planuje ostatecznie dołączyć to narzędzie do standardowej dystrybucji języka Rust, podobnie jak `rustc`. W zależności od tego, kiedy czytasz ten podręcznik, może ono być już zainstalowane na Twoim komputerze! Sprawdź dokumentację online, aby uzyskać więcej szczegółów.

Wewnątrz funkcji `main` znajduje się następujący kod:

```rust
    println!("Witaj, świecie!");
```

Ta linia wykonuje całą pracę w tym małym programie: wyświetla tekst na ekranie. Istnieją cztery istotne szczegóły do zauważenia. Po pierwsze, w stylu Rust wcięcia wykonuje się za pomocą czterech spacji, a nie tabulatora.

Po drugie, println! wywołuje makro w języku Rust. Gdyby zamiast tego wywoływało funkcję, zapisano by je jako `println` (bez `!`). Omówimy makra języka Rust bardziej szczegółowo nieco później. Na razie wystarczy wiedzieć, że użycie `!` oznacza wywołanie makra zamiast normalnej funkcji.

Po trzecie, widzisz ciąg znaków `"Witaj, świecie!"`. Przekazujemy ten ciąg jako argument do `println!`, a ciąg zostaje wyświetlony na ekranie.

Po czwarte, linię kończymy średnikiem (`;`), co wskazuje, że to wyrażenie się zakończyło, a kolejne jest gotowe do rozpoczęcia. Większość linii kodu w języku Rust kończy się średnikiem.

_Możesz odnieść się do następnego rozdziału w podręczniku języka Rust: [Witaj, świecie!](https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html)_