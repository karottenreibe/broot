use {
    super::*,
    crate::errors::ConfError,
    std::path::PathBuf,
};

/// A verb execution definition based on an internal
#[derive(Debug, Clone)]
pub struct InternalExecution {

    /// the internal to use
    pub internal: Internal,

    /// whether to open the resulting state in a new panel
    /// instead of the current ones
    pub bang: bool,

    /// arguments
    /// (for example `"~"` when a verb execution is `:!focus ~`)
    pub arg: Option<String>,
}

impl InternalExecution {
    pub fn from_internal(internal: Internal) -> Self {
        Self {
            internal,
            bang: false,
            arg: None,
        }
    }
    pub fn from_internal_bang(internal: Internal, bang: bool) -> Self {
        Self {
            internal,
            bang,
            arg: None,
        }
    }
    pub fn check_args(
        &self,
        invocation: &VerbInvocation,
        _other_path: &Option<PathBuf>,
    ) -> Option<String> {
        if invocation.args.is_some() && !self.internal.accept_path() {
            Some(format!("{} doesn't take arguments", invocation.name))
        } else {
            None
        }
    }
    pub fn try_from(invocation_str: &str) -> Result<Self, ConfError> {
        let invocation = VerbInvocation::from(invocation_str);
        let internal = Internal::try_from(&invocation.name)?;
        if invocation.args.is_some() && !internal.accept_path() {
            return Err(ConfError::UnexpectedInternalArg {
                invocation: invocation_str.to_string(),
            });
        }
        Ok(Self {
            internal,
            bang: invocation.bang,
            arg: invocation.args,
        })
    }
    pub fn as_desc_code(&self) -> Option<String> {
        self.arg.as_ref().map(|arg| {
            format!(
                ":{}{} {}",
                self.internal.name(),
                if self.bang { "!" } else { "" },
                arg
            )
        })
    }
}
