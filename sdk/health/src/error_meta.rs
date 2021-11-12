// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>
    /// <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_EnableHealthServiceAccessForOrganization.html">EnableHealthServiceAccessForOrganization</a> is already in progress. Wait for the
    /// action to complete before trying again. To get the current status, use the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeHealthServiceStatusForOrganization.html">DescribeHealthServiceStatusForOrganization</a> operation.</p>
    ConcurrentModificationException(crate::error::ConcurrentModificationException),
    /// <p>The specified pagination token (<code>nextToken</code>) is not valid.</p>
    InvalidPaginationToken(crate::error::InvalidPaginationToken),
    /// <p>The specified locale is not supported.</p>
    UnsupportedLocale(crate::error::UnsupportedLocale),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConcurrentModificationException(inner) => inner.fmt(f),
            Error::InvalidPaginationToken(inner) => inner.fmt(f),
            Error::UnsupportedLocale(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::DescribeAffectedAccountsForOrganizationError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::DescribeAffectedAccountsForOrganizationError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DescribeAffectedAccountsForOrganizationErrorKind::InvalidPaginationToken(inner) => Error::InvalidPaginationToken(inner),
                crate::error::DescribeAffectedAccountsForOrganizationErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeAffectedEntitiesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeAffectedEntitiesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeAffectedEntitiesErrorKind::InvalidPaginationToken(inner) => {
                    Error::InvalidPaginationToken(inner)
                }
                crate::error::DescribeAffectedEntitiesErrorKind::UnsupportedLocale(inner) => {
                    Error::UnsupportedLocale(inner)
                }
                crate::error::DescribeAffectedEntitiesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::DescribeAffectedEntitiesForOrganizationError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::DescribeAffectedEntitiesForOrganizationError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DescribeAffectedEntitiesForOrganizationErrorKind::InvalidPaginationToken(inner) => Error::InvalidPaginationToken(inner),
                crate::error::DescribeAffectedEntitiesForOrganizationErrorKind::UnsupportedLocale(inner) => Error::UnsupportedLocale(inner),
                crate::error::DescribeAffectedEntitiesForOrganizationErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeEntityAggregatesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeEntityAggregatesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeEntityAggregatesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeEventAggregatesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeEventAggregatesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeEventAggregatesErrorKind::InvalidPaginationToken(inner) => {
                    Error::InvalidPaginationToken(inner)
                }
                crate::error::DescribeEventAggregatesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeEventDetailsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeEventDetailsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeEventDetailsErrorKind::UnsupportedLocale(inner) => {
                    Error::UnsupportedLocale(inner)
                }
                crate::error::DescribeEventDetailsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::DescribeEventDetailsForOrganizationError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::DescribeEventDetailsForOrganizationError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeEventDetailsForOrganizationErrorKind::UnsupportedLocale(
                    inner,
                ) => Error::UnsupportedLocale(inner),
                crate::error::DescribeEventDetailsForOrganizationErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeEventsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeEventsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeEventsErrorKind::InvalidPaginationToken(inner) => {
                    Error::InvalidPaginationToken(inner)
                }
                crate::error::DescribeEventsErrorKind::UnsupportedLocale(inner) => {
                    Error::UnsupportedLocale(inner)
                }
                crate::error::DescribeEventsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeEventsForOrganizationError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeEventsForOrganizationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeEventsForOrganizationErrorKind::InvalidPaginationToken(
                    inner,
                ) => Error::InvalidPaginationToken(inner),
                crate::error::DescribeEventsForOrganizationErrorKind::UnsupportedLocale(inner) => {
                    Error::UnsupportedLocale(inner)
                }
                crate::error::DescribeEventsForOrganizationErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeEventTypesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeEventTypesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeEventTypesErrorKind::InvalidPaginationToken(inner) => {
                    Error::InvalidPaginationToken(inner)
                }
                crate::error::DescribeEventTypesErrorKind::UnsupportedLocale(inner) => {
                    Error::UnsupportedLocale(inner)
                }
                crate::error::DescribeEventTypesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::DescribeHealthServiceStatusForOrganizationError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::DescribeHealthServiceStatusForOrganizationError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeHealthServiceStatusForOrganizationErrorKind::Unhandled(
                    inner,
                ) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::DisableHealthServiceAccessForOrganizationError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::DisableHealthServiceAccessForOrganizationError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DisableHealthServiceAccessForOrganizationErrorKind::ConcurrentModificationException(inner) => Error::ConcurrentModificationException(inner),
                crate::error::DisableHealthServiceAccessForOrganizationErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::EnableHealthServiceAccessForOrganizationError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::EnableHealthServiceAccessForOrganizationError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::EnableHealthServiceAccessForOrganizationErrorKind::ConcurrentModificationException(inner) => Error::ConcurrentModificationException(inner),
                crate::error::EnableHealthServiceAccessForOrganizationErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}