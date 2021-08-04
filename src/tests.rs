use std::{fmt};

use crate::{Column, Stream};


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
            text: "lorum ipsum dolor sit amet. Or something to\
                  that effect. I don't speak Latin so it's hard\
                  to remember that text off the top of my head.\
                  ".to_string(),
        }
    ]
}

fn cols_3() -> Vec<Column<Person>> {
    vec![
        Column::new(|p: &Person| &p.name).header("Name"),
        Column::new(|p: &Person| &p.age).header("Age"),
        Column::new(|p: &Person| &p.favorite_color).header("Favorite Color"),
    ]
}

fn cols_4() -> Vec<Column<Person>> {
    vec![
        Column::new(|p: &Person| &p.name).header("Name"),
        Column::new(|p: &Person| &p.age).header("Age"),
        Column::new(|p: &Person| &p.favorite_color).header("Favorite Color"),
        Column::new(|p: &Person| &p.text).header("Text"),
    ]
}


#[test]
fn basic() -> fmt::Result {

    let mut out = String::new();
    let mut s = Stream::new(
        &mut out,
        cols_3(),
    );

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

    assert_eq!(expected, out);

    Ok(())
}

#[test]
fn basic_border() -> fmt::Result {

    let mut out = String::new();
    let mut s = Stream::new(
        &mut out,
        cols_3(),
    ).borders(true);

    for person in sample_data().into_iter().take(1) {
        s.row(person)?;
    }

    s.finish()?;

    let expected = "
-------------------------------
| Name | Age | Favorite Color |
-------------------------------
| Cody | 41  | yellow         |
-------------------------------
".trim_start();

    assert_eq!(expected, out);

    Ok(())
}

#[test]
fn longer_text() -> fmt::Result {
    let mut out = String::new();
    let mut s = Stream::new(
        &mut out,
        cols_4(),
    ).borders(true);

    for person in sample_data() {
        s.row(person)?;
    }

    s.finish()?;

    let expected ="\
--------------------------------------------------------------------------------
| Name | Ag | Favori | Text                                                    |
--------------------------------------------------------------------------------
| Cody | 41 | yellow | Here's a long string of text. It's probably going to be |
| Bob  | 99 | beige  | lorum ipsum dolor sit amet. Or something tothat effect. |
--------------------------------------------------------------------------------
";

    assert_eq!(expected, out);

    Ok(())
}

