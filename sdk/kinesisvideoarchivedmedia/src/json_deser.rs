// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn parse_http_generic_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<aws_smithy_types::Error, aws_smithy_json::deserialize::Error> {
    crate::json_errors::parse_generic_error(response.body(), response.headers())
}

pub fn deser_structure_crate_error_client_limit_exceeded_exception_json_err(
    value: &[u8],
    mut builder: crate::error::client_limit_exceeded_exception::Builder,
) -> Result<
    crate::error::client_limit_exceeded_exception::Builder,
    aws_smithy_json::deserialize::Error,
> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(aws_smithy_json::deserialize::Error::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_invalid_argument_exception_json_err(
    value: &[u8],
    mut builder: crate::error::invalid_argument_exception::Builder,
) -> Result<crate::error::invalid_argument_exception::Builder, aws_smithy_json::deserialize::Error>
{
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(aws_smithy_json::deserialize::Error::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_invalid_codec_private_data_exception_json_err(
    value: &[u8],
    mut builder: crate::error::invalid_codec_private_data_exception::Builder,
) -> Result<
    crate::error::invalid_codec_private_data_exception::Builder,
    aws_smithy_json::deserialize::Error,
> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(aws_smithy_json::deserialize::Error::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_invalid_media_frame_exception_json_err(
    value: &[u8],
    mut builder: crate::error::invalid_media_frame_exception::Builder,
) -> Result<crate::error::invalid_media_frame_exception::Builder, aws_smithy_json::deserialize::Error>
{
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(aws_smithy_json::deserialize::Error::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_missing_codec_private_data_exception_json_err(
    value: &[u8],
    mut builder: crate::error::missing_codec_private_data_exception::Builder,
) -> Result<
    crate::error::missing_codec_private_data_exception::Builder,
    aws_smithy_json::deserialize::Error,
> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(aws_smithy_json::deserialize::Error::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_no_data_retention_exception_json_err(
    value: &[u8],
    mut builder: crate::error::no_data_retention_exception::Builder,
) -> Result<crate::error::no_data_retention_exception::Builder, aws_smithy_json::deserialize::Error>
{
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(aws_smithy_json::deserialize::Error::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_not_authorized_exception_json_err(
    value: &[u8],
    mut builder: crate::error::not_authorized_exception::Builder,
) -> Result<crate::error::not_authorized_exception::Builder, aws_smithy_json::deserialize::Error> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(aws_smithy_json::deserialize::Error::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_resource_not_found_exception_json_err(
    value: &[u8],
    mut builder: crate::error::resource_not_found_exception::Builder,
) -> Result<crate::error::resource_not_found_exception::Builder, aws_smithy_json::deserialize::Error>
{
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(aws_smithy_json::deserialize::Error::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_unsupported_stream_media_type_exception_json_err(
    value: &[u8],
    mut builder: crate::error::unsupported_stream_media_type_exception::Builder,
) -> Result<
    crate::error::unsupported_stream_media_type_exception::Builder,
    aws_smithy_json::deserialize::Error,
> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(aws_smithy_json::deserialize::Error::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_operation_crate_operation_get_dash_streaming_session_url(
    value: &[u8],
    mut builder: crate::output::get_dash_streaming_session_url_output::Builder,
) -> Result<
    crate::output::get_dash_streaming_session_url_output::Builder,
    aws_smithy_json::deserialize::Error,
> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "DASHStreamingSessionURL" => {
                        builder = builder.set_dash_streaming_session_url(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(aws_smithy_json::deserialize::Error::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_operation_crate_operation_get_hls_streaming_session_url(
    value: &[u8],
    mut builder: crate::output::get_hls_streaming_session_url_output::Builder,
) -> Result<
    crate::output::get_hls_streaming_session_url_output::Builder,
    aws_smithy_json::deserialize::Error,
> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "HLSStreamingSessionURL" => {
                        builder = builder.set_hls_streaming_session_url(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(aws_smithy_json::deserialize::Error::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_operation_crate_operation_list_fragments(
    value: &[u8],
    mut builder: crate::output::list_fragments_output::Builder,
) -> Result<crate::output::list_fragments_output::Builder, aws_smithy_json::deserialize::Error> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Fragments" => {
                        builder = builder.set_fragments(
                            crate::json_deser::deser_list_com_amazonaws_kinesisvideoarchivedmedia_fragment_list(tokens)?
                        );
                    }
                    "NextToken" => {
                        builder = builder.set_next_token(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(aws_smithy_json::deserialize::Error::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

#[allow(clippy::type_complexity, non_snake_case)]
pub fn deser_list_com_amazonaws_kinesisvideoarchivedmedia_fragment_list<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<std::vec::Vec<crate::model::Fragment>>, aws_smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(aws_smithy_json::deserialize::Token::StartArray { .. }) => {
            let mut items = Vec::new();
            loop {
                match tokens.peek() {
                    Some(Ok(aws_smithy_json::deserialize::Token::EndArray { .. })) => {
                        tokens.next().transpose().unwrap();
                        break;
                    }
                    _ => {
                        let value =
                            crate::json_deser::deser_structure_crate_model_fragment(tokens)?;
                        if let Some(value) = value {
                            items.push(value);
                        }
                    }
                }
            }
            Ok(Some(items))
        }
        _ => Err(aws_smithy_json::deserialize::Error::custom(
            "expected start array or null",
        )),
    }
}

pub fn deser_structure_crate_model_fragment<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::Fragment>, aws_smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::Fragment::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "FragmentNumber" => {
                                builder = builder.set_fragment_number(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "FragmentSizeInBytes" => {
                                builder = builder.set_fragment_size_in_bytes(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|v| v.to_i64()),
                                );
                            }
                            "ProducerTimestamp" => {
                                builder = builder.set_producer_timestamp(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                        tokens.next(),
                                        aws_smithy_types::instant::Format::EpochSeconds,
                                    )?,
                                );
                            }
                            "ServerTimestamp" => {
                                builder = builder.set_server_timestamp(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                        tokens.next(),
                                        aws_smithy_types::instant::Format::EpochSeconds,
                                    )?,
                                );
                            }
                            "FragmentLengthInMilliseconds" => {
                                builder = builder.set_fragment_length_in_milliseconds(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|v| v.to_i64()),
                                );
                            }
                            _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    other => {
                        return Err(aws_smithy_json::deserialize::Error::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(aws_smithy_json::deserialize::Error::custom(
            "expected start object or null",
        )),
    }
}