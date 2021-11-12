#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p>Welcome to the AWS Device Farm API documentation, which contains APIs for:</p>
//! <ul>
//! <li>
//! <p>Testing on desktop browsers</p>
//! <p> Device Farm makes it possible for you to test your web applications on desktop browsers using
//! Selenium. The APIs for desktop browser testing contain <code>TestGrid</code> in their names. For
//! more information, see <a href="https://docs.aws.amazon.com/devicefarm/latest/testgrid/">Testing Web
//! Applications on Selenium with Device Farm</a>.</p>
//! </li>
//! <li>
//! <p>Testing on real mobile devices</p>
//! <p>Device Farm makes it possible for you to test apps on physical phones, tablets, and other
//! devices in the cloud. For more information, see the <a href="https://docs.aws.amazon.com/devicefarm/latest/developerguide/">Device Farm Developer Guide</a>.</p>
//! </li>
//! </ul>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
/// Client and fluent builders for calling the service.
#[cfg(feature = "client")]
pub mod client;
/// Configuration for the service.
pub mod config;
/// Errors that can occur when calling the service.
pub mod error;
mod error_meta;
/// Input structures for operations.
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
/// Data structures used by operation inputs/outputs.
pub mod model;
mod no_credentials;
/// All operations that this crate can perform.
pub mod operation;
mod operation_deser;
mod operation_ser;
/// Output structures for operations.
pub mod output;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use aws_smithy_http::byte_stream::ByteStream;
pub use aws_smithy_http::result::SdkError;
pub use aws_smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("devicefarm", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;