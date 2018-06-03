// Copyright 2017 Serde Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate serde_derive;

#[derive(Deserialize)] //~ ERROR: 12:10: 12:21: #[serde(other)] must be on a unit variant
#[serde(field_identifier)]
enum F {
    A,
    #[serde(other)]
    Other(u8, u8),
}

fn main() {}
