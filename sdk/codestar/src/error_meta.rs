// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>Another modification is being made. That modification must complete before you can make
    /// your change.</p>
    ConcurrentModificationException(crate::error::ConcurrentModificationException),
    /// <p>The next token is not valid.</p>
    InvalidNextTokenException(crate::error::InvalidNextTokenException),
    /// <p>The service role is not valid.</p>
    InvalidServiceRoleException(crate::error::InvalidServiceRoleException),
    /// <p>A resource limit has been exceeded.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// <p>An AWS CodeStar project with the same ID already exists in this region for the AWS account.
    /// AWS CodeStar project IDs must be unique within a region for the AWS account.</p>
    ProjectAlreadyExistsException(crate::error::ProjectAlreadyExistsException),
    /// <p>Project configuration information is required but not specified.</p>
    ProjectConfigurationException(crate::error::ProjectConfigurationException),
    /// <p>The project creation request was valid, but a nonspecific exception or error occurred
    /// during project creation. The project could not be created in AWS CodeStar.</p>
    ProjectCreationFailedException(crate::error::ProjectCreationFailedException),
    /// <p>The specified AWS CodeStar project was not found.</p>
    ProjectNotFoundException(crate::error::ProjectNotFoundException),
    /// <p>The team member is already associated with a role in this project.</p>
    TeamMemberAlreadyAssociatedException(crate::error::TeamMemberAlreadyAssociatedException),
    /// <p>The specified team member was not found.</p>
    TeamMemberNotFoundException(crate::error::TeamMemberNotFoundException),
    /// <p>A user profile with that name already exists in this region for the AWS account. AWS
    /// CodeStar user profile names must be unique within a region for the AWS account. </p>
    UserProfileAlreadyExistsException(crate::error::UserProfileAlreadyExistsException),
    /// <p>The user profile was not found.</p>
    UserProfileNotFoundException(crate::error::UserProfileNotFoundException),
    /// <p>The specified input is either not valid, or it could not be validated.</p>
    ValidationException(crate::error::ValidationException),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConcurrentModificationException(inner) => inner.fmt(f),
            Error::InvalidNextTokenException(inner) => inner.fmt(f),
            Error::InvalidServiceRoleException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::ProjectAlreadyExistsException(inner) => inner.fmt(f),
            Error::ProjectConfigurationException(inner) => inner.fmt(f),
            Error::ProjectCreationFailedException(inner) => inner.fmt(f),
            Error::ProjectNotFoundException(inner) => inner.fmt(f),
            Error::TeamMemberAlreadyAssociatedException(inner) => inner.fmt(f),
            Error::TeamMemberNotFoundException(inner) => inner.fmt(f),
            Error::UserProfileAlreadyExistsException(inner) => inner.fmt(f),
            Error::UserProfileNotFoundException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AssociateTeamMemberError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::AssociateTeamMemberError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::AssociateTeamMemberErrorKind::ConcurrentModificationException(inner) => Error::ConcurrentModificationException(inner),
                crate::error::AssociateTeamMemberErrorKind::InvalidServiceRoleException(inner) => Error::InvalidServiceRoleException(inner),
                crate::error::AssociateTeamMemberErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
                crate::error::AssociateTeamMemberErrorKind::ProjectConfigurationException(inner) => Error::ProjectConfigurationException(inner),
                crate::error::AssociateTeamMemberErrorKind::ProjectNotFoundException(inner) => Error::ProjectNotFoundException(inner),
                crate::error::AssociateTeamMemberErrorKind::TeamMemberAlreadyAssociatedException(inner) => Error::TeamMemberAlreadyAssociatedException(inner),
                crate::error::AssociateTeamMemberErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::AssociateTeamMemberErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateProjectError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateProjectError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateProjectErrorKind::ConcurrentModificationException(inner) => {
                    Error::ConcurrentModificationException(inner)
                }
                crate::error::CreateProjectErrorKind::InvalidServiceRoleException(inner) => {
                    Error::InvalidServiceRoleException(inner)
                }
                crate::error::CreateProjectErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::CreateProjectErrorKind::ProjectAlreadyExistsException(inner) => {
                    Error::ProjectAlreadyExistsException(inner)
                }
                crate::error::CreateProjectErrorKind::ProjectConfigurationException(inner) => {
                    Error::ProjectConfigurationException(inner)
                }
                crate::error::CreateProjectErrorKind::ProjectCreationFailedException(inner) => {
                    Error::ProjectCreationFailedException(inner)
                }
                crate::error::CreateProjectErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CreateProjectErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateUserProfileError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateUserProfileError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateUserProfileErrorKind::UserProfileAlreadyExistsException(
                    inner,
                ) => Error::UserProfileAlreadyExistsException(inner),
                crate::error::CreateUserProfileErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CreateUserProfileErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteProjectError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteProjectError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteProjectErrorKind::ConcurrentModificationException(inner) => {
                    Error::ConcurrentModificationException(inner)
                }
                crate::error::DeleteProjectErrorKind::InvalidServiceRoleException(inner) => {
                    Error::InvalidServiceRoleException(inner)
                }
                crate::error::DeleteProjectErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DeleteProjectErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteUserProfileError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteUserProfileError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteUserProfileErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DeleteUserProfileErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeProjectError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeProjectError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeProjectErrorKind::ConcurrentModificationException(inner) => {
                    Error::ConcurrentModificationException(inner)
                }
                crate::error::DescribeProjectErrorKind::InvalidServiceRoleException(inner) => {
                    Error::InvalidServiceRoleException(inner)
                }
                crate::error::DescribeProjectErrorKind::ProjectConfigurationException(inner) => {
                    Error::ProjectConfigurationException(inner)
                }
                crate::error::DescribeProjectErrorKind::ProjectNotFoundException(inner) => {
                    Error::ProjectNotFoundException(inner)
                }
                crate::error::DescribeProjectErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DescribeProjectErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeUserProfileError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeUserProfileError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeUserProfileErrorKind::UserProfileNotFoundException(inner) => {
                    Error::UserProfileNotFoundException(inner)
                }
                crate::error::DescribeUserProfileErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DescribeUserProfileErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DisassociateTeamMemberError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DisassociateTeamMemberError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DisassociateTeamMemberErrorKind::ConcurrentModificationException(
                    inner,
                ) => Error::ConcurrentModificationException(inner),
                crate::error::DisassociateTeamMemberErrorKind::InvalidServiceRoleException(
                    inner,
                ) => Error::InvalidServiceRoleException(inner),
                crate::error::DisassociateTeamMemberErrorKind::ProjectNotFoundException(inner) => {
                    Error::ProjectNotFoundException(inner)
                }
                crate::error::DisassociateTeamMemberErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DisassociateTeamMemberErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListProjectsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListProjectsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListProjectsErrorKind::InvalidNextTokenException(inner) => {
                    Error::InvalidNextTokenException(inner)
                }
                crate::error::ListProjectsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListProjectsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListResourcesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListResourcesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListResourcesErrorKind::InvalidNextTokenException(inner) => {
                    Error::InvalidNextTokenException(inner)
                }
                crate::error::ListResourcesErrorKind::ProjectNotFoundException(inner) => {
                    Error::ProjectNotFoundException(inner)
                }
                crate::error::ListResourcesErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListResourcesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTagsForProjectError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListTagsForProjectError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListTagsForProjectErrorKind::InvalidNextTokenException(inner) => {
                    Error::InvalidNextTokenException(inner)
                }
                crate::error::ListTagsForProjectErrorKind::ProjectNotFoundException(inner) => {
                    Error::ProjectNotFoundException(inner)
                }
                crate::error::ListTagsForProjectErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListTagsForProjectErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTeamMembersError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListTeamMembersError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListTeamMembersErrorKind::InvalidNextTokenException(inner) => {
                    Error::InvalidNextTokenException(inner)
                }
                crate::error::ListTeamMembersErrorKind::ProjectNotFoundException(inner) => {
                    Error::ProjectNotFoundException(inner)
                }
                crate::error::ListTeamMembersErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListTeamMembersErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListUserProfilesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListUserProfilesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListUserProfilesErrorKind::InvalidNextTokenException(inner) => {
                    Error::InvalidNextTokenException(inner)
                }
                crate::error::ListUserProfilesErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListUserProfilesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TagProjectError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::TagProjectError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::TagProjectErrorKind::ConcurrentModificationException(inner) => {
                    Error::ConcurrentModificationException(inner)
                }
                crate::error::TagProjectErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::TagProjectErrorKind::ProjectNotFoundException(inner) => {
                    Error::ProjectNotFoundException(inner)
                }
                crate::error::TagProjectErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::TagProjectErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UntagProjectError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UntagProjectError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UntagProjectErrorKind::ConcurrentModificationException(inner) => {
                    Error::ConcurrentModificationException(inner)
                }
                crate::error::UntagProjectErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::UntagProjectErrorKind::ProjectNotFoundException(inner) => {
                    Error::ProjectNotFoundException(inner)
                }
                crate::error::UntagProjectErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UntagProjectErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateProjectError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateProjectError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateProjectErrorKind::ProjectNotFoundException(inner) => {
                    Error::ProjectNotFoundException(inner)
                }
                crate::error::UpdateProjectErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UpdateProjectErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateTeamMemberError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateTeamMemberError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateTeamMemberErrorKind::ConcurrentModificationException(inner) => {
                    Error::ConcurrentModificationException(inner)
                }
                crate::error::UpdateTeamMemberErrorKind::InvalidServiceRoleException(inner) => {
                    Error::InvalidServiceRoleException(inner)
                }
                crate::error::UpdateTeamMemberErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::UpdateTeamMemberErrorKind::ProjectConfigurationException(inner) => {
                    Error::ProjectConfigurationException(inner)
                }
                crate::error::UpdateTeamMemberErrorKind::ProjectNotFoundException(inner) => {
                    Error::ProjectNotFoundException(inner)
                }
                crate::error::UpdateTeamMemberErrorKind::TeamMemberNotFoundException(inner) => {
                    Error::TeamMemberNotFoundException(inner)
                }
                crate::error::UpdateTeamMemberErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UpdateTeamMemberErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateUserProfileError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateUserProfileError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateUserProfileErrorKind::UserProfileNotFoundException(inner) => {
                    Error::UserProfileNotFoundException(inner)
                }
                crate::error::UpdateUserProfileErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UpdateUserProfileErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}