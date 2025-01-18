use std::os::unix::fs::MetadataExt;
use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("./tmp.txt").unwrap();

    let mut block = 0;

    while file.metadata().unwrap().len() < 1_048_577 {
        let metadata = file.metadata().unwrap();

        file.write(&[1]);

        if (metadata.blocks() != block) {
            println!("Blocks: {}", metadata.blocks());
            println!("On disk: {} ", metadata.len());

            block = metadata.blocks();
        }
    }
}
