use datafusion::prelude::*;
use std::error::Error;
use std::sync::Arc;
use datafusion::arrow::util::pretty::print_batches;
use datafusion::arrow::datatypes::{DataType, Field, Schema};
use datafusion::datasource::listing::{ListingTable, ListingTableConfig, ListingTableUrl, ListingOptions};
use datafusion::datasource::file_format::csv::CsvFormat;
use datafusion::datasource::file_format::parquet::ParquetFormat;
use datafusion::dataframe::DataFrameWriteOptions;
use tokio::fs;
use crate::config::{get_csv_file_path, get_parquet_file_path};
use crate::udf::register_udfs;

async fn create_sample_files() -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("data").await?;
    
    // Create CSV file
    let csv_content = "id,amount,category\n1,500,Food\n2,1500,Electronics\n3,3000,Travel\n4,7000,Luxury";
    fs::write(get_csv_file_path(), csv_content).await?;

    // Create Parquet file with the same schema
    let ctx = SessionContext::new();
    let df = ctx.read_csv(get_csv_file_path().to_str().unwrap(), CsvReadOptions::new()).await?;
    df.write_parquet(
        get_parquet_file_path().to_str().unwrap(),
        DataFrameWriteOptions::default(),
        None
    ).await?;

    Ok(())
}

pub async fn execute_query() -> Result<(), Box<dyn Error>> {
    let ctx = SessionContext::new();
    
    // Create sample files first
    create_sample_files().await?;

    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int64, false),
        Field::new("amount", DataType::Float64, false),
        Field::new("category", DataType::Utf8, false),
    ]));

    let csv_options = ListingOptions::new(Arc::new(CsvFormat::default()))
        .with_file_extension(".csv");
    let csv_config = ListingTableConfig::new(ListingTableUrl::parse(
        get_csv_file_path().to_str().unwrap()
    )?).with_schema(schema.clone())
        .with_listing_options(csv_options);
    let csv_table = ListingTable::try_new(csv_config)?;
    ctx.register_table("transactions", Arc::new(csv_table))?;

    let parquet_options = ListingOptions::new(Arc::new(ParquetFormat::default()))
        .with_file_extension(".parquet");
    let parquet_config = ListingTableConfig::new(ListingTableUrl::parse(
        get_parquet_file_path().to_str().unwrap()
    )?).with_schema(schema)
        .with_listing_options(parquet_options);
    let parquet_table = ListingTable::try_new(parquet_config)?;
    ctx.register_table("analytics", Arc::new(parquet_table))?;

    register_udfs(&ctx);
    let sql = "SELECT t.id, t.amount, a.category FROM transactions t LEFT JOIN analytics a ON t.id = a.id WHERE t.amount > 1000 ORDER BY t.amount DESC LIMIT 10";
    let df = ctx.sql(sql).await?;
    let batches = df.collect().await?;

    print_batches(&batches)?;
    Ok(())
}
