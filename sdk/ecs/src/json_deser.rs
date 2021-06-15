// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn create_capacity_provider_deser_operation(
    inp: &[u8],
    mut builder: crate::output::create_capacity_provider_output::Builder,
) -> Result<crate::output::create_capacity_provider_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::CreateCapacityProviderOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_capacity_provider(parsed_body.capacity_provider);
    Ok(builder)
}

pub fn create_cluster_deser_operation(
    inp: &[u8],
    mut builder: crate::output::create_cluster_output::Builder,
) -> Result<crate::output::create_cluster_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::CreateClusterOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_cluster(parsed_body.cluster);
    Ok(builder)
}

pub fn create_service_deser_operation(
    inp: &[u8],
    mut builder: crate::output::create_service_output::Builder,
) -> Result<crate::output::create_service_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::CreateServiceOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_service(parsed_body.service);
    Ok(builder)
}

pub fn create_task_set_deser_operation(
    inp: &[u8],
    mut builder: crate::output::create_task_set_output::Builder,
) -> Result<crate::output::create_task_set_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::CreateTaskSetOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_task_set(parsed_body.task_set);
    Ok(builder)
}

pub fn delete_account_setting_deser_operation(
    inp: &[u8],
    mut builder: crate::output::delete_account_setting_output::Builder,
) -> Result<crate::output::delete_account_setting_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::DeleteAccountSettingOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_setting(parsed_body.setting);
    Ok(builder)
}

pub fn delete_attributes_deser_operation(
    inp: &[u8],
    mut builder: crate::output::delete_attributes_output::Builder,
) -> Result<crate::output::delete_attributes_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::DeleteAttributesOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_attributes(parsed_body.attributes);
    Ok(builder)
}

pub fn delete_capacity_provider_deser_operation(
    inp: &[u8],
    mut builder: crate::output::delete_capacity_provider_output::Builder,
) -> Result<crate::output::delete_capacity_provider_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::DeleteCapacityProviderOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_capacity_provider(parsed_body.capacity_provider);
    Ok(builder)
}

pub fn delete_cluster_deser_operation(
    inp: &[u8],
    mut builder: crate::output::delete_cluster_output::Builder,
) -> Result<crate::output::delete_cluster_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::DeleteClusterOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_cluster(parsed_body.cluster);
    Ok(builder)
}

pub fn delete_service_deser_operation(
    inp: &[u8],
    mut builder: crate::output::delete_service_output::Builder,
) -> Result<crate::output::delete_service_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::DeleteServiceOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_service(parsed_body.service);
    Ok(builder)
}

pub fn delete_task_set_deser_operation(
    inp: &[u8],
    mut builder: crate::output::delete_task_set_output::Builder,
) -> Result<crate::output::delete_task_set_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::DeleteTaskSetOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_task_set(parsed_body.task_set);
    Ok(builder)
}

pub fn deregister_container_instance_deser_operation(
    inp: &[u8],
    mut builder: crate::output::deregister_container_instance_output::Builder,
) -> Result<crate::output::deregister_container_instance_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::DeregisterContainerInstanceOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_container_instance(parsed_body.container_instance);
    Ok(builder)
}

pub fn deregister_task_definition_deser_operation(
    inp: &[u8],
    mut builder: crate::output::deregister_task_definition_output::Builder,
) -> Result<crate::output::deregister_task_definition_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::DeregisterTaskDefinitionOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_task_definition(parsed_body.task_definition);
    Ok(builder)
}

pub fn describe_capacity_providers_deser_operation(
    inp: &[u8],
    mut builder: crate::output::describe_capacity_providers_output::Builder,
) -> Result<crate::output::describe_capacity_providers_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::DescribeCapacityProvidersOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_capacity_providers(parsed_body.capacity_providers);
    builder = builder.set_failures(parsed_body.failures);
    builder = builder.set_next_token(parsed_body.next_token);
    Ok(builder)
}

