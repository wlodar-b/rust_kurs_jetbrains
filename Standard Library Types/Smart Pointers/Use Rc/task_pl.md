## Użyj `Rc`

W tym ćwiczeniu chcemy wyrazić koncepcję wielu właścicieli za pomocą typu `Rc<T>`.  
To jest model naszego układu słonecznego — mamy typ `Sun` oraz wiele `Planet`.  
`Planet` przejmują własność `Sun`, wskazując, że krążą wokół Słońca.

Spraw, aby ten kod się kompilował, korzystając z właściwych prymitywów `Rc` do wyrażenia faktu, że `Sun` ma wielu właścicieli.

<div class="hint">
To proste ćwiczenie na użycie typu <code>Rc<T></code>. Każda <code>Planet</code> ma
własność <code>Sun</code> i używa <code>Rc::clone()</code>, aby zwiększyć licznik referencji dla <code>Sun</code>.  
Po użyciu <code>drop()</code>, aby usunąć poszczególne <code>Planet</code> ze zasięgu, licznik referencji zmniejsza się.  
Na końcu Słońce ma tylko jedno odniesienie, do samego siebie. <a href="https://doc.rust-lang.org/book/ch15-04-rc.html">Więcej w Książce</a>.
</div>