pub type ZCError = i8;
pub const Z_OK: ZCError = 0;
pub const Z_EINVAL: ZCError = -1;
pub const Z_EPARSE: ZCError = -2;
pub const Z_EIO: ZCError = -3;
pub const Z_ENETWORK: ZCError = -4;
// negativ pthread error codes (due to convention to return negative values on error)
pub const Z_EBUSY_MUTEX: ZCError = -16;
pub const Z_EINVAL_MUTEX: ZCError = -22;
pub const Z_EAGAIN_MUTEX: ZCError = -11;
pub const Z_EPOISON_MUTEX: ZCError = -22; // same as Z_EINVAL_MUTEX
pub const Z_EGENERIC: ZCError = i8::MIN;