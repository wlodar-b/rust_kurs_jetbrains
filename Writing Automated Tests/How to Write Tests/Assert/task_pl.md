## Baw się z `assert!`

Testy są ważne, aby upewnić się, że Twój kod działa tak, jak zamierzasz.

Ten test ma problem — spraw, aby test się kompilował! Spraw, aby test przeszedł! Spraw, aby test nie przeszedł!

<div class="hint">
  Nawet nie musisz pisać żadnego kodu, aby testować — możesz po prostu testować wartości i uruchamiać je, chociaż nie robiłbyś tego w prawdziwym życiu.
  <code>assert!</code> to makro, które wymaga argumentu.
  W zależności od wartości argumentu, <code>assert!</code> nie zrobi nic (w takim przypadku test przejdzie) albo <code>assert!</code> wywoła panikę (w takim przypadku test nie przejdzie).
  Spróbuj podać różne wartości do <code>assert!</code> i zobacz, które się kompilują, które przechodzą, a które nie przechodzą.
</div>