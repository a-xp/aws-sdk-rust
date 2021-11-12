// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_replication_set_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateReplicationSetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1);
    }
    if let Some(var_2) = &input.regions {
        let mut object_3 = object.key("regions").start_object();
        for (key_4, value_5) in var_2 {
            {
                let mut object_6 = object_3.key(key_4).start_object();
                crate::json_ser::serialize_structure_crate_model_region_map_input_value(
                    &mut object_6,
                    value_5,
                )?;
                object_6.finish();
            }
        }
        object_3.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_response_plan_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateResponsePlanInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_7) = &input.actions {
        let mut array_8 = object.key("actions").start_array();
        for item_9 in var_7 {
            {
                let mut object_10 = array_8.value().start_object();
                crate::json_ser::serialize_union_crate_model_action(&mut object_10, item_9)?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    if let Some(var_11) = &input.chat_channel {
        let mut object_12 = object.key("chatChannel").start_object();
        crate::json_ser::serialize_union_crate_model_chat_channel(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.client_token {
        object.key("clientToken").string(var_13);
    }
    if let Some(var_14) = &input.display_name {
        object.key("displayName").string(var_14);
    }
    if let Some(var_15) = &input.engagements {
        let mut array_16 = object.key("engagements").start_array();
        for item_17 in var_15 {
            {
                array_16.value().string(item_17);
            }
        }
        array_16.finish();
    }
    if let Some(var_18) = &input.incident_template {
        let mut object_19 = object.key("incidentTemplate").start_object();
        crate::json_ser::serialize_structure_crate_model_incident_template(&mut object_19, var_18)?;
        object_19.finish();
    }
    if let Some(var_20) = &input.name {
        object.key("name").string(var_20);
    }
    if let Some(var_21) = &input.tags {
        let mut object_22 = object.key("tags").start_object();
        for (key_23, value_24) in var_21 {
            {
                object_22.key(key_23).string(value_24);
            }
        }
        object_22.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_timeline_event_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateTimelineEventInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_25) = &input.client_token {
        object.key("clientToken").string(var_25);
    }
    if let Some(var_26) = &input.event_data {
        object.key("eventData").string(var_26);
    }
    if let Some(var_27) = &input.event_time {
        object
            .key("eventTime")
            .instant(var_27, aws_smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_28) = &input.event_type {
        object.key("eventType").string(var_28);
    }
    if let Some(var_29) = &input.incident_record_arn {
        object.key("incidentRecordArn").string(var_29);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_incident_record_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteIncidentRecordInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_30) = &input.arn {
        object.key("arn").string(var_30);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_resource_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteResourcePolicyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_31) = &input.policy_id {
        object.key("policyId").string(var_31);
    }
    if let Some(var_32) = &input.resource_arn {
        object.key("resourceArn").string(var_32);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_response_plan_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteResponsePlanInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_33) = &input.arn {
        object.key("arn").string(var_33);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_timeline_event_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteTimelineEventInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_34) = &input.event_id {
        object.key("eventId").string(var_34);
    }
    if let Some(var_35) = &input.incident_record_arn {
        object.key("incidentRecordArn").string(var_35);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_resource_policies_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetResourcePoliciesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_36) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_36).into()),
        );
    }
    if let Some(var_37) = &input.next_token {
        object.key("nextToken").string(var_37);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_incident_records_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListIncidentRecordsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_38) = &input.filters {
        let mut array_39 = object.key("filters").start_array();
        for item_40 in var_38 {
            {
                let mut object_41 = array_39.value().start_object();
                crate::json_ser::serialize_structure_crate_model_filter(&mut object_41, item_40)?;
                object_41.finish();
            }
        }
        array_39.finish();
    }
    if let Some(var_42) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_42).into()),
        );
    }
    if let Some(var_43) = &input.next_token {
        object.key("nextToken").string(var_43);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_related_items_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListRelatedItemsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_44) = &input.incident_record_arn {
        object.key("incidentRecordArn").string(var_44);
    }
    if let Some(var_45) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_45).into()),
        );
    }
    if let Some(var_46) = &input.next_token {
        object.key("nextToken").string(var_46);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_replication_sets_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListReplicationSetsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_47) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_47).into()),
        );
    }
    if let Some(var_48) = &input.next_token {
        object.key("nextToken").string(var_48);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_response_plans_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListResponsePlansInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_49) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_49).into()),
        );
    }
    if let Some(var_50) = &input.next_token {
        object.key("nextToken").string(var_50);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_timeline_events_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTimelineEventsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_51) = &input.filters {
        let mut array_52 = object.key("filters").start_array();
        for item_53 in var_51 {
            {
                let mut object_54 = array_52.value().start_object();
                crate::json_ser::serialize_structure_crate_model_filter(&mut object_54, item_53)?;
                object_54.finish();
            }
        }
        array_52.finish();
    }
    if let Some(var_55) = &input.incident_record_arn {
        object.key("incidentRecordArn").string(var_55);
    }
    if let Some(var_56) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_56).into()),
        );
    }
    if let Some(var_57) = &input.next_token {
        object.key("nextToken").string(var_57);
    }
    if let Some(var_58) = &input.sort_by {
        object.key("sortBy").string(var_58.as_str());
    }
    if let Some(var_59) = &input.sort_order {
        object.key("sortOrder").string(var_59.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_resource_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutResourcePolicyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_60) = &input.policy {
        object.key("policy").string(var_60);
    }
    if let Some(var_61) = &input.resource_arn {
        object.key("resourceArn").string(var_61);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_incident_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartIncidentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_62) = &input.client_token {
        object.key("clientToken").string(var_62);
    }
    if let Some(var_63) = &input.impact {
        object.key("impact").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_63).into()),
        );
    }
    if let Some(var_64) = &input.related_items {
        let mut array_65 = object.key("relatedItems").start_array();
        for item_66 in var_64 {
            {
                let mut object_67 = array_65.value().start_object();
                crate::json_ser::serialize_structure_crate_model_related_item(
                    &mut object_67,
                    item_66,
                )?;
                object_67.finish();
            }
        }
        array_65.finish();
    }
    if let Some(var_68) = &input.response_plan_arn {
        object.key("responsePlanArn").string(var_68);
    }
    if let Some(var_69) = &input.title {
        object.key("title").string(var_69);
    }
    if let Some(var_70) = &input.trigger_details {
        let mut object_71 = object.key("triggerDetails").start_object();
        crate::json_ser::serialize_structure_crate_model_trigger_details(&mut object_71, var_70)?;
        object_71.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_72) = &input.tags {
        let mut object_73 = object.key("tags").start_object();
        for (key_74, value_75) in var_72 {
            {
                object_73.key(key_74).string(value_75);
            }
        }
        object_73.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_deletion_protection_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDeletionProtectionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_76) = &input.arn {
        object.key("arn").string(var_76);
    }
    if let Some(var_77) = &input.client_token {
        object.key("clientToken").string(var_77);
    }
    if let Some(var_78) = &input.deletion_protected {
        object.key("deletionProtected").boolean(*var_78);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_incident_record_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateIncidentRecordInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_79) = &input.arn {
        object.key("arn").string(var_79);
    }
    if let Some(var_80) = &input.chat_channel {
        let mut object_81 = object.key("chatChannel").start_object();
        crate::json_ser::serialize_union_crate_model_chat_channel(&mut object_81, var_80)?;
        object_81.finish();
    }
    if let Some(var_82) = &input.client_token {
        object.key("clientToken").string(var_82);
    }
    if let Some(var_83) = &input.impact {
        object.key("impact").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_83).into()),
        );
    }
    if let Some(var_84) = &input.notification_targets {
        let mut array_85 = object.key("notificationTargets").start_array();
        for item_86 in var_84 {
            {
                let mut object_87 = array_85.value().start_object();
                crate::json_ser::serialize_union_crate_model_notification_target_item(
                    &mut object_87,
                    item_86,
                )?;
                object_87.finish();
            }
        }
        array_85.finish();
    }
    if let Some(var_88) = &input.status {
        object.key("status").string(var_88.as_str());
    }
    if let Some(var_89) = &input.summary {
        object.key("summary").string(var_89);
    }
    if let Some(var_90) = &input.title {
        object.key("title").string(var_90);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_related_items_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateRelatedItemsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_91) = &input.client_token {
        object.key("clientToken").string(var_91);
    }
    if let Some(var_92) = &input.incident_record_arn {
        object.key("incidentRecordArn").string(var_92);
    }
    if let Some(var_93) = &input.related_items_update {
        let mut object_94 = object.key("relatedItemsUpdate").start_object();
        crate::json_ser::serialize_union_crate_model_related_items_update(&mut object_94, var_93)?;
        object_94.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_replication_set_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateReplicationSetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_95) = &input.actions {
        let mut array_96 = object.key("actions").start_array();
        for item_97 in var_95 {
            {
                let mut object_98 = array_96.value().start_object();
                crate::json_ser::serialize_union_crate_model_update_replication_set_action(
                    &mut object_98,
                    item_97,
                )?;
                object_98.finish();
            }
        }
        array_96.finish();
    }
    if let Some(var_99) = &input.arn {
        object.key("arn").string(var_99);
    }
    if let Some(var_100) = &input.client_token {
        object.key("clientToken").string(var_100);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_response_plan_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateResponsePlanInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_101) = &input.actions {
        let mut array_102 = object.key("actions").start_array();
        for item_103 in var_101 {
            {
                let mut object_104 = array_102.value().start_object();
                crate::json_ser::serialize_union_crate_model_action(&mut object_104, item_103)?;
                object_104.finish();
            }
        }
        array_102.finish();
    }
    if let Some(var_105) = &input.arn {
        object.key("arn").string(var_105);
    }
    if let Some(var_106) = &input.chat_channel {
        let mut object_107 = object.key("chatChannel").start_object();
        crate::json_ser::serialize_union_crate_model_chat_channel(&mut object_107, var_106)?;
        object_107.finish();
    }
    if let Some(var_108) = &input.client_token {
        object.key("clientToken").string(var_108);
    }
    if let Some(var_109) = &input.display_name {
        object.key("displayName").string(var_109);
    }
    if let Some(var_110) = &input.engagements {
        let mut array_111 = object.key("engagements").start_array();
        for item_112 in var_110 {
            {
                array_111.value().string(item_112);
            }
        }
        array_111.finish();
    }
    if let Some(var_113) = &input.incident_template_dedupe_string {
        object.key("incidentTemplateDedupeString").string(var_113);
    }
    if let Some(var_114) = &input.incident_template_impact {
        object.key("incidentTemplateImpact").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_114).into()),
        );
    }
    if let Some(var_115) = &input.incident_template_notification_targets {
        let mut array_116 = object
            .key("incidentTemplateNotificationTargets")
            .start_array();
        for item_117 in var_115 {
            {
                let mut object_118 = array_116.value().start_object();
                crate::json_ser::serialize_union_crate_model_notification_target_item(
                    &mut object_118,
                    item_117,
                )?;
                object_118.finish();
            }
        }
        array_116.finish();
    }
    if let Some(var_119) = &input.incident_template_summary {
        object.key("incidentTemplateSummary").string(var_119);
    }
    if let Some(var_120) = &input.incident_template_title {
        object.key("incidentTemplateTitle").string(var_120);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_timeline_event_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateTimelineEventInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_121) = &input.client_token {
        object.key("clientToken").string(var_121);
    }
    if let Some(var_122) = &input.event_data {
        object.key("eventData").string(var_122);
    }
    if let Some(var_123) = &input.event_id {
        object.key("eventId").string(var_123);
    }
    if let Some(var_124) = &input.event_time {
        object
            .key("eventTime")
            .instant(var_124, aws_smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_125) = &input.event_type {
        object.key("eventType").string(var_125);
    }
    if let Some(var_126) = &input.incident_record_arn {
        object.key("incidentRecordArn").string(var_126);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_region_map_input_value(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RegionMapInputValue,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_127) = &input.sse_kms_key_id {
        object.key("sseKmsKeyId").string(var_127);
    }
    Ok(())
}

pub fn serialize_union_crate_model_action(
    object_10: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Action,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::Action::SsmAutomation(inner) => {
            let mut object_128 = object_10.key("ssmAutomation").start_object();
            crate::json_ser::serialize_structure_crate_model_ssm_automation(
                &mut object_128,
                inner,
            )?;
            object_128.finish();
        }
        crate::model::Action::Unknown => {
            return Err(aws_smithy_http::operation::SerializationError::unknown_variant("Action"))
        }
    }
    Ok(())
}

