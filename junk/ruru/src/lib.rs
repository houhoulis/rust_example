#![no_std]

#[macro_use]
extern crate ruru;

use ruru::VM;
use ruru::Hash;
use ruru::Fixnum;
use ruru::Class;
use ruru::AnyObject;
use ruru::types::{Argc, Value};
use ruru::traits::Object;

class!(Calculator);

methods!(
    Calculator,
    itself,

    fn pow_3(num: Fixnum) -> Hash {
        let mut hash = Hash::new();

        for i in 1..num.to_i64() + 1 {
            hash.store(Fixnum::new(i), Fixnum::new(i.pow(3)));
        }

        hash
    }
);

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn Init_librurulol() {
    Class::new("Calculator").define(|itself| {
        itself.def("pow_3", pow_3);
    });
}
