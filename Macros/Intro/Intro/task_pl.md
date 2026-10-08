## Makra

W całej tej książce korzystaliśmy z makr, takich jak `println!`, ale nie omówiliśmy jeszcze w pełni, czym jest makro i jak działa. Termin _makro_ odnosi się do rodziny funkcji w języku Rust: _deklaratywnych_ makr za pomocą `macro_rules!` oraz trzech rodzajów _proceduralnych_ makr:

*   Niestandardowe makra `#[derive]`, które określają kod dodany za pomocą atrybutu `derive`, używanego w strukturach (structs) i wyliczeniach (enums)  
*   Makra przypominające atrybuty, które definiują niestandardowe atrybuty możliwe do użycia na dowolnym elemencie  
*   Makra przypominające funkcje, które wyglądają jak wywołania funkcji, ale działają na tokenach wyspecyfikowanych jako ich argumenty  

Omówimy każdy z tych rodzajów po kolei, ale najpierw przyjrzyjmy się, dlaczego w ogóle potrzebujemy makr, skoro mamy już funkcje.