pub fn serialize_union_crate_model_chat_channel(
    object_12: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ChatChannel,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::ChatChannel::Empty(inner) => {
            let mut object_129 = object_12.key("empty").start_object();
            crate::json_ser::serialize_structure_crate_model_empty_chat_channel(
                &mut object_129,
                inner,
            )?;
            object_129.finish();
        }
        crate::model::ChatChannel::ChatbotSns(inner) => {
            let mut array_130 = object_12.key("chatbotSns").start_array();
            for item_131 in inner {
                {
                    array_130.value().string(item_131);
                }
            }
            array_130.finish();
        }
        crate::model::ChatChannel::Unknown => {
            return Err(
                aws_smithy_http::operation::SerializationError::unknown_variant("ChatChannel"),
            )
        }
    }
    Ok(())
}

pub fn serialize_structure_crate_model_incident_template(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IncidentTemplate,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_132) = &input.title {
        object.key("title").string(var_132);
    }
    if let Some(var_133) = &input.impact {
        object.key("impact").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_133).into()),
        );
    }
    if let Some(var_134) = &input.summary {
        object.key("summary").string(var_134);
    }
    if let Some(var_135) = &input.dedupe_string {
        object.key("dedupeString").string(var_135);
    }
    if let Some(var_136) = &input.notification_targets {
        let mut array_137 = object.key("notificationTargets").start_array();
        for item_138 in var_136 {
            {
                let mut object_139 = array_137.value().start_object();
                crate::json_ser::serialize_union_crate_model_notification_target_item(
                    &mut object_139,
                    item_138,
                )?;
                object_139.finish();
            }
        }
        array_137.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Filter,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_140) = &input.key {
        object.key("key").string(var_140);
    }
    if let Some(var_141) = &input.condition {
        let mut object_142 = object.key("condition").start_object();
        crate::json_ser::serialize_union_crate_model_condition(&mut object_142, var_141)?;
        object_142.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_related_item(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RelatedItem,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_143) = &input.identifier {
        let mut object_144 = object.key("identifier").start_object();
        crate::json_ser::serialize_structure_crate_model_item_identifier(&mut object_144, var_143)?;
        object_144.finish();
    }
    if let Some(var_145) = &input.title {
        object.key("title").string(var_145);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_trigger_details(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TriggerDetails,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_146) = &input.source {
        object.key("source").string(var_146);
    }
    if let Some(var_147) = &input.trigger_arn {
        object.key("triggerArn").string(var_147);
    }
    if let Some(var_148) = &input.timestamp {
        object
            .key("timestamp")
            .instant(var_148, aws_smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_149) = &input.raw_data {
        object.key("rawData").string(var_149);
    }
    Ok(())
}

pub fn serialize_union_crate_model_notification_target_item(
    object_87: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NotificationTargetItem,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::NotificationTargetItem::SnsTopicArn(inner) => {
            object_87.key("snsTopicArn").string(inner);
        }
        crate::model::NotificationTargetItem::Unknown => {
            return Err(
                aws_smithy_http::operation::SerializationError::unknown_variant(
                    "NotificationTargetItem",
                ),
            )
        }
    }
    Ok(())
}

pub fn serialize_union_crate_model_related_items_update(
    object_94: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RelatedItemsUpdate,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::RelatedItemsUpdate::ItemToAdd(inner) => {
            let mut object_150 = object_94.key("itemToAdd").start_object();
            crate::json_ser::serialize_structure_crate_model_related_item(&mut object_150, inner)?;
            object_150.finish();
        }
        crate::model::RelatedItemsUpdate::ItemToRemove(inner) => {
            let mut object_151 = object_94.key("itemToRemove").start_object();
            crate::json_ser::serialize_structure_crate_model_item_identifier(
                &mut object_151,
                inner,
            )?;
            object_151.finish();
        }
        crate::model::RelatedItemsUpdate::Unknown => {
            return Err(
                aws_smithy_http::operation::SerializationError::unknown_variant(
                    "RelatedItemsUpdate",
                ),
            )
        }
    }
    Ok(())
}

