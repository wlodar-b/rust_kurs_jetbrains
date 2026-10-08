### Czas życia: wychodzenie poza zakres

Następnie spróbujmy przykładu, który pokazuje, że czas życia referencji w 
`result` musi być krótszy z dwóch czasów życia argumentów. Przeniesiemy 
deklarację zmiennej `result` poza wewnętrzny zakres, ale pozostawimy przypisanie wartości 
do zmiennej `result` wewnątrz zakresu z `string2`. Następnie przeniesiemy 
`println!`, który używa `result`, poza wewnętrzny zakres, po jego zakończeniu. Kod poniżej 
nie będzie się kompilował.

```rust,ignore,does_not_compile
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

#### Próba użycia `result` po tym, jak `string2` wyszedł poza zakres

Kiedy spróbujemy skompilować ten kod, otrzymamy następujący błąd:

```console
error[E0597]: `string2` does not live long enough
 --> src/main.rs:6:44
  |
6 |         result = longest(string1.as_str(), string2.as_str());
  |                                            ^^^^^^^ pożyczona wartość nie żyje wystarczająco długo
7 |     }
  |     - `string2` zostało tutaj usunięte, gdy wciąż było pożyczone
8 |     println!("The longest string is {}", result);
  |                                          ------ pożyczka użyta później tutaj
```

Błąd pokazuje, że aby `result` było ważne dla instrukcji `println!`,
`string2` musiałoby być ważne aż do końca zewnętrznego zakresu. Rust wie 
o tym, ponieważ oznaczyliśmy czasy życia parametrów funkcji i wartości 
zwracanych, używając tego samego parametru czasu życia `'a`.

Jako ludzie możemy spojrzeć na ten kod i zauważyć, że `string1` jest dłuższe niż 
`string2` i dlatego `result` będzie zawierać referencję do `string1`. Ponieważ 
`string1` nie wyszło jeszcze poza zakres, referencja do `string1` wciąż będzie 
ważna dla instrukcji `println!`. Jednak kompilator nie jest w stanie zauważyć, że 
referencja jest w tym przypadku ważna. Powiedzieliśmy Rustowi, że czas życia 
referencji zwróconej przez funkcję `longest` jest taki sam, jak krótszy z czasów życia 
referencji przekazanych jako argumenty. Dlatego kontroler pożyczek nie 
pozwala na kod z poprzedniego listingu, uznając go za potencjalnie zawierający 
nieprawidłową referencję.

Spróbuj zaprojektować więcej eksperymentów, które będą różniły się wartościami 
i czasami życia referencji przekazanych do funkcji `longest` oraz sposobem użycia 
zwracanej referencji. Formułuj hipotezy na temat tego, czy Twoje eksperymenty 
przejdą przez kontroler pożyczek przed ich kompilacją; następnie sprawdź, czy miałeś rację!