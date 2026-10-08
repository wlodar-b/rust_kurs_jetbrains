### Wartości Enum z różnymi danymi

Istnieje jeszcze jedna zaleta używania enum zamiast struktury: każda wariant może mieć różne typy i ilości danych powiązanych. Adresy IP wersji czwartej zawsze będą miały cztery składniki numeryczne o wartościach pomiędzy 0 a 255. Gdybyśmy chcieli przechowywać adresy `V4` jako cztery wartości typu `u8`, a jednocześnie wyrażać adresy `V6` jako jedną wartość typu `String`, struktura nie pozwoliłaby nam tego łatwo osiągnąć. Enumy rozwiązują ten problem z łatwością:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

Przedstawiliśmy kilka różnych sposobów definiowania struktur danych, które przechowują adresy IP wersji czwartej i szóstej. Jak się okazuje, potrzeba przechowywania adresów IP i określania, jakiego są rodzaju, jest na tyle powszechna, że [standardowa biblioteka zawiera definicję, którą możemy wykorzystać!][IpAddr]<!-- ignore --> Spójrzmy, jak standardowa biblioteka definiuje `IpAddr`: enum i jego warianty są takie same, jak te, które zdefiniowaliśmy i używaliśmy, ale dane adresu w wariantach są osadzone w formie dwóch różnych struktur, które są odpowiednio zdefiniowane dla każdego wariantu:

[IpAddr]: https://doc.rust-lang.org/std/net/enum.IpAddr.html

```rust
struct Ipv4Addr {
    // --pominięto--
}

struct Ipv6Addr {
    // --pominięto--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

Ten kod pokazuje, że w wariancie enum można umieszczać dowolny rodzaj danych: ciągi znaków, typy numeryczne czy struktury. Można nawet włączyć inny enum! Typy w standardowej bibliotece są często niezbyt bardziej skomplikowane od tych, które sami moglibyśmy wymyślić.

Zauważ, że mimo iż standardowa biblioteka zawiera definicję `IpAddr`, nadal możemy stworzyć i używać własnej definicji bez konfliktu, ponieważ nie wprowadziliśmy definicji z biblioteki standardowej do naszego zakresu. Więcej na temat wprowadzania typów do zakresu omówimy [później](course://Modules/Modules/Bringing Paths into Scope with the use Keyword).