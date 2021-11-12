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

/// An ergonomic service client for `AWSHabaneroPublicAPI`.
///
/// This client allows ergonomic access to a `AWSHabaneroPublicAPI`-shaped service.
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
    /// Constructs a fluent builder for the `CreateChangeset` operation.
    ///
    /// See [`CreateChangeset`](crate::client::fluent_builders::CreateChangeset) for more information about the
    /// operation and its arguments.
    pub fn create_changeset(&self) -> fluent_builders::CreateChangeset<C, M, R> {
        fluent_builders::CreateChangeset::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `GetProgrammaticAccessCredentials` operation.
    ///
    /// See [`GetProgrammaticAccessCredentials`](crate::client::fluent_builders::GetProgrammaticAccessCredentials) for more information about the
    /// operation and its arguments.
    pub fn get_programmatic_access_credentials(
        &self,
    ) -> fluent_builders::GetProgrammaticAccessCredentials<C, M, R> {
        fluent_builders::GetProgrammaticAccessCredentials::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `GetWorkingLocation` operation.
    ///
    /// See [`GetWorkingLocation`](crate::client::fluent_builders::GetWorkingLocation) for more information about the
    /// operation and its arguments.
    pub fn get_working_location(&self) -> fluent_builders::GetWorkingLocation<C, M, R> {
        fluent_builders::GetWorkingLocation::new(self.handle.clone())
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
    /// Fluent builder constructing a request to `CreateChangeset`.
    ///
    /// <p>Creates a new changeset in a FinSpace dataset.</p>
    #[derive(std::fmt::Debug)]
    pub struct CreateChangeset<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::create_changeset_input::Builder,
    }
    impl<C, M, R> CreateChangeset<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `CreateChangeset`.
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
            crate::output::CreateChangesetOutput,
            aws_smithy_http::result::SdkError<crate::error::CreateChangesetError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::CreateChangesetInputOperationOutputAlias,
                crate::output::CreateChangesetOutput,
                crate::error::CreateChangesetError,
                crate::input::CreateChangesetInputOperationRetryAlias,
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
        /// <p>The unique identifier for the FinSpace dataset in which the changeset will be
        /// created.</p>
        pub fn dataset_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.dataset_id(inp);
            self
        }
        /// <p>The unique identifier for the FinSpace dataset in which the changeset will be
        /// created.</p>
        pub fn set_dataset_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_dataset_id(input);
            self
        }
        /// <p>Option to indicate how a changeset will be applied to a dataset.</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>REPLACE</code> - Changeset will be considered as a replacement to all prior
        /// loaded changesets.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>APPEND</code> - Changeset will be considered as an addition to the end of all
        /// prior loaded changesets.</p>
        /// </li>
        /// </ul>
        pub fn change_type(mut self, inp: crate::model::ChangeType) -> Self {
            self.inner = self.inner.change_type(inp);
            self
        }
        /// <p>Option to indicate how a changeset will be applied to a dataset.</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>REPLACE</code> - Changeset will be considered as a replacement to all prior
        /// loaded changesets.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>APPEND</code> - Changeset will be considered as an addition to the end of all
        /// prior loaded changesets.</p>
        /// </li>
        /// </ul>
        pub fn set_change_type(
            mut self,
            input: std::option::Option<crate::model::ChangeType>,
        ) -> Self {
            self.inner = self.inner.set_change_type(input);
            self
        }
        /// <p>Type of the data source from which the files to create the changeset will be
        /// sourced.</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>S3</code> - Amazon S3.</p>
        /// </li>
        /// </ul>
        pub fn source_type(mut self, inp: crate::model::SourceType) -> Self {
            self.inner = self.inner.source_type(inp);
            self
        }
        /// <p>Type of the data source from which the files to create the changeset will be
        /// sourced.</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>S3</code> - Amazon S3.</p>
        /// </li>
        /// </ul>
        pub fn set_source_type(
            mut self,
            input: std::option::Option<crate::model::SourceType>,
        ) -> Self {
            self.inner = self.inner.set_source_type(input);
            self
        }
        /// Adds a key-value pair to `sourceParams`.
        ///
        /// To override the contents of this collection use [`set_source_params`](Self::set_source_params).
        ///
        /// <p>Source path from which the files to create the changeset will be sourced.</p>
        pub fn source_params(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.source_params(k, v);
            self
        }
        /// <p>Source path from which the files to create the changeset will be sourced.</p>
        pub fn set_source_params(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_source_params(input);
            self
        }
        /// <p>Format type of the input files being loaded into the changeset.</p>
        pub fn format_type(mut self, inp: crate::model::FormatType) -> Self {
            self.inner = self.inner.format_type(inp);
            self
        }
        /// <p>Format type of the input files being loaded into the changeset.</p>
        pub fn set_format_type(
            mut self,
            input: std::option::Option<crate::model::FormatType>,
        ) -> Self {
            self.inner = self.inner.set_format_type(input);
            self
        }
        /// Adds a key-value pair to `formatParams`.
        ///
        /// To override the contents of this collection use [`set_format_params`](Self::set_format_params).
        ///
        /// <p>Options that define the structure of the source file(s).</p>
        pub fn format_params(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.format_params(k, v);
            self
        }
        /// <p>Options that define the structure of the source file(s).</p>
        pub fn set_format_params(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_format_params(input);
            self
        }
        /// Adds a key-value pair to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>Metadata tags to apply to this changeset.</p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.tags(k, v);
            self
        }
        /// <p>Metadata tags to apply to this changeset.</p>
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_tags(input);
            self
        }
    }
    /// Fluent builder constructing a request to `GetProgrammaticAccessCredentials`.
    ///
    /// <p>Request programmatic credentials to use with Habanero SDK.</p>
    #[derive(std::fmt::Debug)]
    pub struct GetProgrammaticAccessCredentials<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_programmatic_access_credentials_input::Builder,
    }
    impl<C, M, R> GetProgrammaticAccessCredentials<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `GetProgrammaticAccessCredentials`.
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
            crate::output::GetProgrammaticAccessCredentialsOutput,
            aws_smithy_http::result::SdkError<crate::error::GetProgrammaticAccessCredentialsError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetProgrammaticAccessCredentialsInputOperationOutputAlias,
                crate::output::GetProgrammaticAccessCredentialsOutput,
                crate::error::GetProgrammaticAccessCredentialsError,
                crate::input::GetProgrammaticAccessCredentialsInputOperationRetryAlias,
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
        /// <p>The time duration in which the credentials remain valid. </p>
        pub fn duration_in_minutes(mut self, inp: i64) -> Self {
            self.inner = self.inner.duration_in_minutes(inp);
            self
        }
        /// <p>The time duration in which the credentials remain valid. </p>
        pub fn set_duration_in_minutes(mut self, input: std::option::Option<i64>) -> Self {
            self.inner = self.inner.set_duration_in_minutes(input);
            self
        }
        /// <p>The habanero environment identifier.</p>
        pub fn environment_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.environment_id(inp);
            self
        }
        /// <p>The habanero environment identifier.</p>
        pub fn set_environment_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_environment_id(input);
            self
        }
    }
    /// Fluent builder constructing a request to `GetWorkingLocation`.
    ///
    /// <p>A temporary Amazon S3 location to copy your files from a source location to stage or use
    /// as a scratch space in Habanero notebook.</p>
    #[derive(std::fmt::Debug)]
    pub struct GetWorkingLocation<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_working_location_input::Builder,
    }
    impl<C, M, R> GetWorkingLocation<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `GetWorkingLocation`.
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
            crate::output::GetWorkingLocationOutput,
            aws_smithy_http::result::SdkError<crate::error::GetWorkingLocationError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetWorkingLocationInputOperationOutputAlias,
                crate::output::GetWorkingLocationOutput,
                crate::error::GetWorkingLocationError,
                crate::input::GetWorkingLocationInputOperationRetryAlias,
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
        /// <p>Specify the type of the working location.</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>SAGEMAKER</code> - Use the Amazon S3 location as a temporary location to store data content when
        /// working with FinSpace Notebooks that run on SageMaker studio.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>INGESTION</code> - Use the Amazon S3 location as a staging location to copy your
        /// data content and then use the location with the changeset creation operation.</p>
        /// </li>
        /// </ul>
        pub fn location_type(mut self, inp: crate::model::LocationType) -> Self {
            self.inner = self.inner.location_type(inp);
            self
        }
        /// <p>Specify the type of the working location.</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>SAGEMAKER</code> - Use the Amazon S3 location as a temporary location to store data content when
        /// working with FinSpace Notebooks that run on SageMaker studio.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>INGESTION</code> - Use the Amazon S3 location as a staging location to copy your
        /// data content and then use the location with the changeset creation operation.</p>
        /// </li>
        /// </ul>
        pub fn set_location_type(
            mut self,
            input: std::option::Option<crate::model::LocationType>,
        ) -> Self {
            self.inner = self.inner.set_location_type(input);
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