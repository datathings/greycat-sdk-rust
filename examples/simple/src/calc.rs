use greycat::prelude::*;

#[greycat_fn]
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[greycat_fn]
fn sub(a: i32, b: i32) -> i32 {
    a - b
}


#[greycat_type]
pub struct DoubleInputCalc;

#[greycat_impl]
impl DoubleInputCalc {
    pub fn sum(&self, ctx: GcMachine) -> i64 {
        self.a(ctx) + self.b(ctx)
    }

    pub fn sub(&self, ctx: GcMachine) -> i64 {
        self.a(ctx) - self.b(ctx)
    }
}
