use datafusion::arrow::array::{ArrayRef, Int32Array, Int64Array};
use datafusion::arrow::datatypes::{DataType, Field, Schema};
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::datasource::MemTable;
use datafusion::prelude::SessionContext;
use std::error::Error;
use std::sync::Arc;
use tokio::select;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int32, false),
        Field::new("bank_account", DataType::Int64, true),
    ]));

    let ids = vec![1, 2, 3, 4, 5];
    let accs = vec![2, 3, 4, 5, 6];
    let columns: Vec<ArrayRef> = vec![
        Arc::new(Int32Array::from(ids)),
        Arc::new(Int64Array::from(accs)),
    ];

    let record_batch = RecordBatch::try_new(schema.clone(), columns)?;
    let mem_table = MemTable::try_new(schema, vec![vec![record_batch]])?;

    let context = SessionContext::new();
    context.register_table("idacs", Arc::new(mem_table))?;

    let df = context.sql("select * from idacs").await?;

    df.show().await?;

    Ok(())
}