pub fn serialize_union_crate_model_update_replication_set_action(
    object_98: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpdateReplicationSetAction,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::UpdateReplicationSetAction::AddRegionAction(inner) => {
            let mut object_152 = object_98.key("addRegionAction").start_object();
            crate::json_ser::serialize_structure_crate_model_add_region_action(
                &mut object_152,
                inner,
            )?;
            object_152.finish();
        }
        crate::model::UpdateReplicationSetAction::DeleteRegionAction(inner) => {
            let mut object_153 = object_98.key("deleteRegionAction").start_object();
            crate::json_ser::serialize_structure_crate_model_delete_region_action(
                &mut object_153,
                inner,
            )?;
            object_153.finish();
        }
        crate::model::UpdateReplicationSetAction::Unknown => {
            return Err(
                aws_smithy_http::operation::SerializationError::unknown_variant(
                    "UpdateReplicationSetAction",
                ),
            )
        }
    }
    Ok(())
}

pub fn serialize_structure_crate_model_ssm_automation(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SsmAutomation,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_154) = &input.role_arn {
        object.key("roleArn").string(var_154);
    }
    if let Some(var_155) = &input.document_name {
        object.key("documentName").string(var_155);
    }
    if let Some(var_156) = &input.document_version {
        object.key("documentVersion").string(var_156);
    }
    if let Some(var_157) = &input.target_account {
        object.key("targetAccount").string(var_157.as_str());
    }
    if let Some(var_158) = &input.parameters {
        let mut object_159 = object.key("parameters").start_object();
        for (key_160, value_161) in var_158 {
            {
                let mut array_162 = object_159.key(key_160).start_array();
                for item_163 in value_161 {
                    {
                        array_162.value().string(item_163);
                    }
                }
                array_162.finish();
            }
        }
        object_159.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_empty_chat_channel(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EmptyChatChannel,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    let (_, _) = (object, input);
    Ok(())
}

pub fn serialize_union_crate_model_condition(
    object_142: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Condition,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::Condition::Before(inner) => {
            object_142
                .key("before")
                .instant(inner, aws_smithy_types::instant::Format::EpochSeconds);
        }
        crate::model::Condition::After(inner) => {
            object_142
                .key("after")
                .instant(inner, aws_smithy_types::instant::Format::EpochSeconds);
        }
        crate::model::Condition::Equals(inner) => {
            let mut object_164 = object_142.key("equals").start_object();
            crate::json_ser::serialize_union_crate_model_attribute_value_list(
                &mut object_164,
                inner,
            )?;
            object_164.finish();
        }
        crate::model::Condition::Unknown => {
            return Err(
                aws_smithy_http::operation::SerializationError::unknown_variant("Condition"),
            )
        }
    }
    Ok(())
}