pub fn describe_clusters_deser_operation(
    inp: &[u8],
    mut builder: crate::output::describe_clusters_output::Builder,
) -> Result<crate::output::describe_clusters_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::DescribeClustersOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_clusters(parsed_body.clusters);
    builder = builder.set_failures(parsed_body.failures);
    Ok(builder)
}

pub fn describe_container_instances_deser_operation(
    inp: &[u8],
    mut builder: crate::output::describe_container_instances_output::Builder,
) -> Result<crate::output::describe_container_instances_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::DescribeContainerInstancesOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_container_instances(parsed_body.container_instances);
    builder = builder.set_failures(parsed_body.failures);
    Ok(builder)
}

pub fn describe_services_deser_operation(
    inp: &[u8],
    mut builder: crate::output::describe_services_output::Builder,
) -> Result<crate::output::describe_services_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::DescribeServicesOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_services(parsed_body.services);
    builder = builder.set_failures(parsed_body.failures);
    Ok(builder)
}

pub fn describe_task_definition_deser_operation(
    inp: &[u8],
    mut builder: crate::output::describe_task_definition_output::Builder,
) -> Result<crate::output::describe_task_definition_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::DescribeTaskDefinitionOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_task_definition(parsed_body.task_definition);
    builder = builder.set_tags(parsed_body.tags);
    Ok(builder)
}

pub fn describe_tasks_deser_operation(
    inp: &[u8],
    mut builder: crate::output::describe_tasks_output::Builder,
) -> Result<crate::output::describe_tasks_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::DescribeTasksOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_tasks(parsed_body.tasks);
    builder = builder.set_failures(parsed_body.failures);
    Ok(builder)
}

pub fn describe_task_sets_deser_operation(
    inp: &[u8],
    mut builder: crate::output::describe_task_sets_output::Builder,
) -> Result<crate::output::describe_task_sets_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::DescribeTaskSetsOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_task_sets(parsed_body.task_sets);
    builder = builder.set_failures(parsed_body.failures);
    Ok(builder)
}

pub fn discover_poll_endpoint_deser_operation(
    inp: &[u8],
    mut builder: crate::output::discover_poll_endpoint_output::Builder,
) -> Result<crate::output::discover_poll_endpoint_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::DiscoverPollEndpointOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_endpoint(parsed_body.endpoint);
    builder = builder.set_telemetry_endpoint(parsed_body.telemetry_endpoint);
    Ok(builder)
}

pub fn execute_command_deser_operation(
    inp: &[u8],
    mut builder: crate::output::execute_command_output::Builder,
) -> Result<crate::output::execute_command_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ExecuteCommandOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_cluster_arn(parsed_body.cluster_arn);
    builder = builder.set_container_arn(parsed_body.container_arn);
    builder = builder.set_container_name(parsed_body.container_name);
    builder = builder.set_interactive(parsed_body.interactive);
    builder = builder.set_session(parsed_body.session);
    builder = builder.set_task_arn(parsed_body.task_arn);
    Ok(builder)
}

pub fn list_account_settings_deser_operation(
    inp: &[u8],
    mut builder: crate::output::list_account_settings_output::Builder,
) -> Result<crate::output::list_account_settings_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ListAccountSettingsOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_settings(parsed_body.settings);
    builder = builder.set_next_token(parsed_body.next_token);
    Ok(builder)
}

pub fn list_attributes_deser_operation(
    inp: &[u8],
    mut builder: crate::output::list_attributes_output::Builder,
) -> Result<crate::output::list_attributes_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ListAttributesOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_attributes(parsed_body.attributes);
    builder = builder.set_next_token(parsed_body.next_token);
    Ok(builder)
}

pub fn list_clusters_deser_operation(
    inp: &[u8],
    mut builder: crate::output::list_clusters_output::Builder,
) -> Result<crate::output::list_clusters_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ListClustersOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_cluster_arns(parsed_body.cluster_arns);
    builder = builder.set_next_token(parsed_body.next_token);
    Ok(builder)
}

