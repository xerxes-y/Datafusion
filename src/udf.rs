use datafusion::prelude::*;
use datafusion::arrow::array::{Array, Float64Array, StringBuilder};
use datafusion::arrow::datatypes::DataType;
use datafusion::logical_expr::{ColumnarValue, Signature, Volatility};
use datafusion::error::{Result, DataFusionError};
use std::sync::Arc;

/// Registers the classify_transaction UDF with the provided SessionContext
pub fn register_udfs(ctx: &SessionContext) {
    let udf = create_udf(
        "classify_transaction",
        vec![DataType::Float64],
        Arc::new(DataType::Utf8),
        Volatility::Immutable,
        Arc::new(move |args: &[ColumnarValue]| -> Result<ColumnarValue> {
            let array = match &args[0] {
                ColumnarValue::Array(array) => array.clone(),
                _ => {
                    return Err(DataFusionError::Execution(
                        "Expected array input".to_string(),
                    ))
                }
            };

            let amounts = array.as_any().downcast_ref::<Float64Array>().unwrap();
            let mut builder = StringBuilder::new();

            for i in 0..amounts.len() {
                if amounts.is_null(i) {
                    builder.append_null();
                } else {
                    let category = if amounts.value(i) > 5000.0 {
                        "High Value"
                    } else {
                        "Regular"
                    };
                    builder.append_value(category);
                }
            }

            Ok(ColumnarValue::Array(Arc::new(builder.finish())))
        }),
    );
    ctx.register_udf(udf);
}
