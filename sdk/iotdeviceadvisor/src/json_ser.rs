// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_suite_definition_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSuiteDefinitionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.suite_definition_configuration {
        let mut object_2 = object.key("suiteDefinitionConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_suite_definition_configuration(
            &mut object_2,
            var_1,
        )?;
        object_2.finish();
    }
    if let Some(var_3) = &input.tags {
        let mut object_4 = object.key("tags").start_object();
        for (key_5, value_6) in var_3 {
            {
                object_4.key(key_5).string(value_6);
            }
        }
        object_4.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_suite_run_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartSuiteRunInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_7) = &input.suite_definition_version {
        object.key("suiteDefinitionVersion").string(var_7);
    }
    if let Some(var_8) = &input.suite_run_configuration {
        let mut object_9 = object.key("suiteRunConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_suite_run_configuration(
            &mut object_9,
            var_8,
        )?;
        object_9.finish();
    }
    if let Some(var_10) = &input.tags {
        let mut object_11 = object.key("tags").start_object();
        for (key_12, value_13) in var_10 {
            {
                object_11.key(key_12).string(value_13);
            }
        }
        object_11.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_14) = &input.tags {
        let mut object_15 = object.key("tags").start_object();
        for (key_16, value_17) in var_14 {
            {
                object_15.key(key_16).string(value_17);
            }
        }
        object_15.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_suite_definition_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateSuiteDefinitionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_18) = &input.suite_definition_configuration {
        let mut object_19 = object.key("suiteDefinitionConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_suite_definition_configuration(
            &mut object_19,
            var_18,
        )?;
        object_19.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_suite_definition_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SuiteDefinitionConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_20) = &input.suite_definition_name {
        object.key("suiteDefinitionName").string(var_20);
    }
    if let Some(var_21) = &input.devices {
        let mut array_22 = object.key("devices").start_array();
        for item_23 in var_21 {
            {
                let mut object_24 = array_22.value().start_object();
                crate::json_ser::serialize_structure_crate_model_device_under_test(
                    &mut object_24,
                    item_23,
                )?;
                object_24.finish();
            }
        }
        array_22.finish();
    }
    if input.intended_for_qualification {
        object
            .key("intendedForQualification")
            .boolean(input.intended_for_qualification);
    }
    if let Some(var_25) = &input.root_group {
        object.key("rootGroup").string(var_25);
    }
    if let Some(var_26) = &input.device_permission_role_arn {
        object.key("devicePermissionRoleArn").string(var_26);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_suite_run_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SuiteRunConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_27) = &input.primary_device {
        let mut object_28 = object.key("primaryDevice").start_object();
        crate::json_ser::serialize_structure_crate_model_device_under_test(&mut object_28, var_27)?;
        object_28.finish();
    }
    if let Some(var_29) = &input.selected_test_list {
        let mut array_30 = object.key("selectedTestList").start_array();
        for item_31 in var_29 {
            {
                array_30.value().string(item_31);
            }
        }
        array_30.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_device_under_test(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DeviceUnderTest,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_32) = &input.thing_arn {
        object.key("thingArn").string(var_32);
    }
    if let Some(var_33) = &input.certificate_arn {
        object.key("certificateArn").string(var_33);
    }
    Ok(())
}