#[macro_use]
extern crate serde_derive;

/// Represents errors, which can happen in (almost) each method.
pub enum CommonError {
    /// Authorization failed
    AccessDenied,
    /// Internal error in JJS, config, plugin, etc
    InternalError,
    /// Auth token is malformed or expired
    AuthTokenFault,
}

// some typedefs
pub type ToolchainId = u32;
pub type SubmissionId = u32;
pub type EmptyParams = ();

// auth
/// Opaque struct that represents auth token
/// You mustn't make any assumptions regarding 'buf' field, except that is ASCII string
/// without any whitespaces
pub struct AuthToken {
    pub buf: String,
}

pub struct AuthSimpleParams {
    pub login: String,
    pub password: String,
}

pub enum AuthSimpleError {
    UnknownLogin,
    IncorrectPassword,
    NotSuitable,
    Common(CommonError),
}

// submissions
pub struct SubmissionSendParams {
    pub toolchain: ToolchainId,
    /// Must be correct base64-encoded string
    pub code: String,
}

pub enum SubmitError {
    UnknownToolchain,
    ContestIsOver,
    SizeLimitExceeded,
    Base64,
    Common(CommonError),
}

pub struct Status {
    pub kind: String,
    pub code: String,
}

pub struct SubmissionInformation {
    pub id: SubmissionId,
    pub toolchain_name: String,
    pub status: Status,
    pub score: Option<u32>,
}

pub struct SubmissionsListParams {
    pub limit: u32,
}

pub struct SubmissionsSetInfoParams {
    pub id: SubmissionId,
    pub status: Option<Status>,
    pub delete: bool,
}

// toolchains
pub struct ToolchainInformation {
    pub name: String,
    pub id: u32,
}

// users
pub struct UsersCreateParams {
    pub login: String,
    pub password: String,
}

pub enum UsersCreateError {
    InvalidLogin,
    PasswordRejected,
    Common(CommonError),
}

/// This trait serves for documentation-only purposes
///
/// Argument must be JSON-encoded and sent as a body (not form!)
pub trait Frontend {
    fn auth_anonymous(nope: EmptyParams) -> Result<AuthToken, CommonError>;

    fn auth_simple(auth_params: AuthSimpleParams) -> Result<AuthToken, AuthSimpleError>;

    fn submissions_send(sd: SubmissionSendParams) -> Result<SubmissionId, SubmitError>;

    fn submissions_list(selection_params: SubmissionsListParams) -> Result<Vec<SubmissionInformation>, CommonError>;

    fn submissions_set_info(info: SubmissionsSetInfoParams) -> Result<(), CommonError>;

    fn toolchains_list(nope: EmptyParams) -> Result<Vec<ToolchainInformation>, CommonError>;

    fn users_create(params: UsersCreateParams) -> Result<(), UsersCreateError>;
}
