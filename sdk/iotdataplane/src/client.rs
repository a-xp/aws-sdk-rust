// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle<
    C = aws_smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    client: aws_smithy_client::Client<C, M, R>,
    conf: crate::Config,
}

/// An ergonomic service client for `IotMoonrakerService`.
///
/// This client allows ergonomic access to a `IotMoonrakerService`-shaped service.
/// Each method corresponds to an endpoint defined in the service's Smithy model,
/// and the request and response shapes are auto-generated from that same model.
///
/// # Using a Client
///
/// Once you have a client set up, you can access the service's endpoints
/// by calling the appropriate method on [`Client`]. Each such method
/// returns a request builder for that endpoint, with methods for setting
/// the various fields of the request. Once your request is complete, use
/// the `send` method to send the request. `send` returns a future, which
/// you then have to `.await` to get the service's response.
///
/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#c-builder
/// [SigV4-signed requests]: https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html
#[derive(std::fmt::Debug)]
pub struct Client<
    C = aws_smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<Handle<C, M, R>>,
}

impl<C, M, R> std::clone::Clone for Client<C, M, R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use aws_smithy_client::Builder;

impl<C, M, R> From<aws_smithy_client::Client<C, M, R>> for Client<C, M, R> {
    fn from(client: aws_smithy_client::Client<C, M, R>) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl<C, M, R> Client<C, M, R> {
    /// Creates a client with the given service configuration.
    pub fn with_config(client: aws_smithy_client::Client<C, M, R>, conf: crate::Config) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Returns the client's configuration.
    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl<C, M, R> Client<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Constructs a fluent builder for the `DeleteThingShadow` operation.
    ///
    /// See [`DeleteThingShadow`](crate::client::fluent_builders::DeleteThingShadow) for more information about the
    /// operation and its arguments.
    pub fn delete_thing_shadow(&self) -> fluent_builders::DeleteThingShadow<C, M, R> {
        fluent_builders::DeleteThingShadow::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `GetRetainedMessage` operation.
    ///
    /// See [`GetRetainedMessage`](crate::client::fluent_builders::GetRetainedMessage) for more information about the
    /// operation and its arguments.
    pub fn get_retained_message(&self) -> fluent_builders::GetRetainedMessage<C, M, R> {
        fluent_builders::GetRetainedMessage::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `GetThingShadow` operation.
    ///
    /// See [`GetThingShadow`](crate::client::fluent_builders::GetThingShadow) for more information about the
    /// operation and its arguments.
    pub fn get_thing_shadow(&self) -> fluent_builders::GetThingShadow<C, M, R> {
        fluent_builders::GetThingShadow::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `ListNamedShadowsForThing` operation.
    ///
    /// See [`ListNamedShadowsForThing`](crate::client::fluent_builders::ListNamedShadowsForThing) for more information about the
    /// operation and its arguments.
    pub fn list_named_shadows_for_thing(
        &self,
    ) -> fluent_builders::ListNamedShadowsForThing<C, M, R> {
        fluent_builders::ListNamedShadowsForThing::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `ListRetainedMessages` operation.
    ///
    /// See [`ListRetainedMessages`](crate::client::fluent_builders::ListRetainedMessages) for more information about the
    /// operation and its arguments.
    pub fn list_retained_messages(&self) -> fluent_builders::ListRetainedMessages<C, M, R> {
        fluent_builders::ListRetainedMessages::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `Publish` operation.
    ///
    /// See [`Publish`](crate::client::fluent_builders::Publish) for more information about the
    /// operation and its arguments.
    pub fn publish(&self) -> fluent_builders::Publish<C, M, R> {
        fluent_builders::Publish::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `UpdateThingShadow` operation.
    ///
    /// See [`UpdateThingShadow`](crate::client::fluent_builders::UpdateThingShadow) for more information about the
    /// operation and its arguments.
    pub fn update_thing_shadow(&self) -> fluent_builders::UpdateThingShadow<C, M, R> {
        fluent_builders::UpdateThingShadow::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    //!
    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    //!
    /// Fluent builder constructing a request to `DeleteThingShadow`.
    ///
    /// <p>Deletes the shadow for the specified thing.</p>
    /// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">DeleteThingShadow</a> action.</p>
    /// <p>For more information, see <a href="http://docs.aws.amazon.com/iot/latest/developerguide/API_DeleteThingShadow.html">DeleteThingShadow</a> in the IoT Developer Guide.</p>
    #[derive(std::fmt::Debug)]
    pub struct DeleteThingShadow<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::delete_thing_shadow_input::Builder,
    }
    impl<C, M, R> DeleteThingShadow<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `DeleteThingShadow`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DeleteThingShadowOutput,
            aws_smithy_http::result::SdkError<crate::error::DeleteThingShadowError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DeleteThingShadowInputOperationOutputAlias,
                crate::output::DeleteThingShadowOutput,
                crate::error::DeleteThingShadowError,
                crate::input::DeleteThingShadowInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The name of the thing.</p>
        pub fn thing_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.thing_name(inp);
            self
        }
        /// <p>The name of the thing.</p>
        pub fn set_thing_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_thing_name(input);
            self
        }
        /// <p>The name of the shadow.</p>
        pub fn shadow_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.shadow_name(inp);
            self
        }
        /// <p>The name of the shadow.</p>
        pub fn set_shadow_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_shadow_name(input);
            self
        }
    }
    /// Fluent builder constructing a request to `GetRetainedMessage`.
    ///
    /// <p>Gets the details of a single retained message for the specified topic.</p>
    /// <p>This action returns the message payload of the retained message, which can
    /// incur messaging costs. To list only the topic names of the retained messages, call
    /// <a href="/iot/latest/developerguide/API_iotdata_ListRetainedMessages.html">ListRetainedMessages</a>.</p>
    /// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiotfleethubfordevicemanagement.html#awsiotfleethubfordevicemanagement-actions-as-permissions">GetRetainedMessage</a> action.</p>
    /// <p>For more information about messaging costs, see <a href="http://aws.amazon.com/iot-core/pricing/#Messaging">IoT Core
    /// pricing - Messaging</a>.</p>
    #[derive(std::fmt::Debug)]
    pub struct GetRetainedMessage<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_retained_message_input::Builder,
    }
    impl<C, M, R> GetRetainedMessage<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `GetRetainedMessage`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetRetainedMessageOutput,
            aws_smithy_http::result::SdkError<crate::error::GetRetainedMessageError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetRetainedMessageInputOperationOutputAlias,
                crate::output::GetRetainedMessageOutput,
                crate::error::GetRetainedMessageError,
                crate::input::GetRetainedMessageInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The topic name of the retained message to retrieve.</p>
        pub fn topic(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.topic(inp);
            self
        }
        /// <p>The topic name of the retained message to retrieve.</p>
        pub fn set_topic(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_topic(input);
            self
        }
    }
    /// Fluent builder constructing a request to `GetThingShadow`.
    ///
    /// <p>Gets the shadow for the specified thing.</p>
    /// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">GetThingShadow</a> action.</p>
    /// <p>For more information, see <a href="http://docs.aws.amazon.com/iot/latest/developerguide/API_GetThingShadow.html">GetThingShadow</a> in the
    /// IoT Developer Guide.</p>
    #[derive(std::fmt::Debug)]
    pub struct GetThingShadow<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_thing_shadow_input::Builder,
    }
    impl<C, M, R> GetThingShadow<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `GetThingShadow`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetThingShadowOutput,
            aws_smithy_http::result::SdkError<crate::error::GetThingShadowError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetThingShadowInputOperationOutputAlias,
                crate::output::GetThingShadowOutput,
                crate::error::GetThingShadowError,
                crate::input::GetThingShadowInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The name of the thing.</p>
        pub fn thing_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.thing_name(inp);
            self
        }
        /// <p>The name of the thing.</p>
        pub fn set_thing_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_thing_name(input);
            self
        }
        /// <p>The name of the shadow.</p>
        pub fn shadow_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.shadow_name(inp);
            self
        }
        /// <p>The name of the shadow.</p>
        pub fn set_shadow_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_shadow_name(input);
            self
        }
    }
    /// Fluent builder constructing a request to `ListNamedShadowsForThing`.
    ///
    /// <p>Lists the shadows for the specified thing.</p>
    /// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">ListNamedShadowsForThing</a> action.</p>
    #[derive(std::fmt::Debug)]
    pub struct ListNamedShadowsForThing<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_named_shadows_for_thing_input::Builder,
    }
    impl<C, M, R> ListNamedShadowsForThing<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `ListNamedShadowsForThing`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListNamedShadowsForThingOutput,
            aws_smithy_http::result::SdkError<crate::error::ListNamedShadowsForThingError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListNamedShadowsForThingInputOperationOutputAlias,
                crate::output::ListNamedShadowsForThingOutput,
                crate::error::ListNamedShadowsForThingError,
                crate::input::ListNamedShadowsForThingInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The name of the thing.</p>
        pub fn thing_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.thing_name(inp);
            self
        }
        /// <p>The name of the thing.</p>
        pub fn set_thing_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_thing_name(input);
            self
        }
        /// <p>The token to retrieve the next set of results.</p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        /// <p>The token to retrieve the next set of results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The result page size.</p>
        pub fn page_size(mut self, inp: i32) -> Self {
            self.inner = self.inner.page_size(inp);
            self
        }
        /// <p>The result page size.</p>
        pub fn set_page_size(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_page_size(input);
            self
        }
    }
    /// Fluent builder constructing a request to `ListRetainedMessages`.
    ///
    /// <p>Lists summary information about the retained messages stored for the account.</p>
    /// <p>This action returns only the topic names of the retained messages. It doesn't
    /// return any message payloads. Although this action doesn't return a message payload,
    /// it can still incur messaging costs.</p>
    /// <p>To get the message payload of a retained message, call
    /// <a href="https://docs.aws.amazon.com/iot/latest/developerguide/API_iotdata_GetRetainedMessage.html">GetRetainedMessage</a>
    /// with the topic name of the retained message.</p>
    /// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiotfleethubfordevicemanagement.html#awsiotfleethubfordevicemanagement-actions-as-permissions">ListRetainedMessages</a> action.</p>
    /// <p>For more information about messaging costs, see <a href="http://aws.amazon.com/iot-core/pricing/#Messaging">IoT Core
    /// pricing - Messaging</a>.</p>
    #[derive(std::fmt::Debug)]
    pub struct ListRetainedMessages<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_retained_messages_input::Builder,
    }
    impl<C, M, R> ListRetainedMessages<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `ListRetainedMessages`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListRetainedMessagesOutput,
            aws_smithy_http::result::SdkError<crate::error::ListRetainedMessagesError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListRetainedMessagesInputOperationOutputAlias,
                crate::output::ListRetainedMessagesOutput,
                crate::error::ListRetainedMessagesError,
                crate::input::ListRetainedMessagesInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>To retrieve the next set of results, the <code>nextToken</code>
        /// value from a previous response; otherwise <b>null</b> to receive
        /// the first set of results.</p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        /// <p>To retrieve the next set of results, the <code>nextToken</code>
        /// value from a previous response; otherwise <b>null</b> to receive
        /// the first set of results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The maximum number of results to return at one time.</p>
        pub fn max_results(mut self, inp: i32) -> Self {
            self.inner = self.inner.max_results(inp);
            self
        }
        /// <p>The maximum number of results to return at one time.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
    }
    /// Fluent builder constructing a request to `Publish`.
    ///
    /// <p>Publishes an MQTT message.</p>
    /// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">Publish</a> action.</p>
    /// <p>For more information about MQTT messages, see
    /// <a href="http://docs.aws.amazon.com/iot/latest/developerguide/mqtt.html">MQTT Protocol</a> in the
    /// IoT Developer Guide.</p>
    /// <p>For more information about messaging costs, see <a href="http://aws.amazon.com/iot-core/pricing/#Messaging">IoT Core
    /// pricing - Messaging</a>.</p>
    #[derive(std::fmt::Debug)]
    pub struct Publish<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::publish_input::Builder,
    }
    impl<C, M, R> Publish<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `Publish`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::PublishOutput,
            aws_smithy_http::result::SdkError<crate::error::PublishError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::PublishInputOperationOutputAlias,
                crate::output::PublishOutput,
                crate::error::PublishError,
                crate::input::PublishInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The name of the MQTT topic.</p>
        pub fn topic(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.topic(inp);
            self
        }
        /// <p>The name of the MQTT topic.</p>
        pub fn set_topic(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_topic(input);
            self
        }
        /// <p>The Quality of Service (QoS) level.</p>
        pub fn qos(mut self, inp: i32) -> Self {
            self.inner = self.inner.qos(inp);
            self
        }
        /// <p>The Quality of Service (QoS) level.</p>
        pub fn set_qos(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_qos(input);
            self
        }
        /// <p>A Boolean value that determines whether to set the RETAIN flag when the message is published.</p>
        /// <p>Setting the RETAIN flag causes the message to be retained and sent to new subscribers to the topic.</p>
        /// <p>Valid values: <code>true</code> | <code>false</code>
        /// </p>
        /// <p>Default value: <code>false</code>
        /// </p>
        pub fn retain(mut self, inp: bool) -> Self {
            self.inner = self.inner.retain(inp);
            self
        }
        /// <p>A Boolean value that determines whether to set the RETAIN flag when the message is published.</p>
        /// <p>Setting the RETAIN flag causes the message to be retained and sent to new subscribers to the topic.</p>
        /// <p>Valid values: <code>true</code> | <code>false</code>
        /// </p>
        /// <p>Default value: <code>false</code>
        /// </p>
        pub fn set_retain(mut self, input: std::option::Option<bool>) -> Self {
            self.inner = self.inner.set_retain(input);
            self
        }
        /// <p>The message body. MQTT accepts text, binary, and empty (null) message payloads.</p>
        /// <p>Publishing an empty (null) payload with <b>retain</b> =
        /// <code>true</code> deletes the retained message identified by <b>topic</b> from IoT Core.</p>
        pub fn payload(mut self, inp: aws_smithy_types::Blob) -> Self {
            self.inner = self.inner.payload(inp);
            self
        }
        /// <p>The message body. MQTT accepts text, binary, and empty (null) message payloads.</p>
        /// <p>Publishing an empty (null) payload with <b>retain</b> =
        /// <code>true</code> deletes the retained message identified by <b>topic</b> from IoT Core.</p>
        pub fn set_payload(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
            self.inner = self.inner.set_payload(input);
            self
        }
    }
    /// Fluent builder constructing a request to `UpdateThingShadow`.
    ///
    /// <p>Updates the shadow for the specified thing.</p>
    /// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">UpdateThingShadow</a> action.</p>
    /// <p>For more information, see <a href="http://docs.aws.amazon.com/iot/latest/developerguide/API_UpdateThingShadow.html">UpdateThingShadow</a> in the
    /// IoT Developer Guide.</p>
    #[derive(std::fmt::Debug)]
    pub struct UpdateThingShadow<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::update_thing_shadow_input::Builder,
    }
    impl<C, M, R> UpdateThingShadow<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `UpdateThingShadow`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::UpdateThingShadowOutput,
            aws_smithy_http::result::SdkError<crate::error::UpdateThingShadowError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::UpdateThingShadowInputOperationOutputAlias,
                crate::output::UpdateThingShadowOutput,
                crate::error::UpdateThingShadowError,
                crate::input::UpdateThingShadowInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The name of the thing.</p>
        pub fn thing_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.thing_name(inp);
            self
        }
        /// <p>The name of the thing.</p>
        pub fn set_thing_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_thing_name(input);
            self
        }
        /// <p>The name of the shadow.</p>
        pub fn shadow_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.shadow_name(inp);
            self
        }
        /// <p>The name of the shadow.</p>
        pub fn set_shadow_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_shadow_name(input);
            self
        }
        /// <p>The state information, in JSON format.</p>
        pub fn payload(mut self, inp: aws_smithy_types::Blob) -> Self {
            self.inner = self.inner.payload(inp);
            self
        }
        /// <p>The state information, in JSON format.</p>
        pub fn set_payload(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
            self.inner = self.inner.set_payload(input);
            self
        }
    }
}
impl<C> Client<C, aws_hyper::AwsMiddleware, aws_smithy_client::retry::Standard> {
    /// Creates a client with the given service config and connector override.
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let client = aws_hyper::Client::new(conn).with_retry_config(retry_config.into());
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl
    Client<
        aws_smithy_client::erase::DynConnector,
        aws_hyper::AwsMiddleware,
        aws_smithy_client::retry::Standard,
    >
{
    /// Creates a new client from a shared config.
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(config: &aws_types::config::Config) -> Self {
        Self::from_conf(config.into())
    }

    /// Creates a new client from the service [`Config`](crate::Config).
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let client = aws_hyper::Client::https().with_retry_config(retry_config.into());
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}