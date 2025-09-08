pub mod mint;

use pinocchio::program_error::ProgramError;

pub type COption<T> = ([u8; 4], T);

/// Marker trait for types that can be cast from a raw pointer.
///
/// # Safety
///
/// It is up to the type implementing this trait to guarantee that the cast is
/// safe, i.e., the fields of the type are well aligned and there are no padding
/// bytes.
pub unsafe trait Transmutable {
    const LEN: usize;
}

pub trait Initializable {
    fn is_initialized(&self) -> Result<bool, ProgramError>;
}

/// returns mutable reference to 'T' from bytes
#[inline(always)]
pub unsafe fn load_mut_unchecked<T: Transmutable>(
    bytes: &mut [u8],
) -> Result<&mut T, ProgramError> {
    if bytes.len() != T::LEN {
        return Err(ProgramError::InvalidAccountData);
    };

    Ok(unsafe { &mut *(bytes.as_mut_ptr() as *mut T) })
}
