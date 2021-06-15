// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn parse_generic_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<smithy_types::Error, serde_json::Error> {
    let body =
        serde_json::from_slice(response.body().as_ref()).unwrap_or_else(|_| serde_json::json!({}));
    Ok(crate::aws_json_errors::parse_generic_error(
        &response, &body,
    ))
}

pub fn bad_request_exception(
    inp: &[u8],
    mut builder: crate::error::bad_request_error::Builder,
) -> Result<crate::error::bad_request_error::Builder, serde_json::Error> {
    let parsed_body: crate::error::BadRequestError = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_message(parsed_body.message);
    Ok(builder)
}

pub fn forbidden_exception(
    inp: &[u8],
    mut builder: crate::error::forbidden_error::Builder,
) -> Result<crate::error::forbidden_error::Builder, serde_json::Error> {
    let parsed_body: crate::error::ForbiddenError = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_message(parsed_body.message);
    Ok(builder)
}

pub fn statement_timeout_exception(
    inp: &[u8],
    mut builder: crate::error::statement_timeout_error::Builder,
) -> Result<crate::error::statement_timeout_error::Builder, serde_json::Error> {
    let parsed_body: crate::error::StatementTimeoutError = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_message(parsed_body.message);
    builder = builder.set_db_connection_id(parsed_body.db_connection_id);
    Ok(builder)
}

pub fn batch_execute_statement_deser_operation(
    inp: &[u8],
    mut builder: crate::output::batch_execute_statement_output::Builder,
) -> Result<crate::output::batch_execute_statement_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::BatchExecuteStatementOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_update_results(parsed_body.update_results);
    Ok(builder)
}

pub fn begin_transaction_deser_operation(
    inp: &[u8],
    mut builder: crate::output::begin_transaction_output::Builder,
) -> Result<crate::output::begin_transaction_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::BeginTransactionOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_transaction_id(parsed_body.transaction_id);
    Ok(builder)
}

pub fn not_found_exception(
    inp: &[u8],
    mut builder: crate::error::not_found_error::Builder,
) -> Result<crate::error::not_found_error::Builder, serde_json::Error> {
    let parsed_body: crate::error::NotFoundError = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_message(parsed_body.message);
    Ok(builder)
}

pub fn commit_transaction_deser_operation(
    inp: &[u8],
    mut builder: crate::output::commit_transaction_output::Builder,
) -> Result<crate::output::commit_transaction_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::CommitTransactionOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_transaction_status(parsed_body.transaction_status);
    Ok(builder)
}

pub fn execute_sql_deser_operation(
    inp: &[u8],
    mut builder: crate::output::execute_sql_output::Builder,
) -> Result<crate::output::execute_sql_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ExecuteSqlOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_sql_statement_results(parsed_body.sql_statement_results);
    Ok(builder)
}

pub fn execute_statement_deser_operation(
    inp: &[u8],
    mut builder: crate::output::execute_statement_output::Builder,
) -> Result<crate::output::execute_statement_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ExecuteStatementOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_records(parsed_body.records);
    builder = builder.set_column_metadata(parsed_body.column_metadata);
    builder = builder.set_number_of_records_updated(parsed_body.number_of_records_updated);
    builder = builder.set_generated_fields(parsed_body.generated_fields);
    Ok(builder)
}

pub fn rollback_transaction_deser_operation(
    inp: &[u8],
    mut builder: crate::output::rollback_transaction_output::Builder,
) -> Result<crate::output::rollback_transaction_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::RollbackTransactionOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_transaction_status(parsed_body.transaction_status);
    Ok(builder)
}