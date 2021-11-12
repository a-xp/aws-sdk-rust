// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_operation_crate_operation_create_home_region_control(
    input: &crate::input::CreateHomeRegionControlInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_create_home_region_control_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_describe_home_region_controls(
    input: &crate::input::DescribeHomeRegionControlsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_describe_home_region_controls_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_get_home_region(
    _input: &crate::input::GetHomeRegionInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
    Ok(aws_smithy_http::body::SdkBody::from("{}"))
}