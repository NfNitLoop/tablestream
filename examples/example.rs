use std::io::{self, stdout};

use tablestream::{Column, Stream, col};

fn main() -> io::Result<()> {
    let stdout = stdout();
    let mut handle = stdout.lock();



    let mut stream = Stream::new(&mut handle, vec![
        Column::new(|f, c: &City| write!(f, "{}", &c.name)).header("City"),
        col!(City: .country).header("Country"),
        col!(City: "{:.2e}", .population).header("Population"),
    ]);

    for city in largest_cities() {
        stream.row(city)?;
    }

    stream.finish()?;

    Ok(())
}


struct City {
    name: String,
    country: String,
    population: u32,
}

impl City {
    fn new(name: &str, country: &str, population: u32) -> Self {
        Self {
            name: name.to_string(),
            country: country.to_string(),
            population,
        }
    }
}

fn largest_cities() -> Vec<City> {
    vec![
        City::new("Shanghai", "China", 24_150_000),
        City::new("Beijing", "China", 21_700_000),
        City::new("Lagos", "Nigeria", 21_320_000),
        City::new("Tianjin", "China", 15_470_000),
        City::new("Karachi", "Pakistan", 14_920_000),
        City::new("Istanbul", "Turkey", 14_800_000),
        City::new("Dhaka", "Bangladesh", 14_540_000),
        City::new("Chengdu", "China", 14_430_000),
        City::new("Tokyo", "Japan", 13_620_000),
        City::new("Guangzhou", "China",  13_500_000),
        City::new("Mumbai", "India", 12_440_000),
        City::new("Moscow", "Russia", 12_380_000),
        City::new("Bengaluru", "India", 12_340_000),
        City::new("Zhoukou", "China", 12_070_000),
        City::new("São Paulo", "Brazil", 12_040_000),
        City::new("Kinshasa", "Democratic Republic of the Congo", 11_860_000),
        City::new("Nanyang", "China", 11_680_000),
        City::new("Baoding", "China", 11_190_000),
        City::new("Delhi", "India", 11_030_000),
        City::new("Lima", "Peru", 10_850_000),    ]
}

