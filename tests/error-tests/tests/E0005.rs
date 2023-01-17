// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![allow(unused)]

fn main() {
    let x = Some(1);
    let Some(y) = x;
//      ^^^^^^^ERR pattern `None` not covered
//      ^^^^^^^ERR refutable pattern in local binding
//      ^^^^^^^MSG(>=1.39.0-beta,<1.44.0-beta) See Also: ↑:1
//      ^^^^^^^MSG(>=1.44.0-beta,<1.68.0-beta) See Also (external): option.rs:
//      ^^^^^^^MSG(>=1.61.0-beta,<1.68.0-beta) See Also (external): option.rs:
//      ^^^^^^^NOTE(>=1.40.0-beta) `let` bindings require
//      ^^^^^^^NOTE(>=1.40.0-beta) for more information
//      ^^^^^^^NOTE(>=1.44.0-beta) the matched value
//  ^^^^^^^^^^^^^^^^HELP(>=1.40.0-beta,<1.61.0-beta) you might want to use `if let`
//  ^^^^^^^^^^^^^^^^HELP(>=1.40.0-beta,<1.61.0-beta) /Accept Replacement:.*/
//  |HELP(>=1.61.0-beta,<1.68.0-beta) /Accept Replacement:.*/
//                 |HELP(>=1.61.0-beta,<1.68.0-beta) you might want to use `if let`
//                 |HELP(>=1.61.0-beta,<1.68.0-beta) you might want to use `if let`
//                 |HELP(>=1.68.0-beta) you might want to use `let else`
//                 |HELP(>=1.61.0-beta,<1.68.0-beta) /Accept Replacement:.*/
//                 |HELP(>=1.66.0-beta,<1.68.0-beta) alternatively, you might want to use
//                 |HELP(>=1.66.0-beta) /Accept Replacement:.*todo!.*/
}
// Bug: https://github.com/rust-lang/rust/issues/64769
// start-msg: ERR(>=1.39.0-beta,<1.44.0-beta) not covered
// start-msg: MSG(>=1.39.0-beta,<1.44.0-beta) See Primary: ↓:14