pub fn serialize_structure_crate_model_item_identifier(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ItemIdentifier,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_165) = &input.value {
        let mut object_166 = object.key("value").start_object();
        crate::json_ser::serialize_union_crate_model_item_value(&mut object_166, var_165)?;
        object_166.finish();
    }
    if let Some(var_167) = &input.r#type {
        object.key("type").string(var_167.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_add_region_action(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AddRegionAction,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_168) = &input.region_name {
        object.key("regionName").string(var_168);
    }
    if let Some(var_169) = &input.sse_kms_key_id {
        object.key("sseKmsKeyId").string(var_169);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_delete_region_action(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DeleteRegionAction,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_170) = &input.region_name {
        object.key("regionName").string(var_170);
    }
    Ok(())
}

pub fn serialize_union_crate_model_attribute_value_list(
    object_164: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AttributeValueList,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::AttributeValueList::StringValues(inner) => {
            let mut array_171 = object_164.key("stringValues").start_array();
            for item_172 in inner {
                {
                    array_171.value().string(item_172);
                }
            }
            array_171.finish();
        }
        crate::model::AttributeValueList::IntegerValues(inner) => {
            let mut array_173 = object_164.key("integerValues").start_array();
            for item_174 in inner {
                {
                    array_173.value().number(
                        #[allow(clippy::useless_conversion)]
                        aws_smithy_types::Number::NegInt((*item_174).into()),
                    );
                }
            }
            array_173.finish();
        }
        crate::model::AttributeValueList::Unknown => {
            return Err(
                aws_smithy_http::operation::SerializationError::unknown_variant(
                    "AttributeValueList",
                ),
            )
        }
    }
    Ok(())
}

pub fn serialize_union_crate_model_item_value(
    object_166: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ItemValue,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::ItemValue::Arn(inner) => {
            object_166.key("arn").string(inner);
        }
        crate::model::ItemValue::Url(inner) => {
            object_166.key("url").string(inner);
        }
        crate::model::ItemValue::MetricDefinition(inner) => {
            object_166.key("metricDefinition").string(inner);
        }
        crate::model::ItemValue::Unknown => {
            return Err(
                aws_smithy_http::operation::SerializationError::unknown_variant("ItemValue"),
            )
        }
    }
    Ok(())
}