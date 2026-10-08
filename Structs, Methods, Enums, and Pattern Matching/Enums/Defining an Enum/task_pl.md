## Definiowanie Enum

Przyjrzyjmy się sytuacji, którą możemy chcieć wyrazić w kodzie, i zobaczmy, dlaczego enumy są przydatne i bardziej odpowiednie niż struktury w tym przypadku. Powiedzmy, że musimy pracować z adresami IP. Obecnie istnieją dwa główne standardy adresów IP: wersja czwarta i wersja szósta. To są jedyne możliwości dla adresu IP, które nasz program napotka: możemy *wyliczyć* wszystkie możliwe warianty, co jest genezą nazwy "enumeracja".

Każdy adres IP może być albo w wersji czwartej, albo w wersji szóstej, ale nie jednocześnie w obu. Ta właściwość adresów IP sprawia, że struktura danych enum jest odpowiednia, ponieważ wartości enum mogą być tylko jednym z jej wariantów. Zarówno adresy w wersji czwartej, jak i w wersji szóstej są fundamentalnie adresami IP, więc powinny być traktowane jako ten sam typ, gdy kod obsługuje sytuacje dotyczące dowolnego rodzaju adresu IP.

Możemy wyrazić tę koncepcję w kodzie, definiując enumerację `IpAddrKind` i wymieniając możliwe rodzaje adresu IP: `V4` i `V6`. Są to warianty enumu:

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

`IpAddrKind` jest teraz typem danych niestandardowym, który możemy używać w innych miejscach w naszym kodzie.

### Wartości Enum

Możemy tworzyć instancje każdego z dwóch wariantów `IpAddrKind` w następujący sposób:

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

Zauważ, że warianty enum są przestrzenią nazw pod jego identyfikatorem, a do ich oddzielenia używamy podwójnego dwukropka. Powodem, dla którego jest to użyteczne, jest to, że teraz obie wartości `IpAddrKind::V4` i `IpAddrKind::V6` należą do tego samego typu: `IpAddrKind`. Możemy więc na przykład zdefiniować funkcję, która przyjmuje dowolny `IpAddrKind`:

```rust
fn route(ip_kind: IpAddrKind) {}
```

I możemy wywołać tę funkcję z dowolnym wariantem:

```rust
route(IpAddrKind::V4);
route(IpAddrKind::V6);