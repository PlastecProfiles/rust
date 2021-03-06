// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test that the type variable in the type(`Vec<_>`) of a closed over
// variable does not interfere with type inference.

// pretty-expanded FIXME #23616

fn f<F: FnMut()>(mut f: F) {
    f();
}

fn main() {
    let mut v: Vec<_> = vec![];
    f(|| v.push(0));
    assert_eq!(v, [0]);
}