pub fn list_container_instances_deser_operation(
    inp: &[u8],
    mut builder: crate::output::list_container_instances_output::Builder,
) -> Result<crate::output::list_container_instances_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ListContainerInstancesOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_container_instance_arns(parsed_body.container_instance_arns);
    builder = builder.set_next_token(parsed_body.next_token);
    Ok(builder)
}

pub fn list_services_deser_operation(
    inp: &[u8],
    mut builder: crate::output::list_services_output::Builder,
) -> Result<crate::output::list_services_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ListServicesOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_service_arns(parsed_body.service_arns);
    builder = builder.set_next_token(parsed_body.next_token);
    Ok(builder)
}

pub fn list_tags_for_resource_deser_operation(
    inp: &[u8],
    mut builder: crate::output::list_tags_for_resource_output::Builder,
) -> Result<crate::output::list_tags_for_resource_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ListTagsForResourceOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_tags(parsed_body.tags);
    Ok(builder)
}

pub fn list_task_definition_families_deser_operation(
    inp: &[u8],
    mut builder: crate::output::list_task_definition_families_output::Builder,
) -> Result<crate::output::list_task_definition_families_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ListTaskDefinitionFamiliesOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_families(parsed_body.families);
    builder = builder.set_next_token(parsed_body.next_token);
    Ok(builder)
}

pub fn list_task_definitions_deser_operation(
    inp: &[u8],
    mut builder: crate::output::list_task_definitions_output::Builder,
) -> Result<crate::output::list_task_definitions_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ListTaskDefinitionsOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_task_definition_arns(parsed_body.task_definition_arns);
    builder = builder.set_next_token(parsed_body.next_token);
    Ok(builder)
}

pub fn list_tasks_deser_operation(
    inp: &[u8],
    mut builder: crate::output::list_tasks_output::Builder,
) -> Result<crate::output::list_tasks_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ListTasksOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_task_arns(parsed_body.task_arns);
    builder = builder.set_next_token(parsed_body.next_token);
    Ok(builder)
}

pub fn put_account_setting_deser_operation(
    inp: &[u8],
    mut builder: crate::output::put_account_setting_output::Builder,
) -> Result<crate::output::put_account_setting_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::PutAccountSettingOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_setting(parsed_body.setting);
    Ok(builder)
}

pub fn put_account_setting_default_deser_operation(
    inp: &[u8],
    mut builder: crate::output::put_account_setting_default_output::Builder,
) -> Result<crate::output::put_account_setting_default_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::PutAccountSettingDefaultOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_setting(parsed_body.setting);
    Ok(builder)
}

pub fn put_attributes_deser_operation(
    inp: &[u8],
    mut builder: crate::output::put_attributes_output::Builder,
) -> Result<crate::output::put_attributes_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::PutAttributesOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_attributes(parsed_body.attributes);
    Ok(builder)
}

pub fn put_cluster_capacity_providers_deser_operation(
    inp: &[u8],
    mut builder: crate::output::put_cluster_capacity_providers_output::Builder,
) -> Result<crate::output::put_cluster_capacity_providers_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::PutClusterCapacityProvidersOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_cluster(parsed_body.cluster);
    Ok(builder)
}

pub fn register_container_instance_deser_operation(
    inp: &[u8],
    mut builder: crate::output::register_container_instance_output::Builder,
) -> Result<crate::output::register_container_instance_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::RegisterContainerInstanceOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_container_instance(parsed_body.container_instance);
    Ok(builder)
}

pub fn register_task_definition_deser_operation(
    inp: &[u8],
    mut builder: crate::output::register_task_definition_output::Builder,
) -> Result<crate::output::register_task_definition_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::RegisterTaskDefinitionOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_task_definition(parsed_body.task_definition);
    builder = builder.set_tags(parsed_body.tags);
    Ok(builder)
}

pub fn run_task_deser_operation(
    inp: &[u8],
    mut builder: crate::output::run_task_output::Builder,
) -> Result<crate::output::run_task_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::RunTaskOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_tasks(parsed_body.tasks);
    builder = builder.set_failures(parsed_body.failures);
    Ok(builder)
}

