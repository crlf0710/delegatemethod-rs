#[macro_export]
macro_rules! delegate_method {
    ( $fld:ident as $fldty:ty : ) => {

    };

    ( $fld:ident : $($rest:tt)*) => {
        delegate_method!($fld as () : $($rest)*)
    };

    ( $fld_last:ident as $fldty_last:ty : $fld:ident : $($rest:tt)*) => {
        delegate_method!($fld : $($rest)*);
    };

    ( $fld_last:ident as $fldty_last:ty : $fld:ident as $fldty:ty: $($rest:tt)*) => {
        delegate_method!($fld as $fldty: $($rest)*);
    };

    ( $fld:ident as $fldty:ty : fn $fcn:ident (self) $(-> $r:ty)* ; $($rest:tt)* ) => {
        fn $fcn ( self ) $(-> $r)* {
            (self.$fld).$fcn()
        }
        delegate_method!($fld as $fldty: $($rest)*);
    };

    ( $fld:ident as $fldty:ty : pub fn $fcn:ident (self) $(-> $r:ty)*; $($rest:tt)* ) => {
        pub fn $fcn ( self ) $(-> $r)* {
            (self.$fld).$fcn()
        }
        delegate_method!($fld as $fldty: $($rest)*);
    };

    ( $fld:ident as $fldty:ty : fn $fcn:ident (&self) $(-> $r:ty)*; $($rest:tt)* ) => {
        fn $fcn ( &self ) $(-> $r)* {
            (self.$fld).$fcn()
        }
        delegate_method!($fld as $fldty: $($rest)*);
    };

    ( $fld:ident as $fldty:ty : pub fn $fcn:ident (&self) $(-> $r:ty)*; $($rest:tt)* ) => {
        pub fn $fcn ( &self ) $(-> $r)* {
            (self.$fld).$fcn()
        }
        delegate_method!($fld as $fldty: $($rest)*);
    };

    ( $fld:ident as $fldty:ty : fn $fcn:ident (&mut self) $(-> $r:ty)*; $($rest:tt)* ) => {
        fn $fcn ( &mut self ) $(-> $r)* {
            (self.$fld).$fcn()
        }

        delegate_method!($fld as $fldty: $($rest)*);
    };

    ( $fld:ident as $fldty:ty : pub fn $fcn:ident (&mut self) $(-> $r:ty)*; $($rest:tt)* ) => {
        pub fn $fcn ( &mut self ) $(-> $r)* {
            (self.$fld).$fcn()
        }

        delegate_method!($fld as $fldty: $($rest)*);
    };

    ( $fld:ident as $fldty:ty : fn $fcn:ident (self, $( $a:ident : $at:ty ),* ) $(-> $r:ty)*; $($rest:tt)* ) => {
        fn $fcn ( self, $( $a : $at ),* ) $(-> $r)* {
            (self.$fld).$fcn( $( $a ),* )
        }
        delegate_method!($fld as $fldty: $($rest)*);
    };

    ( $fld:ident as $fldty:ty : pub fn $fcn:ident (self, $( $a:ident : $at:ty ),* ) $(-> $r:ty)*; $($rest:tt)* ) => {
        pub fn $fcn ( self, $( $a : $at ),* ) $(-> $r)* {
            (self.$fld).$fcn( $( $a ),* )
        }
        delegate_method!($fld as $fldty: $($rest)*);
    };

    ( $fld:ident as $fldty:ty : fn $fcn:ident (&self, $( $a:ident : $at:ty ),* ) $(-> $r:ty)*; $($rest:tt)* ) => {
        fn $fcn ( &self, $( $a : $at ),* ) $(-> $r)* {
            (self.$fld).$fcn( $( $a ),* )
        }
        delegate_method!($fld as $fldty: $($rest)*);
    };

    ( $fld:ident as $fldty:ty : pub fn $fcn:ident (&self, $( $a:ident : $at:ty ),* ) $(-> $r:ty)*; $($rest:tt)* ) => {
        pub fn $fcn ( &self, $( $a : $at ),* ) $(-> $r)* {
            (self.$fld).$fcn( $( $a ),* )
        }
        delegate_method!($fld as $fldty: $($rest)*);
    };

    ( $fld:ident as $fldty:ty : fn $fcn:ident (&mut self, $( $a:ident : $at:ty ),* ) $(-> $r:ty)*; $($rest:tt)* ) => {
        fn $fcn ( &mut self, $( $a : $at ),* ) $(-> $r)* {
            (self.$fld).$fcn( $( $a ),* )
        }

        delegate_method!($fld as $fldty: $($rest)*);
    };

    ( $fld:ident as $fldty:ty : pub fn $fcn:ident (&mut self, $( $a:ident : $at:ty ),* ) $(-> $r:ty)*; $($rest:tt)* ) => {
        pub fn $fcn ( &mut self, $( $a : $at ),* ) $(-> $r)* {
            (self.$fld).$fcn( $( $a ),* )
        }

        delegate_method!($fld as $fldty: $($rest)*);
    };


    ( $fld:ident as $fldty:ty : fn $fcn:ident ($( $a:ident : $at:ty ),* ) $(-> $r:ty)*; $($rest:tt)* ) => {
        fn $fcn ( $( $a : $at ),* ) $(-> $r)* {
            <$fldty>::$fcn( $( $a ),* )
        }
        delegate_method!($fld as $fldty: $($rest)*);
    };

    ( $fld:ident as $fldty:ty : pub fn $fcn:ident ($( $a:ident : $at:ty ),* ) $(-> $r:ty)*; $($rest:tt)* ) => {
        pub fn $fcn ( $( $a : $at ),* ) $(-> $r)* {
            <$fldty>::$fcn( $( $a ),* )
        }
        delegate_method!($fld as $fldty: $($rest)*);
    };

}


