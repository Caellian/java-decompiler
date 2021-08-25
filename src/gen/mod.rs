use crate::class::Class;
use std::io::Write;

pub mod java;

pub trait CodeGenerator {
    const NAME: &'static str;

    fn generate<W: Write>(&self, class: &Class, w: &mut W);
}
