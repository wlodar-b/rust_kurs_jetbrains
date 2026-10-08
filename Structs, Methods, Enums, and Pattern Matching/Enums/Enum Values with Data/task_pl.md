### Wartości enum z danymi

Korzystanie z enumów ma jeszcze więcej zalet. Myśląc dalej o naszym typie adresu IP, w tej chwili nie mamy sposobu, aby przechowywać faktyczne *dane* adresu IP; wiemy jedynie, jakiego jest *rodzaju*. Biorąc pod uwagę, że właśnie nauczyłeś/aś się o strukturach w poprzednim rozdziale, możesz rozwiązać ten problem w sposób pokazany w poniższym fragmencie kodu.

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

#### Przechowywanie danych oraz wariantu `IpAddrKind` adresu IP za pomocą `struct`

Tutaj zdefiniowaliśmy strukturę `IpAddr`, która posiada dwa pola: pole `kind` typu `IpAddrKind` (enum, który wcześniej zdefiniowaliśmy) oraz pole `address` typu `String`. Mamy dwa wystąpienia tej struktury. Pierwsze, `home`, posiada wartość `IpAddrKind::V4` jako `kind` z powiązanymi danymi adresu `127.0.0.1`. Drugie wystąpienie, `loopback`, posiada inną wersję `IpAddrKind` jako wartość dla `kind`, czyli `V6`, oraz ma powiązany z nim adres `::1`. Użyliśmy struktury, aby połączyć wartości `kind` i `address`, dzięki czemu teraz wariant jest powiązany z wartością.

Możemy przedstawić ten sam koncept w bardziej zwięzły sposób, wykorzystując tylko enum, zamiast używania enum w strukturze, poprzez umieszczenie danych bezpośrednio w każdym wariancie enum. Nowa definicja enuma `IpAddr` wskazuje, że zarówno warianty `V4`, jak i `V6` będą miały powiązane wartości `String`:

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

Dołączamy dane bezpośrednio do każdego wariantu enum, więc nie ma potrzeby dodatkowej struktury.