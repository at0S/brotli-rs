#![feature(test)]

extern crate test;
extern crate brotli_rs;

use test::Bencher;


#[bench]
fn bench_monkey(b: &mut Bencher) {
	use std::io::{ Cursor, Read };
	use brotli_rs::brotli::Decompressor;

	b.iter(|| {
		let brotli_stream = Cursor::new(vec![
			0x1b, 0x4a, 0x03, 0x00, 0x8c, 0x94, 0x6e, 0xde, 0xb4, 0xd7, 0x96, 0xb1, 0x78, 0x86, 0xf2, 0x2d,
			0xe1, 0x1a, 0xbc, 0x0b, 0x1c, 0xba, 0xa9, 0xc7, 0xf7, 0xcc, 0x6e, 0xb2, 0x42, 0x34, 0x51, 0x44,
			0x8b, 0x4e, 0x13, 0x08, 0xa0, 0xcd, 0x6e, 0xe8, 0x2c, 0xa5, 0x53, 0xa1, 0x9c, 0x5d, 0x2c, 0x1d,
			0x23, 0x1a, 0xd2, 0x56, 0xbe, 0xdb, 0xeb, 0x26, 0xba, 0x03, 0x65, 0x7c, 0x96, 0x6a, 0xa2, 0x76,
			0xec, 0xef, 0x87, 0x47, 0x33, 0xd6, 0x27, 0x0e, 0x63, 0x95, 0xe2, 0x1d, 0x8d, 0x2c, 0xc5, 0xd1,
			0x28, 0x9f, 0x60, 0x94, 0x6f, 0x02, 0x8b, 0xdd, 0xaa, 0x64, 0x94, 0x2c, 0x1e, 0x3b, 0x65, 0x7c,
			0x07, 0x45, 0x5a, 0xb2, 0xe2, 0xfc, 0x49, 0x81, 0x2c, 0x9f, 0x40, 0xae, 0xef, 0x68, 0x81, 0xac,
			0x16, 0x7a, 0x0f, 0xf5, 0x3b, 0x6d, 0x1c, 0xb9, 0x1e, 0x2d, 0x5f, 0xd5, 0xc8, 0xaf, 0x5e, 0x85,
			0xaa, 0x05, 0xbe, 0x53, 0x75, 0xc2, 0xb0, 0x22, 0x8a, 0x15, 0xc6, 0xa3, 0xb1, 0xe6, 0x42, 0x14,
			0xf4, 0x84, 0x54, 0x53, 0x19, 0x5f, 0xbe, 0xc3, 0xf2, 0x1d, 0xd1, 0xb7, 0xe5, 0xdd, 0xb6, 0xd9,
			0x23, 0xc6, 0xf6, 0x9f, 0x9e, 0xf6, 0x4d, 0x65, 0x30, 0xfb, 0xc0, 0x71, 0x45, 0x04, 0xad, 0x03,
			0xb5, 0xbe, 0xc9, 0xcb, 0xfd, 0xe2, 0x50, 0x5a, 0x46, 0x74, 0x04, 0x0d, 0xff, 0x20, 0x04, 0x77,
			0xb2, 0x6d, 0x27, 0xbf, 0x47, 0xa9, 0x9d, 0x1b, 0x96, 0x2c, 0x62, 0x90, 0x23, 0x8b, 0xe0, 0xf8,
			0x1d, 0xcf, 0xaf, 0x1d, 0x3d, 0xee, 0x8a, 0xc8, 0x75, 0x23, 0x66, 0xdd, 0xde, 0xd6, 0x6d, 0xe3,
			0x2a, 0x82, 0x8a, 0x78, 0x8a, 0xdb, 0xe6, 0x20, 0x4c, 0xb7, 0x5c, 0x63, 0xba, 0x30, 0xe3, 0x3f,
			0xb6, 0xee, 0x8c, 0x22, 0xa2, 0x2a, 0xb0, 0x22, 0x0a, 0x99, 0xff, 0x3d, 0x62, 0x51, 0xee, 0x08,
			0xf6, 0x3d, 0x4a, 0xe4, 0xcc, 0xef, 0x22, 0x87, 0x11, 0xe2, 0x83, 0x28, 0xe4, 0xf5, 0x8f, 0x35,
			0x19, 0x63, 0x5b, 0xe1, 0x5a, 0x92, 0x73, 0xdd, 0xa1, 0x50, 0x9d, 0x38, 0x5c, 0xeb, 0xb5, 0x03,
			0x6a, 0x64, 0x90, 0x94, 0xc8, 0x8d, 0xfb, 0x2f, 0x8a, 0x86, 0x22, 0xcc, 0x1d, 0x87, 0xe0, 0x48,
			0x0a, 0x96, 0x77, 0x90, 0x39, 0xc6, 0x23, 0x23, 0x48, 0xfb, 0x11, 0x47, 0x56, 0xca, 0x20, 0xe3,
			0x42, 0x81, 0xf7, 0x77, 0x32, 0xc1, 0xa5, 0x5c, 0x40, 0x21, 0x65, 0x17, 0x40, 0x29, 0x17, 0x17,
			0x6c, 0x56, 0x32, 0x98, 0x38, 0x06, 0xdc, 0x99, 0x4d, 0x33, 0x29, 0xbb, 0x02, 0xdf, 0x4c, 0x26,
			0x93, 0x6c, 0x17, 0x82, 0x86, 0x20, 0xd7, 0x03, 0x79, 0x7d, 0x9a, 0x00, 0xd7, 0x87, 0x00, 0xe7,
			0x0b, 0x66, 0xe3, 0x4c, 0x66, 0x71, 0x67, 0x08, 0x32, 0xf9, 0x08, 0x3e, 0x81, 0x33, 0xcd, 0x17,
			0x72, 0x31, 0xf0, 0xb8, 0x94, 0x52, 0x4b, 0x90, 0x31, 0x8e, 0x68, 0xc1, 0xef, 0x90, 0xc9, 0xe5,
			0xf2, 0x61, 0x09, 0x72, 0x25, 0xad, 0xec, 0xc5, 0x62, 0xc0, 0x0b, 0x12, 0x05, 0xf7, 0x91, 0x75,
			0x0d, 0xee, 0x61, 0x2e, 0x2e, 0x19, 0x09, 0xc2, 0x03,
		]);

		let mut decompressed = &mut String::new();
		Decompressor::new(brotli_stream).read_to_string(&mut decompressed)
	});
}

