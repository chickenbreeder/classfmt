use std::{fs::File, io, io::Read, path::Path};

use classfmt::ClassParser;

fn main() -> io::Result<()> {
    let mut f = File::open(Path::new("./tests/Fields.class"))?;
    let mut buf = Vec::with_capacity(64);
    f.read_to_end(&mut buf)?;

    let class = ClassParser::from_bytes(&buf).parse().unwrap();

    println!("{:#?}", class);
    Ok(())
}
