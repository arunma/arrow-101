use arrow::array::{ArrayRef, Float32Array, Int8Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use std::sync::Arc;

fn main() {
    let fields = vec![
        Field::new("name", DataType::Utf8, false),
        Field::new("age", DataType::Int8, false),
        Field::new("score", DataType::Float32, false),
    ];
    let schema = Arc::new(Schema::new(fields));

    let names = StringArray::from(vec!["a", "b", "c", "d"]);
    let ages = Int8Array::from(vec![1, 2, 3, 4]);
    let scores = Float32Array::from(vec![1.0, 2.0, 3.0, 4.0]);

    let columns: Vec<ArrayRef> = vec![Arc::new(names), Arc::new(ages), Arc::new(scores)];

    let record_batch = RecordBatch::try_new(schema, columns);

    println!("{record_batch:?}")
}
