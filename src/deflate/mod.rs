use ::bitreader::BitReader;
use ::huffman;

use std::error::Error;
use std::fmt;
use std::fmt::{ Display, Formatter };
use std::io;
use std::io::Read;
use std::result;

/// Wraps an input stream and provides methods for decompressing.
///
/// # Examples
///
/// extern crate compression;
///
/// use compression::deflate::Decompressor;
/// use std::fs::File;
///
/// let mut f = try!(File::open("compressed.txt.gz"));
/// let mut deflate = Decompressor::new(f);
pub struct Decompressor {
	header: Header,
	state: State,
	huffman_codes: HuffmanCodes,
}

type BFinal = bool;

#[derive(Debug, Clone, PartialEq)]
enum BType {
	NoCompression,
	CompressedWithFixedHuffmanCodes,
	CompressedWithDynamicHuffmanCodes,
}

type HuffmanCodes = huffman::tree::Tree;

#[derive(Debug, Clone, PartialEq)]
struct Header {
	bfinal: Option<BFinal>,
	btype: Option<BType>,
}

impl Header {
	fn new() -> Header {
		Header{
			bfinal: None,
			btype: None,
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
enum State {
	HeaderBegin,
	BFinal(BFinal),
	BType(BType),
	HandlingHuffmanCodes(BType),
	HuffmanCodes(HuffmanCodes),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DecompressorError {
	UnexpectedEOF,
	BlockTypeReserved
}

impl Display for DecompressorError {
	fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
		fmt.write_str(self.description())
	}
}

impl Error for DecompressorError {
	fn description(&self) -> &str {
		match self {
			&DecompressorError::UnexpectedEOF => "Encountered unexpected EOF",
			&DecompressorError::BlockTypeReserved => "Reserved block type in deflate header",
		}
	}
}

impl Decompressor {
	pub fn new() -> Decompressor {
		Decompressor{
			header: Header::new(),
			state: State::HeaderBegin,
		}
	}

	fn parse_bfinal<R: Read>(ref mut in_stream: &mut BitReader<R>) -> result::Result<State, DecompressorError> {
		match in_stream.read_bit() {
			Ok(bfinal) => Ok(State::BFinal(bfinal)),
			Err(_) => Err(DecompressorError::UnexpectedEOF),
		}
	}

	fn parse_btype<R: Read>(ref mut in_stream: &mut BitReader<R>) -> result::Result<State, DecompressorError> {
		match in_stream.read_n_bits(2) {
			Ok(btype) => match (btype[1], btype[0]) {
				(false, false) => Ok(State::BType(BType::NoCompression)),
				(false, true) => Ok(State::BType(BType::CompressedWithFixedHuffmanCodes)),
				(true, false) => Ok(State::BType(BType::CompressedWithDynamicHuffmanCodes)),
				(true, true) => Err(DecompressorError::BlockTypeReserved),
			},
			Err(_) => Err(DecompressorError::UnexpectedEOF),
		}
	}

	fn create_fixed_huffman_codes() -> result::Result<State, DecompressorError> {
		let lengths = [vec!(8; 144), vec!(9; 112), vec!(7; 24), vec!(8; 8)].concat();

		State::HuffmanCodes(huffman::codes_from_lengths(lengths))
	}

	fn parse_next_symbol<R: Read>(ref mut in_stream: &mut BitReader<R>) -> result::Result<State, DecompressorError> {

		unimplemented!();
	}

	pub fn decompress<R: Read>(&mut self, ref mut in_stream: &mut BitReader<R>) -> io::Result<Vec<u8>>{
		loop {
			match self.state.clone() {
				State::HeaderBegin => {
					self.state = match Self::parse_bfinal(*in_stream) {
						Ok(state) => state,
						Err(e) => panic!(e),
					}
				},
				State::BFinal(bfinal) => {
					self.header.bfinal = Some(bfinal);
					self.state = match Self::parse_btype(*in_stream) {
						Ok(state) => state,
						Err(e) => panic!(e),
					}
				},
				State::BType(btype) => {
					self.header.btype = Some(btype.clone());
					self.state = State::HandlingHuffmanCodes(btype);
				},
				State::HandlingHuffmanCodes(BType::NoCompression) => {
					unimplemented!();
				},
				State::HandlingHuffmanCodes(BType::CompressedWithFixedHuffmanCodes) => {
					self.state = Self::create_fixed_huffman_codes();
				},
				State::HandlingHuffmanCodes(BType::CompressedWithDynamicHuffmanCodes) => {
					unimplemented!();
				},
				State::HuffmanCodes(huffman_codes) {
					self.huffman_codes = Some(huffman_codes);
					self.state = Self::parse_next_symbol();
				},
				State::Symbol(byte @ 0...255) => {
					// literal byte
					println!("byte {:?}", byte);
					unimplemented!()
				},
				State::Symbol(256) => {
					// end of block
					println!("end-of-block");
					unimplemented!()
				},
				State::Symbol(length_code @ 257...285) => {
					// length code
					println!("length code {:?}", length_code);
					unimplemented!()
				},
			}
		}
	}
}


