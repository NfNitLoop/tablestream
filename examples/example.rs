use std::io::{self, stdout};

use tablestream::{Stream, Column};

fn main() -> io::Result<()> {
    let stdout = stdout();
    let mut handle = stdout.lock();



    let mut stream = Stream::new(&mut handle, vec![
        Column::new(|c: &City| &c.name).header("City"),
        Column::new(|c: &City| &c.country).header("Country"),
        Column::new(|c: &City| &c.population).header("Population"),
        // TODO: Column::new(|c: &City| &format!("{}", &c.population)).header("Population"),
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
    ]
}

// 1 | Shanghai, China | 24.15 million people | 
// 2 | Beijing, China | 21.7 million people | 
// 3 | Lagos, Nigeria | 21.32 million people | 
// 4 | Tianjin, China | 15.47 million people | 
// 5 | Karachi, Sindh, Pakistan | 14.92 million people | 
// 6 | Istanbul, Turkey | 14.8 million people | 
// 7 | Dhaka, Bangladesh | 14.54 million people | 
// 8 | Chengdu, Sichuan, China | 14.43 million people | 
// 9 | Tokyo, Japan | 13.62 million people | 
// 10 | Guangzhou, Guangdong, China | 13.5 million people | 
// 11 | Mumbai, Maharashtra, India | 12.44 million people | 
// 12 | Moscow, Russia | 12.38 million people | 
// 13 | Bengaluru, Karnataka, India | 12.34 million people | 
// 14 | Zhoukou, Henan, China | 12.07 million people | 
// 15 | São Paulo, Brazil | 12.04 million people | 
// 16 | Kinshasa, Kinshasa City, Democratic Republic of the Congo | 11.86 million people | 
// 17 | Nanyang, Henan, China | 11.68 million people | 
// 18 | Baoding, Hebei, China | 11.19 million people | 
// 19 | Delhi, India | 11.03 million people | 
// 20 | Lima, Peru | 10.85 million people | 