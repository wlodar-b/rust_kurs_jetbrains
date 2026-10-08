## Opanowanie IDE: Wprowadzenie refaktoryzacji zmiennej

W 1993 roku programista napisał kod, aby obliczyć, jaki był rok dziesięć, pięć i jeden rok wcześniej.

W 2021 roku inny programista natknął się na ten kod i zauważył, że nie daje on już poprawnych wyników.

Ten programista postanowił [zrefaktoryzować kod](https://pl.wikipedia.org/wiki/Refaktoryzacja_kodu)  
i wyodrębnić `1993` do nowej zmiennej o nazwie `year`.  
Dzięki temu przyszli programiści, którzy natkną się na ten kod, będą mogli go naprawić, zmieniając tylko jedno miejsce, zamiast trzech.

Zrefaktoryzujmy również kod!

%IDE_NAME% oferuje przydatną funkcję o nazwie **„Wprowadzenie refaktoryzacji zmiennej”**, która pozwala na szybkie przeprowadzenie tej operacji.

Zaznacz dowolne wystąpienie `1993`, a następnie naciśnij &shortcut:IntroduceVariable;  
lub wybierz *Refaktoryzuj -> Wprowadź zmienną...* z menu kontekstowego.  

*Uwaga*: możesz również użyć &shortcut:IntroduceConstant; lub *Refaktoryzuj -> Wprowadź stałą...* z menu kontekstowego,  
aby utworzyć stałe.