pub fn start_task_deser_operation(
    inp: &[u8],
    mut builder: crate::output::start_task_output::Builder,
) -> Result<crate::output::start_task_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::StartTaskOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_tasks(parsed_body.tasks);
    builder = builder.set_failures(parsed_body.failures);
    Ok(builder)
}

pub fn stop_task_deser_operation(
    inp: &[u8],
    mut builder: crate::output::stop_task_output::Builder,
) -> Result<crate::output::stop_task_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::StopTaskOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_task(parsed_body.task);
    Ok(builder)
}

pub fn submit_attachment_state_changes_deser_operation(
    inp: &[u8],
    mut builder: crate::output::submit_attachment_state_changes_output::Builder,
) -> Result<crate::output::submit_attachment_state_changes_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::SubmitAttachmentStateChangesOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_acknowledgment(parsed_body.acknowledgment);
    Ok(builder)
}

pub fn submit_container_state_change_deser_operation(
    inp: &[u8],
    mut builder: crate::output::submit_container_state_change_output::Builder,
) -> Result<crate::output::submit_container_state_change_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::SubmitContainerStateChangeOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_acknowledgment(parsed_body.acknowledgment);
    Ok(builder)
}

pub fn submit_task_state_change_deser_operation(
    inp: &[u8],
    mut builder: crate::output::submit_task_state_change_output::Builder,
) -> Result<crate::output::submit_task_state_change_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::SubmitTaskStateChangeOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_acknowledgment(parsed_body.acknowledgment);
    Ok(builder)
}

pub fn update_capacity_provider_deser_operation(
    inp: &[u8],
    mut builder: crate::output::update_capacity_provider_output::Builder,
) -> Result<crate::output::update_capacity_provider_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::UpdateCapacityProviderOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_capacity_provider(parsed_body.capacity_provider);
    Ok(builder)
}

pub fn update_cluster_deser_operation(
    inp: &[u8],
    mut builder: crate::output::update_cluster_output::Builder,
) -> Result<crate::output::update_cluster_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::UpdateClusterOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_cluster(parsed_body.cluster);
    Ok(builder)
}

pub fn update_cluster_settings_deser_operation(
    inp: &[u8],
    mut builder: crate::output::update_cluster_settings_output::Builder,
) -> Result<crate::output::update_cluster_settings_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::UpdateClusterSettingsOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_cluster(parsed_body.cluster);
    Ok(builder)
}

pub fn update_container_agent_deser_operation(
    inp: &[u8],
    mut builder: crate::output::update_container_agent_output::Builder,
) -> Result<crate::output::update_container_agent_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::UpdateContainerAgentOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_container_instance(parsed_body.container_instance);
    Ok(builder)
}

pub fn update_container_instances_state_deser_operation(
    inp: &[u8],
    mut builder: crate::output::update_container_instances_state_output::Builder,
) -> Result<crate::output::update_container_instances_state_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::UpdateContainerInstancesStateOutputBody = if inp.is_empty()
    {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_container_instances(parsed_body.container_instances);
    builder = builder.set_failures(parsed_body.failures);
    Ok(builder)
}

pub fn update_service_deser_operation(
    inp: &[u8],
    mut builder: crate::output::update_service_output::Builder,
) -> Result<crate::output::update_service_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::UpdateServiceOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_service(parsed_body.service);
    Ok(builder)
}

pub fn update_service_primary_task_set_deser_operation(
    inp: &[u8],
    mut builder: crate::output::update_service_primary_task_set_output::Builder,
) -> Result<crate::output::update_service_primary_task_set_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::UpdateServicePrimaryTaskSetOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_task_set(parsed_body.task_set);
    Ok(builder)
}

pub fn update_task_set_deser_operation(
    inp: &[u8],
    mut builder: crate::output::update_task_set_output::Builder,
) -> Result<crate::output::update_task_set_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::UpdateTaskSetOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_task_set(parsed_body.task_set);
    Ok(builder)
}