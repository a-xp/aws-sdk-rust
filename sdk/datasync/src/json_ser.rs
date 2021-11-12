// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_cancel_task_execution_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CancelTaskExecutionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.task_execution_arn {
        object.key("TaskExecutionArn").string(var_1);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_agent_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateAgentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_2) = &input.activation_key {
        object.key("ActivationKey").string(var_2);
    }
    if let Some(var_3) = &input.agent_name {
        object.key("AgentName").string(var_3);
    }
    if let Some(var_4) = &input.tags {
        let mut array_5 = object.key("Tags").start_array();
        for item_6 in var_4 {
            {
                let mut object_7 = array_5.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag_list_entry(
                    &mut object_7,
                    item_6,
                )?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.vpc_endpoint_id {
        object.key("VpcEndpointId").string(var_8);
    }
    if let Some(var_9) = &input.subnet_arns {
        let mut array_10 = object.key("SubnetArns").start_array();
        for item_11 in var_9 {
            {
                array_10.value().string(item_11);
            }
        }
        array_10.finish();
    }
    if let Some(var_12) = &input.security_group_arns {
        let mut array_13 = object.key("SecurityGroupArns").start_array();
        for item_14 in var_12 {
            {
                array_13.value().string(item_14);
            }
        }
        array_13.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_location_efs_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLocationEfsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_15) = &input.subdirectory {
        object.key("Subdirectory").string(var_15);
    }
    if let Some(var_16) = &input.efs_filesystem_arn {
        object.key("EfsFilesystemArn").string(var_16);
    }
    if let Some(var_17) = &input.ec2_config {
        let mut object_18 = object.key("Ec2Config").start_object();
        crate::json_ser::serialize_structure_crate_model_ec2_config(&mut object_18, var_17)?;
        object_18.finish();
    }
    if let Some(var_19) = &input.tags {
        let mut array_20 = object.key("Tags").start_array();
        for item_21 in var_19 {
            {
                let mut object_22 = array_20.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag_list_entry(
                    &mut object_22,
                    item_21,
                )?;
                object_22.finish();
            }
        }
        array_20.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_location_fsx_windows_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLocationFsxWindowsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_23) = &input.subdirectory {
        object.key("Subdirectory").string(var_23);
    }
    if let Some(var_24) = &input.fsx_filesystem_arn {
        object.key("FsxFilesystemArn").string(var_24);
    }
    if let Some(var_25) = &input.security_group_arns {
        let mut array_26 = object.key("SecurityGroupArns").start_array();
        for item_27 in var_25 {
            {
                array_26.value().string(item_27);
            }
        }
        array_26.finish();
    }
    if let Some(var_28) = &input.tags {
        let mut array_29 = object.key("Tags").start_array();
        for item_30 in var_28 {
            {
                let mut object_31 = array_29.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag_list_entry(
                    &mut object_31,
                    item_30,
                )?;
                object_31.finish();
            }
        }
        array_29.finish();
    }
    if let Some(var_32) = &input.user {
        object.key("User").string(var_32);
    }
    if let Some(var_33) = &input.domain {
        object.key("Domain").string(var_33);
    }
    if let Some(var_34) = &input.password {
        object.key("Password").string(var_34);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_location_nfs_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLocationNfsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_35) = &input.subdirectory {
        object.key("Subdirectory").string(var_35);
    }
    if let Some(var_36) = &input.server_hostname {
        object.key("ServerHostname").string(var_36);
    }
    if let Some(var_37) = &input.on_prem_config {
        let mut object_38 = object.key("OnPremConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_on_prem_config(&mut object_38, var_37)?;
        object_38.finish();
    }
    if let Some(var_39) = &input.mount_options {
        let mut object_40 = object.key("MountOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_nfs_mount_options(&mut object_40, var_39)?;
        object_40.finish();
    }
    if let Some(var_41) = &input.tags {
        let mut array_42 = object.key("Tags").start_array();
        for item_43 in var_41 {
            {
                let mut object_44 = array_42.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag_list_entry(
                    &mut object_44,
                    item_43,
                )?;
                object_44.finish();
            }
        }
        array_42.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_location_object_storage_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLocationObjectStorageInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_45) = &input.server_hostname {
        object.key("ServerHostname").string(var_45);
    }
    if let Some(var_46) = &input.server_port {
        object.key("ServerPort").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_46).into()),
        );
    }
    if let Some(var_47) = &input.server_protocol {
        object.key("ServerProtocol").string(var_47.as_str());
    }
    if let Some(var_48) = &input.subdirectory {
        object.key("Subdirectory").string(var_48);
    }
    if let Some(var_49) = &input.bucket_name {
        object.key("BucketName").string(var_49);
    }
    if let Some(var_50) = &input.access_key {
        object.key("AccessKey").string(var_50);
    }
    if let Some(var_51) = &input.secret_key {
        object.key("SecretKey").string(var_51);
    }
    if let Some(var_52) = &input.agent_arns {
        let mut array_53 = object.key("AgentArns").start_array();
        for item_54 in var_52 {
            {
                array_53.value().string(item_54);
            }
        }
        array_53.finish();
    }
    if let Some(var_55) = &input.tags {
        let mut array_56 = object.key("Tags").start_array();
        for item_57 in var_55 {
            {
                let mut object_58 = array_56.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag_list_entry(
                    &mut object_58,
                    item_57,
                )?;
                object_58.finish();
            }
        }
        array_56.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_location_s3_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLocationS3Input,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_59) = &input.subdirectory {
        object.key("Subdirectory").string(var_59);
    }
    if let Some(var_60) = &input.s3_bucket_arn {
        object.key("S3BucketArn").string(var_60);
    }
    if let Some(var_61) = &input.s3_storage_class {
        object.key("S3StorageClass").string(var_61.as_str());
    }
    if let Some(var_62) = &input.s3_config {
        let mut object_63 = object.key("S3Config").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_config(&mut object_63, var_62)?;
        object_63.finish();
    }
    if let Some(var_64) = &input.agent_arns {
        let mut array_65 = object.key("AgentArns").start_array();
        for item_66 in var_64 {
            {
                array_65.value().string(item_66);
            }
        }
        array_65.finish();
    }
    if let Some(var_67) = &input.tags {
        let mut array_68 = object.key("Tags").start_array();
        for item_69 in var_67 {
            {
                let mut object_70 = array_68.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag_list_entry(
                    &mut object_70,
                    item_69,
                )?;
                object_70.finish();
            }
        }
        array_68.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_location_smb_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLocationSmbInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_71) = &input.subdirectory {
        object.key("Subdirectory").string(var_71);
    }
    if let Some(var_72) = &input.server_hostname {
        object.key("ServerHostname").string(var_72);
    }
    if let Some(var_73) = &input.user {
        object.key("User").string(var_73);
    }
    if let Some(var_74) = &input.domain {
        object.key("Domain").string(var_74);
    }
    if let Some(var_75) = &input.password {
        object.key("Password").string(var_75);
    }
    if let Some(var_76) = &input.agent_arns {
        let mut array_77 = object.key("AgentArns").start_array();
        for item_78 in var_76 {
            {
                array_77.value().string(item_78);
            }
        }
        array_77.finish();
    }
    if let Some(var_79) = &input.mount_options {
        let mut object_80 = object.key("MountOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_smb_mount_options(&mut object_80, var_79)?;
        object_80.finish();
    }
    if let Some(var_81) = &input.tags {
        let mut array_82 = object.key("Tags").start_array();
        for item_83 in var_81 {
            {
                let mut object_84 = array_82.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag_list_entry(
                    &mut object_84,
                    item_83,
                )?;
                object_84.finish();
            }
        }
        array_82.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_task_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateTaskInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_85) = &input.source_location_arn {
        object.key("SourceLocationArn").string(var_85);
    }
    if let Some(var_86) = &input.destination_location_arn {
        object.key("DestinationLocationArn").string(var_86);
    }
    if let Some(var_87) = &input.cloud_watch_log_group_arn {
        object.key("CloudWatchLogGroupArn").string(var_87);
    }
    if let Some(var_88) = &input.name {
        object.key("Name").string(var_88);
    }
    if let Some(var_89) = &input.options {
        let mut object_90 = object.key("Options").start_object();
        crate::json_ser::serialize_structure_crate_model_options(&mut object_90, var_89)?;
        object_90.finish();
    }
    if let Some(var_91) = &input.excludes {
        let mut array_92 = object.key("Excludes").start_array();
        for item_93 in var_91 {
            {
                let mut object_94 = array_92.value().start_object();
                crate::json_ser::serialize_structure_crate_model_filter_rule(
                    &mut object_94,
                    item_93,
                )?;
                object_94.finish();
            }
        }
        array_92.finish();
    }
    if let Some(var_95) = &input.schedule {
        let mut object_96 = object.key("Schedule").start_object();
        crate::json_ser::serialize_structure_crate_model_task_schedule(&mut object_96, var_95)?;
        object_96.finish();
    }
    if let Some(var_97) = &input.tags {
        let mut array_98 = object.key("Tags").start_array();
        for item_99 in var_97 {
            {
                let mut object_100 = array_98.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag_list_entry(
                    &mut object_100,
                    item_99,
                )?;
                object_100.finish();
            }
        }
        array_98.finish();
    }
    if let Some(var_101) = &input.includes {
        let mut array_102 = object.key("Includes").start_array();
        for item_103 in var_101 {
            {
                let mut object_104 = array_102.value().start_object();
                crate::json_ser::serialize_structure_crate_model_filter_rule(
                    &mut object_104,
                    item_103,
                )?;
                object_104.finish();
            }
        }
        array_102.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_agent_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteAgentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_105) = &input.agent_arn {
        object.key("AgentArn").string(var_105);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_location_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteLocationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_106) = &input.location_arn {
        object.key("LocationArn").string(var_106);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_task_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteTaskInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_107) = &input.task_arn {
        object.key("TaskArn").string(var_107);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_agent_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeAgentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_108) = &input.agent_arn {
        object.key("AgentArn").string(var_108);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_location_efs_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeLocationEfsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_109) = &input.location_arn {
        object.key("LocationArn").string(var_109);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_location_fsx_windows_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeLocationFsxWindowsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_110) = &input.location_arn {
        object.key("LocationArn").string(var_110);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_location_nfs_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeLocationNfsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_111) = &input.location_arn {
        object.key("LocationArn").string(var_111);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_location_object_storage_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeLocationObjectStorageInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_112) = &input.location_arn {
        object.key("LocationArn").string(var_112);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_location_s3_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeLocationS3Input,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_113) = &input.location_arn {
        object.key("LocationArn").string(var_113);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_location_smb_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeLocationSmbInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_114) = &input.location_arn {
        object.key("LocationArn").string(var_114);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_task_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeTaskInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_115) = &input.task_arn {
        object.key("TaskArn").string(var_115);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_task_execution_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeTaskExecutionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_116) = &input.task_execution_arn {
        object.key("TaskExecutionArn").string(var_116);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_agents_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAgentsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_117) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_117).into()),
        );
    }
    if let Some(var_118) = &input.next_token {
        object.key("NextToken").string(var_118);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_locations_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListLocationsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_119) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_119).into()),
        );
    }
    if let Some(var_120) = &input.next_token {
        object.key("NextToken").string(var_120);
    }
    if let Some(var_121) = &input.filters {
        let mut array_122 = object.key("Filters").start_array();
        for item_123 in var_121 {
            {
                let mut object_124 = array_122.value().start_object();
                crate::json_ser::serialize_structure_crate_model_location_filter(
                    &mut object_124,
                    item_123,
                )?;
                object_124.finish();
            }
        }
        array_122.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_125) = &input.resource_arn {
        object.key("ResourceArn").string(var_125);
    }
    if let Some(var_126) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_126).into()),
        );
    }
    if let Some(var_127) = &input.next_token {
        object.key("NextToken").string(var_127);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_task_executions_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTaskExecutionsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_128) = &input.task_arn {
        object.key("TaskArn").string(var_128);
    }
    if let Some(var_129) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_129).into()),
        );
    }
    if let Some(var_130) = &input.next_token {
        object.key("NextToken").string(var_130);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tasks_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTasksInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_131) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_131).into()),
        );
    }
    if let Some(var_132) = &input.next_token {
        object.key("NextToken").string(var_132);
    }
    if let Some(var_133) = &input.filters {
        let mut array_134 = object.key("Filters").start_array();
        for item_135 in var_133 {
            {
                let mut object_136 = array_134.value().start_object();
                crate::json_ser::serialize_structure_crate_model_task_filter(
                    &mut object_136,
                    item_135,
                )?;
                object_136.finish();
            }
        }
        array_134.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_task_execution_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartTaskExecutionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_137) = &input.task_arn {
        object.key("TaskArn").string(var_137);
    }
    if let Some(var_138) = &input.override_options {
        let mut object_139 = object.key("OverrideOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_options(&mut object_139, var_138)?;
        object_139.finish();
    }
    if let Some(var_140) = &input.includes {
        let mut array_141 = object.key("Includes").start_array();
        for item_142 in var_140 {
            {
                let mut object_143 = array_141.value().start_object();
                crate::json_ser::serialize_structure_crate_model_filter_rule(
                    &mut object_143,
                    item_142,
                )?;
                object_143.finish();
            }
        }
        array_141.finish();
    }
    if let Some(var_144) = &input.excludes {
        let mut array_145 = object.key("Excludes").start_array();
        for item_146 in var_144 {
            {
                let mut object_147 = array_145.value().start_object();
                crate::json_ser::serialize_structure_crate_model_filter_rule(
                    &mut object_147,
                    item_146,
                )?;
                object_147.finish();
            }
        }
        array_145.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_148) = &input.resource_arn {
        object.key("ResourceArn").string(var_148);
    }
    if let Some(var_149) = &input.tags {
        let mut array_150 = object.key("Tags").start_array();
        for item_151 in var_149 {
            {
                let mut object_152 = array_150.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag_list_entry(
                    &mut object_152,
                    item_151,
                )?;
                object_152.finish();
            }
        }
        array_150.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_153) = &input.resource_arn {
        object.key("ResourceArn").string(var_153);
    }
    if let Some(var_154) = &input.keys {
        let mut array_155 = object.key("Keys").start_array();
        for item_156 in var_154 {
            {
                array_155.value().string(item_156);
            }
        }
        array_155.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_agent_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateAgentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_157) = &input.agent_arn {
        object.key("AgentArn").string(var_157);
    }
    if let Some(var_158) = &input.name {
        object.key("Name").string(var_158);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_location_nfs_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLocationNfsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_159) = &input.location_arn {
        object.key("LocationArn").string(var_159);
    }
    if let Some(var_160) = &input.subdirectory {
        object.key("Subdirectory").string(var_160);
    }
    if let Some(var_161) = &input.on_prem_config {
        let mut object_162 = object.key("OnPremConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_on_prem_config(&mut object_162, var_161)?;
        object_162.finish();
    }
    if let Some(var_163) = &input.mount_options {
        let mut object_164 = object.key("MountOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_nfs_mount_options(
            &mut object_164,
            var_163,
        )?;
        object_164.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_location_object_storage_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLocationObjectStorageInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_165) = &input.location_arn {
        object.key("LocationArn").string(var_165);
    }
    if let Some(var_166) = &input.server_port {
        object.key("ServerPort").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_166).into()),
        );
    }
    if let Some(var_167) = &input.server_protocol {
        object.key("ServerProtocol").string(var_167.as_str());
    }
    if let Some(var_168) = &input.subdirectory {
        object.key("Subdirectory").string(var_168);
    }
    if let Some(var_169) = &input.access_key {
        object.key("AccessKey").string(var_169);
    }
    if let Some(var_170) = &input.secret_key {
        object.key("SecretKey").string(var_170);
    }
    if let Some(var_171) = &input.agent_arns {
        let mut array_172 = object.key("AgentArns").start_array();
        for item_173 in var_171 {
            {
                array_172.value().string(item_173);
            }
        }
        array_172.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_location_smb_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLocationSmbInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_174) = &input.location_arn {
        object.key("LocationArn").string(var_174);
    }
    if let Some(var_175) = &input.subdirectory {
        object.key("Subdirectory").string(var_175);
    }
    if let Some(var_176) = &input.user {
        object.key("User").string(var_176);
    }
    if let Some(var_177) = &input.domain {
        object.key("Domain").string(var_177);
    }
    if let Some(var_178) = &input.password {
        object.key("Password").string(var_178);
    }
    if let Some(var_179) = &input.agent_arns {
        let mut array_180 = object.key("AgentArns").start_array();
        for item_181 in var_179 {
            {
                array_180.value().string(item_181);
            }
        }
        array_180.finish();
    }
    if let Some(var_182) = &input.mount_options {
        let mut object_183 = object.key("MountOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_smb_mount_options(
            &mut object_183,
            var_182,
        )?;
        object_183.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_task_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateTaskInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_184) = &input.task_arn {
        object.key("TaskArn").string(var_184);
    }
    if let Some(var_185) = &input.options {
        let mut object_186 = object.key("Options").start_object();
        crate::json_ser::serialize_structure_crate_model_options(&mut object_186, var_185)?;
        object_186.finish();
    }
    if let Some(var_187) = &input.excludes {
        let mut array_188 = object.key("Excludes").start_array();
        for item_189 in var_187 {
            {
                let mut object_190 = array_188.value().start_object();
                crate::json_ser::serialize_structure_crate_model_filter_rule(
                    &mut object_190,
                    item_189,
                )?;
                object_190.finish();
            }
        }
        array_188.finish();
    }
    if let Some(var_191) = &input.schedule {
        let mut object_192 = object.key("Schedule").start_object();
        crate::json_ser::serialize_structure_crate_model_task_schedule(&mut object_192, var_191)?;
        object_192.finish();
    }
    if let Some(var_193) = &input.name {
        object.key("Name").string(var_193);
    }
    if let Some(var_194) = &input.cloud_watch_log_group_arn {
        object.key("CloudWatchLogGroupArn").string(var_194);
    }
    if let Some(var_195) = &input.includes {
        let mut array_196 = object.key("Includes").start_array();
        for item_197 in var_195 {
            {
                let mut object_198 = array_196.value().start_object();
                crate::json_ser::serialize_structure_crate_model_filter_rule(
                    &mut object_198,
                    item_197,
                )?;
                object_198.finish();
            }
        }
        array_196.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_task_execution_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateTaskExecutionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_199) = &input.task_execution_arn {
        object.key("TaskExecutionArn").string(var_199);
    }
    if let Some(var_200) = &input.options {
        let mut object_201 = object.key("Options").start_object();
        crate::json_ser::serialize_structure_crate_model_options(&mut object_201, var_200)?;
        object_201.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag_list_entry(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TagListEntry,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_202) = &input.key {
        object.key("Key").string(var_202);
    }
    if let Some(var_203) = &input.value {
        object.key("Value").string(var_203);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_ec2_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Ec2Config,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_204) = &input.subnet_arn {
        object.key("SubnetArn").string(var_204);
    }
    if let Some(var_205) = &input.security_group_arns {
        let mut array_206 = object.key("SecurityGroupArns").start_array();
        for item_207 in var_205 {
            {
                array_206.value().string(item_207);
            }
        }
        array_206.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_on_prem_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OnPremConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_208) = &input.agent_arns {
        let mut array_209 = object.key("AgentArns").start_array();
        for item_210 in var_208 {
            {
                array_209.value().string(item_210);
            }
        }
        array_209.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_nfs_mount_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NfsMountOptions,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_211) = &input.version {
        object.key("Version").string(var_211.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_s3_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3Config,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_212) = &input.bucket_access_role_arn {
        object.key("BucketAccessRoleArn").string(var_212);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_smb_mount_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SmbMountOptions,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_213) = &input.version {
        object.key("Version").string(var_213.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Options,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_214) = &input.verify_mode {
        object.key("VerifyMode").string(var_214.as_str());
    }
    if let Some(var_215) = &input.overwrite_mode {
        object.key("OverwriteMode").string(var_215.as_str());
    }
    if let Some(var_216) = &input.atime {
        object.key("Atime").string(var_216.as_str());
    }
    if let Some(var_217) = &input.mtime {
        object.key("Mtime").string(var_217.as_str());
    }
    if let Some(var_218) = &input.uid {
        object.key("Uid").string(var_218.as_str());
    }
    if let Some(var_219) = &input.gid {
        object.key("Gid").string(var_219.as_str());
    }
    if let Some(var_220) = &input.preserve_deleted_files {
        object.key("PreserveDeletedFiles").string(var_220.as_str());
    }
    if let Some(var_221) = &input.preserve_devices {
        object.key("PreserveDevices").string(var_221.as_str());
    }
    if let Some(var_222) = &input.posix_permissions {
        object.key("PosixPermissions").string(var_222.as_str());
    }
    if let Some(var_223) = &input.bytes_per_second {
        object.key("BytesPerSecond").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_223).into()),
        );
    }
    if let Some(var_224) = &input.task_queueing {
        object.key("TaskQueueing").string(var_224.as_str());
    }
    if let Some(var_225) = &input.log_level {
        object.key("LogLevel").string(var_225.as_str());
    }
    if let Some(var_226) = &input.transfer_mode {
        object.key("TransferMode").string(var_226.as_str());
    }
    if let Some(var_227) = &input.security_descriptor_copy_flags {
        object
            .key("SecurityDescriptorCopyFlags")
            .string(var_227.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_filter_rule(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FilterRule,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_228) = &input.filter_type {
        object.key("FilterType").string(var_228.as_str());
    }
    if let Some(var_229) = &input.value {
        object.key("Value").string(var_229);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_task_schedule(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TaskSchedule,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_230) = &input.schedule_expression {
        object.key("ScheduleExpression").string(var_230);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_location_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LocationFilter,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_231) = &input.name {
        object.key("Name").string(var_231.as_str());
    }
    if let Some(var_232) = &input.values {
        let mut array_233 = object.key("Values").start_array();
        for item_234 in var_232 {
            {
                array_233.value().string(item_234);
            }
        }
        array_233.finish();
    }
    if let Some(var_235) = &input.operator {
        object.key("Operator").string(var_235.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_task_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TaskFilter,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_236) = &input.name {
        object.key("Name").string(var_236.as_str());
    }
    if let Some(var_237) = &input.values {
        let mut array_238 = object.key("Values").start_array();
        for item_239 in var_237 {
            {
                array_238.value().string(item_239);
            }
        }
        array_238.finish();
    }
    if let Some(var_240) = &input.operator {
        object.key("Operator").string(var_240.as_str());
    }
    Ok(())
}