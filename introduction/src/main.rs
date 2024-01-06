use chrono::NaiveDate;
use polars::{df, frame::DataFrame, prelude::*};

fn main() {
    let df: DataFrame = df!("integer" => &[1,2,3], "date" => &[NaiveDate::from_ymd_opt(2022, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),NaiveDate::from_ymd_opt(2022, 1, 2).unwrap().and_hms_opt(0, 0, 0).unwrap(),NaiveDate::from_ymd_opt(2022, 1, 3).unwrap().and_hms_opt(0, 0, 0).unwrap()], "float" => &[4.0, 5.0, 6.0]).unwrap();

    println!("{:?}", df);
}
