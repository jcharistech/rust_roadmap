use polars::prelude::*;
use plotly::{Plot,Bar, Scatter};

fn main() {
    println!("Hello, world!");

    // Read A CSV File in Polars
    let df = CsvReadOptions::default()
    .try_into_reader_with_file_path(Some("diamonds.csv".into()))
    .unwrap()
    .finish()
    .unwrap();

    // Preview
    // println!("{}",df);

    // // // Head
    // println!("{}",df.head(Some(4)));

    //Tail
    println!("{}",df.tail(Some(4)));

    //Shape
    println!("{:?}",df.shape());

    // Get the size,nrows
    println!("{:?}",df.size());

    // Selection of Columns
    // Select one column
    // println!("{:?}",df.column("color"));

    // Select Multiple columns
    let sv: Vec<&Column> = df.columns(["color","clarity"]).expect("Failed to Work");
    println!("{:?}",sv);

    // Select using the Select
    let result = df
    .clone()
    .select(["cut","price"]);
    println!("{:?}", result);

    // Missing Nan and Null
    let null_count = df.null_count();
    println!("{}",null_count);


    // Fill NaN /Null
    let df2 = df! (
        "col1" => [0.5, 1.0, 1.5, 2.0, 2.5],
        "col2" => [Some(1), None, Some(3), None, Some(5)],
    ).expect("Failed");

    println!("{}", df2);

    // with_column: Modify and create a new column
    let fill_literal_df = df2
    .clone()
    .lazy()
    .with_column(col("col2").fill_null(3))
    .collect().expect("Failed");

println!("{}", fill_literal_df);

let is_null_series = df
    .clone()
    .lazy()
    .select([col("cut").is_null()])
    .collect().expect("Failed");
println!("{}", is_null_series);

// Unique Values or Class Distrubution
let class_distr = df
.clone()
.lazy()
.select([col("cut").n_unique().alias("n_unique"),])
.collect().expect("Failed");
println!("{}",class_distr);


// Filter Groupby
let grouped_diamonds = df.clone().lazy().group_by(["cut"]).agg([
    col("price").sum().alias("total_price"),
    col("price").mean().alias("average_price"),
    col("price").count().alias("counts"),
]).collect().expect("Failed");

println!("{:?}", grouped_diamonds);

// Concat Two DF
let df_v1 = df!(
    "a"=> &[1],
    "b"=> &[3],
    "c"=> &[4],).unwrap();

let df_v2 = df!(
        "a"=> &[5],
        "b"=> &[6],
        "c"=> &[8],).unwrap();

// Vertical Concat
let df_vertical = concat([df_v1.clone().lazy(),df_v2.clone().lazy()],
UnionArgs::default())
.unwrap()
.collect().expect("Failed to concat");

println!("{}", &df_vertical);

// Thanks for your
// Jesus Saves
let mut dataset = LazyCsvReader::new("diamonds.csv").finish().unwrap().collect().unwrap();
let x = dataset.column("cut").unwrap().str().into_iter().flatten().collect(); // convert to vec assuming no nulls
let y = dataset.column("price").unwrap().f64().into_iter().flatten().collect();
let trace = Bar::new(x, y).show_legend(true).opacity(0.5);

let mut plot = Plot::new();
plot.add_trace(trace);
plot.show();

}
