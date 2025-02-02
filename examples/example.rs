use std::io::{self, stdout};


use clap::Parser;
use tablestream::{Column, Stream, col};

fn main() -> io::Result<()> {

    let opts = Opts::parse();

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

#[derive(Parser)]
struct Opts {
    /// Grow the table to the full terminal width.
    #[arg(long)]
    grow: Option<bool>,

    #[arg(long)]
    borders: bool,

    /// Repeat the data to simulate lots of data.
    #[arg(long, default_value = "1")]
    repeat: u16,

    #[arg(long)]
    no_padding: bool,

    #[arg(long)]
    format_pop: bool,

    #[arg(long)]
    unicode: bool,

    #[arg(long)]
    title: Option<String>,

    /// Show total population of these cities.
    #[arg(long)]
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
        City::new("São Paulo", "Brazil", 12_040_000),
        City::new("Kinshasa", "Democratic Republic of the Congo", 11_860_000),
        City::new("Nanyang", "China", 11_680_000),
        City::new("Baoding", "China", 11_190_000),
        City::new("Delhi", "India", 11_030_000),
        City::new("Lima", "Peru", 10_850_000),    ]
}

fn cities_unicode() -> Vec<City> {
    vec![
        City::new("上海市", "中国", 24_150_000),
        City::new("北京市", "中国", 21_700_000),
        City::new("Lagos", "Nigeria", 21_320_000),
        City::new("天津市", "中国", 15_470_000),
        
        // TODO: OK, Urdu is right-to-left and that'll come later. 
        City::new("Karachi", "Pakistan", 14_920_000),
        
        City::new("İstanbul", "Türkiye Cumhuriyeti", 14_800_000),
        
        // TODO: Windows Terminal really does not like this for some reason:
        // City::new("ঢাকা", "গণপ্রজাতন্ত্রী বাংলাদেশ", 14_540_000),
        City::new("Dhaka", "Bangladesh", 14_540_000),


        City::new("成都市", "中国", 14_430_000),
        City::new("東京都", "日本", 13_620_000),
        City::new("广州市", "中国",  13_500_000),
        City::new("Mumbai", "India", 12_440_000),
        City::new("Москва", "Российская Федерация", 12_380_000),
        City::new("Bengaluru", "India", 12_340_000),
        City::new("周口市", "中国", 12_070_000),
        City::new("São Paulo", "Brazil", 12_040_000),
        City::new("Kinshasa", "République démocratique du Congo", 11_860_000),
        City::new("南阳市", "中国", 11_680_000),
        City::new("保定市", "中国", 11_190_000),
        City::new("Delhi", "India", 11_030_000),
        City::new("Lima", "República del Perú", 10_850_000),    ]
}