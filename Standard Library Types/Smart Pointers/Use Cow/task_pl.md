## Użycie `Cow`

To zadanie bada typ `Cow`, czyli Clone-On-Write.  
`Cow` jest inteligentnym wskaźnikiem typu clone-on-write.  
Może obejmować i zapewniać niemutowalny dostęp do wypożyczonych danych, a także klonować dane leniwie, gdy wymagana jest mutacja lub przeniesienie własności.  
Typ ten został zaprojektowany, aby działać z ogólnie wypożyczonymi danymi za pomocą cechy `Borrow`.

Najpierw spójrz na funkcję `abs_all`: zmienia ona podany slice, jeśli zawiera elementy ujemne.  

Twoim celem w każdym z czterech przypadków (`case1`, `case2`, `case3` i `case4`) jest określenie, czy wyniki wywołań funkcji `Cow::from` i `abs_all` są `Cow::Borrowed` czy `Cow::Owned`.  

<div class="hint">
Upewnij się, że rozumiesz dokładnie, co jest przekazywane do <code>Cow::from</code> w każdym przypadku.  
Czy w każdym z nich zostaje przekazana własność?
</div>

<div class="hint">
A co z wektorem w <code>case3</code> i <code>case4</code>, czy jest wypożyczony, czy posiada własność po <code>Cow::from</code>?
</div>

<div class="hint">
Ponieważ wektor w <code>case4</code> już posiada własność, typ <code>Cow</code> nie musi go klonować,  
nawet jeśli musimy go zmodyfikować.
</div>

<div class="hint">Zapoznaj się z <a href="https://doc.rust-lang.org/std/borrow/enum.Cow.html">dokumentacją  
na temat typu <code>Cow</code></a>.</div>