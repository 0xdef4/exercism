pub use std::collections::HashMap;

#[macro_export]
macro_rules! hashmap {
    () => {
        $crate::HashMap::new()
    };
    ($($key: expr => $value: expr),+ $(,)?) => {{
        // check that count is const
        const count: usize = $crate::count![@COUNT; $($key),*];

        let mut hm = $crate::HashMap::with_capacity(count);
        $(hm.insert($key, $value);)+
        hm
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($element: expr),*) => {
        <[()]>::len(&[$($crate::count![@SUBST; $element]),*])
    };
    (@SUBST; $_element:expr) => { () };
}
