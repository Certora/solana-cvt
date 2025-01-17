
#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_assume(c: bool){
    crate::asserts::cvlr_assume!(c);
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_assert(c: bool){
    crate::asserts::cvlr_assert!(c)
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_satisfy(c: bool){
    crate::asserts::cvlr_satisfy!(c);
}

