pub const LUT_0: [usize; 256] = [
	 0,  0,  0,  0,  0,  0,  0,  0,  0,  4,  4,  0,  0,  4,  0,  0,
	 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
	 8, 12, 16, 12, 12, 20, 12, 16, 24, 28, 12, 12, 32, 12, 36, 12,
	44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 32, 32, 24, 40, 28, 12,
	12, 48, 52, 52, 52, 48, 52, 52, 52, 48, 52, 52, 52, 52, 52, 48,
	52, 52, 52, 52, 52, 48, 52, 52, 52, 52, 52, 24, 12, 28, 12, 12,
	12, 56, 60, 60, 60, 56, 60, 60, 60, 56, 60, 60, 60, 60, 60, 56,
	60, 60, 60, 60, 60, 56, 60, 60, 60, 60, 60, 24, 12, 28, 12,  0,
	 0,  1,  0,  1,  0,  1,  0,  1,  0,  1,  0,  1,  0,  1,  0,  1,
	 0,  1,  0,  1,  0,  1,  0,  1,  0,  1,  0,  1,  0,  1,  0,  1,
	 0,  1,  0,  1,  0,  1,  0,  1,  0,  1,  0,  1,  0,  1,  0,  1,
	 0,  1,  0,  1,  0,  1,  0,  1,  0,  1,  0,  1,  0,  1,  0,  1,
	 2,  3,  2,  3,  2,  3,  2,  3,  2,  3,  2,  3,  2,  3,  2,  3,
	 2,  3,  2,  3,  2,  3,  2,  3,  2,  3,  2,  3,  2,  3,  2,  3,
	 2,  3,  2,  3,  2,  3,  2,  3,  2,  3,  2,  3,  2,  3,  2,  3,
	 2,  3,  2,  3,  2,  3,  2,  3,  2,  3,  2,  3,  2,  3,  2,  3
];

pub const LUT_1: [usize; 256] =[
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
	2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1,
	1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
	2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1,
	1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
	3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1, 0,
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
	2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2
];

pub const LUT_2: [usize; 256] = [
	0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
	2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
	2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
	2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
	3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
	3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
	3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
	3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
	4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
	4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
	4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
	4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
	5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
	5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
	5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
	6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 7
];


// @NOTE Code to generate INSERT_LENGTHS_AND_COPY_LENGTHS below
//
// let mut table = vec![((0, 0), (0, 0)); 704];

// for insert_and_copy_length in 0..704 {
// 	let (mut insert_length_code, mut copy_length_code) = match insert_and_copy_length {
// 		0...63 => (0, 0),
// 		64...127 => (0, 8),
// 		128...191 => (0, 0),
// 		192...255 => (0, 8),
// 		256...319 => (8, 0),
// 		320...383 => (8, 8),
// 		384...447 => (0, 16),
// 		448...511 => (16, 0),
// 		512...575 => (8, 16),
// 		576...639 => (16, 8),
// 		640...703 => (16, 16),
// 		_ => unreachable!(),
// 	};

// 	insert_length_code += 0x07 & (insert_and_copy_length >> 3);
// 	copy_length_code += 0x07 & insert_and_copy_length;

// 	// debug(&format!("(insert code, copy code) = {:?}", (insert_length_code, copy_length_code)));

// 	let (insert_length, extra_bits_insert) = match insert_length_code {
// 		0...5 => (insert_length_code, 0),
// 		6...7 => (6 + 2 * (insert_length_code - 6) , 1),
// 		8...9 => (10 + 4 * (insert_length_code - 8) , 2),
// 		10...11 => (18 + 8 * (insert_length_code - 10) , 3),
// 		12...13 => (34 + 16 * (insert_length_code - 12) , 4),
// 		14...15 => (66 + 32 * (insert_length_code - 14) , 5),
// 		16 => (130, 6),
// 		17 => (194, 7),
// 		18 => (322, 8),
// 		19 => (578, 9),
// 		20 => (1090, 10),
// 		21 => (2114, 12),
// 		22 => (6210, 14),
// 		23 => (22594, 24),
// 		_ => unreachable!(), // confirmed unreachable, possible value is entirely defined in code above
// 	};

