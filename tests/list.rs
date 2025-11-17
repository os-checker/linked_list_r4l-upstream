use linked_list_r4l_upstream::{list::*, prelude::*};

#[test]
fn t1() -> Result<()> {
    trait Foo {
        fn foo(&self) -> (&'static str, i32);
    }

    #[pin_data]
    struct DTWrap<T: ?Sized> {
        #[pin]
        links: ListLinksSelfPtr<DTWrap<dyn Foo>>,
        value: T,
    }

    impl<T> DTWrap<T> {
        fn new(value: T) -> Result<ListArc<Self>> {
            ListArc::pin_init(
                try_pin_init!(Self {
                    value,
                    links <- ListLinksSelfPtr::new(),
                }),
                GFP_KERNEL,
            )
        }
    }

    impl_list_arc_safe! {
        impl{T: ?Sized} ListArcSafe<0> for DTWrap<T> { untracked; }
    }
    impl_list_item! {
        impl ListItem<0> for DTWrap<dyn Foo> { using ListLinksSelfPtr { self.links }; }
    }

    // Create a new empty list.
    let mut list = List::<DTWrap<dyn Foo>>::new();
    {
        assert!(list.is_empty());
    }

    struct A(i32);
    // `A` returns the inner value for `foo`.
    impl Foo for A {
        fn foo(&self) -> (&'static str, i32) {
            ("a", self.0)
        }
    }

    struct B;
    // `B` always returns 42.
    impl Foo for B {
        fn foo(&self) -> (&'static str, i32) {
            ("b", 42)
        }
    }

    // Insert 3 element using `push_back()`.
    list.push_back(DTWrap::new(A(15))?);
    list.push_back(DTWrap::new(A(32))?);
    list.push_back(DTWrap::new(B)?);

    // Iterate over the list to verify the nodes were inserted correctly.
    // [A(15), A(32), B]
    {
        let mut iter = list.iter();
        assert_eq!(iter.next().ok_or(EINVAL)?.value.foo(), ("a", 15));
        assert_eq!(iter.next().ok_or(EINVAL)?.value.foo(), ("a", 32));
        assert_eq!(iter.next().ok_or(EINVAL)?.value.foo(), ("b", 42));
        assert!(iter.next().is_none());

        // Verify the length of the list.
        assert_eq!(list.iter().count(), 3);
    }

    // Pop the items from the list using `pop_back()` and verify the content.
    {
        assert_eq!(list.pop_back().ok_or(EINVAL)?.value.foo(), ("b", 42));
        assert_eq!(list.pop_back().ok_or(EINVAL)?.value.foo(), ("a", 32));
        assert_eq!(list.pop_back().ok_or(EINVAL)?.value.foo(), ("a", 15));
    }

    // Insert 3 elements using `push_front()`.
    list.push_front(DTWrap::new(A(15))?);
    list.push_front(DTWrap::new(A(32))?);
    list.push_front(DTWrap::new(B)?);

    // Iterate over the list to verify the nodes were inserted correctly.
    // [B, A(32), A(15)]
    {
        let mut iter = list.iter();
        assert_eq!(iter.next().ok_or(EINVAL)?.value.foo(), ("b", 42));
        assert_eq!(iter.next().ok_or(EINVAL)?.value.foo(), ("a", 32));
        assert_eq!(iter.next().ok_or(EINVAL)?.value.foo(), ("a", 15));
        assert!(iter.next().is_none());

        // Verify the length of the list.
        assert_eq!(list.iter().count(), 3);
    }

    // Pop the items from the list using `pop_front()` and verify the content.
    {
        assert_eq!(list.pop_back().ok_or(EINVAL)?.value.foo(), ("a", 15));
        assert_eq!(list.pop_back().ok_or(EINVAL)?.value.foo(), ("a", 32));
    }

    // Push `list2` to `list` through `push_all_back()`.
    // list: [B]
    // list2: [B, A(25)]
    {
        let mut list2 = List::<DTWrap<dyn Foo>>::new();
        list2.push_back(DTWrap::new(B)?);
        list2.push_back(DTWrap::new(A(25))?);

        list.push_all_back(&mut list2);

        // list: [B, B, A(25)]
        // list2: []
        let mut iter = list.iter();
        assert_eq!(iter.next().ok_or(EINVAL)?.value.foo(), ("b", 42));
        assert_eq!(iter.next().ok_or(EINVAL)?.value.foo(), ("b", 42));
        assert_eq!(iter.next().ok_or(EINVAL)?.value.foo(), ("a", 25));
        assert!(iter.next().is_none());
        assert!(list2.is_empty());
    }

    Ok(())
}
