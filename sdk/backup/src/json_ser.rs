// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_backup_plan_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateBackupPlanInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.backup_plan {
        let mut object_2 = object.key("BackupPlan").start_object();
        crate::json_ser::serialize_structure_crate_model_backup_plan_input(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.backup_plan_tags {
        let mut object_4 = object.key("BackupPlanTags").start_object();
        for (key_5, value_6) in var_3 {
            {
                object_4.key(key_5).string(value_6);
            }
        }
        object_4.finish();
    }
    if let Some(var_7) = &input.creator_request_id {
        object.key("CreatorRequestId").string(var_7);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_backup_selection_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateBackupSelectionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_8) = &input.backup_selection {
        let mut object_9 = object.key("BackupSelection").start_object();
        crate::json_ser::serialize_structure_crate_model_backup_selection(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.creator_request_id {
        object.key("CreatorRequestId").string(var_10);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_backup_vault_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateBackupVaultInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_11) = &input.backup_vault_tags {
        let mut object_12 = object.key("BackupVaultTags").start_object();
        for (key_13, value_14) in var_11 {
            {
                object_12.key(key_13).string(value_14);
            }
        }
        object_12.finish();
    }
    if let Some(var_15) = &input.creator_request_id {
        object.key("CreatorRequestId").string(var_15);
    }
    if let Some(var_16) = &input.encryption_key_arn {
        object.key("EncryptionKeyArn").string(var_16);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_framework_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateFrameworkInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_17) = &input.framework_controls {
        let mut array_18 = object.key("FrameworkControls").start_array();
        for item_19 in var_17 {
            {
                let mut object_20 = array_18.value().start_object();
                crate::json_ser::serialize_structure_crate_model_framework_control(
                    &mut object_20,
                    item_19,
                )?;
                object_20.finish();
            }
        }
        array_18.finish();
    }
    if let Some(var_21) = &input.framework_description {
        object.key("FrameworkDescription").string(var_21);
    }
    if let Some(var_22) = &input.framework_name {
        object.key("FrameworkName").string(var_22);
    }
    if let Some(var_23) = &input.framework_tags {
        let mut object_24 = object.key("FrameworkTags").start_object();
        for (key_25, value_26) in var_23 {
            {
                object_24.key(key_25).string(value_26);
            }
        }
        object_24.finish();
    }
    if let Some(var_27) = &input.idempotency_token {
        object.key("IdempotencyToken").string(var_27);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_report_plan_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateReportPlanInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_28) = &input.idempotency_token {
        object.key("IdempotencyToken").string(var_28);
    }
    if let Some(var_29) = &input.report_delivery_channel {
        let mut object_30 = object.key("ReportDeliveryChannel").start_object();
        crate::json_ser::serialize_structure_crate_model_report_delivery_channel(
            &mut object_30,
            var_29,
        )?;
        object_30.finish();
    }
    if let Some(var_31) = &input.report_plan_description {
        object.key("ReportPlanDescription").string(var_31);
    }
    if let Some(var_32) = &input.report_plan_name {
        object.key("ReportPlanName").string(var_32);
    }
    if let Some(var_33) = &input.report_plan_tags {
        let mut object_34 = object.key("ReportPlanTags").start_object();
        for (key_35, value_36) in var_33 {
            {
                object_34.key(key_35).string(value_36);
            }
        }
        object_34.finish();
    }
    if let Some(var_37) = &input.report_setting {
        let mut object_38 = object.key("ReportSetting").start_object();
        crate::json_ser::serialize_structure_crate_model_report_setting(&mut object_38, var_37)?;
        object_38.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_backup_plan_from_json_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetBackupPlanFromJsonInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_39) = &input.backup_plan_template_json {
        object.key("BackupPlanTemplateJson").string(var_39);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_backup_vault_access_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutBackupVaultAccessPolicyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_40) = &input.policy {
        object.key("Policy").string(var_40);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_backup_vault_lock_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutBackupVaultLockConfigurationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_41) = &input.changeable_for_days {
        object.key("ChangeableForDays").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_41).into()),
        );
    }
    if let Some(var_42) = &input.max_retention_days {
        object.key("MaxRetentionDays").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_42).into()),
        );
    }
    if let Some(var_43) = &input.min_retention_days {
        object.key("MinRetentionDays").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_43).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_backup_vault_notifications_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutBackupVaultNotificationsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_44) = &input.backup_vault_events {
        let mut array_45 = object.key("BackupVaultEvents").start_array();
        for item_46 in var_44 {
            {
                array_45.value().string(item_46.as_str());
            }
        }
        array_45.finish();
    }
    if let Some(var_47) = &input.sns_topic_arn {
        object.key("SNSTopicArn").string(var_47);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_backup_job_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartBackupJobInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_48) = &input.backup_options {
        let mut object_49 = object.key("BackupOptions").start_object();
        for (key_50, value_51) in var_48 {
            {
                object_49.key(key_50).string(value_51);
            }
        }
        object_49.finish();
    }
    if let Some(var_52) = &input.backup_vault_name {
        object.key("BackupVaultName").string(var_52);
    }
    if let Some(var_53) = &input.complete_window_minutes {
        object.key("CompleteWindowMinutes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_53).into()),
        );
    }
    if let Some(var_54) = &input.iam_role_arn {
        object.key("IamRoleArn").string(var_54);
    }
    if let Some(var_55) = &input.idempotency_token {
        object.key("IdempotencyToken").string(var_55);
    }
    if let Some(var_56) = &input.lifecycle {
        let mut object_57 = object.key("Lifecycle").start_object();
        crate::json_ser::serialize_structure_crate_model_lifecycle(&mut object_57, var_56)?;
        object_57.finish();
    }
    if let Some(var_58) = &input.recovery_point_tags {
        let mut object_59 = object.key("RecoveryPointTags").start_object();
        for (key_60, value_61) in var_58 {
            {
                object_59.key(key_60).string(value_61);
            }
        }
        object_59.finish();
    }
    if let Some(var_62) = &input.resource_arn {
        object.key("ResourceArn").string(var_62);
    }
    if let Some(var_63) = &input.start_window_minutes {
        object.key("StartWindowMinutes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_63).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_copy_job_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartCopyJobInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_64) = &input.destination_backup_vault_arn {
        object.key("DestinationBackupVaultArn").string(var_64);
    }
    if let Some(var_65) = &input.iam_role_arn {
        object.key("IamRoleArn").string(var_65);
    }
    if let Some(var_66) = &input.idempotency_token {
        object.key("IdempotencyToken").string(var_66);
    }
    if let Some(var_67) = &input.lifecycle {
        let mut object_68 = object.key("Lifecycle").start_object();
        crate::json_ser::serialize_structure_crate_model_lifecycle(&mut object_68, var_67)?;
        object_68.finish();
    }
    if let Some(var_69) = &input.recovery_point_arn {
        object.key("RecoveryPointArn").string(var_69);
    }
    if let Some(var_70) = &input.source_backup_vault_name {
        object.key("SourceBackupVaultName").string(var_70);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_report_job_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartReportJobInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_71) = &input.idempotency_token {
        object.key("IdempotencyToken").string(var_71);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_restore_job_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartRestoreJobInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_72) = &input.iam_role_arn {
        object.key("IamRoleArn").string(var_72);
    }
    if let Some(var_73) = &input.idempotency_token {
        object.key("IdempotencyToken").string(var_73);
    }
    if let Some(var_74) = &input.metadata {
        let mut object_75 = object.key("Metadata").start_object();
        for (key_76, value_77) in var_74 {
            {
                object_75.key(key_76).string(value_77);
            }
        }
        object_75.finish();
    }
    if let Some(var_78) = &input.recovery_point_arn {
        object.key("RecoveryPointArn").string(var_78);
    }
    if let Some(var_79) = &input.resource_type {
        object.key("ResourceType").string(var_79);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_80) = &input.tags {
        let mut object_81 = object.key("Tags").start_object();
        for (key_82, value_83) in var_80 {
            {
                object_81.key(key_82).string(value_83);
            }
        }
        object_81.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_84) = &input.tag_key_list {
        let mut array_85 = object.key("TagKeyList").start_array();
        for item_86 in var_84 {
            {
                array_85.value().string(item_86);
            }
        }
        array_85.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_backup_plan_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateBackupPlanInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_87) = &input.backup_plan {
        let mut object_88 = object.key("BackupPlan").start_object();
        crate::json_ser::serialize_structure_crate_model_backup_plan_input(&mut object_88, var_87)?;
        object_88.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_framework_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateFrameworkInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_89) = &input.framework_controls {
        let mut array_90 = object.key("FrameworkControls").start_array();
        for item_91 in var_89 {
            {
                let mut object_92 = array_90.value().start_object();
                crate::json_ser::serialize_structure_crate_model_framework_control(
                    &mut object_92,
                    item_91,
                )?;
                object_92.finish();
            }
        }
        array_90.finish();
    }
    if let Some(var_93) = &input.framework_description {
        object.key("FrameworkDescription").string(var_93);
    }
    if let Some(var_94) = &input.idempotency_token {
        object.key("IdempotencyToken").string(var_94);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_global_settings_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateGlobalSettingsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_95) = &input.global_settings {
        let mut object_96 = object.key("GlobalSettings").start_object();
        for (key_97, value_98) in var_95 {
            {
                object_96.key(key_97).string(value_98);
            }
        }
        object_96.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_recovery_point_lifecycle_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateRecoveryPointLifecycleInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_99) = &input.lifecycle {
        let mut object_100 = object.key("Lifecycle").start_object();
        crate::json_ser::serialize_structure_crate_model_lifecycle(&mut object_100, var_99)?;
        object_100.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_region_settings_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateRegionSettingsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_101) = &input.resource_type_opt_in_preference {
        let mut object_102 = object.key("ResourceTypeOptInPreference").start_object();
        for (key_103, value_104) in var_101 {
            {
                object_102.key(key_103).boolean(*value_104);
            }
        }
        object_102.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_report_plan_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateReportPlanInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_105) = &input.idempotency_token {
        object.key("IdempotencyToken").string(var_105);
    }
    if let Some(var_106) = &input.report_delivery_channel {
        let mut object_107 = object.key("ReportDeliveryChannel").start_object();
        crate::json_ser::serialize_structure_crate_model_report_delivery_channel(
            &mut object_107,
            var_106,
        )?;
        object_107.finish();
    }
    if let Some(var_108) = &input.report_plan_description {
        object.key("ReportPlanDescription").string(var_108);
    }
    if let Some(var_109) = &input.report_setting {
        let mut object_110 = object.key("ReportSetting").start_object();
        crate::json_ser::serialize_structure_crate_model_report_setting(&mut object_110, var_109)?;
        object_110.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_backup_plan_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BackupPlanInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_111) = &input.backup_plan_name {
        object.key("BackupPlanName").string(var_111);
    }
    if let Some(var_112) = &input.rules {
        let mut array_113 = object.key("Rules").start_array();
        for item_114 in var_112 {
            {
                let mut object_115 = array_113.value().start_object();
                crate::json_ser::serialize_structure_crate_model_backup_rule_input(
                    &mut object_115,
                    item_114,
                )?;
                object_115.finish();
            }
        }
        array_113.finish();
    }
    if let Some(var_116) = &input.advanced_backup_settings {
        let mut array_117 = object.key("AdvancedBackupSettings").start_array();
        for item_118 in var_116 {
            {
                let mut object_119 = array_117.value().start_object();
                crate::json_ser::serialize_structure_crate_model_advanced_backup_setting(
                    &mut object_119,
                    item_118,
                )?;
                object_119.finish();
            }
        }
        array_117.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_backup_selection(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BackupSelection,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_120) = &input.selection_name {
        object.key("SelectionName").string(var_120);
    }
    if let Some(var_121) = &input.iam_role_arn {
        object.key("IamRoleArn").string(var_121);
    }
    if let Some(var_122) = &input.resources {
        let mut array_123 = object.key("Resources").start_array();
        for item_124 in var_122 {
            {
                array_123.value().string(item_124);
            }
        }
        array_123.finish();
    }
    if let Some(var_125) = &input.list_of_tags {
        let mut array_126 = object.key("ListOfTags").start_array();
        for item_127 in var_125 {
            {
                let mut object_128 = array_126.value().start_object();
                crate::json_ser::serialize_structure_crate_model_condition(
                    &mut object_128,
                    item_127,
                )?;
                object_128.finish();
            }
        }
        array_126.finish();
    }
    if let Some(var_129) = &input.not_resources {
        let mut array_130 = object.key("NotResources").start_array();
        for item_131 in var_129 {
            {
                array_130.value().string(item_131);
            }
        }
        array_130.finish();
    }
    if let Some(var_132) = &input.conditions {
        let mut object_133 = object.key("Conditions").start_object();
        crate::json_ser::serialize_structure_crate_model_conditions(&mut object_133, var_132)?;
        object_133.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_framework_control(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FrameworkControl,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_134) = &input.control_name {
        object.key("ControlName").string(var_134);
    }
    if let Some(var_135) = &input.control_input_parameters {
        let mut array_136 = object.key("ControlInputParameters").start_array();
        for item_137 in var_135 {
            {
                let mut object_138 = array_136.value().start_object();
                crate::json_ser::serialize_structure_crate_model_control_input_parameter(
                    &mut object_138,
                    item_137,
                )?;
                object_138.finish();
            }
        }
        array_136.finish();
    }
    if let Some(var_139) = &input.control_scope {
        let mut object_140 = object.key("ControlScope").start_object();
        crate::json_ser::serialize_structure_crate_model_control_scope(&mut object_140, var_139)?;
        object_140.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_report_delivery_channel(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ReportDeliveryChannel,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_141) = &input.s3_bucket_name {
        object.key("S3BucketName").string(var_141);
    }
    if let Some(var_142) = &input.s3_key_prefix {
        object.key("S3KeyPrefix").string(var_142);
    }
    if let Some(var_143) = &input.formats {
        let mut array_144 = object.key("Formats").start_array();
        for item_145 in var_143 {
            {
                array_144.value().string(item_145);
            }
        }
        array_144.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_report_setting(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ReportSetting,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_146) = &input.report_template {
        object.key("ReportTemplate").string(var_146);
    }
    if let Some(var_147) = &input.framework_arns {
        let mut array_148 = object.key("FrameworkArns").start_array();
        for item_149 in var_147 {
            {
                array_148.value().string(item_149);
            }
        }
        array_148.finish();
    }
    if input.number_of_frameworks != 0 {
        object.key("NumberOfFrameworks").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.number_of_frameworks).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_lifecycle(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Lifecycle,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_150) = &input.move_to_cold_storage_after_days {
        object.key("MoveToColdStorageAfterDays").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_150).into()),
        );
    }
    if let Some(var_151) = &input.delete_after_days {
        object.key("DeleteAfterDays").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_151).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_backup_rule_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BackupRuleInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_152) = &input.rule_name {
        object.key("RuleName").string(var_152);
    }
    if let Some(var_153) = &input.target_backup_vault_name {
        object.key("TargetBackupVaultName").string(var_153);
    }
    if let Some(var_154) = &input.schedule_expression {
        object.key("ScheduleExpression").string(var_154);
    }
    if let Some(var_155) = &input.start_window_minutes {
        object.key("StartWindowMinutes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_155).into()),
        );
    }
    if let Some(var_156) = &input.completion_window_minutes {
        object.key("CompletionWindowMinutes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_156).into()),
        );
    }
    if let Some(var_157) = &input.lifecycle {
        let mut object_158 = object.key("Lifecycle").start_object();
        crate::json_ser::serialize_structure_crate_model_lifecycle(&mut object_158, var_157)?;
        object_158.finish();
    }
    if let Some(var_159) = &input.recovery_point_tags {
        let mut object_160 = object.key("RecoveryPointTags").start_object();
        for (key_161, value_162) in var_159 {
            {
                object_160.key(key_161).string(value_162);
            }
        }
        object_160.finish();
    }
    if let Some(var_163) = &input.copy_actions {
        let mut array_164 = object.key("CopyActions").start_array();
        for item_165 in var_163 {
            {
                let mut object_166 = array_164.value().start_object();
                crate::json_ser::serialize_structure_crate_model_copy_action(
                    &mut object_166,
                    item_165,
                )?;
                object_166.finish();
            }
        }
        array_164.finish();
    }
    if let Some(var_167) = &input.enable_continuous_backup {
        object.key("EnableContinuousBackup").boolean(*var_167);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_advanced_backup_setting(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AdvancedBackupSetting,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_168) = &input.resource_type {
        object.key("ResourceType").string(var_168);
    }
    if let Some(var_169) = &input.backup_options {
        let mut object_170 = object.key("BackupOptions").start_object();
        for (key_171, value_172) in var_169 {
            {
                object_170.key(key_171).string(value_172);
            }
        }
        object_170.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_condition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Condition,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_173) = &input.condition_type {
        object.key("ConditionType").string(var_173.as_str());
    }
    if let Some(var_174) = &input.condition_key {
        object.key("ConditionKey").string(var_174);
    }
    if let Some(var_175) = &input.condition_value {
        object.key("ConditionValue").string(var_175);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_conditions(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Conditions,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_176) = &input.string_equals {
        let mut array_177 = object.key("StringEquals").start_array();
        for item_178 in var_176 {
            {
                let mut object_179 = array_177.value().start_object();
                crate::json_ser::serialize_structure_crate_model_condition_parameter(
                    &mut object_179,
                    item_178,
                )?;
                object_179.finish();
            }
        }
        array_177.finish();
    }
    if let Some(var_180) = &input.string_not_equals {
        let mut array_181 = object.key("StringNotEquals").start_array();
        for item_182 in var_180 {
            {
                let mut object_183 = array_181.value().start_object();
                crate::json_ser::serialize_structure_crate_model_condition_parameter(
                    &mut object_183,
                    item_182,
                )?;
                object_183.finish();
            }
        }
        array_181.finish();
    }
    if let Some(var_184) = &input.string_like {
        let mut array_185 = object.key("StringLike").start_array();
        for item_186 in var_184 {
            {
                let mut object_187 = array_185.value().start_object();
                crate::json_ser::serialize_structure_crate_model_condition_parameter(
                    &mut object_187,
                    item_186,
                )?;
                object_187.finish();
            }
        }
        array_185.finish();
    }
    if let Some(var_188) = &input.string_not_like {
        let mut array_189 = object.key("StringNotLike").start_array();
        for item_190 in var_188 {
            {
                let mut object_191 = array_189.value().start_object();
                crate::json_ser::serialize_structure_crate_model_condition_parameter(
                    &mut object_191,
                    item_190,
                )?;
                object_191.finish();
            }
        }
        array_189.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_control_input_parameter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ControlInputParameter,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_192) = &input.parameter_name {
        object.key("ParameterName").string(var_192);
    }
    if let Some(var_193) = &input.parameter_value {
        object.key("ParameterValue").string(var_193);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_control_scope(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ControlScope,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_194) = &input.compliance_resource_ids {
        let mut array_195 = object.key("ComplianceResourceIds").start_array();
        for item_196 in var_194 {
            {
                array_195.value().string(item_196);
            }
        }
        array_195.finish();
    }
    if let Some(var_197) = &input.compliance_resource_types {
        let mut array_198 = object.key("ComplianceResourceTypes").start_array();
        for item_199 in var_197 {
            {
                array_198.value().string(item_199);
            }
        }
        array_198.finish();
    }
    if let Some(var_200) = &input.tags {
        let mut object_201 = object.key("Tags").start_object();
        for (key_202, value_203) in var_200 {
            {
                object_201.key(key_202).string(value_203);
            }
        }
        object_201.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_copy_action(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CopyAction,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_204) = &input.lifecycle {
        let mut object_205 = object.key("Lifecycle").start_object();
        crate::json_ser::serialize_structure_crate_model_lifecycle(&mut object_205, var_204)?;
        object_205.finish();
    }
    if let Some(var_206) = &input.destination_backup_vault_arn {
        object.key("DestinationBackupVaultArn").string(var_206);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_condition_parameter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ConditionParameter,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_207) = &input.condition_key {
        object.key("ConditionKey").string(var_207);
    }
    if let Some(var_208) = &input.condition_value {
        object.key("ConditionValue").string(var_208);
    }
    Ok(())
}
