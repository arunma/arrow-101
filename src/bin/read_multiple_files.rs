use std::error::Error;
use std::sync::Arc;

use datafusion::arrow::array::{Int32Array, Int64Array};
use datafusion::arrow::datatypes::{Field, Schema};
use datafusion::datasource::file_format::csv::CsvFormat;
use datafusion::datasource::listing::ListingOptions;
use datafusion::prelude::SessionContext;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let context = SessionContext::new();
    let file_format = CsvFormat::default()
        .with_has_header(true)
        .with_delimiter(b',');
    let listing_options = ListingOptions::new(Arc::new(file_format)).with_file_extension(".ssv");

    context
        .register_listing_table("students_ssv", format!("data"), listing_options, None, None)
        .await?;

    println!("Showing dataframe");
    let df = context.sql("select * from students_ssv").await?;
    println!("Count of records: {}", df.count().await?);
    //df.show().await?;

    Ok(())
}
