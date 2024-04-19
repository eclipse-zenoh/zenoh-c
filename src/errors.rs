pub type ZCError = i8;
pub const Z_OK: ZCError = 0;
pub const Z_EINVAL: ZCError = -1;
pub const Z_EPARSE: ZCError = -2;
pub const Z_EIO: ZCError = -3;
pub const Z_ENETWORK: ZCError = -4;
pub const Z_EGENERIC: ZCError = i8::MIN;