// 	let (copy_length, extra_bits_copy) = match copy_length_code {
// 		0...7 => (copy_length_code + 2, 0),
// 		8...9 => (10 + 2 * (copy_length_code - 8) , 1),
// 		10...11 => (14 + 4 * (copy_length_code - 10) , 2),
// 		12...13 => (22 + 8 * (copy_length_code - 12) , 3),
// 		14...15 => (38 + 16 * (copy_length_code - 14) , 4),
// 		16...17 => (70 + 32 * (copy_length_code - 16) , 5),
// 		18 => (134, 6),
// 		19 => (198, 7),
// 		20 => (326, 8),
// 		21 => (582, 9),
// 		22 => (1094, 10),
// 		23 => (2118, 24),
// 		_ => unreachable!(), // confirmed unreachable, possible value is entirely defined in code above
// 	};

// 	table[insert_and_copy_length] = ((insert_length, extra_bits_insert), (copy_length, extra_bits_copy));
// }

// println!("{:?}", table);

pub const INSERT_LENGTHS_AND_COPY_LENGTHS: [((u32, usize), (u32, usize)); 704] = [((0, 0), (2, 0)), ((0, 0), (3, 0)), ((0, 0), (4, 0)), ((0, 0), (5, 0)), ((0, 0), (6, 0)), ((0, 0), (7, 0)), ((0, 0), (8, 0)), ((0, 0), (9, 0)), ((1, 0), (2, 0)), ((1, 0), (3, 0)), ((1, 0), (4, 0)), ((1, 0), (5, 0)), ((1, 0), (6, 0)), ((1, 0), (7, 0)), ((1, 0), (8, 0)), ((1, 0), (9, 0)), ((2, 0), (2, 0)), ((2, 0), (3, 0)), ((2, 0), (4, 0)), ((2, 0), (5, 0)), ((2, 0), (6, 0)), ((2, 0), (7, 0)), ((2, 0), (8, 0)), ((2, 0), (9, 0)), ((3, 0), (2, 0)), ((3, 0), (3, 0)), ((3, 0), (4, 0)), ((3, 0), (5, 0)), ((3, 0), (6, 0)), ((3, 0), (7, 0)), ((3, 0), (8, 0)), ((3, 0), (9, 0)), ((4, 0), (2, 0)), ((4, 0), (3, 0)), ((4, 0), (4, 0)), ((4, 0), (5, 0)), ((4, 0), (6, 0)), ((4, 0), (7, 0)), ((4, 0), (8, 0)), ((4, 0), (9, 0)), ((5, 0), (2, 0)), ((5, 0), (3, 0)), ((5, 0), (4, 0)), ((5, 0), (5, 0)), ((5, 0), (6, 0)), ((5, 0), (7, 0)), ((5, 0), (8, 0)), ((5, 0), (9, 0)), ((6, 1), (2, 0)), ((6, 1), (3, 0)), ((6, 1), (4, 0)), ((6, 1), (5, 0)), ((6, 1), (6, 0)), ((6, 1), (7, 0)), ((6, 1), (8, 0)), ((6, 1), (9, 0)), ((8, 1), (2, 0)), ((8, 1), (3, 0)), ((8, 1), (4, 0)), ((8, 1), (5, 0)), ((8, 1), (6, 0)), ((8, 1), (7, 0)), ((8, 1), (8, 0)), ((8, 1), (9, 0)), ((0, 0), (10, 1)), ((0, 0), (12, 1)), ((0, 0), (14, 2)), ((0, 0), (18, 2)), ((0, 0), (22, 3)), ((0, 0), (30, 3)), ((0, 0), (38, 4)), ((0, 0), (54, 4)), ((1, 0), (10, 1)), ((1, 0), (12, 1)), ((1, 0), (14, 2)), ((1, 0), (18, 2)), ((1, 0), (22, 3)), ((1, 0), (30, 3)), ((1, 0), (38, 4)), ((1, 0), (54, 4)), ((2, 0), (10, 1)), ((2, 0), (12, 1)), ((2, 0), (14, 2)), ((2, 0), (18, 2)), ((2, 0), (22, 3)), ((2, 0), (30, 3)), ((2, 0), (38, 4)), ((2, 0), (54, 4)), ((3, 0), (10, 1)), ((3, 0), (12, 1)), ((3, 0), (14, 2)), ((3, 0), (18, 2)), ((3, 0), (22, 3)), ((3, 0), (30, 3)), ((3, 0), (38, 4)), ((3, 0), (54, 4)), ((4, 0), (10, 1)), ((4, 0), (12, 1)), ((4, 0), (14, 2)), ((4, 0), (18, 2)), ((4, 0), (22, 3)), ((4, 0), (30, 3)), ((4, 0), (38, 4)), ((4, 0), (54, 4)), ((5, 0), (10, 1)), ((5, 0), (12, 1)), ((5, 0), (14, 2)), ((5, 0), (18, 2)), ((5, 0), (22, 3)), ((5, 0), (30, 3)), ((5, 0), (38, 4)), ((5, 0), (54, 4)), ((6, 1), (10, 1)), ((6, 1), (12, 1)), ((6, 1), (14, 2)), ((6, 1), (18, 2)), ((6, 1), (22, 3)), ((6, 1), (30, 3)), ((6, 1), (38, 4)), ((6, 1), (54, 4)), ((8, 1), (10, 1)), ((8, 1), (12, 1)), ((8, 1), (14, 2)), ((8, 1), (18, 2)), ((8, 1), (22, 3)), ((8, 1), (30, 3)), ((8, 1), (38, 4)), ((8, 1), (54, 4)), ((0, 0), (2, 0)), ((0, 0), (3, 0)), ((0, 0), (4, 0)), ((0, 0), (5, 0)), ((0, 0), (6, 0)), ((0, 0), (7, 0)), ((0, 0), (8, 0)), ((0, 0), (9, 0)), ((1, 0), (2, 0)), ((1, 0), (3, 0)), ((1, 0), (4, 0)), ((1, 0), (5, 0)), ((1, 0), (6, 0)), ((1, 0), (7, 0)), ((1, 0), (8, 0)), ((1, 0), (9, 0)), ((2, 0), (2, 0)), ((2, 0), (3, 0)), ((2, 0), (4, 0)), ((2, 0), (5, 0)), ((2, 0), (6, 0)), ((2, 0), (7, 0)), ((2, 0), (8, 0)), ((2, 0), (9, 0)), ((3, 0), (2, 0)), ((3, 0), (3, 0)), ((3, 0), (4, 0)), ((3, 0), (5, 0)), ((3, 0), (6, 0)), ((3, 0), (7, 0)), ((3, 0), (8, 0)), ((3, 0), (9, 0)), ((4, 0), (2, 0)), ((4, 0), (3, 0)), ((4, 0), (4, 0)), ((4, 0), (5, 0)), ((4, 0), (6, 0)), ((4, 0), (7, 0)), ((4, 0), (8, 0)), ((4, 0), (9, 0)), ((5, 0), (2, 0)), ((5, 0), (3, 0)), ((5, 0), (4, 0)), ((5, 0), (5, 0)), ((5, 0), (6, 0)), ((5, 0), (7, 0)), ((5, 0), (8, 0)), ((5, 0), (9, 0)), ((6, 1), (2, 0)), ((6, 1), (3, 0)), ((6, 1), (4, 0)), ((6, 1), (5, 0)), ((6, 1), (6, 0)), ((6, 1), (7, 0)), ((6, 1), (8, 0)), ((6, 1), (9, 0)), ((8, 1), (2, 0)), ((8, 1), (3, 0)), ((8, 1), (4, 0)), ((8, 1), (5, 0)), ((8, 1), (6, 0)), ((8, 1), (7, 0)), ((8, 1), (8, 0)), ((8, 1), (9, 0)), ((0, 0), (10, 1)), ((0, 0), (12, 1)), ((0, 0), (14, 2)), ((0, 0), (18, 2)), ((0, 0), (22, 3)), ((0, 0), (30, 3)), ((0, 0), (38, 4)), ((0, 0), (54, 4)), ((1, 0), (10, 1)), ((1, 0), (12, 1)), ((1, 0), (14, 2)), ((1, 0), (18, 2)), ((1, 0), (22, 3)), ((1, 0), (30, 3)), ((1, 0), (38, 4)), ((1, 0), (54, 4)), ((2, 0), (10, 1)), ((2, 0), (12, 1)), ((2, 0), (14, 2)), ((2, 0), (18, 2)), ((2, 0), (22, 3)), ((2, 0), (30, 3)), ((2, 0), (38, 4)), ((2, 0), (54, 4)), ((3, 0), (10, 1)), ((3, 0), (12, 1)), ((3, 0), (14, 2)), ((3, 0), (18, 2)), ((3, 0), (22, 3)), ((3, 0), (30, 3)), ((3, 0), (38, 4)), ((3, 0), (54, 4)), ((4, 0), (10, 1)), ((4, 0), (12, 1)), ((4, 0), (14, 2)), ((4, 0), (18, 2)), ((4, 0), (22, 3)), ((4, 0), (30, 3)), ((4, 0), (38, 4)), ((4, 0), (54, 4)), ((5, 0), (10, 1)), ((5, 0), (12, 1)), ((5, 0), (14, 2)), ((5, 0), (18, 2)), ((5, 0), (22, 3)), ((5, 0), (30, 3)), ((5, 0), (38, 4)), ((5, 0), (54, 4)), ((6, 1), (10, 1)), ((6, 1), (12, 1)), ((6, 1), (14, 2)), ((6, 1), (18, 2)), ((6, 1), (22, 3)), ((6, 1), (30, 3)), ((6, 1), (38, 4)), ((6, 1), (54, 4)), ((8, 1), (10, 1)), ((8, 1), (12, 1)), ((8, 1), (14, 2)), ((8, 1), (18, 2)), ((8, 1), (22, 3)), ((8, 1), (30, 3)), ((8, 1), (38, 4)), ((8, 1), (54, 4)), ((10, 2), (2, 0)), ((10, 2), (3, 0)), ((10, 2), (4, 0)), ((10, 2), (5, 0)), ((10, 2), (6, 0)), ((10, 2), (7, 0)), ((10, 2), (8, 0)), ((10, 2), (9, 0)), ((14, 2), (2, 0)), ((14, 2), (3, 0)), ((14, 2), (4, 0)), ((14, 2), (5, 0)), ((14, 2), (6, 0)), ((14, 2), (7, 0)), ((14, 2), (8, 0)), ((14, 2), (9, 0)), ((18, 3), (2, 0)), ((18, 3), (3, 0)), ((18, 3), (4, 0)), ((18, 3), (5, 0)), ((18, 3), (6, 0)), ((18, 3), (7, 0)), ((18, 3), (8, 0)), ((18, 3), (9, 0)), ((26, 3), (2, 0)), ((26, 3), (3, 0)), ((26, 3), (4, 0)), ((26, 3), (5, 0)), ((26, 3), (6, 0)), ((26, 3), (7, 0)), ((26, 3), (8, 0)), ((26, 3), (9, 0)), ((34, 4), (2, 0)), ((34, 4), (3, 0)), ((34, 4), (4, 0)), ((34, 4), (5, 0)), ((34, 4), (6, 0)), ((34, 4), (7, 0)), ((34, 4), (8, 0)), ((34, 4), (9, 0)), ((50, 4), (2, 0)), ((50, 4), (3, 0)), ((50, 4), (4, 0)), ((50, 4), (5, 0)), ((50, 4), (6, 0)), ((50, 4), (7, 0)), ((50, 4), (8, 0)), ((50, 4), (9, 0)), ((66, 5), (2, 0)), ((66, 5), (3, 0)), ((66, 5), (4, 0)), ((66, 5), (5, 0)), ((66, 5), (6, 0)), ((66, 5), (7, 0)), ((66, 5), (8, 0)), ((66, 5), (9, 0)), ((98, 5), (2, 0)), ((98, 5), (3, 0)), ((98, 5), (4, 0)), ((98, 5), (5, 0)), ((98, 5), (6, 0)), ((98, 5), (7, 0)), ((98, 5), (8, 0)), ((98, 5), (9, 0)), ((10, 2), (10, 1)), ((10, 2), (12, 1)), ((10, 2), (14, 2)), ((10, 2), (18, 2)), ((10, 2), (22, 3)), ((10, 2), (30, 3)), ((10, 2), (38, 4)), ((10, 2), (54, 4)), ((14, 2), (10, 1)), ((14, 2), (12, 1)), ((14, 2), (14, 2)), ((14, 2), (18, 2)), ((14, 2), (22, 3)), ((14, 2), (30, 3)), ((14, 2), (38, 4)), ((14, 2), (54, 4)), ((18, 3), (10, 1)), ((18, 3), (12, 1)), ((18, 3), (14, 2)), ((18, 3), (18, 2)), ((18, 3), (22, 3)), ((18, 3), (30, 3)), ((18, 3), (38, 4)), ((18, 3), (54, 4)), ((26, 3), (10, 1)), ((26, 3), (12, 1)), ((26, 3), (14, 2)), ((26, 3), (18, 2)), ((26, 3), (22, 3)), ((26, 3), (30, 3)), ((26, 3), (38, 4)), ((26, 3), (54, 4)), ((34, 4), (10, 1)), ((34, 4), (12, 1)), ((34, 4), (14, 2)), ((34, 4), (18, 2)), ((34, 4), (22, 3)), ((34, 4), (30, 3)), ((34, 4), (38, 4)), ((34, 4), (54, 4)), ((50, 4), (10, 1)), ((50, 4), (12, 1)), ((50, 4), (14, 2)), ((50, 4), (18, 2)), ((50, 4), (22, 3)), ((50, 4), (30, 3)), ((50, 4), (38, 4)), ((50, 4), (54, 4)), ((66, 5), (10, 1)), ((66, 5), (12, 1)), ((66, 5), (14, 2)), ((66, 5), (18, 2)), ((66, 5), (22, 3)), ((66, 5), (30, 3)), ((66, 5), (38, 4)), ((66, 5), (54, 4)), ((98, 5), (10, 1)), ((98, 5), (12, 1)), ((98, 5), (14, 2)), ((98, 5), (18, 2)), ((98, 5), (22, 3)), ((98, 5), (30, 3)), ((98, 5), (38, 4)), ((98, 5), (54, 4)), ((0, 0), (70, 5)), ((0, 0), (102, 5)), ((0, 0), (134, 6)), ((0, 0), (198, 7)), ((0, 0), (326, 8)), ((0, 0), (582, 9)), ((0, 0), (1094, 10)), ((0, 0), (2118, 24)), ((1, 0), (70, 5)), ((1, 0), (102, 5)), ((1, 0), (134, 6)), ((1, 0), (198, 7)), ((1, 0), (326, 8)), ((1, 0), (582, 9)), ((1, 0), (1094, 10)), ((1, 0), (2118, 24)), ((2, 0), (70, 5)), ((2, 0), (102, 5)), ((2, 0), (134, 6)), ((2, 0), (198, 7)), ((2, 0), (326, 8)), ((2, 0), (582, 9)), ((2, 0), (1094, 10)), ((2, 0), (2118, 24)), ((3, 0), (70, 5)), ((3, 0), (102, 5)), ((3, 0), (134, 6)), ((3, 0), (198, 7)), ((3, 0), (326, 8)), ((3, 0), (582, 9)), ((3, 0), (1094, 10)), ((3, 0), (2118, 24)), ((4, 0), (70, 5)), ((4, 0), (102, 5)), ((4, 0), (134, 6)), ((4, 0), (198, 7)), ((4, 0), (326, 8)), ((4, 0), (582, 9)), ((4, 0), (1094, 10)), ((4, 0), (2118, 24)), ((5, 0), (70, 5)), ((5, 0), (102, 5)), ((5, 0), (134, 6)), ((5, 0), (198, 7)), ((5, 0), (326, 8)), ((5, 0), (582, 9)), ((5, 0), (1094, 10)), ((5, 0), (2118, 24)), ((6, 1), (70, 5)), ((6, 1), (102, 5)), ((6, 1), (134, 6)), ((6, 1), (198, 7)), ((6, 1), (326, 8)), ((6, 1), (582, 9)), ((6, 1), (1094, 10)), ((6, 1), (2118, 24)), ((8, 1), (70, 5)), ((8, 1), (102, 5)), ((8, 1), (134, 6)), ((8, 1), (198, 7)), ((8, 1), (326, 8)), ((8, 1), (582, 9)), ((8, 1), (1094, 10)), ((8, 1), (2118, 24)), ((130, 6), (2, 0)), ((130, 6), (3, 0)), ((130, 6), (4, 0)), ((130, 6), (5, 0)), ((130, 6), (6, 0)), ((130, 6), (7, 0)), ((130, 6), (8, 0)), ((130, 6), (9, 0)), ((194, 7), (2, 0)), ((194, 7), (3, 0)), ((194, 7), (4, 0)), ((194, 7), (5, 0)), ((194, 7), (6, 0)), ((194, 7), (7, 0)), ((194, 7), (8, 0)), ((194, 7), (9, 0)), ((322, 8), (2, 0)), ((322, 8), (3, 0)), ((322, 8), (4, 0)), ((322, 8), (5, 0)), ((322, 8), (6, 0)), ((322, 8), (7, 0)), ((322, 8), (8, 0)), ((322, 8), (9, 0)), ((578, 9), (2, 0)), ((578, 9), (3, 0)), ((578, 9), (4, 0)), ((578, 9), (5, 0)), ((578, 9), (6, 0)), ((578, 9), (7, 0)), ((578, 9), (8, 0)), ((578, 9), (9, 0)), ((1090, 10), (2, 0)), ((1090, 10), (3, 0)), ((1090, 10), (4, 0)), ((1090, 10), (5, 0)), ((1090, 10), (6, 0)), ((1090, 10), (7, 0)), ((1090, 10), (8, 0)), ((1090, 10), (9, 0)), ((2114, 12), (2, 0)), ((2114, 12), (3, 0)), ((2114, 12), (4, 0)), ((2114, 12), (5, 0)), ((2114, 12), (6, 0)), ((2114, 12), (7, 0)), ((2114, 12), (8, 0)), ((2114, 12), (9, 0)), ((6210, 14), (2, 0)), ((6210, 14), (3, 0)), ((6210, 14), (4, 0)), ((6210, 14), (5, 0)), ((6210, 14), (6, 0)), ((6210, 14), (7, 0)), ((6210, 14), (8, 0)), ((6210, 14), (9, 0)), ((22594, 24), (2, 0)), ((22594, 24), (3, 0)), ((22594, 24), (4, 0)), ((22594, 24), (5, 0)), ((22594, 24), (6, 0)), ((22594, 24), (7, 0)), ((22594, 24), (8, 0)), ((22594, 24), (9, 0)), ((10, 2), (70, 5)), ((10, 2), (102, 5)), ((10, 2), (134, 6)), ((10, 2), (198, 7)), ((10, 2), (326, 8)), ((10, 2), (582, 9)), ((10, 2), (1094, 10)), ((10, 2), (2118, 24)), ((14, 2), (70, 5)), ((14, 2), (102, 5)), ((14, 2), (134, 6)), ((14, 2), (198, 7)), ((14, 2), (326, 8)), ((14, 2), (582, 9)), ((14, 2), (1094, 10)), ((14, 2), (2118, 24)), ((18, 3), (70, 5)), ((18, 3), (102, 5)), ((18, 3), (134, 6)), ((18, 3), (198, 7)), ((18, 3), (326, 8)), ((18, 3), (582, 9)), ((18, 3), (1094, 10)), ((18, 3), (2118, 24)), ((26, 3), (70, 5)), ((26, 3), (102, 5)), ((26, 3), (134, 6)), ((26, 3), (198, 7)), ((26, 3), (326, 8)), ((26, 3), (582, 9)), ((26, 3), (1094, 10)), ((26, 3), (2118, 24)), ((34, 4), (70, 5)), ((34, 4), (102, 5)), ((34, 4), (134, 6)), ((34, 4), (198, 7)), ((34, 4), (326, 8)), ((34, 4), (582, 9)), ((34, 4), (1094, 10)), ((34, 4), (2118, 24)), ((50, 4), (70, 5)), ((50, 4), (102, 5)), ((50, 4), (134, 6)), ((50, 4), (198, 7)), ((50, 4), (326, 8)), ((50, 4), (582, 9)), ((50, 4), (1094, 10)), ((50, 4), (2118, 24)), ((66, 5), (70, 5)), ((66, 5), (102, 5)), ((66, 5), (134, 6)), ((66, 5), (198, 7)), ((66, 5), (326, 8)), ((66, 5), (582, 9)), ((66, 5), (1094, 10)), ((66, 5), (2118, 24)), ((98, 5), (70, 5)), ((98, 5), (102, 5)), ((98, 5), (134, 6)), ((98, 5), (198, 7)), ((98, 5), (326, 8)), ((98, 5), (582, 9)), ((98, 5), (1094, 10)), ((98, 5), (2118, 24)), ((130, 6), (10, 1)), ((130, 6), (12, 1)), ((130, 6), (14, 2)), ((130, 6), (18, 2)), ((130, 6), (22, 3)), ((130, 6), (30, 3)), ((130, 6), (38, 4)), ((130, 6), (54, 4)), ((194, 7), (10, 1)), ((194, 7), (12, 1)), ((194, 7), (14, 2)), ((194, 7), (18, 2)), ((194, 7), (22, 3)), ((194, 7), (30, 3)), ((194, 7), (38, 4)), ((194, 7), (54, 4)), ((322, 8), (10, 1)), ((322, 8), (12, 1)), ((322, 8), (14, 2)), ((322, 8), (18, 2)), ((322, 8), (22, 3)), ((322, 8), (30, 3)), ((322, 8), (38, 4)), ((322, 8), (54, 4)), ((578, 9), (10, 1)), ((578, 9), (12, 1)), ((578, 9), (14, 2)), ((578, 9), (18, 2)), ((578, 9), (22, 3)), ((578, 9), (30, 3)), ((578, 9), (38, 4)), ((578, 9), (54, 4)), ((1090, 10), (10, 1)), ((1090, 10), (12, 1)), ((1090, 10), (14, 2)), ((1090, 10), (18, 2)), ((1090, 10), (22, 3)), ((1090, 10), (30, 3)), ((1090, 10), (38, 4)), ((1090, 10), (54, 4)), ((2114, 12), (10, 1)), ((2114, 12), (12, 1)), ((2114, 12), (14, 2)), ((2114, 12), (18, 2)), ((2114, 12), (22, 3)), ((2114, 12), (30, 3)), ((2114, 12), (38, 4)), ((2114, 12), (54, 4)), ((6210, 14), (10, 1)), ((6210, 14), (12, 1)), ((6210, 14), (14, 2)), ((6210, 14), (18, 2)), ((6210, 14), (22, 3)), ((6210, 14), (30, 3)), ((6210, 14), (38, 4)), ((6210, 14), (54, 4)), ((22594, 24), (10, 1)), ((22594, 24), (12, 1)), ((22594, 24), (14, 2)), ((22594, 24), (18, 2)), ((22594, 24), (22, 3)), ((22594, 24), (30, 3)), ((22594, 24), (38, 4)), ((22594, 24), (54, 4)), ((130, 6), (70, 5)), ((130, 6), (102, 5)), ((130, 6), (134, 6)), ((130, 6), (198, 7)), ((130, 6), (326, 8)), ((130, 6), (582, 9)), ((130, 6), (1094, 10)), ((130, 6), (2118, 24)), ((194, 7), (70, 5)), ((194, 7), (102, 5)), ((194, 7), (134, 6)), ((194, 7), (198, 7)), ((194, 7), (326, 8)), ((194, 7), (582, 9)), ((194, 7), (1094, 10)), ((194, 7), (2118, 24)), ((322, 8), (70, 5)), ((322, 8), (102, 5)), ((322, 8), (134, 6)), ((322, 8), (198, 7)), ((322, 8), (326, 8)), ((322, 8), (582, 9)), ((322, 8), (1094, 10)), ((322, 8), (2118, 24)), ((578, 9), (70, 5)), ((578, 9), (102, 5)), ((578, 9), (134, 6)), ((578, 9), (198, 7)), ((578, 9), (326, 8)), ((578, 9), (582, 9)), ((578, 9), (1094, 10)), ((578, 9), (2118, 24)), ((1090, 10), (70, 5)), ((1090, 10), (102, 5)), ((1090, 10), (134, 6)), ((1090, 10), (198, 7)), ((1090, 10), (326, 8)), ((1090, 10), (582, 9)), ((1090, 10), (1094, 10)), ((1090, 10), (2118, 24)), ((2114, 12), (70, 5)), ((2114, 12), (102, 5)), ((2114, 12), (134, 6)), ((2114, 12), (198, 7)), ((2114, 12), (326, 8)), ((2114, 12), (582, 9)), ((2114, 12), (1094, 10)), ((2114, 12), (2118, 24)), ((6210, 14), (70, 5)), ((6210, 14), (102, 5)), ((6210, 14), (134, 6)), ((6210, 14), (198, 7)), ((6210, 14), (326, 8)), ((6210, 14), (582, 9)), ((6210, 14), (1094, 10)), ((6210, 14), (2118, 24)), ((22594, 24), (70, 5)), ((22594, 24), (102, 5)), ((22594, 24), (134, 6)), ((22594, 24), (198, 7)), ((22594, 24), (326, 8)), ((22594, 24), (582, 9)), ((22594, 24), (1094, 10)), ((22594, 24), (2118, 24))];

