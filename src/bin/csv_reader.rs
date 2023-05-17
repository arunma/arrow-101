use arrow::csv;
use arrow::datatypes::{DataType, Field, Schema};
use arrow_csv::reader::Format;
use arrow_csv::ReaderBuilder;
use std::error::Error;
use std::fmt::format;
use std::fs::File;
use std::io;
use std::io::Seek;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn Error>> {
    /*    let schema = Schema::new(vec![
        Field::new("student", DataType::Int32, true),
        Field::new("attended_study_group", DataType::Boolean, true),
        Field::new("group", DataType::Int32, true),
        Field::new("english", DataType::Int32, true),
        Field::new("reading", DataType::Int32, true),
        Field::new("math", DataType::Int32, true),
        Field::new("science", DataType::Int32, true),
    ]);*/
    let mut file = File::open("data/student_results.csv")?;
    let format = Format::default().with_header(true).with_delimiter(b',');
    let (schema, _) = format.infer_schema(&mut file, None)?;
    file.rewind()?; //Very Important

    println!("Schema is {schema:?}");

    let mut csv = ReaderBuilder::new(Arc::new(schema))
        .with_format(format)
        .has_header(true)
        //.with_batch_size(100)
        .build(file)?;

    let batch = csv.next().unwrap().unwrap();
    println!("Num columns : {:?}", batch.columns());

    println!("About to print csv");
    for each in csv {
        let each = each?;
        println!("{each:?}")
    }

    /*    let mut file = File::open("data/student_results.csv").unwrap();

    let (schema, _) = Format::default().infer_schema(&mut file, None).unwrap();
    file.rewind().unwrap();

    let mut csv = ReaderBuilder::new(Arc::new(schema))
        .has_header(true)
        .build(file)
        .unwrap();

    let batch = csv.next().unwrap().unwrap();
    assert_eq!(1000, batch.num_rows());
    assert_eq!(7, batch.num_columns());*/

    Ok(())
}
