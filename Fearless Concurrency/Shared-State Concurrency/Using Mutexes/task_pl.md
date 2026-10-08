### Używanie Mutexów do Udostępniania Danych Jednemu Wątkowi na Raz

_Mutex_ to skrót od _mutual exclusion_ (wzajemne wykluczanie), co oznacza, że mutex pozwala tylko jednemu wątkowi na dostęp do danych w danym momencie. Aby uzyskać dostęp do danych przechowywanych w mutexie, wątek musi najpierw sygnalizować chęć dostępu, próbując przejąć _lock_ mutexa. Lock to struktura danych będąca częścią mutexa, odpowiedzialna za śledzenie, kto aktualnie ma wyłączny dostęp do danych. W związku z tym mutex opisuje się jako _chroniący_ dane, które przechowuje, za pomocą systemu blokad.

Mutexy mają reputację trudnych w użyciu, ponieważ trzeba pamiętać o dwóch zasadach:

*   Należy spróbować przejąć lock przed użyciem danych.
*   Po zakończeniu korzystania z danych chronionych przez mutex należy je odblokować, aby inne wątki mogły przejąć lock.

Dla zobrazowania działania mutexa można przywołać metaforę panelu dyskusyjnego na konferencji, gdzie dostępny jest tylko jeden mikrofon. Zanim panelista będzie mógł coś powiedzieć, musi poprosić lub zasygnalizować chęć użycia mikrofonu. Kiedy go otrzyma, może mówić tak długo, jak chce, a następnie przekazać mikrofon kolejnemu paneliście, który zgłosi chęć wypowiedzi. Jeśli któryś panelista zapomni przekazać mikrofon dalej po skończeniu wypowiedzi, nikt inny nie będzie miał możliwości zabrania głosu. Jeśli zarządzanie współdzielonym mikrofonem zostanie źle poprowadzone, panel nie będzie funkcjonował zgodnie z planem!

Zarządzanie mutexami może być niezwykle trudne do prawidłowego wykonania, dlatego wiele osób woli korzystać z kanałów. Jednak dzięki systemowi typów i zasadom własności w języku Rust, nie można popełnić błędu w blokowaniu i odblokowywaniu mutexa.