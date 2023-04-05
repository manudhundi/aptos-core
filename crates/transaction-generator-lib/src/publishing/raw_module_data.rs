// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

// This file was generated. Do not modify!
//
// To update this code, run `cargo run` from `testsuite/module-publish` in aptos core.
// That test compiles the set of modules defined in
// `testsuite/simple/src/simple/sources/`
// and it writes the binaries here.
// The module name (prefixed with `MODULE_`) is a `Lazy` instance that returns the
// byte array of the module binary.
// This create should also provide a Rust file that allows proper manipulation of each
// module defined below.

use once_cell::sync::Lazy;

#[rustfmt::skip]
pub static PACKAGE_METADATA_SIMPLE: Lazy<Vec<u8>> = Lazy::new(|| {
	vec![
		13, 71, 101, 110, 101, 114, 105, 99, 77, 111, 100, 117, 108, 101, 1, 0, 0, 0,
		0, 0, 0, 0, 0, 64, 69, 56, 55, 48, 54, 52, 53, 49, 69, 53, 51, 57,
		65, 54, 56, 49, 70, 50, 54, 49, 48, 55, 50, 55, 57, 52, 66, 66, 67, 69,
		53, 56, 54, 55, 51, 67, 49, 70, 49, 57, 69, 69, 65, 52, 50, 69, 66, 66,
		69, 70, 51, 51, 69, 51, 70, 67, 67, 53, 49, 56, 52, 48, 69, 56, 132, 1,
		31, 139, 8, 0, 0, 0, 0, 0, 2, 255, 77, 139, 59, 14, 194, 48, 16, 68,
		251, 61, 133, 229, 30, 135, 11, 80, 208, 64, 197, 9, 162, 20, 43, 123, 64, 86,
		156, 93, 203, 134, 80, 32, 238, 142, 45, 1, 138, 102, 154, 249, 188, 49, 179, 159,
		249, 134, 137, 132, 23, 152, 131, 177, 103, 8, 74, 244, 23, 13, 143, 4, 75, 43,
		74, 141, 42, 125, 217, 187, 38, 75, 52, 6, 100, 72, 128, 248, 136, 58, 209, 49,
		223, 181, 158, 74, 195, 159, 90, 230, 118, 124, 153, 164, 158, 83, 71, 156, 27, 182,
		230, 126, 221, 45, 186, 98, 184, 254, 128, 111, 249, 207, 214, 188, 233, 3, 132, 221,
		66, 189, 150, 0, 0, 0, 1, 6, 83, 105, 109, 112, 108, 101, 0, 0, 0, 3,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 14, 65, 112, 116,
		111, 115, 70, 114, 97, 109, 101, 119, 111, 114, 107, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 1, 11, 65, 112, 116, 111, 115, 83, 116, 100, 108, 105,
		98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 10, 77, 111,
		118, 101, 83, 116, 100, 108, 105, 98, 0,
	]
});

