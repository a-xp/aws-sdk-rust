// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>The specified activity does not exist.</p>
    ActivityDoesNotExist(crate::error::ActivityDoesNotExist),
    /// <p>The maximum number of activities has been reached. Existing activities must be deleted
    /// before a new activity can be created.</p>
    ActivityLimitExceeded(crate::error::ActivityLimitExceeded),
    /// <p>The maximum number of workers concurrently polling for activity tasks has been
    /// reached.</p>
    ActivityWorkerLimitExceeded(crate::error::ActivityWorkerLimitExceeded),
    /// <p>The execution has the same <code>name</code> as another execution (but a different
    /// <code>input</code>).</p>
    /// <note>
    /// <p>Executions with the same <code>name</code> and <code>input</code> are considered
    /// idempotent.</p>
    /// </note>
    ExecutionAlreadyExists(crate::error::ExecutionAlreadyExists),
    /// <p>The specified execution does not exist.</p>
    ExecutionDoesNotExist(crate::error::ExecutionDoesNotExist),
    /// <p>The maximum number of running executions has been reached. Running executions must end or
    /// be stopped before a new execution can be started.</p>
    ExecutionLimitExceeded(crate::error::ExecutionLimitExceeded),
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(crate::error::InvalidArn),
    /// <p>The provided Amazon States Language definition is invalid.</p>
    InvalidDefinition(crate::error::InvalidDefinition),
    /// <p>The provided JSON input data is invalid.</p>
    InvalidExecutionInput(crate::error::InvalidExecutionInput),
    /// <p></p>
    InvalidLoggingConfiguration(crate::error::InvalidLoggingConfiguration),
    /// <p>The provided name is invalid.</p>
    InvalidName(crate::error::InvalidName),
    /// <p>The provided JSON output data is invalid.</p>
    InvalidOutput(crate::error::InvalidOutput),
    /// <p>The provided token is invalid.</p>
    InvalidToken(crate::error::InvalidToken),
    /// <p>Your <code>tracingConfiguration</code> key does not match, or <code>enabled</code> has not been set to <code>true</code> or <code>false</code>.</p>
    InvalidTracingConfiguration(crate::error::InvalidTracingConfiguration),
    /// <p>Request is missing a required parameter. This error occurs if both <code>definition</code>
    /// and <code>roleArn</code> are not specified.</p>
    MissingRequiredParameter(crate::error::MissingRequiredParameter),
    /// <p>Could not find the referenced resource. Only state machine and activity ARNs are
    /// supported.</p>
    ResourceNotFound(crate::error::ResourceNotFound),
    /// <p>A state machine with the same name but a different definition or role ARN already
    /// exists.</p>
    StateMachineAlreadyExists(crate::error::StateMachineAlreadyExists),
    /// <p>The specified state machine is being deleted.</p>
    StateMachineDeleting(crate::error::StateMachineDeleting),
    /// <p>The specified state machine does not exist.</p>
    StateMachineDoesNotExist(crate::error::StateMachineDoesNotExist),
    /// <p>The maximum number of state machines has been reached. Existing state machines must be
    /// deleted before a new state machine can be created.</p>
    StateMachineLimitExceeded(crate::error::StateMachineLimitExceeded),
    /// <p></p>
    StateMachineTypeNotSupported(crate::error::StateMachineTypeNotSupported),
    #[allow(missing_docs)] // documentation missing in model
    TaskDoesNotExist(crate::error::TaskDoesNotExist),
    #[allow(missing_docs)] // documentation missing in model
    TaskTimedOut(crate::error::TaskTimedOut),
    /// <p>You've exceeded the number of tags allowed for a resource. See the <a href="https://docs.aws.amazon.com/step-functions/latest/dg/limits.html"> Limits Topic</a> in the
    /// AWS Step Functions Developer Guide.</p>
    TooManyTags(crate::error::TooManyTags),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ActivityDoesNotExist(inner) => inner.fmt(f),
            Error::ActivityLimitExceeded(inner) => inner.fmt(f),
            Error::ActivityWorkerLimitExceeded(inner) => inner.fmt(f),
            Error::ExecutionAlreadyExists(inner) => inner.fmt(f),
            Error::ExecutionDoesNotExist(inner) => inner.fmt(f),
            Error::ExecutionLimitExceeded(inner) => inner.fmt(f),
            Error::InvalidArn(inner) => inner.fmt(f),
            Error::InvalidDefinition(inner) => inner.fmt(f),
            Error::InvalidExecutionInput(inner) => inner.fmt(f),
            Error::InvalidLoggingConfiguration(inner) => inner.fmt(f),
            Error::InvalidName(inner) => inner.fmt(f),
            Error::InvalidOutput(inner) => inner.fmt(f),
            Error::InvalidToken(inner) => inner.fmt(f),
            Error::InvalidTracingConfiguration(inner) => inner.fmt(f),
            Error::MissingRequiredParameter(inner) => inner.fmt(f),
            Error::ResourceNotFound(inner) => inner.fmt(f),
            Error::StateMachineAlreadyExists(inner) => inner.fmt(f),
            Error::StateMachineDeleting(inner) => inner.fmt(f),
            Error::StateMachineDoesNotExist(inner) => inner.fmt(f),
            Error::StateMachineLimitExceeded(inner) => inner.fmt(f),
            Error::StateMachineTypeNotSupported(inner) => inner.fmt(f),
            Error::TaskDoesNotExist(inner) => inner.fmt(f),
            Error::TaskTimedOut(inner) => inner.fmt(f),
            Error::TooManyTags(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateActivityError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateActivityError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateActivityErrorKind::ActivityLimitExceeded(inner) => {
                    Error::ActivityLimitExceeded(inner)
                }
                crate::error::CreateActivityErrorKind::InvalidName(inner) => {
                    Error::InvalidName(inner)
                }
                crate::error::CreateActivityErrorKind::TooManyTags(inner) => {
                    Error::TooManyTags(inner)
                }
                crate::error::CreateActivityErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateStateMachineError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateStateMachineError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateStateMachineErrorKind::InvalidArn(inner) => {
                    Error::InvalidArn(inner)
                }
                crate::error::CreateStateMachineErrorKind::InvalidDefinition(inner) => {
                    Error::InvalidDefinition(inner)
                }
                crate::error::CreateStateMachineErrorKind::InvalidLoggingConfiguration(inner) => {
                    Error::InvalidLoggingConfiguration(inner)
                }
                crate::error::CreateStateMachineErrorKind::InvalidName(inner) => {
                    Error::InvalidName(inner)
                }
                crate::error::CreateStateMachineErrorKind::InvalidTracingConfiguration(inner) => {
                    Error::InvalidTracingConfiguration(inner)
                }
                crate::error::CreateStateMachineErrorKind::StateMachineAlreadyExists(inner) => {
                    Error::StateMachineAlreadyExists(inner)
                }
                crate::error::CreateStateMachineErrorKind::StateMachineDeleting(inner) => {
                    Error::StateMachineDeleting(inner)
                }
                crate::error::CreateStateMachineErrorKind::StateMachineLimitExceeded(inner) => {
                    Error::StateMachineLimitExceeded(inner)
                }
                crate::error::CreateStateMachineErrorKind::StateMachineTypeNotSupported(inner) => {
                    Error::StateMachineTypeNotSupported(inner)
                }
                crate::error::CreateStateMachineErrorKind::TooManyTags(inner) => {
                    Error::TooManyTags(inner)
                }
                crate::error::CreateStateMachineErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteActivityError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteActivityError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteActivityErrorKind::InvalidArn(inner) => {
                    Error::InvalidArn(inner)
                }
                crate::error::DeleteActivityErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteStateMachineError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteStateMachineError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteStateMachineErrorKind::InvalidArn(inner) => {
                    Error::InvalidArn(inner)
                }
                crate::error::DeleteStateMachineErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeActivityError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeActivityError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeActivityErrorKind::ActivityDoesNotExist(inner) => {
                    Error::ActivityDoesNotExist(inner)
                }
                crate::error::DescribeActivityErrorKind::InvalidArn(inner) => {
                    Error::InvalidArn(inner)
                }
                crate::error::DescribeActivityErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeExecutionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeExecutionError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeExecutionErrorKind::ExecutionDoesNotExist(inner) => {
                    Error::ExecutionDoesNotExist(inner)
                }
                crate::error::DescribeExecutionErrorKind::InvalidArn(inner) => {
                    Error::InvalidArn(inner)
                }
                crate::error::DescribeExecutionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeStateMachineError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeStateMachineError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeStateMachineErrorKind::InvalidArn(inner) => {
                    Error::InvalidArn(inner)
                }
                crate::error::DescribeStateMachineErrorKind::StateMachineDoesNotExist(inner) => {
                    Error::StateMachineDoesNotExist(inner)
                }
                crate::error::DescribeStateMachineErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::DescribeStateMachineForExecutionError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::DescribeStateMachineForExecutionError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeStateMachineForExecutionErrorKind::ExecutionDoesNotExist(
                    inner,
                ) => Error::ExecutionDoesNotExist(inner),
                crate::error::DescribeStateMachineForExecutionErrorKind::InvalidArn(inner) => {
                    Error::InvalidArn(inner)
                }
                crate::error::DescribeStateMachineForExecutionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetActivityTaskError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetActivityTaskError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetActivityTaskErrorKind::ActivityDoesNotExist(inner) => {
                    Error::ActivityDoesNotExist(inner)
                }
                crate::error::GetActivityTaskErrorKind::ActivityWorkerLimitExceeded(inner) => {
                    Error::ActivityWorkerLimitExceeded(inner)
                }
                crate::error::GetActivityTaskErrorKind::InvalidArn(inner) => {
                    Error::InvalidArn(inner)
                }
                crate::error::GetActivityTaskErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetExecutionHistoryError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetExecutionHistoryError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetExecutionHistoryErrorKind::ExecutionDoesNotExist(inner) => {
                    Error::ExecutionDoesNotExist(inner)
                }
                crate::error::GetExecutionHistoryErrorKind::InvalidArn(inner) => {
                    Error::InvalidArn(inner)
                }
                crate::error::GetExecutionHistoryErrorKind::InvalidToken(inner) => {
                    Error::InvalidToken(inner)
                }
                crate::error::GetExecutionHistoryErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListActivitiesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListActivitiesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListActivitiesErrorKind::InvalidToken(inner) => {
                    Error::InvalidToken(inner)
                }
                crate::error::ListActivitiesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListExecutionsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListExecutionsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListExecutionsErrorKind::InvalidArn(inner) => {
                    Error::InvalidArn(inner)
                }
                crate::error::ListExecutionsErrorKind::InvalidToken(inner) => {
                    Error::InvalidToken(inner)
                }
                crate::error::ListExecutionsErrorKind::StateMachineDoesNotExist(inner) => {
                    Error::StateMachineDoesNotExist(inner)
                }
                crate::error::ListExecutionsErrorKind::StateMachineTypeNotSupported(inner) => {
                    Error::StateMachineTypeNotSupported(inner)
                }
                crate::error::ListExecutionsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListStateMachinesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListStateMachinesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListStateMachinesErrorKind::InvalidToken(inner) => {
                    Error::InvalidToken(inner)
                }
                crate::error::ListStateMachinesErrorKind::Unhandled(inner) => {
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
                crate::error::ListTagsForResourceErrorKind::InvalidArn(inner) => {
                    Error::InvalidArn(inner)
                }
                crate::error::ListTagsForResourceErrorKind::ResourceNotFound(inner) => {
                    Error::ResourceNotFound(inner)
                }
                crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SendTaskFailureError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::SendTaskFailureError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SendTaskFailureErrorKind::InvalidToken(inner) => {
                    Error::InvalidToken(inner)
                }
                crate::error::SendTaskFailureErrorKind::TaskDoesNotExist(inner) => {
                    Error::TaskDoesNotExist(inner)
                }
                crate::error::SendTaskFailureErrorKind::TaskTimedOut(inner) => {
                    Error::TaskTimedOut(inner)
                }
                crate::error::SendTaskFailureErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SendTaskHeartbeatError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::SendTaskHeartbeatError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SendTaskHeartbeatErrorKind::InvalidToken(inner) => {
                    Error::InvalidToken(inner)
                }
                crate::error::SendTaskHeartbeatErrorKind::TaskDoesNotExist(inner) => {
                    Error::TaskDoesNotExist(inner)
                }
                crate::error::SendTaskHeartbeatErrorKind::TaskTimedOut(inner) => {
                    Error::TaskTimedOut(inner)
                }
                crate::error::SendTaskHeartbeatErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SendTaskSuccessError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::SendTaskSuccessError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SendTaskSuccessErrorKind::InvalidOutput(inner) => {
                    Error::InvalidOutput(inner)
                }
                crate::error::SendTaskSuccessErrorKind::InvalidToken(inner) => {
                    Error::InvalidToken(inner)
                }
                crate::error::SendTaskSuccessErrorKind::TaskDoesNotExist(inner) => {
                    Error::TaskDoesNotExist(inner)
                }
                crate::error::SendTaskSuccessErrorKind::TaskTimedOut(inner) => {
                    Error::TaskTimedOut(inner)
                }
                crate::error::SendTaskSuccessErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartExecutionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::StartExecutionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StartExecutionErrorKind::ExecutionAlreadyExists(inner) => {
                    Error::ExecutionAlreadyExists(inner)
                }
                crate::error::StartExecutionErrorKind::ExecutionLimitExceeded(inner) => {
                    Error::ExecutionLimitExceeded(inner)
                }
                crate::error::StartExecutionErrorKind::InvalidArn(inner) => {
                    Error::InvalidArn(inner)
                }
                crate::error::StartExecutionErrorKind::InvalidExecutionInput(inner) => {
                    Error::InvalidExecutionInput(inner)
                }
                crate::error::StartExecutionErrorKind::InvalidName(inner) => {
                    Error::InvalidName(inner)
                }
                crate::error::StartExecutionErrorKind::StateMachineDeleting(inner) => {
                    Error::StateMachineDeleting(inner)
                }
                crate::error::StartExecutionErrorKind::StateMachineDoesNotExist(inner) => {
                    Error::StateMachineDoesNotExist(inner)
                }
                crate::error::StartExecutionErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartSyncExecutionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::StartSyncExecutionError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StartSyncExecutionErrorKind::InvalidArn(inner) => {
                    Error::InvalidArn(inner)
                }
                crate::error::StartSyncExecutionErrorKind::InvalidExecutionInput(inner) => {
                    Error::InvalidExecutionInput(inner)
                }
                crate::error::StartSyncExecutionErrorKind::InvalidName(inner) => {
                    Error::InvalidName(inner)
                }
                crate::error::StartSyncExecutionErrorKind::StateMachineDeleting(inner) => {
                    Error::StateMachineDeleting(inner)
                }
                crate::error::StartSyncExecutionErrorKind::StateMachineDoesNotExist(inner) => {
                    Error::StateMachineDoesNotExist(inner)
                }
                crate::error::StartSyncExecutionErrorKind::StateMachineTypeNotSupported(inner) => {
                    Error::StateMachineTypeNotSupported(inner)
                }
                crate::error::StartSyncExecutionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StopExecutionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::StopExecutionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StopExecutionErrorKind::ExecutionDoesNotExist(inner) => {
                    Error::ExecutionDoesNotExist(inner)
                }
                crate::error::StopExecutionErrorKind::InvalidArn(inner) => Error::InvalidArn(inner),
                crate::error::StopExecutionErrorKind::Unhandled(inner) => Error::Unhandled(inner),
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
                crate::error::TagResourceErrorKind::InvalidArn(inner) => Error::InvalidArn(inner),
                crate::error::TagResourceErrorKind::ResourceNotFound(inner) => {
                    Error::ResourceNotFound(inner)
                }
                crate::error::TagResourceErrorKind::TooManyTags(inner) => Error::TooManyTags(inner),
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
                crate::error::UntagResourceErrorKind::InvalidArn(inner) => Error::InvalidArn(inner),
                crate::error::UntagResourceErrorKind::ResourceNotFound(inner) => {
                    Error::ResourceNotFound(inner)
                }
                crate::error::UntagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateStateMachineError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateStateMachineError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateStateMachineErrorKind::InvalidArn(inner) => {
                    Error::InvalidArn(inner)
                }
                crate::error::UpdateStateMachineErrorKind::InvalidDefinition(inner) => {
                    Error::InvalidDefinition(inner)
                }
                crate::error::UpdateStateMachineErrorKind::InvalidLoggingConfiguration(inner) => {
                    Error::InvalidLoggingConfiguration(inner)
                }
                crate::error::UpdateStateMachineErrorKind::InvalidTracingConfiguration(inner) => {
                    Error::InvalidTracingConfiguration(inner)
                }
                crate::error::UpdateStateMachineErrorKind::MissingRequiredParameter(inner) => {
                    Error::MissingRequiredParameter(inner)
                }
                crate::error::UpdateStateMachineErrorKind::StateMachineDeleting(inner) => {
                    Error::StateMachineDeleting(inner)
                }
                crate::error::UpdateStateMachineErrorKind::StateMachineDoesNotExist(inner) => {
                    Error::StateMachineDoesNotExist(inner)
                }
                crate::error::UpdateStateMachineErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
