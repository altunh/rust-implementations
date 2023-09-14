//! This is an implementation of a proxy pattern heavily based on the `leptos_reactive` crate,
//! Part of the `leptos` framework https://crates.io/crates/leptos.
//!
//! This implementation combines the proxy + singleton + observer patterns.
//! For an implementation of the observer pattern, check out the `observer` module in this crate.
//!
//! Many UI frameworks have reactive values that appear in this module.
//! It's a good way to see and understand how values can be proxied using their getters/setters
//! and made reactive. For a good explanation of reactivity in Vue, check out:
//! https://vuejs.org/guide/extras/reactivity-in-depth.html
//!
//! Also, check out the video by the creator of `leptos`:
//! https://youtu.be/UrMHPrumJEs?si=S5LrQNGS-fnNL8WJ
//!
//! Crate Link: https://crates.io/crates/leptos_reactive
//! Github Link: https://github.com/leptos-rs/leptos/blob/main/leptos_reactive

mod effect;
mod memo;
mod node;
mod reference;
mod runtime;
mod watch;
mod computed;

#[cfg(test)]
mod tests {
    use crate::proxy::reference::create_ref;

    #[test]
    fn ref_test() {
        // create a ref
        let number = reference(0);

        // watch value change
        let mut number_change_counter = 0;
        number.watch(|curr, prev| number_change_counter += 1);

        // memoized value also watches for change
        let doubled = number.memoized(|x| (*x) * 2);
        assert_eq!(number.get(), 0);

        // create computed values with multiple dependencies
        let counter = create_ref(0);
        let computed = computed(|| number.get() * counter.get());

        // update the value two times
        value.set(1);
        value.update(|x| *x = 2);

        // at this point there should be two changes to `value`
        assert_eq!(change_counter, 2);

        // and doubled should be equal to 2
        assert_eq!(doubled.get(), 4);
    }
}
