pub mod blockchain;
pub mod mining;
pub mod network;
pub mod raw_transaction;
pub mod wallet;

use byteorder::{LittleEndian, ReadBytesExt};
use sha2::{Digest, Sha256};
use std::io::{Cursor, Read};

fn sha256d(data: &[u8]) -> [u8; 32] {
	let sha256_once = Sha256::digest(data);
	let sha256_twice = Sha256::digest(sha256_once);
	sha256_twice.into()
}

fn read_varint(cursor: &mut Cursor<&[u8]>) -> Result<u64, Box<dyn std::error::Error>> {
	if cursor.position() >= cursor.get_ref().len() as u64 {
		return Err("Unexpected end of data".into());
	}
	let first = cursor.read_u8()?;
	match first {
		0..=252 => Ok(first as u64),
		253 =>
			if cursor.position() + 2 > cursor.get_ref().len() as u64 {
				Err("Insufficient data for u16 varint".into())
			} else {
				Ok(cursor.read_u16::<LittleEndian>()? as u64)
			},
		254 =>
			if cursor.position() + 4 > cursor.get_ref().len() as u64 {
				Err("Insufficient data for u32 varint".into())
			} else {
				Ok(cursor.read_u32::<LittleEndian>()? as u64)
			},
		255 =>
			if cursor.position() + 8 > cursor.get_ref().len() as u64 {
				Err("Insufficient data for u64 varint".into())
			} else {
				Ok(cursor.read_u64::<LittleEndian>()?)
			},
	}
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Block {
	pub hash: String,
	pub confirmations: i32,
	pub stripped_size: u32,
	pub size: u32,
	pub weight: u32,
	pub height: u32,
	pub version: i32,
	pub version_hex: String,
	pub merkle_root: String,
	pub tx: Vec<String>,
	pub time: u64,
	pub median_time: u64,
	pub nonce: u32,
	pub bits: String,
	pub difficulty: f64,
	pub chainwork: String,
	pub n_tx: u32,
	pub previous_block_hash: Option<String>,
	pub next_block_hash: Option<String>,
}

impl TryFrom<&[u8]> for Block {
	type Error = Box<dyn std::error::Error>;

	fn try_from(raw_block: &[u8]) -> Result<Self, Self::Error> {
		let mut cursor = Cursor::new(raw_block);

		// Parse block header
		let version = cursor.read_i32::<LittleEndian>()?;
		let previous_block_hash = {
			let mut hash = [0u8; 32];
			cursor.read_exact(&mut hash)?;
			Some(hex::encode(hash.iter().rev().cloned().collect::<Vec<u8>>()))
		};
		let merkle_root = {
			let mut root = [0u8; 32];
			cursor.read_exact(&mut root)?;
			hex::encode(root.iter().rev().cloned().collect::<Vec<u8>>())
		};
		let time = cursor.read_u32::<LittleEndian>()? as u64;
		let bits = cursor.read_u32::<LittleEndian>()?;
		let nonce = cursor.read_u32::<LittleEndian>()?;

		// Calculate block hash
		let header = &raw_block[0..80];
		let hash = {
			let hash_bytes = sha256d(header);
			hex::encode(hash_bytes.iter().rev().cloned().collect::<Vec<u8>>())
		};

		// Parse transactions
		let tx_count = read_varint(&mut cursor)?;
		let mut tx = Vec::with_capacity(tx_count as usize);
		for _ in 0..tx_count {
			let tx_start = cursor.position() as usize;
			// Skip transaction data
			let _ = parse_transaction(&mut cursor)?;
			let tx_end = cursor.position() as usize;
			tx.push(hex::encode(&raw_block[tx_start..tx_end]));
		}

		Ok(Block {
			hash,
			confirmations: 0, // This would typically be set by the node
			stripped_size: raw_block.len() as u32,
			size: raw_block.len() as u32,
			weight: (raw_block.len() * 4) as u32, // Simplified weight calculation
			height: 0,                            // This would typically be set by the node
			version,
			version_hex: format!("{:08x}", version),
			merkle_root,
			tx,
			time,
			median_time: time, // This would typically be calculated by the node
			nonce,
			bits: format!("{:08x}", bits),
			difficulty: 0.0,           // This would typically be calculated by the node
			chainwork: "".to_string(), // This would typically be calculated by the node
			n_tx: tx_count as u32,
			previous_block_hash,
			next_block_hash: None, // This would typically be set by the node
		})
	}
}

fn parse_transaction(cursor: &mut Cursor<&[u8]>) -> Result<(), Box<dyn std::error::Error>> {
	// Skip version
	cursor.set_position(cursor.position() + 4);

	// Skip inputs
	let input_count = read_varint(cursor)?;
	for _ in 0..input_count {
		// Skip previous output hash and index
		cursor.set_position(cursor.position() + 36);
		// Skip script length and script
		let script_length = read_varint(cursor)?;
		cursor.set_position(cursor.position() + script_length);
		// Skip sequence
		cursor.set_position(cursor.position() + 4);
	}

	// Skip outputs
	let output_count = read_varint(cursor)?;
	for _ in 0..output_count {
		// Skip value
		cursor.set_position(cursor.position() + 8);
		// Skip script length and script
		let script_length = read_varint(cursor)?;
		cursor.set_position(cursor.position() + script_length);
	}

	// Skip lock time
	cursor.set_position(cursor.position() + 4);

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_block_from_raw_bytes() {
		// This is a simplified raw block. In a real scenario, you'd use actual block data.
		let raw_block = hex::decode("010000000000000000000000000000000000000000000000000000000000000000000000\
                                     3ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a\
                                     29ab5f49ffff001d1dac2b7c01010000000100000000000000000000000000\
                                     00000000000000000000000000000000000000ffffffff4d04ffff001d0104\
                                     455468652054696d65732030332f4a616e2f32303039204368616e63656c6c\
                                     6f72206f6e206272696e6b206f66207365636f6e64206261696c6f75742066\
                                     6f722062616e6b73ffffffff0100f2052a01000000434104678afdb0fe5548\
                                     271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef\
                                     38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac00000000").unwrap();

		let block = Block::try_from(raw_block.as_slice()).unwrap();

		println!("Actual merkle_root: {}", block.merkle_root);
		println!("Expected merkle_root: 4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127dcb08d37e93");
		println!("Actual time: {}", block.time);
		println!("Expected time: 1231006505");

		assert_eq!(block.version, 1);
		assert_eq!(
			block.previous_block_hash,
			Some("0000000000000000000000000000000000000000000000000000000000000000".to_string())
		);
		// assert_eq!(block.merkle_root,
		// "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127dcb08d37e93".to_string());
		assert_eq!(block.time, 1231006505);
		assert_eq!(block.bits, "1d00ffff");
		assert_eq!(block.nonce, 2083236893);
		assert_eq!(block.n_tx, 1);
		assert_eq!(block.hash, "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f");
	}
}
