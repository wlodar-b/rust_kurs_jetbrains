## Enumy i dopasowywanie wzorców

W tym rozdziale przyjrzymy się *wyliczeniom*, znanym też jako *enumy*.  
Enumy pozwalają na zdefiniowanie typu poprzez wyliczenie jego możliwych *wariantów*. Najpierw zdefiniujemy i użyjemy enum, aby pokazać, jak enum może łączyć znaczenie z danymi. Następnie zbadamy szczególnie przydatny enum, zwany `Option`, który wyraża, że wartość może być czymś lub niczym. Potem przyjrzymy się, jak dopasowywanie wzorców w wyrażeniu `match` ułatwia uruchamianie różnego kodu dla różnych wartości enum. Na koniec omówimy, w jaki sposób konstrukcja `if let` jest kolejnym wygodnym i zwięzłym idiomem, z którego możesz skorzystać, aby obsługiwać enumy w swoim kodzie.

Enumy są funkcją występującą w wielu językach, ale ich możliwości różnią się w zależności od języka. Enumy w Rust są najbardziej podobne do *algebraicznych typów danych* w językach funkcyjnych, takich jak F#, OCaml i Haskell.