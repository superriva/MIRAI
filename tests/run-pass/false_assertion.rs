// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

// A test that uses a false assertion

pub fn test_assert() {
    let i = 1;
    debug_assert!(i == 2);  //~ assertion failed: i == 2
}
