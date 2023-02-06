window.SIDEBAR_ITEMS = {"derive":[["Yokeable","Custom derive for `yoke::Yokeable`,"]],"mod":[["either","Types to enable polymorphic carts."],["erased","This module contains helper types for erasing Cart types."],["trait_hack","Workarounds for adding trait bounds to `yoke` objects."]],"struct":[["Yoke","A Cow-like borrowed object “yoked” to its backing data."]],"trait":[["CloneableCart","This trait marks cart types that do not change source on cloning"],["Yokeable","The `Yokeable<'a>` trait is implemented on the `'static` version of any zero-copy type; for example, `Cow<'static, T>` implements `Yokeable<'a>` (for all `'a`). One can use `Yokeable::Output` on this trait to obtain the “lifetime’d” value of the `Cow<'static, T>`, e.g. `<Cow<'static, T> as Yokeable<'a>'>::Output` is `Cow<'a, T>`."]]};