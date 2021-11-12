// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_data_set_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDataSetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.asset_type {
        object.key("AssetType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("Description").string(var_2);
    }
    if let Some(var_3) = &input.name {
        object.key("Name").string(var_3);
    }
    if let Some(var_4) = &input.tags {
        let mut object_5 = object.key("Tags").start_object();
        for (key_6, value_7) in var_4 {
            {
                object_5.key(key_6).string(value_7);
            }
        }
        object_5.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_event_action_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateEventActionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_8) = &input.action {
        let mut object_9 = object.key("Action").start_object();
        crate::json_ser::serialize_structure_crate_model_action(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.event {
        let mut object_11 = object.key("Event").start_object();
        crate::json_ser::serialize_structure_crate_model_event(&mut object_11, var_10)?;
        object_11.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_job_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateJobInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_12) = &input.details {
        let mut object_13 = object.key("Details").start_object();
        crate::json_ser::serialize_structure_crate_model_request_details(&mut object_13, var_12)?;
        object_13.finish();
    }
    if let Some(var_14) = &input.r#type {
        object.key("Type").string(var_14.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_revision_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateRevisionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_15) = &input.comment {
        object.key("Comment").string(var_15);
    }
    if let Some(var_16) = &input.tags {
        let mut object_17 = object.key("Tags").start_object();
        for (key_18, value_19) in var_16 {
            {
                object_17.key(key_18).string(value_19);
            }
        }
        object_17.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_20) = &input.tags {
        let mut object_21 = object.key("tags").start_object();
        for (key_22, value_23) in var_20 {
            {
                object_21.key(key_22).string(value_23);
            }
        }
        object_21.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_asset_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateAssetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_24) = &input.name {
        object.key("Name").string(var_24);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_data_set_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDataSetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_25) = &input.description {
        object.key("Description").string(var_25);
    }
    if let Some(var_26) = &input.name {
        object.key("Name").string(var_26);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_event_action_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateEventActionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_27) = &input.action {
        let mut object_28 = object.key("Action").start_object();
        crate::json_ser::serialize_structure_crate_model_action(&mut object_28, var_27)?;
        object_28.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_revision_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateRevisionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_29) = &input.comment {
        object.key("Comment").string(var_29);
    }
    if input.finalized {
        object.key("Finalized").boolean(input.finalized);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_action(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Action,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_30) = &input.export_revision_to_s3 {
        let mut object_31 = object.key("ExportRevisionToS3").start_object();
        crate::json_ser::serialize_structure_crate_model_auto_export_revision_to_s3_request_details(&mut object_31, var_30)?;
        object_31.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_event(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Event,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_32) = &input.revision_published {
        let mut object_33 = object.key("RevisionPublished").start_object();
        crate::json_ser::serialize_structure_crate_model_revision_published(
            &mut object_33,
            var_32,
        )?;
        object_33.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_request_details(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RequestDetails,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_34) = &input.export_asset_to_signed_url {
        let mut object_35 = object.key("ExportAssetToSignedUrl").start_object();
        crate::json_ser::serialize_structure_crate_model_export_asset_to_signed_url_request_details(&mut object_35, var_34)?;
        object_35.finish();
    }
    if let Some(var_36) = &input.export_assets_to_s3 {
        let mut object_37 = object.key("ExportAssetsToS3").start_object();
        crate::json_ser::serialize_structure_crate_model_export_assets_to_s3_request_details(
            &mut object_37,
            var_36,
        )?;
        object_37.finish();
    }
    if let Some(var_38) = &input.export_revisions_to_s3 {
        let mut object_39 = object.key("ExportRevisionsToS3").start_object();
        crate::json_ser::serialize_structure_crate_model_export_revisions_to_s3_request_details(
            &mut object_39,
            var_38,
        )?;
        object_39.finish();
    }
    if let Some(var_40) = &input.import_asset_from_signed_url {
        let mut object_41 = object.key("ImportAssetFromSignedUrl").start_object();
        crate::json_ser::serialize_structure_crate_model_import_asset_from_signed_url_request_details(&mut object_41, var_40)?;
        object_41.finish();
    }
    if let Some(var_42) = &input.import_assets_from_s3 {
        let mut object_43 = object.key("ImportAssetsFromS3").start_object();
        crate::json_ser::serialize_structure_crate_model_import_assets_from_s3_request_details(
            &mut object_43,
            var_42,
        )?;
        object_43.finish();
    }
    if let Some(var_44) = &input.import_assets_from_redshift_data_shares {
        let mut object_45 = object
            .key("ImportAssetsFromRedshiftDataShares")
            .start_object();
        crate::json_ser::serialize_structure_crate_model_import_assets_from_redshift_data_shares_request_details(&mut object_45, var_44)?;
        object_45.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_auto_export_revision_to_s3_request_details(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AutoExportRevisionToS3RequestDetails,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_46) = &input.encryption {
        let mut object_47 = object.key("Encryption").start_object();
        crate::json_ser::serialize_structure_crate_model_export_server_side_encryption(
            &mut object_47,
            var_46,
        )?;
        object_47.finish();
    }
    if let Some(var_48) = &input.revision_destination {
        let mut object_49 = object.key("RevisionDestination").start_object();
        crate::json_ser::serialize_structure_crate_model_auto_export_revision_destination_entry(
            &mut object_49,
            var_48,
        )?;
        object_49.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_revision_published(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RevisionPublished,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_50) = &input.data_set_id {
        object.key("DataSetId").string(var_50);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_export_asset_to_signed_url_request_details(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ExportAssetToSignedUrlRequestDetails,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_51) = &input.asset_id {
        object.key("AssetId").string(var_51);
    }
    if let Some(var_52) = &input.data_set_id {
        object.key("DataSetId").string(var_52);
    }
    if let Some(var_53) = &input.revision_id {
        object.key("RevisionId").string(var_53);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_export_assets_to_s3_request_details(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ExportAssetsToS3RequestDetails,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_54) = &input.asset_destinations {
        let mut array_55 = object.key("AssetDestinations").start_array();
        for item_56 in var_54 {
            {
                let mut object_57 = array_55.value().start_object();
                crate::json_ser::serialize_structure_crate_model_asset_destination_entry(
                    &mut object_57,
                    item_56,
                )?;
                object_57.finish();
            }
        }
        array_55.finish();
    }
    if let Some(var_58) = &input.data_set_id {
        object.key("DataSetId").string(var_58);
    }
    if let Some(var_59) = &input.encryption {
        let mut object_60 = object.key("Encryption").start_object();
        crate::json_ser::serialize_structure_crate_model_export_server_side_encryption(
            &mut object_60,
            var_59,
        )?;
        object_60.finish();
    }
    if let Some(var_61) = &input.revision_id {
        object.key("RevisionId").string(var_61);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_export_revisions_to_s3_request_details(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ExportRevisionsToS3RequestDetails,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_62) = &input.data_set_id {
        object.key("DataSetId").string(var_62);
    }
    if let Some(var_63) = &input.encryption {
        let mut object_64 = object.key("Encryption").start_object();
        crate::json_ser::serialize_structure_crate_model_export_server_side_encryption(
            &mut object_64,
            var_63,
        )?;
        object_64.finish();
    }
    if let Some(var_65) = &input.revision_destinations {
        let mut array_66 = object.key("RevisionDestinations").start_array();
        for item_67 in var_65 {
            {
                let mut object_68 = array_66.value().start_object();
                crate::json_ser::serialize_structure_crate_model_revision_destination_entry(
                    &mut object_68,
                    item_67,
                )?;
                object_68.finish();
            }
        }
        array_66.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_import_asset_from_signed_url_request_details(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ImportAssetFromSignedUrlRequestDetails,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_69) = &input.asset_name {
        object.key("AssetName").string(var_69);
    }
    if let Some(var_70) = &input.data_set_id {
        object.key("DataSetId").string(var_70);
    }
    if let Some(var_71) = &input.md5_hash {
        object.key("Md5Hash").string(var_71);
    }
    if let Some(var_72) = &input.revision_id {
        object.key("RevisionId").string(var_72);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_import_assets_from_s3_request_details(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ImportAssetsFromS3RequestDetails,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_73) = &input.asset_sources {
        let mut array_74 = object.key("AssetSources").start_array();
        for item_75 in var_73 {
            {
                let mut object_76 = array_74.value().start_object();
                crate::json_ser::serialize_structure_crate_model_asset_source_entry(
                    &mut object_76,
                    item_75,
                )?;
                object_76.finish();
            }
        }
        array_74.finish();
    }
    if let Some(var_77) = &input.data_set_id {
        object.key("DataSetId").string(var_77);
    }
    if let Some(var_78) = &input.revision_id {
        object.key("RevisionId").string(var_78);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_import_assets_from_redshift_data_shares_request_details(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ImportAssetsFromRedshiftDataSharesRequestDetails,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_79) = &input.asset_sources {
        let mut array_80 = object.key("AssetSources").start_array();
        for item_81 in var_79 {
            {
                let mut object_82 = array_80.value().start_object();
                crate::json_ser::serialize_structure_crate_model_redshift_data_share_asset_source_entry(&mut object_82, item_81)?;
                object_82.finish();
            }
        }
        array_80.finish();
    }
    if let Some(var_83) = &input.data_set_id {
        object.key("DataSetId").string(var_83);
    }
    if let Some(var_84) = &input.revision_id {
        object.key("RevisionId").string(var_84);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_export_server_side_encryption(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ExportServerSideEncryption,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_85) = &input.kms_key_arn {
        object.key("KmsKeyArn").string(var_85);
    }
    if let Some(var_86) = &input.r#type {
        object.key("Type").string(var_86.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_auto_export_revision_destination_entry(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AutoExportRevisionDestinationEntry,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_87) = &input.bucket {
        object.key("Bucket").string(var_87);
    }
    if let Some(var_88) = &input.key_pattern {
        object.key("KeyPattern").string(var_88);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_asset_destination_entry(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AssetDestinationEntry,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_89) = &input.asset_id {
        object.key("AssetId").string(var_89);
    }
    if let Some(var_90) = &input.bucket {
        object.key("Bucket").string(var_90);
    }
    if let Some(var_91) = &input.key {
        object.key("Key").string(var_91);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_revision_destination_entry(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RevisionDestinationEntry,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_92) = &input.bucket {
        object.key("Bucket").string(var_92);
    }
    if let Some(var_93) = &input.key_pattern {
        object.key("KeyPattern").string(var_93);
    }
    if let Some(var_94) = &input.revision_id {
        object.key("RevisionId").string(var_94);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_asset_source_entry(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AssetSourceEntry,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_95) = &input.bucket {
        object.key("Bucket").string(var_95);
    }
    if let Some(var_96) = &input.key {
        object.key("Key").string(var_96);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_redshift_data_share_asset_source_entry(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RedshiftDataShareAssetSourceEntry,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_97) = &input.data_share_arn {
        object.key("DataShareArn").string(var_97);
    }
    Ok(())
}