use greycat::prelude::*;

pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

#[greycat_object]
pub struct DoubleInputCalc;

impl DoubleInputCalc {
    pub fn sum(&self, ctx: GcMachine) -> i64 {
        self.a(ctx) + self.b(ctx)
    }

    pub fn sub(&self, ctx: GcMachine) -> i64 {
        self.a(ctx) - self.b(ctx)
    }

    pub fn finalize(&mut self, _: GcMachine) {
        // noop
    }
}
