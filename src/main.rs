fn main() {
    // load data/ETH_1H.csv file
    // Unix Timestamp,Date,Symbol,Open,High,Low,Close,Volume is expected column headers
    let data = load_csv("data/ETH_1H.csv");

    // print first 5 rows
    println!("{:?}", &data[0..5]);

    // convert all rows to candles
    let candles: Vec<Candle> = data.iter().map(|row| {
        let hour = (row.timestamp / 1000 / 3600) % 24; // divide by 1000 because timestamp is in milliseconds
        Candle {
            hour,
            open: row.open,
            close: row.close,
            difference: (row.close - row.open),
        }
    }).collect();

    // print first 5 candles
    println!("{:?}", &candles[0..5]);

    // for each hour, calculate the average difference
    let mut average_differences: Vec<(u64, f64)> = Vec::new();

    for hour in 0..24 {
        let mut total_difference = 0.0;
        let mut count = 0;
        for candle in &candles {
            if candle.hour == hour {
                total_difference += candle.difference;
                count += 1;
            }
        }
        let average_difference = total_difference / count as f64;
        average_differences.push((hour, average_difference));
    }

    // print average differences
    println!("{:?}", average_differences);

    // find the hour with the highest average difference
    let mut highest_average_difference = 0.0;
    let mut highest_average_difference_hour = 0;
    for (hour, average_difference) in &average_differences {
        if *average_difference > highest_average_difference {
            highest_average_difference = *average_difference;
            highest_average_difference_hour = *hour;
        }
    }

    // print the hour with the highest average difference
    println!("The hour with the highest average difference is {}:00-{}:00 UTC with a difference of ${}", highest_average_difference_hour, highest_average_difference_hour + 1, highest_average_difference);

}

fn load_csv(path: &str) -> Vec<Row> {
    let mut rdr = csv::Reader::from_path(path).unwrap();
    let mut candles: Vec<Row> = Vec::new();
    for result in rdr.deserialize() {
        let candle: Row = result.unwrap();
        candles.push(candle);
    }
    candles
}

#[derive(Debug, serde::Deserialize)]
struct Row {
    #[serde(rename = "Unix Timestamp")]
    timestamp: u64,
    #[serde(rename = "Open")]
    open: f64,
    #[serde(rename = "Close")]
    close: f64,
}

#[derive(Debug)]
struct Candle {
    hour: u64, // 0-23 UTC
    open: f64,
    close: f64,
    difference: f64,
}