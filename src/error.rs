use pinocchio::program_error::{ProgramError, ToStr};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenError {
    //0
    InvalidInstruction,
    InvalidArgument,
    AlreadyInUse,
}

impl From<TokenError> for ProgramError {
    fn from(e: TokenError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl TryFrom<u32> for TokenError {
    type Error = ProgramError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TokenError::InvalidInstruction),
            1 => Ok(TokenError::AlreadyInUse.into()),
            _ => Err(TokenError::InvalidArgument.into()),
        }
    }
}

impl ToStr for TokenError {
    fn to_str<E>(&self) -> &'static str
    where
        E: 'static + ToStr + TryFrom<u32>,
    {
        match self {
            TokenError::InvalidInstruction => "Error: Invalid instruction",
            TokenError::InvalidArgument => "Error: Invalid argument",
            TokenError::AlreadyInUse => "Error: Account already initialized!",
        }
    }
}