#[cfg(test)]
mod tests {
    #[derive(Copy, Clone)]
    struct Inner {
        data: usize,
    }

    impl Inner {
        pub fn new_inner() -> Self {
            Inner { data: 42 }
        }

        pub fn new_inner_add(val: usize) -> Self {
            Inner { data: 42 + val }
        }

        pub fn noop(&self) {}

        pub fn get(&self) -> usize {
            self.data
        }

        pub fn get_add(&self, val: usize) -> usize {
            self.data + val
        }

        pub fn reset(&mut self) -> usize {
            self.data = 0;
            self.data
        }

        pub fn set(&mut self, val: usize) -> usize {
            self.data = val;
            self.data
        }

        pub fn to_data(self) -> usize {
            self.data
        }

        pub fn to_data_add(self, val: usize) -> usize {
            self.data + val
        }
    }

    #[test]
    fn test_delegate() {
        #[derive(Copy, Clone)]
        struct Outer {
            inner: Inner,
        }

        impl Outer {
            delegate_method! {
                inner as Inner:
                pub fn new_inner() -> Inner;
                pub fn new_inner_add(val: usize) -> Inner;
                pub fn noop(&self);
                pub fn get(&self) -> usize;
                pub fn get_add(&self, val: usize) -> usize;
                pub fn reset(&mut self) -> usize;
                pub fn set(&mut self, val: usize) -> usize;
                inner as Inner:
                pub fn to_data(self) -> usize;
                pub fn to_data_add(self, val: usize) -> usize;
            }
        }

        assert_eq!(Outer::new_inner().get(), 42);
        assert_eq!(Outer::new_inner_add(58).get(), 100);

        let mut x = Outer { inner: Inner::new_inner() };

        assert_eq!(x.get(), 42);
        assert_eq!(x.get_add(58), 100);
        x.noop();

        x.reset();
        assert_eq!(x.get(), 0);

        x.set(36);
        assert_eq!(x.get(), 36);

        assert_eq!(x.clone().to_data(), 36);
        assert_eq!(x.clone().to_data_add(4), 40);
    }
}
