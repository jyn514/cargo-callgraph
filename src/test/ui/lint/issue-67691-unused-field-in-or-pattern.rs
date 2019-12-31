// run-rustfix

#![feature(or_patterns)]
#![deny(unused)]

pub enum MyEnum {
    A { i: i32, j: i32 },
    B { i: i32, j: i32 },
}

pub fn no_ref(x: MyEnum) {
    use MyEnum::*;

    match x {
        A { i, j } | B { i, j } => { //~ ERROR unused variable
            println!("{}", i);
        }
    }
}

pub fn with_ref(x: MyEnum) {
    use MyEnum::*;

    match x {
        A { i, ref j } | B { i, ref j } => { //~ ERROR unused variable
            println!("{}", i);
        }
    }
}

pub fn inner_no_ref(x: Option<MyEnum>) {
    use MyEnum::*;

    match x {
        Some(A { i, j } | B { i, j }) => { //~ ERROR unused variable
            println!("{}", i);
        }

        _ => {}
    }
}

pub fn inner_with_ref(x: Option<MyEnum>) {
    use MyEnum::*;

    match x {
        Some(A { i, ref j } | B { i, ref j }) => { //~ ERROR unused variable
            println!("{}", i);
        }

        _ => {}
    }
}

pub fn main() {
    no_ref(MyEnum::A { i: 1, j: 2 });
    with_ref(MyEnum::A { i: 1, j: 2 });

    inner_no_ref(Some(MyEnum::A { i: 1, j: 2 }));
    inner_with_ref(Some(MyEnum::A { i: 1, j: 2 }));
}
