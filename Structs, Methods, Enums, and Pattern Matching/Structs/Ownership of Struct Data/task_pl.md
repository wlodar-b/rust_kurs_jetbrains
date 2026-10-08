### Własność danych w strukturach

W definicji struktury `User` w pierwszym fragmencie kodu tej sekcji użyliśmy typu `String` — będącego własnością — zamiast typu `&str`, który jest wycinkiem ciągu znaków. To celowy wybór, ponieważ chcemy, aby instancje tej struktury były właścicielami wszystkich swoich danych, a dane te były ważne tak długo, jak długo ważna jest cała struktura.

Możliwe jest, aby struktury przechowywały referencje do danych będących własnością czegoś innego, ale do tego konieczne jest użycie *lifetime'ów* — funkcjonalności Rust, o której będziemy mówić w rozdziale „Typy generyczne, cechy i lifetime’y”. Lifetime’y zapewniają, że dane, do których struktura odwołuje się za pomocą referencji, pozostają ważne tak długo, jak ważna jest sama struktura. Załóżmy, że próbujesz przechowywać referencję w strukturze bez określania lifetime'ów, jak w poniższym przykładzie. Taki kod jednak nie zadziała:

```rust,ignore,does_not_compile
 struct User {
     username: &str,
     email: &str,
     sign_in_count: u64,
     active: bool,
 }

 fn main() {
     let user1 = User {
         email: "someone@example.com",
         username: "someusername123",
         active: true,
         sign_in_count: 1,
     };
 }
```

Kompilator zgłosi, że wymagane są specyfikatory lifetime'ów:

```console
 $ cargo run
    Compiling structs v0.1.0 (file:///projects/structs)
 error[E0106]: missing lifetime specifier
  --> src/main.rs:2:15
   |
 2 |     username: &str,
   |               ^ oczekiwany nazwany parametr lifetime
   |
 help: rozważ wprowadzenie nazwanego parametru lifetime
   |
 1 | struct User<'a> {
 2 |     username: &'a str,
   |

 error[E0106]: missing lifetime specifier
  --> src/main.rs:3:12
   |
 3 |     email: &str,
   |            ^ oczekiwany nazwany parametr lifetime
   |
 help: rozważ wprowadzenie nazwanego parametru lifetime
   |
 1 | struct User<'a> {
 2 |     username: &str,
 3 |     email: &'a str,
   |

 error: przerwano z powodu 2 wcześniejszych błędów

 Aby uzyskać więcej informacji na temat tego błędu, użyj `rustc --explain E0106`.
 error: nie udało się skompilować `structs`

 Aby uzyskać więcej informacji, uruchom ponownie polecenie z argumentem --verbose.
```

W rozdziale „Typy generyczne, cechy i lifetime’y” omówimy, jak naprawić tego rodzaju błędy, aby móc przechowywać referencje w strukturach. Na razie jednak naprawimy te błędy, używając typów będących własnością, takich jak `String`, zamiast referencji, takich jak `&str`.