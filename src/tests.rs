use std::{io};

use crate::{Column, Stream, col};


struct Person {
    name: String,
    age: u8,
    favorite_color: String,
    text: String,
}


fn sample_data() -> Vec<Person> {
    vec![
        Person{
            name: "Cody".to_string(),
            age: 41,
            favorite_color: "yellow".to_string(),
            text: "Here's a long string of text. It's probably \
                  going to be too long to fit on-screen without \
                  wrapping. It should probably get truncated.\
                  ".to_string(),
        },

        Person{
            name: "Bob".to_string(),
            age: 99,
            favorite_color: "beige".to_string(),
            text: "lorum ipsum dolor sit amet. Or something to \
                  that effect. I don't speak Latin so it's hard \
                  to remember that text off the top of my head.\
                  ".to_string(),
        }
    ]
}

fn cols_3() -> Vec<Column<Person>> {
    vec![
        col!(Person: .name).header("Name"),
        col!(Person: .age).header("Age"),
        col!(Person: .favorite_color).header("Favorite Color"),
    ]
}

fn cols_4() -> Vec<Column<Person>> {
    vec![
        col!(Person: .name).header("Name"),
        col!(Person: .age).header("Age"),
        col!(Person: .favorite_color).header("Favorite Color"),
        col!(Person: .text).header("Text"),
    ]
}


#[test]
fn basic() -> io::Result<()> {

    let mut out = Vec::new();
    let mut s = Stream::new(
        &mut out,
        cols_3(),
    ).max_width(80);

    for person in sample_data().into_iter().take(1) {
        s.row(person)?;
    }

    s.finish()?;

    let expected = "
---------------------------
Name | Age | Favorite Color
---------------------------
Cody | 41  | yellow        
---------------------------
".trim_start();

    let out = String::from_utf8(out).unwrap();
    assert_eq!(expected, out);

    Ok(())
}

#[test]
fn basic_border() -> io::Result<()> {

    let mut out = Vec::new();
    let mut s = Stream::new(
        &mut out,
        cols_3(),
    ).borders(true).max_width(80);

    for person in sample_data().into_iter().take(1) {
        s.row(person)?;
    }

    s.finish()?;

    let expected = "\
-------------------------------
| Name | Age | Favorite Color |
-------------------------------
| Cody | 41  | yellow         |
-------------------------------
";

    let out = String::from_utf8(out).unwrap();
    assert_eq!(expected, out);

    Ok(())
}

#[test]
fn basic_border_grow() -> io::Result<()> {

    let mut out = Vec::new();
    let mut s = Stream::new(
        &mut out,
        cols_3(),
    ).borders(true).max_width(80).grow(true);

    for person in sample_data().into_iter() {
        s.row(person)?;
    }

    s.finish()?;

    let expected = "\
--------------------------------------------------------------------------------
|         Name         |         Age         |         Favorite Color          |
--------------------------------------------------------------------------------
| Cody                 | 41                  | yellow                          |
| Bob                  | 99                  | beige                           |
--------------------------------------------------------------------------------
";

    let out = String::from_utf8(out).unwrap();
    assert_eq!(expected, out);

    Ok(())
}

#[test]
fn test_align() -> io::Result<()> {

    let mut out = Vec::new();
    let mut cols = cols_3();
    cols[0] = col!(Person: .name).header("Name").right();
    cols[1] = col!(Person: .age).header("Age").center();

    let mut s = Stream::new( &mut out, cols).borders(true).max_width(80).grow(true);

    for person in sample_data().into_iter() {
        s.row(person)?;
    }

    s.finish()?;

    let expected = "\
--------------------------------------------------------------------------------
|         Name         |         Age         |         Favorite Color          |
--------------------------------------------------------------------------------
|                 Cody |         41          | yellow                          |
|                  Bob |         99          | beige                           |
--------------------------------------------------------------------------------
";

    let out = String::from_utf8(out).unwrap();
    assert_eq!(expected, out);

    Ok(())
}

#[test]
fn longer_text_border() -> io::Result<()> {
    let mut out = Vec::new();
    let mut s = Stream::new(
        &mut out,
        cols_4(),
    ).borders(true).max_width(80);

    for person in sample_data() {
        s.row(person)?;
    }

    s.finish()?;

    let expected ="\
--------------------------------------------------------------------------------
| Name | Age | Favorite Color |                      Text                      |
--------------------------------------------------------------------------------
| Cody | 41  | yellow         | Here's a long string of text. It's probably go |
| Bob  | 99  | beige          | lorum ipsum dolor sit amet. Or something to th |
--------------------------------------------------------------------------------
";

    let out = String::from_utf8(out).unwrap();
    assert_eq!(expected, out);

    Ok(())
}

#[test]
fn longer_text() -> io::Result<()> {
    let mut out = Vec::new();
    let mut s = Stream::new(
        &mut out,
        cols_4(),
    ).max_width(80);

    for person in sample_data() {
        s.row(person)?;
    }

    s.finish()?;

    let expected ="\
--------------------------------------------------------------------------------
Name | Age | Favorite Color |                        Text                       
--------------------------------------------------------------------------------
Cody | 41  | yellow         | Here's a long string of text. It's probably going 
Bob  | 99  | beige          | lorum ipsum dolor sit amet. Or something to that e
--------------------------------------------------------------------------------
";

    let out = String::from_utf8(out).unwrap();
    assert_eq!(expected, out);

    Ok(())
}
