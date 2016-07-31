#[macro_export]
macro_rules! delegate_method {
    {impl $($tt:tt)*} =>
    {delegate_method!{@impl_collect () $($tt)*}};

    {@impl_collect ($($headtt:tt)*) {$($tailtt:tt)*}} =>
    {delegate_method!{@impl_expansion ($($headtt)*) ($($tailtt)*) ()}};

    {@impl_collect ($($headtt:tt)*) $tt:tt $($tailtt:tt)*} =>
    {delegate_method!{@impl_collect ($($headtt)* $tt) $($tailtt)*}};

    {@impl_expansion_impl ($($headtt:tt)*) ($($itemtt:tt)*) ($($resttt:tt)*) ($($tailtt:tt)*)} =>
    {delegate_method!{@impl_expansion ($($headtt)*) ($($resttt)*) ($($tailtt)* #[inline] $($itemtt)*)}};

    (@impl_finalization ($($headtt:tt)*) ($($tailtt:tt)*)) =>
    {delegate_method!(@as_item #[allow(dead_code)]impl $($headtt)* { $($tailtt)* });};

    (@as_item $i:item) => { $i };

    {@impl_expansion ($($headtt:tt)*) () ($($tailtt:tt)*)} =>
    {delegate_method!(@impl_finalization ($($headtt)*) ($($tailtt)*));};

    {@impl_expansion ($($headtt:tt)*)
     (type $ascty:ident = $dstty:ty; $($resttt:tt)*) ($($tailtt:tt)*)} =>
    {delegate_method!(@impl_expansion ($($headtt)*) ($($resttt)*) ($($tailtt)* type $ascty = $dstty; ));};

    {@impl_expansion ($($headtt:tt)*)
     ($fld:tt $(as $fldty:ty)* : ) ($($tailtt:tt)*)} =>
    {delegate_method!(@impl_expansion ($($headtt)*) () ($($tailtt)*));};

    {@impl_expansion ($($headtt:tt)*)
     ($fld_last:tt $(as $fldty_last:ty)* :
      $fld:tt $(as $fldty:ty)* : $($resttt:tt)*) ($($tailtt:tt)*)} =>
    {delegate_method!(@impl_expansion ($($headtt)*)
                      ($fld $(as $fldty)* : $($resttt)*) ($($tailtt)*));};

    {@impl_expansion ($($headtt:tt)*)
     ($fld:tt : $($resttt:tt)*) ($($tailtt:tt)*)} =>
    {delegate_method!(@impl_expansion ($($headtt)*) ($fld as () : $($resttt)*) ($($tailtt)*))};

    {@impl_expansion ($($headtt:tt)*)
     ($fld:tt as $fldty:ty :
      pub fn $fcn:ident $(< $($lt:tt),* >)* ($($args:tt)*) $(-> $r:ty)* ; $($resttt:tt)* )
     ($($tailtt:tt)*)} =>
    {delegate_method!(@impl_expansion_item ($($headtt)*)
                      ($fld) ($fldty) ($fcn) (pub fn) ($($($lt),*),*) ($($args)*) ($(-> $r)*)
                      ($fld as $fldty : $($resttt)*) ($($tailtt)*));};

    {@impl_expansion ($($headtt:tt)*)
     ($fld:tt as $fldty:ty :
      fn $fcn:ident $(< $($lt:tt),* >)* ($($args:tt)*) $(-> $r:ty)* ; $($resttt:tt)* )
     ($($tailtt:tt)*)} =>
    {delegate_method!(@impl_expansion_item ($($headtt)*)
                      ($fld) ($fldty) ($fcn) (fn) ($($($lt),*),*) ($($args)*) ($(-> $r)*)
                      ($fld as $fldty : $($resttt)*) ($($tailtt)*));};

    (@impl_expansion_item ($($headtt:tt)*)
     ($fld:tt) ($fldty:ty) ($fcn:ident) ($($kwtt:tt)*) ($($gp:tt)+) ($($args:tt)*) ($($r:tt)*)
     ($($resttt:tt)*) ($($tailtt:tt)*)) =>
    {delegate_method!(@impl_expansion_gp ($($headtt)*)
                      ($fld) ($fldty) ($fcn) ($($kwtt)*) (<$($gp)*>) ($($args)*) ($($r)*)
                      ($($resttt)*) ($($tailtt)*));};

    (@impl_expansion_item ($($headtt:tt)*)
     ($fld:tt) ($fldty:ty) ($fcn:ident) ($($kwtt:tt)*) () ($($args:tt)*) ($($r:tt)*)
     ($($resttt:tt)*) ($($tailtt:tt)*)) =>
    {delegate_method!(@impl_expansion_gp ($($headtt)*)
                      ($fld) ($fldty) ($fcn) ($($kwtt)*) () ($($args)*) ($($r)*)
                      ($($resttt)*) ($($tailtt)*));};

    (@impl_expansion_gp ($($headtt:tt)*)
     ($fld:tt) ($fldty:ty) ($fcn:ident) ($($kwtt:tt)*) ($($gp:tt)*) (self) ($($r:tt)*)
     ($($resttt:tt)*) ($($tailtt:tt)*)) =>
    {delegate_method!(@impl_expansion_impl ($($headtt)*)
                      ($($kwtt)* $fcn $($gp)* (self) $($r)* { (self.$fld).$fcn() })
                      ($($resttt)*) ($($tailtt)*));};

    (@impl_expansion_gp ($($headtt:tt)*)
     ($fld:tt) ($fldty:ty) ($fcn:ident) ($($kwtt:tt)*) ($($gp:tt)*) (&self) ($($r:tt)*)
     ($($resttt:tt)*) ($($tailtt:tt)*)) =>
    {delegate_method!(@impl_expansion_impl ($($headtt)*)
                      ($($kwtt)* $fcn $($gp)* (&self) $($r)* { (self.$fld).$fcn() })
      ($($resttt)*) ($($tailtt)*));};

    (@impl_expansion_gp ($($headtt:tt)*)
    ($fld:tt) ($fldty:ty) ($fcn:ident) ($($kwtt:tt)*) ($($gp:tt)*) (&mut self) ($($r:tt)*)
    ($($resttt:tt)*) ($($tailtt:tt)*)) =>
    {delegate_method!(@impl_expansion_impl ($($headtt)*)
      ($($kwtt)* $fcn $($gp)* (&mut self) $($r)* { (self.$fld).$fcn() })
      ($($resttt)*) ($($tailtt)*));};

    (@impl_expansion_gp ($($headtt:tt)*)
   ($fld:tt) ($fldty:ty) ($fcn:ident) ($($kwtt:tt)*) ($($gp:tt)*)
   (self, $( $a:ident : $at:ty ),*) ($($r:tt)*)
    ($($resttt:tt)*) ($($tailtt:tt)*)) =>
    {delegate_method!(@impl_expansion_impl ($($headtt)*)
      ($($kwtt)* $fcn $($gp)* (self,$($a : $at)*) $($r)* { (self.$fld).$fcn($($a),*) })
      ($($resttt)*) ($($tailtt)*));};

    (@impl_expansion_gp ($($headtt:tt)*)
   ($fld:tt) ($fldty:ty) ($fcn:ident) ($($kwtt:tt)*) ($($gp:tt)*)
   (&self, $( $a:ident : $at:ty ),*) ($($r:tt)*)
    ($($resttt:tt)*) ($($tailtt:tt)*)) =>
    {delegate_method!(@impl_expansion_impl ($($headtt)*)
      ($($kwtt)* $fcn $($gp)* (&self,$($a : $at)*) $($r)* { (self.$fld).$fcn($($a),*) })
      ($($resttt)*) ($($tailtt)*));};

    (@impl_expansion_gp ($($headtt:tt)*)
   ($fld:tt) ($fldty:ty) ($fcn:ident) ($($kwtt:tt)*) ($($gp:tt)*)
   (&mut self, $( $a:ident : $at:ty ),*) ($($r:tt)*)
    ($($resttt:tt)*) ($($tailtt:tt)*)) =>
    {delegate_method!(@impl_expansion_impl ($($headtt)*)
      ($($kwtt)* $fcn $($gp)* (&mut self,$($a : $at)*) $($r)* { (self.$fld).$fcn($($a),*) })
      ($($resttt)*) ($($tailtt)*));};

    (@impl_expansion_gp ($($headtt:tt)*)
   ($fld:tt) ($fldty:ty) ($fcn:ident) ($($kwtt:tt)*) ($($gp:tt)*)
   ($( $a:ident : $at:ty ),*) ($($r:tt)*)
    ($($resttt:tt)*) ($($tailtt:tt)*)) =>
    {delegate_method!(@ impl_expansion_impl ($($headtt)*)
      ($($kwtt)* $fcn $($gp)* ($($a : $at)*) $($r)* { <$fldty>::$fcn($($a),*) })
      ($($resttt)*) ($($tailtt)*));};
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

        pub fn str_to_owned<'a>(str: &'a str) -> String {
            str.to_owned()
        }

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

        delegate_method! {
			      impl Outer {
                inner as Inner:
                fn new_inner() -> Inner;
                fn new_inner_add(val: usize) -> Inner;
                fn noop(&self);
				        fn str_to_owned<'a>(str: &'a str) -> String;
                fn get(&self) -> usize;
                fn get_add(&self, val: usize) -> usize;
                fn reset(&mut self) -> usize;
                fn set(&mut self, val: usize) -> usize;
                inner as Inner:
                fn to_data(self) -> usize;
                fn to_data_add(self, val: usize) -> usize;
            }
        }


        assert_eq!(Outer::new_inner().get(), 42);
        assert_eq!(Outer::new_inner_add(58).get(), 100);

        assert_eq!(Outer::str_to_owned("Hello"), "Hello".to_owned());

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