#[rustfmt::skip]
pub static MODULE_SIMPLE: Lazy<Vec<u8>> = Lazy::new(|| {
	vec![
		161, 28, 235, 11, 6, 0, 0, 0, 10, 1, 0, 6, 2, 6, 20, 3, 26, 95,
		5, 121, 150, 1, 7, 143, 2, 151, 2, 8, 166, 4, 64, 6, 230, 4, 115, 10,
		217, 5, 28, 12, 245, 5, 179, 10, 13, 168, 16, 12, 0, 0, 1, 1, 1, 2,
		0, 3, 8, 0, 0, 4, 8, 0, 0, 5, 7, 0, 0, 6, 8, 0, 2, 16,
		7, 0, 0, 7, 0, 1, 0, 0, 8, 2, 1, 0, 0, 9, 3, 4, 0, 0,
		10, 5, 1, 0, 0, 11, 5, 1, 0, 0, 12, 6, 1, 0, 0, 13, 5, 1,
		0, 0, 14, 5, 1, 0, 0, 15, 6, 1, 0, 0, 17, 7, 1, 0, 0, 18,
		8, 1, 0, 0, 19, 8, 1, 0, 0, 20, 5, 1, 0, 0, 21, 5, 1, 0,
		0, 22, 6, 1, 0, 0, 23, 9, 1, 0, 0, 24, 5, 1, 0, 1, 29, 5,
		13, 0, 2, 30, 16, 17, 0, 2, 7, 10, 2, 6, 10, 2, 0, 2, 6, 12,
		10, 2, 4, 6, 8, 3, 6, 8, 3, 6, 8, 1, 6, 8, 1, 1, 6, 3,
		1, 6, 12, 2, 6, 12, 3, 4, 6, 12, 3, 8, 4, 10, 2, 2, 6, 12,
		5, 2, 6, 12, 8, 4, 1, 3, 1, 2, 2, 7, 8, 0, 8, 0, 1, 5,
		3, 6, 3, 6, 3, 6, 3, 3, 3, 8, 3, 7, 8, 3, 1, 10, 2, 1,
		8, 4, 3, 10, 3, 10, 3, 3, 3, 8, 2, 7, 8, 3, 8, 3, 8, 1,
		10, 2, 7, 8, 3, 10, 2, 3, 3, 8, 3, 7, 8, 3, 9, 3, 7, 8,
		3, 3, 3, 3, 8, 3, 7, 8, 3, 6, 8, 3, 6, 8, 3, 2, 7, 8,
		3, 8, 3, 2, 8, 3, 7, 8, 3, 1, 7, 8, 1, 6, 83, 105, 109, 112,
		108, 101, 6, 115, 105, 103, 110, 101, 114, 6, 115, 116, 114, 105, 110, 103, 12, 66,
		121, 116, 101, 82, 101, 115, 111, 117, 114, 99, 101, 7, 67, 111, 117, 110, 116, 101,
		114, 4, 68, 97, 116, 97, 8, 82, 101, 115, 111, 117, 114, 99, 101, 11, 97, 112,
		112, 101, 110, 100, 95, 100, 97, 116, 97, 20, 98, 121, 116, 101, 115, 95, 109, 97,
		107, 101, 95, 111, 114, 95, 99, 104, 97, 110, 103, 101, 14, 99, 111, 112, 121, 95,
		112, 97, 115, 116, 97, 95, 114, 101, 102, 6, 100, 111, 117, 98, 108, 101, 11, 103,
		101, 116, 95, 99, 111, 117, 110, 116, 101, 114, 21, 103, 101, 116, 95, 102, 114, 111,
		109, 95, 114, 97, 110, 100, 111, 109, 95, 99, 111, 110, 115, 116, 4, 104, 97, 108,
		102, 11, 105, 110, 105, 116, 95, 109, 111, 100, 117, 108, 101, 5, 108, 111, 111, 112,
		121, 6, 83, 116, 114, 105, 110, 103, 14, 109, 97, 107, 101, 95, 111, 114, 95, 99,
		104, 97, 110, 103, 101, 8, 109, 97, 120, 105, 109, 105, 122, 101, 8, 109, 105, 110,
		105, 109, 105, 122, 101, 3, 110, 111, 112, 10, 114, 101, 115, 101, 116, 95, 100, 97,
		116, 97, 6, 115, 101, 116, 95, 105, 100, 8, 115, 101, 116, 95, 110, 97, 109, 101,
		4, 115, 116, 101, 112, 4, 100, 97, 116, 97, 5, 99, 111, 117, 110, 116, 2, 105,
		100, 4, 110, 97, 109, 101, 10, 97, 100, 100, 114, 101, 115, 115, 95, 111, 102, 4,
		117, 116, 102, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 171, 205,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 3, 8, 1, 0,
		0, 0, 0, 0, 0, 0, 10, 2, 9, 8, 1, 35, 69, 103, 137, 171, 205, 239,
		10, 2, 6, 5, 104, 101, 108, 108, 111, 10, 3, 81, 10, 0, 0, 0, 0, 0,
		0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0,
		0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 5,
		0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0,
		0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0,
		0, 0, 0, 0, 2, 1, 25, 10, 2, 1, 2, 1, 26, 3, 2, 2, 1, 25,
		10, 2, 3, 2, 3, 27, 3, 28, 8, 4, 25, 8, 2, 0, 0, 0, 0, 10,
		26, 10, 1, 65, 11, 12, 2, 10, 2, 6, 0, 0, 0, 0, 0, 0, 0, 0,
		36, 4, 21, 5, 8, 10, 0, 10, 1, 10, 2, 6, 1, 0, 0, 0, 0, 0,
		0, 0, 23, 66, 11, 20, 68, 11, 11, 2, 6, 1, 0, 0, 0, 0, 0, 0,
		0, 23, 12, 2, 5, 3, 11, 1, 1, 11, 0, 1, 2, 1, 1, 4, 1, 0,
		12, 20, 10, 0, 17, 17, 41, 0, 4, 13, 11, 0, 17, 17, 42, 0, 12, 2,
		11, 1, 11, 2, 15, 0, 21, 5, 19, 11, 1, 18, 0, 12, 3, 11, 0, 11,
		3, 45, 0, 2, 2, 0, 0, 0, 14, 103, 10, 0, 16, 1, 12, 5, 10, 1,
		16, 1, 12, 6, 11, 5, 20, 10, 6, 20, 35, 4, 18, 11, 6, 12, 5, 10,
		2, 16, 2, 12, 6, 5, 26, 11, 6, 1, 10, 1, 16, 1, 12, 5, 10, 3,
		16, 2, 12, 6, 10, 6, 20, 10, 1, 16, 1, 20, 35, 4, 47, 11, 5, 1,
		11, 1, 1, 11, 0, 1, 11, 2, 1, 11, 6, 12, 5, 11, 3, 16, 2, 12,
		6, 5, 69, 11, 3, 1, 10, 5, 11, 0, 16, 1, 34, 4, 65, 11, 6, 1,
		11, 5, 1, 11, 2, 16, 2, 12, 5, 11, 1, 16, 1, 12, 6, 5, 69, 11,
		1, 1, 11, 2, 1, 10, 5, 20, 10, 6, 20, 35, 4, 82, 11, 6, 1, 10,
		5, 12, 6, 10, 5, 1, 5, 88, 11, 5, 1, 10, 6, 12, 5, 10, 6, 1,
		10, 5, 10, 6, 33, 4, 97, 11, 6, 1, 11, 5, 12, 4, 5, 101, 11, 5,
		1, 11, 6, 12, 4, 11, 4, 2, 3, 1, 4, 1, 3, 15, 44, 10, 0, 17,
		17, 41, 3, 32, 4, 16, 6, 0, 0, 0, 0, 0, 0, 0, 0, 7, 2, 17,
		18, 7, 1, 18, 2, 18, 3, 12, 2, 11, 0, 11, 2, 45, 3, 5, 43, 11,
		0, 17, 17, 42, 3, 12, 3, 10, 3, 16, 3, 16, 4, 65, 11, 6, 2, 0,
		0, 0, 0, 0, 0, 0, 24, 12, 1, 10, 3, 16, 3, 16, 4, 65, 11, 10,
		1, 35, 4, 41, 5, 35, 10, 3, 15, 3, 15, 4, 49, 255, 68, 11, 5, 27,
		11, 3, 1, 2, 4, 1, 4, 1, 1, 1, 7, 11, 0, 17, 17, 43, 1, 16,
		2, 20, 1, 2, 5, 1, 4, 0, 18, 25, 7, 3, 12, 2, 14, 2, 65, 10,
		12, 4, 10, 4, 6, 0, 0, 0, 0, 0, 0, 0, 0, 34, 4, 24, 10, 1,
		10, 4, 38, 4, 17, 11, 4, 6, 1, 0, 0, 0, 0, 0, 0, 0, 23, 12,
		1, 7, 3, 12, 3, 14, 3, 11, 1, 66, 10, 20, 1, 2, 6, 1, 4, 1,
		3, 15, 44, 10, 0, 17, 17, 41, 3, 32, 4, 16, 6, 0, 0, 0, 0, 0,
		0, 0, 0, 7, 2, 17, 18, 7, 1, 18, 2, 18, 3, 12, 2, 11, 0, 11,
		2, 45, 3, 5, 43, 11, 0, 17, 17, 42, 3, 12, 3, 10, 3, 16, 3, 16,
		4, 65, 11, 6, 2, 0, 0, 0, 0, 0, 0, 0, 26, 12, 1, 10, 3, 16,
		3, 16, 4, 65, 11, 10, 1, 36, 4, 41, 5, 35, 10, 3, 15, 3, 15, 4,
		69, 11, 1, 5, 27, 11, 3, 1, 2, 7, 0, 0, 0, 1, 5, 11, 0, 6,
		0, 0, 0, 0, 0, 0, 0, 0, 18, 1, 45, 1, 2, 8, 1, 4, 0, 1,
		11, 10, 1, 6, 0, 0, 0, 0, 0, 0, 0, 0, 36, 4, 10, 5, 5, 11,
		1, 6, 1, 0, 0, 0, 0, 0, 0, 0, 23, 12, 1, 5, 0, 2, 9, 1,
		4, 1, 3, 19, 34, 10, 0, 17, 17, 41, 3, 4, 22, 11, 0, 17, 17, 42,
		3, 12, 5, 11, 1, 10, 5, 15, 1, 21, 11, 2, 10, 5, 15, 5, 21, 11,
		3, 11, 5, 15, 3, 15, 4, 21, 5, 33, 11, 3, 18, 2, 12, 4, 11, 1,
		11, 2, 11, 4, 18, 3, 12, 6, 11, 0, 11, 6, 45, 3, 2, 10, 1, 4,
		1, 3, 20, 93, 10, 1, 41, 3, 4, 6, 11, 0, 1, 2, 10, 0, 17, 17,
		41, 3, 32, 4, 21, 6, 0, 0, 0, 0, 0, 0, 0, 0, 7, 2, 17, 18,
		7, 1, 18, 2, 18, 3, 12, 8, 10, 0, 11, 8, 45, 3, 10, 0, 17, 17,
		43, 3, 16, 3, 16, 4, 65, 11, 12, 6, 10, 1, 43, 3, 16, 3, 16, 4,
		65, 11, 12, 7, 11, 6, 11, 7, 36, 4, 49, 11, 0, 17, 17, 43, 3, 16,
		3, 16, 4, 20, 11, 1, 42, 3, 12, 4, 12, 3, 5, 59, 11, 1, 43, 3,
		16, 3, 16, 4, 20, 11, 0, 17, 17, 42, 3, 12, 4, 12, 3, 11, 3, 11,
		4, 12, 9, 12, 5, 14, 5, 65, 11, 10, 9, 16, 3, 16, 4, 65, 11, 36,
		4, 75, 5, 72, 8, 12, 2, 5, 82, 10, 9, 16, 3, 16, 4, 65, 11, 6,
		16, 39, 0, 0, 0, 0, 0, 0, 35, 12, 2, 11, 2, 4, 90, 10, 9, 15,
		3, 15, 4, 14, 5, 17, 0, 5, 63, 11, 9, 1, 2, 11, 1, 4, 1, 3,
		21, 81, 10, 1, 41, 3, 4, 6, 11, 0, 1, 2, 10, 0, 17, 17, 41, 3,
		32, 4, 21, 6, 0, 0, 0, 0, 0, 0, 0, 0, 7, 2, 17, 18, 7, 1,
		18, 2, 18, 3, 12, 7, 10, 0, 11, 7, 45, 3, 10, 0, 17, 17, 43, 3,
		12, 9, 10, 1, 43, 3, 12, 10, 11, 9, 16, 3, 16, 4, 65, 11, 11, 10,
		16, 3, 16, 4, 65, 11, 12, 5, 12, 4, 10, 4, 10, 5, 36, 4, 51, 11,
		5, 6, 2, 0, 0, 0, 0, 0, 0, 0, 26, 11, 0, 17, 17, 42, 3, 12,
		3, 12, 2, 5, 60, 11, 0, 1, 11, 4, 6, 2, 0, 0, 0, 0, 0, 0,
		0, 26, 11, 1, 42, 3, 12, 3, 12, 2, 11, 2, 11, 3, 12, 8, 12, 6,
		10, 8, 16, 3, 16, 4, 65, 11, 10, 6, 36, 4, 78, 5, 72, 10, 8, 15,
		3, 15, 4, 69, 11, 1, 5, 64, 11, 8, 1, 2, 12, 1, 4, 0, 1, 1,
		2, 13, 1, 4, 1, 3, 22, 34, 10, 0, 17, 17, 41, 3, 4, 23, 11, 0,
		17, 17, 42, 3, 12, 1, 6, 0, 0, 0, 0, 0, 0, 0, 0, 10, 1, 15,
		1, 21, 7, 2, 17, 18, 10, 1, 15, 5, 21, 7, 1, 11, 1, 15, 3, 15,
		4, 21, 5, 33, 6, 0, 0, 0, 0, 0, 0, 0, 0, 7, 2, 17, 18, 7,
		1, 18, 2, 18, 3, 12, 2, 11, 0, 11, 2, 45, 3, 2, 14, 1, 4, 1,
		3, 23, 25, 10, 0, 17, 17, 41, 3, 32, 4, 16, 11, 1, 7, 2, 17, 18,
		7, 1, 18, 2, 18, 3, 12, 2, 11, 0, 11, 2, 45, 3, 5, 24, 11, 0,
		17, 17, 42, 3, 12, 3, 11, 1, 11, 3, 15, 1, 21, 2, 15, 1, 4, 1,
		3, 23, 24, 10, 0, 17, 17, 41, 3, 32, 4, 15, 6, 0, 0, 0, 0, 0,
		0, 0, 0, 11, 1, 7, 1, 18, 2, 18, 3, 12, 2, 11, 0, 11, 2, 45,
		3, 5, 23, 11, 0, 17, 17, 42, 3, 12, 3, 11, 1, 11, 3, 15, 5, 21,
		2, 16, 1, 4, 1, 1, 24, 13, 11, 0, 17, 17, 42, 1, 12, 1, 10, 1,
		16, 2, 20, 7, 0, 22, 11, 1, 15, 2, 21, 2, 0, 0, 3, 0, 1, 0,
		3, 2, 2, 0, 3, 1, 0,
	]
});