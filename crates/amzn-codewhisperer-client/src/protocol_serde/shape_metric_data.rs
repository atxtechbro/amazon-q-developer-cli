// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_metric_data(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::MetricData,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("metricName").string(input.metric_name.as_str());
    }
    {
        object.key("metricValue").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((input.metric_value).into()),
        );
    }
    {
        object
            .key("timestamp")
            .date_time(&input.timestamp, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    {
        object.key("product").string(input.product.as_str());
    }
    if let Some(var_1) = &input.dimensions {
        let mut array_2 = object.key("dimensions").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_dimension::ser_dimension(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    Ok(())
}
