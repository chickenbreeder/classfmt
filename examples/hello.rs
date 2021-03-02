use std::{
    fs::File,
    path::Path,
    io,
    io::Read
};

use classfmt::parser::ClassParser;

fn main() -> io::Result<()> {
    let mut f = File::open(Path::new("./tests/Fields.class"))?;
    let mut buf = Vec::with_capacity(64);

    f.read_to_end(&mut buf)?;

    let mut parser = ClassParser::from_bytes(&buf);
    let class = parser.parse().unwrap();
    println!("{:#?}", class);

    Ok(())
}
