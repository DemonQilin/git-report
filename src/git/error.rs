use std::{
    error,
    fmt::{self, Debug, Display},
    io,
    path::Path,
    result,
};

const MISSING_GIT_MESSAGE: &str =
    "Git should be installed. Visit: https://git-scm.com/book/en/v2/Getting-Started-Installing-Git";

pub type Result<'a, T> = result::Result<T, GitError<'a>>;

pub enum GitError<'a> {
    MissingGitCli,
    NoRepository(&'a Path),
    Other(String),
}

impl<'a> Debug for GitError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitError::MissingGitCli => write!(f, "{}", MISSING_GIT_MESSAGE),
            GitError::NoRepository(path) => write!(f, "No git repository exists in {path:?}"),
            GitError::Other(message) => write!(f, "{}", message),
        }
    }
}

impl<'a> Display for GitError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitError::MissingGitCli => write!(f, "{}", MISSING_GIT_MESSAGE),
            GitError::NoRepository(path) => {
                write!(f, "No git repository exists in {}", path.to_string_lossy())
            }
            GitError::Other(message) => write!(f, "{}", message),
        }
    }
}

impl error::Error for GitError<'_> {}

impl From<io::Error> for GitError<'_> {
    fn from(error: io::Error) -> Self {
        GitError::Other(error.to_string())
    }
}
