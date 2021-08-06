use std::io;

use tablestream::{Stream, col};

/// You shouldn't need to import Column to use col!().
#[test]
fn macro_without_import() -> io::Result<()> {

    let mut out = vec![];
    struct Row { name: &'static str }
    let mut stream = Stream::new(&mut out, vec![
        col!(Row: .name),
    ]);

    stream.row(Row{name: "Hello, world!"})?;
    stream.finish()?;

    let out = String::from_utf8(out).unwrap();
    println!("{}", &out);
    let expected = "\
-------------
Hello, world!
-------------
";

    assert_eq!(expected, out);
    Ok(())
}
