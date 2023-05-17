use datafusion::prelude::{CsvReadOptions, SessionContext};
use std::error::Error;
use datafusion::common::context;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let context = SessionContext::new();
    context.register
    context
        .register_csv(
            "student_results",
            "data/student_results.csv",
            CsvReadOptions::new(),
        )
        .await?;

    let results = context.sql("select * from student_results").await?;

    let average = context
        .sql("select avg(english) as english_avg, avg(math) as math_avg from student_results")
        .await?;

    results.show_limit(10).await?;
    average.show_limit(10).await?;
    Ok(())
}
