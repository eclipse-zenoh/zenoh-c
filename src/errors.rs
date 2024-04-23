pub type z_error_t = i8;
pub const Z_OK: z_error_t = 0;
pub const Z_EINVAL: z_error_t = -1;
pub const Z_EPARSE: z_error_t = -2;
pub const Z_EIO: z_error_t = -3;
pub const Z_ENETWORK: z_error_t = -4;
// negativ pthread error codes (due to convention to return negative values on error)
pub const Z_EBUSY_MUTEX: z_error_t = -16;
pub const Z_EINVAL_MUTEX: z_error_t = -22;
pub const Z_EAGAIN_MUTEX: z_error_t = -11;
pub const Z_EPOISON_MUTEX: z_error_t = -22; // same as Z_EINVAL_MUTEX
pub const Z_EGENERIC: z_error_t = i8::MIN;
