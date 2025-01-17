// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>The service failed in an unexpected way.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>Bad request. The request is missing required parameters or has invalid
    /// parameters.</p>
    InvalidRequestException(crate::error::InvalidRequestException),
    /// <p>The request failed because a limit was exceeded.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// <p>A requested resource was not found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::InvalidRequestException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateLifecyclePolicyError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateLifecyclePolicyError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateLifecyclePolicyErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::CreateLifecyclePolicyErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::CreateLifecyclePolicyErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::CreateLifecyclePolicyErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteLifecyclePolicyError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteLifecyclePolicyError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteLifecyclePolicyErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DeleteLifecyclePolicyErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::DeleteLifecyclePolicyErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteLifecyclePolicyErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetLifecyclePoliciesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetLifecyclePoliciesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetLifecyclePoliciesErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::GetLifecyclePoliciesErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::GetLifecyclePoliciesErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::GetLifecyclePoliciesErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::GetLifecyclePoliciesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetLifecyclePolicyError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetLifecyclePolicyError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetLifecyclePolicyErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::GetLifecyclePolicyErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::GetLifecyclePolicyErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::GetLifecyclePolicyErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListTagsForResourceErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::TagResourceErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::TagResourceErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::TagResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::TagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UntagResourceErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::UntagResourceErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UntagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateLifecyclePolicyError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateLifecyclePolicyError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateLifecyclePolicyErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::UpdateLifecyclePolicyErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::UpdateLifecyclePolicyErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::UpdateLifecyclePolicyErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UpdateLifecyclePolicyErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
