
struct QoiHeader {
	magic: [u8; 4], // can use char but size will grow
	width: u32,
	height: u32,
	channels: u8,
	colorspace: u8
}

fn index_hash(r: u8, g: u8, b: u8, a: u8) -> usize {
	((r*3 + g*5 + b*7 + a*11) % 64) as usize
}
