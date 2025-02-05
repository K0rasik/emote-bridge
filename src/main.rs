mod avif_decoder;
use avif_decoder::*;

fn main() {
    let decoder = AvifDecoder::new("emote.avif").expect("Failed to decode AVIF");
    let frames = decoder.get_frames();
    println!("Decoded {} frames.", frames.len());
}

