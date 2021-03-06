use std::io::{self, stdout};

use structopt::StructOpt;

use tablestream::{Column, Stream, col};

fn main() -> io::Result<()> {

    let opts = Opts::from_args();

    let stdout = stdout();
    let mut handle = stdout.lock();

    let mut cols = vec![
        Column::new(|f, c: &City| write!(f, "{}", &c.name)).header("City"),
        col!(City: .country).header("Country"),
        
    ];

    if opts.format_pop {
        cols.push(col!(City: "{:.2e}", .population).header("Population").right());
    } else {
        cols.push(col!(City: .population).header("Population").right());
    }


    let mut stream = Stream::new(&mut handle, cols);
    if let Some(g) = opts.grow {
        stream = stream.grow(g);
    }
    if let Some(title) = opts.title {
        stream = stream.title(&title);
    }
    stream = stream.borders(opts.borders).padding(!opts.no_padding);

    let cities = if opts.unicode { cities_unicode() } else { largest_cities() };
    let total_pop: u32 = cities.iter().map(|c| c.population).sum();

    // Generally don't want to clone like this but just doing so to simulate long tables:
    for city in cities.iter().cycle().take(opts.repeat as usize * cities.len()).cloned() {
        stream.row(city)?;
    }

    if opts.total {
        let footer = format!("Total Population: {}", total_pop);
        stream.footer(&footer)?;
    } else {
        stream.finish()?;
    }

    Ok(())
}

#[derive(StructOpt)]
struct Opts {
    /// Grow the table to the full terminal width.
    #[structopt(long)]
    grow: Option<bool>,

    #[structopt(long)]
    borders: bool,

    /// Repeat the data to simulate lots of data.
    #[structopt(long, default_value = "1")]
    repeat: u16,

    #[structopt(long)]
    no_padding: bool,

    #[structopt(long)]
    format_pop: bool,

    #[structopt(long)]
    unicode: bool,

    #[structopt(long)]
    title: Option<String>,

    /// Show total population of these cities.
    #[structopt(long)]
    total: bool,

}


#[derive(Clone)]
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
        City::new("S??o Paulo", "Brazil", 12_040_000),
        City::new("Kinshasa", "Democratic Republic of the Congo", 11_860_000),
        City::new("Nanyang", "China", 11_680_000),
        City::new("Baoding", "China", 11_190_000),
        City::new("Delhi", "India", 11_030_000),
        City::new("Lima", "Peru", 10_850_000),    ]
}

fn cities_unicode() -> Vec<City> {
    vec![
        City::new("?????????", "??????", 24_150_000),
        City::new("?????????", "??????", 21_700_000),
        City::new("Lagos", "Nigeria", 21_320_000),
        City::new("?????????", "??????", 15_470_000),
        
        // TODO: OK, Urdu is right-to-left and that'll come later. 
        City::new("Karachi", "Pakistan", 14_920_000),
        
        City::new("??stanbul", "T??rkiye Cumhuriyeti", 14_800_000),
        
        // TODO: Windows Terminal really does not like this for some reason:
        // City::new("????????????", "?????????????????????????????????????????? ????????????????????????", 14_540_000),
        City::new("Dhaka", "Bangladesh", 14_540_000),


        City::new("?????????", "??????", 14_430_000),
        City::new("?????????", "??????", 13_620_000),
        City::new("?????????", "??????",  13_500_000),
        City::new("Mumbai", "India", 12_440_000),
        City::new("????????????", "???????????????????? ??????????????????", 12_380_000),
        City::new("Bengaluru", "India", 12_340_000),
        City::new("?????????", "??????", 12_070_000),
        City::new("S??o Paulo", "Brazil", 12_040_000),
        City::new("Kinshasa", "R??publique d??mocratique du Congo", 11_860_000),
        City::new("?????????", "??????", 11_680_000),
        City::new("?????????", "??????", 11_190_000),
        City::new("Delhi", "India", 11_030_000),
        City::new("Lima", "Rep??blica del Per??", 10_850_000),    ]
}