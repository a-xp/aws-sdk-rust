// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_get_clip_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetClipInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.clip_fragment_selector {
        let mut object_2 = object.key("ClipFragmentSelector").start_object();
        crate::json_ser::serialize_structure_crate_model_clip_fragment_selector(
            &mut object_2,
            var_1,
        )?;
        object_2.finish();
    }
    if let Some(var_3) = &input.stream_arn {
        object.key("StreamARN").string(var_3);
    }
    if let Some(var_4) = &input.stream_name {
        object.key("StreamName").string(var_4);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_dash_streaming_session_url_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetDashStreamingSessionUrlInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_5) = &input.dash_fragment_selector {
        let mut object_6 = object.key("DASHFragmentSelector").start_object();
        crate::json_ser::serialize_structure_crate_model_dash_fragment_selector(
            &mut object_6,
            var_5,
        )?;
        object_6.finish();
    }
    if let Some(var_7) = &input.display_fragment_number {
        object.key("DisplayFragmentNumber").string(var_7.as_str());
    }
    if let Some(var_8) = &input.display_fragment_timestamp {
        object
            .key("DisplayFragmentTimestamp")
            .string(var_8.as_str());
    }
    if let Some(var_9) = &input.expires {
        object.key("Expires").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_9).into()),
        );
    }
    if let Some(var_10) = &input.max_manifest_fragment_results {
        object.key("MaxManifestFragmentResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_10).into()),
        );
    }
    if let Some(var_11) = &input.playback_mode {
        object.key("PlaybackMode").string(var_11.as_str());
    }
    if let Some(var_12) = &input.stream_arn {
        object.key("StreamARN").string(var_12);
    }
    if let Some(var_13) = &input.stream_name {
        object.key("StreamName").string(var_13);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_hls_streaming_session_url_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetHlsStreamingSessionUrlInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_14) = &input.container_format {
        object.key("ContainerFormat").string(var_14.as_str());
    }
    if let Some(var_15) = &input.discontinuity_mode {
        object.key("DiscontinuityMode").string(var_15.as_str());
    }
    if let Some(var_16) = &input.display_fragment_timestamp {
        object
            .key("DisplayFragmentTimestamp")
            .string(var_16.as_str());
    }
    if let Some(var_17) = &input.expires {
        object.key("Expires").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_17).into()),
        );
    }
    if let Some(var_18) = &input.hls_fragment_selector {
        let mut object_19 = object.key("HLSFragmentSelector").start_object();
        crate::json_ser::serialize_structure_crate_model_hls_fragment_selector(
            &mut object_19,
            var_18,
        )?;
        object_19.finish();
    }
    if let Some(var_20) = &input.max_media_playlist_fragment_results {
        object.key("MaxMediaPlaylistFragmentResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_20).into()),
        );
    }
    if let Some(var_21) = &input.playback_mode {
        object.key("PlaybackMode").string(var_21.as_str());
    }
    if let Some(var_22) = &input.stream_arn {
        object.key("StreamARN").string(var_22);
    }
    if let Some(var_23) = &input.stream_name {
        object.key("StreamName").string(var_23);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_media_for_fragment_list_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetMediaForFragmentListInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_24) = &input.fragments {
        let mut array_25 = object.key("Fragments").start_array();
        for item_26 in var_24 {
            {
                array_25.value().string(item_26);
            }
        }
        array_25.finish();
    }
    if let Some(var_27) = &input.stream_arn {
        object.key("StreamARN").string(var_27);
    }
    if let Some(var_28) = &input.stream_name {
        object.key("StreamName").string(var_28);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_fragments_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListFragmentsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_29) = &input.fragment_selector {
        let mut object_30 = object.key("FragmentSelector").start_object();
        crate::json_ser::serialize_structure_crate_model_fragment_selector(&mut object_30, var_29)?;
        object_30.finish();
    }
    if let Some(var_31) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_31).into()),
        );
    }
    if let Some(var_32) = &input.next_token {
        object.key("NextToken").string(var_32);
    }
    if let Some(var_33) = &input.stream_arn {
        object.key("StreamARN").string(var_33);
    }
    if let Some(var_34) = &input.stream_name {
        object.key("StreamName").string(var_34);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_clip_fragment_selector(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ClipFragmentSelector,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_35) = &input.fragment_selector_type {
        object.key("FragmentSelectorType").string(var_35.as_str());
    }
    if let Some(var_36) = &input.timestamp_range {
        let mut object_37 = object.key("TimestampRange").start_object();
        crate::json_ser::serialize_structure_crate_model_clip_timestamp_range(
            &mut object_37,
            var_36,
        )?;
        object_37.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dash_fragment_selector(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DashFragmentSelector,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_38) = &input.fragment_selector_type {
        object.key("FragmentSelectorType").string(var_38.as_str());
    }
    if let Some(var_39) = &input.timestamp_range {
        let mut object_40 = object.key("TimestampRange").start_object();
        crate::json_ser::serialize_structure_crate_model_dash_timestamp_range(
            &mut object_40,
            var_39,
        )?;
        object_40.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_hls_fragment_selector(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HlsFragmentSelector,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_41) = &input.fragment_selector_type {
        object.key("FragmentSelectorType").string(var_41.as_str());
    }
    if let Some(var_42) = &input.timestamp_range {
        let mut object_43 = object.key("TimestampRange").start_object();
        crate::json_ser::serialize_structure_crate_model_hls_timestamp_range(
            &mut object_43,
            var_42,
        )?;
        object_43.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_fragment_selector(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FragmentSelector,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_44) = &input.fragment_selector_type {
        object.key("FragmentSelectorType").string(var_44.as_str());
    }
    if let Some(var_45) = &input.timestamp_range {
        let mut object_46 = object.key("TimestampRange").start_object();
        crate::json_ser::serialize_structure_crate_model_timestamp_range(&mut object_46, var_45)?;
        object_46.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_clip_timestamp_range(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ClipTimestampRange,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_47) = &input.start_timestamp {
        object
            .key("StartTimestamp")
            .instant(var_47, aws_smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_48) = &input.end_timestamp {
        object
            .key("EndTimestamp")
            .instant(var_48, aws_smithy_types::instant::Format::EpochSeconds);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dash_timestamp_range(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DashTimestampRange,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_49) = &input.start_timestamp {
        object
            .key("StartTimestamp")
            .instant(var_49, aws_smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_50) = &input.end_timestamp {
        object
            .key("EndTimestamp")
            .instant(var_50, aws_smithy_types::instant::Format::EpochSeconds);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_hls_timestamp_range(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HlsTimestampRange,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_51) = &input.start_timestamp {
        object
            .key("StartTimestamp")
            .instant(var_51, aws_smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_52) = &input.end_timestamp {
        object
            .key("EndTimestamp")
            .instant(var_52, aws_smithy_types::instant::Format::EpochSeconds);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_timestamp_range(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TimestampRange,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_53) = &input.start_timestamp {
        object
            .key("StartTimestamp")
            .instant(var_53, aws_smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_54) = &input.end_timestamp {
        object
            .key("EndTimestamp")
            .instant(var_54, aws_smithy_types::instant::Format::EpochSeconds);
    }
    Ok(())
}