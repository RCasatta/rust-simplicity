/* This file has been automatically generated. */

use crate::jet::type_name::TypeName;
use crate::jet::Jet;
use crate::merkle::cmr::Cmr;
use crate::decode_bits;
use crate::{decode, BitIter, BitWriter};
use crate::analysis::Cost;
use hashes::sha256::Midstate;
use simplicity_sys::CFrameItem;
use std::io::Write;
use std::{fmt, str};

/// Core jet family
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Core {
    Add16,
    Add32,
    Add64,
    Add8,
    All16,
    All32,
    All64,
    All8,
    And16,
    And32,
    And64,
    And8,
    Bip0340Verify,
    Ch16,
    Ch32,
    Ch64,
    Ch8,
    CheckSigVerify,
    Complement16,
    Complement32,
    Complement64,
    Complement8,
    Decompress,
    Decrement16,
    Decrement32,
    Decrement64,
    Decrement8,
    DivMod16,
    DivMod32,
    DivMod64,
    DivMod8,
    Divide16,
    Divide32,
    Divide64,
    Divide8,
    Divides16,
    Divides32,
    Divides64,
    Divides8,
    Eq16,
    Eq256,
    Eq32,
    Eq64,
    Eq8,
    FeAdd,
    FeInvert,
    FeIsOdd,
    FeIsZero,
    FeMultiply,
    FeMultiplyBeta,
    FeNegate,
    FeNormalize,
    FeSquare,
    FeSquareRoot,
    FullAdd16,
    FullAdd32,
    FullAdd64,
    FullAdd8,
    FullDecrement16,
    FullDecrement32,
    FullDecrement64,
    FullDecrement8,
    FullIncrement16,
    FullIncrement32,
    FullIncrement64,
    FullIncrement8,
    FullMultiply16,
    FullMultiply32,
    FullMultiply64,
    FullMultiply8,
    FullSubtract16,
    FullSubtract32,
    FullSubtract64,
    FullSubtract8,
    GeIsOnCurve,
    GeNegate,
    GejAdd,
    GejDouble,
    GejGeAdd,
    GejGeAddEx,
    GejInfinity,
    GejIsInfinity,
    GejIsOnCurve,
    GejNegate,
    GejNormalize,
    GejRescale,
    GejXEquiv,
    GejYIsOdd,
    Generate,
    High16,
    High32,
    High64,
    High8,
    Increment16,
    Increment32,
    Increment64,
    Increment8,
    IsOne16,
    IsOne32,
    IsOne64,
    IsOne8,
    IsZero16,
    IsZero32,
    IsZero64,
    IsZero8,
    Le16,
    Le32,
    Le64,
    Le8,
    LinearCombination1,
    LinearVerify1,
    Low16,
    Low32,
    Low64,
    Low8,
    Lt16,
    Lt32,
    Lt64,
    Lt8,
    Maj16,
    Maj32,
    Maj64,
    Maj8,
    Max16,
    Max32,
    Max64,
    Max8,
    Median16,
    Median32,
    Median64,
    Median8,
    Min16,
    Min32,
    Min64,
    Min8,
    Modulo16,
    Modulo32,
    Modulo64,
    Modulo8,
    Multiply16,
    Multiply32,
    Multiply64,
    Multiply8,
    Negate16,
    Negate32,
    Negate64,
    Negate8,
    One16,
    One32,
    One64,
    One8,
    Or16,
    Or32,
    Or64,
    Or8,
    ParseLock,
    ParseSequence,
    PointVerify1,
    ScalarAdd,
    ScalarInvert,
    ScalarIsZero,
    ScalarMultiply,
    ScalarMultiplyLambda,
    ScalarNegate,
    ScalarNormalize,
    ScalarSquare,
    Scale,
    Sha256Block,
    Sha256Ctx8Add1,
    Sha256Ctx8Add128,
    Sha256Ctx8Add16,
    Sha256Ctx8Add2,
    Sha256Ctx8Add256,
    Sha256Ctx8Add32,
    Sha256Ctx8Add4,
    Sha256Ctx8Add512,
    Sha256Ctx8Add64,
    Sha256Ctx8Add8,
    Sha256Ctx8AddBuffer511,
    Sha256Ctx8Finalize,
    Sha256Ctx8Init,
    Sha256Iv,
    Some16,
    Some32,
    Some64,
    Some8,
    Subtract16,
    Subtract32,
    Subtract64,
    Subtract8,
    Verify,
    Xor16,
    Xor32,
    Xor64,
    Xor8,
    XorXor16,
    XorXor32,
    XorXor64,
    XorXor8,
}

impl Jet for Core {

    type Environment = ();
    type CJetEnvironment = ();

    fn c_jet_env<'env>(&self, env: &'env Self::Environment) -> &'env Self::CJetEnvironment {
        env
    }

    fn cmr(&self) -> Cmr {
        let bytes = match self {
            Core::Add16 => [
                0x90, 0xe5, 0xc3, 0x02, 0xc2, 0xa1, 0x02, 0xf7, 0x6c, 0xc2, 0x0b, 0xcd, 0xdd, 0xc0,
                0x62, 0x2b, 0xf3, 0xb8, 0xa5, 0x44, 0x6a, 0xb6, 0x8d, 0x1b, 0x2a, 0x03, 0x39, 0x5e,
                0x6f, 0xae, 0x92, 0x50,
            ],
            Core::Add32 => [
                0x6d, 0x78, 0x30, 0xd6, 0x5e, 0x69, 0xf2, 0x7d, 0x55, 0x1c, 0x4b, 0xb3, 0x71, 0x8d,
                0xa9, 0x35, 0x22, 0x2f, 0xae, 0xc4, 0xaf, 0x7f, 0x49, 0xe9, 0x5e, 0x98, 0x33, 0xed,
                0x2f, 0x84, 0xc1, 0x06,
            ],
            Core::Add64 => [
                0x9e, 0xee, 0xbe, 0xa8, 0x47, 0x38, 0x8e, 0x14, 0xc7, 0xc9, 0xe7, 0xca, 0x14, 0x7c,
                0x70, 0x71, 0x4d, 0xe1, 0x44, 0xeb, 0xab, 0x39, 0x8a, 0x53, 0xf4, 0xf5, 0x8a, 0x34,
                0x93, 0x0c, 0xc4, 0x5c,
            ],
            Core::Add8 => [
                0x37, 0x59, 0xb0, 0x40, 0x6a, 0xa2, 0xc4, 0xe1, 0x15, 0x26, 0x21, 0xe0, 0x11, 0xe4,
                0xef, 0xdb, 0x55, 0x5c, 0x6a, 0xb6, 0xd9, 0x1a, 0xe9, 0xeb, 0xf1, 0xd6, 0x31, 0x5b,
                0x43, 0x82, 0xa6, 0xc4,
            ],
            Core::All16 => [
                0x3e, 0x7e, 0x95, 0x3d, 0x28, 0xa6, 0xdc, 0x5e, 0xd7, 0xdf, 0xbd, 0x4c, 0x63, 0xf2,
                0xe0, 0x80, 0x47, 0xc7, 0x1b, 0xb9, 0xc4, 0x37, 0xe8, 0xc7, 0x1e, 0xe0, 0x14, 0x7a,
                0x60, 0xf7, 0x8b, 0xee,
            ],
            Core::All32 => [
                0x5d, 0xd0, 0x85, 0x16, 0x26, 0x03, 0x6c, 0xb6, 0xa4, 0x2a, 0xa9, 0x52, 0x93, 0xdb,
                0xb8, 0xf3, 0x6e, 0xfe, 0x5e, 0x0e, 0xa5, 0x7a, 0x31, 0x08, 0x27, 0xfa, 0x6d, 0x79,
                0x2f, 0x46, 0xe0, 0xc1,
            ],
            Core::All64 => [
                0xba, 0xd0, 0x2f, 0xd1, 0x58, 0x90, 0x8a, 0x1b, 0xe0, 0x68, 0x72, 0x5e, 0xb0, 0x52,
                0x76, 0xde, 0xfa, 0x41, 0x12, 0x07, 0xd0, 0xc8, 0xa9, 0x1b, 0xbe, 0x6f, 0xa5, 0x7d,
                0x2c, 0xda, 0x29, 0xdf,
            ],
            Core::All8 => [
                0xdc, 0x09, 0x88, 0x54, 0xdd, 0x06, 0xae, 0x1e, 0x6e, 0x3e, 0x73, 0xa4, 0xae, 0x94,
                0xd0, 0xb2, 0xac, 0xce, 0x5c, 0xb3, 0xec, 0xc1, 0x2e, 0x8c, 0xb8, 0x16, 0x7f, 0x7b,
                0x6e, 0xaa, 0x40, 0x69,
            ],
            Core::And16 => [
                0x4f, 0x42, 0xc6, 0x88, 0xcc, 0x3c, 0xeb, 0x76, 0x1f, 0x66, 0x2a, 0x0b, 0x8e, 0x77,
                0xb1, 0x96, 0x42, 0xda, 0xb5, 0x40, 0x7d, 0x9a, 0x87, 0x3c, 0xd9, 0xc1, 0x86, 0x09,
                0x97, 0x7e, 0xbd, 0x91,
            ],
            Core::And32 => [
                0x16, 0x80, 0xcf, 0xd2, 0x64, 0x3a, 0xd3, 0x89, 0x6b, 0xfb, 0x45, 0xe9, 0x6f, 0xc6,
                0x2d, 0xc2, 0xbb, 0x0c, 0xc0, 0xac, 0x48, 0x6f, 0xa8, 0x33, 0x30, 0xa8, 0x89, 0x0c,
                0x09, 0xee, 0x96, 0x9b,
            ],
            Core::And64 => [
                0xa2, 0xdd, 0x3d, 0x49, 0xf1, 0x2c, 0xd4, 0x42, 0xce, 0xa7, 0xc8, 0x32, 0x38, 0x68,
                0xbe, 0x3c, 0xe6, 0xe9, 0xaf, 0x67, 0x6f, 0x72, 0x51, 0x10, 0xf3, 0xce, 0x49, 0x4e,
                0xdd, 0x6a, 0x5a, 0x4a,
            ],
            Core::And8 => [
                0x27, 0x55, 0x41, 0x78, 0x19, 0x86, 0x58, 0xf5, 0xbf, 0xb5, 0xb0, 0x41, 0x30, 0x89,
                0x87, 0x42, 0xbf, 0x82, 0xdb, 0x60, 0x39, 0xdc, 0xbb, 0x17, 0x26, 0x6d, 0xfe, 0x03,
                0x0c, 0xc1, 0x20, 0x75,
            ],
            Core::Bip0340Verify => [
                0xba, 0x22, 0x11, 0x7f, 0xcd, 0x13, 0x81, 0x5b, 0xf3, 0xe3, 0xc8, 0x8a, 0x46, 0x55,
                0x1a, 0x6e, 0x31, 0xf0, 0x33, 0x1b, 0x00, 0xd1, 0x34, 0xac, 0xab, 0x79, 0x49, 0xd5,
                0x9e, 0x32, 0x21, 0x94,
            ],
            Core::Ch16 => [
                0xdc, 0xcd, 0xe6, 0xa9, 0x54, 0x58, 0x75, 0xc5, 0xcb, 0x46, 0xe7, 0x2c, 0x7b, 0x04,
                0xce, 0xeb, 0x92, 0x0c, 0x20, 0x3d, 0x1c, 0x04, 0x2a, 0xec, 0x91, 0x24, 0x1d, 0xbe,
                0xca, 0x23, 0xf4, 0x35,
            ],
            Core::Ch32 => [
                0x8c, 0x0a, 0x87, 0xee, 0x8e, 0x1d, 0xfc, 0xa3, 0x4b, 0xdf, 0xf4, 0x21, 0x2b, 0xa1,
                0xaf, 0x75, 0x44, 0xd2, 0xde, 0x6f, 0x4a, 0xcb, 0x76, 0x18, 0x08, 0x23, 0x1a, 0x5f,
                0x57, 0x78, 0x8e, 0x62,
            ],
            Core::Ch64 => [
                0x5d, 0x5a, 0xe5, 0x7d, 0x76, 0x47, 0xf0, 0x1e, 0xc3, 0xfd, 0x79, 0x7e, 0xd8, 0x9d,
                0x62, 0xbe, 0x5f, 0xe5, 0x85, 0xfb, 0xb9, 0xfd, 0xb6, 0xc3, 0x20, 0x8b, 0x2c, 0x08,
                0x57, 0x9b, 0x9e, 0x2c,
            ],
            Core::Ch8 => [
                0xdf, 0xc5, 0x78, 0xf2, 0x5c, 0xb3, 0x14, 0x02, 0xe2, 0x2a, 0x81, 0x0e, 0x98, 0x95,
                0x0a, 0xe1, 0x7b, 0x19, 0x08, 0x96, 0x36, 0x4c, 0xde, 0xe8, 0xee, 0xa3, 0x00, 0xcf,
                0x79, 0xd8, 0x78, 0x01,
            ],
            Core::CheckSigVerify => [
                0xe8, 0xfe, 0xb5, 0x34, 0x34, 0xe2, 0xbd, 0xb5, 0xcf, 0xe6, 0xa9, 0x1f, 0xa8, 0xf9,
                0xe1, 0x77, 0xf9, 0x41, 0xa6, 0x7c, 0xc6, 0xce, 0xd8, 0x69, 0x74, 0x6f, 0x1a, 0x8e,
                0xb6, 0x50, 0x6f, 0x76,
            ],
            Core::Complement16 => [
                0x7f, 0x16, 0xd1, 0x24, 0x15, 0x99, 0x87, 0x68, 0x34, 0x7f, 0x43, 0xa9, 0xb9, 0x1c,
                0x89, 0x87, 0x59, 0x1d, 0xfe, 0xc1, 0xcf, 0x78, 0x50, 0xb4, 0x23, 0xfe, 0xa3, 0x3a,
                0x37, 0x26, 0xf4, 0x63,
            ],
            Core::Complement32 => [
                0xe6, 0x5a, 0x4f, 0x75, 0x54, 0x11, 0xb4, 0x80, 0xb8, 0x05, 0x85, 0xfd, 0xc0, 0x2e,
                0xe2, 0x4c, 0xbb, 0xd9, 0x50, 0x2a, 0x48, 0xa7, 0x34, 0x7a, 0x71, 0x02, 0x94, 0xe7,
                0x82, 0xac, 0xff, 0xba,
            ],
            Core::Complement64 => [
                0xd5, 0xc6, 0x57, 0x13, 0x8a, 0x6c, 0xf5, 0x53, 0x22, 0xc3, 0x6e, 0x6f, 0x3c, 0x4a,
                0x0f, 0x9f, 0xb6, 0x39, 0x88, 0xa1, 0x12, 0x0d, 0x6e, 0xeb, 0x03, 0xfc, 0x42, 0x7c,
                0xad, 0x97, 0x18, 0x69,
            ],
            Core::Complement8 => [
                0x1f, 0x5c, 0x6a, 0xb4, 0x1d, 0x40, 0x6b, 0xad, 0xfc, 0x9d, 0x5a, 0x2c, 0x76, 0xbe,
                0xf7, 0xe3, 0x28, 0x7a, 0xab, 0xee, 0x19, 0xd0, 0x85, 0x0f, 0x09, 0x38, 0x98, 0x46,
                0xea, 0xbf, 0x45, 0x8e,
            ],
            Core::Decompress => [
                0x99, 0xca, 0xc8, 0xf0, 0x6f, 0xac, 0x40, 0x51, 0x42, 0x37, 0x49, 0xc0, 0xb0, 0x99,
                0x45, 0x20, 0xf9, 0x14, 0xd1, 0x96, 0x6f, 0x9f, 0x02, 0x97, 0x78, 0xc4, 0xe7, 0x45,
                0x68, 0x90, 0xb7, 0xd6,
            ],
            Core::Decrement16 => [
                0x19, 0x6b, 0x2d, 0xc6, 0x32, 0xe6, 0xc5, 0xd0, 0x94, 0xff, 0x9d, 0x34, 0xa2, 0x09,
                0x2b, 0x80, 0x5e, 0x4b, 0x94, 0xd0, 0x36, 0xba, 0x48, 0x26, 0xac, 0xef, 0xed, 0x60,
                0x3a, 0x48, 0x9d, 0xc7,
            ],
            Core::Decrement32 => [
                0x29, 0xb3, 0x15, 0x92, 0x66, 0xe9, 0xf6, 0x08, 0x80, 0x88, 0x4f, 0x6e, 0xb2, 0x81,
                0x6f, 0x19, 0xa2, 0x07, 0x29, 0x1c, 0x82, 0x9f, 0xd1, 0x39, 0xfb, 0x0d, 0x78, 0xa3,
                0x9e, 0x08, 0xd0, 0xc5,
            ],
            Core::Decrement64 => [
                0x67, 0x17, 0x5a, 0x67, 0x6d, 0xc1, 0x78, 0x0a, 0x20, 0xfa, 0xf5, 0xe5, 0xaa, 0xd5,
                0x82, 0xec, 0x79, 0x7b, 0xae, 0xa5, 0x5f, 0x29, 0x05, 0xec, 0xad, 0x2a, 0xaa, 0x1a,
                0xe7, 0x6b, 0xe9, 0x43,
            ],
            Core::Decrement8 => [
                0x2b, 0x59, 0x11, 0xe6, 0x67, 0xe6, 0x96, 0xf9, 0xcd, 0x03, 0xe9, 0xdf, 0x7d, 0x40,
                0x0a, 0x55, 0x45, 0xe7, 0xe9, 0x74, 0x4a, 0xfb, 0xf9, 0x2e, 0x0b, 0xa8, 0x7f, 0x6c,
                0x7c, 0xc4, 0x42, 0x87,
            ],
            Core::DivMod16 => [
                0x15, 0x85, 0xd1, 0xbf, 0xe9, 0x52, 0xf7, 0x1b, 0x1c, 0xfe, 0xbb, 0xef, 0x29, 0x2c,
                0xed, 0x56, 0x60, 0x91, 0x57, 0x23, 0x05, 0x18, 0x27, 0x20, 0x62, 0x5a, 0x1a, 0xc0,
                0x08, 0x81, 0xa4, 0x89,
            ],
            Core::DivMod32 => [
                0xba, 0xb7, 0xbc, 0x9a, 0x9f, 0x64, 0x4e, 0xdc, 0xba, 0x73, 0xe3, 0x30, 0xc1, 0xa5,
                0x05, 0x43, 0x67, 0xf5, 0xfb, 0x86, 0x78, 0xc1, 0x23, 0x6a, 0xb6, 0xe7, 0xf4, 0xd7,
                0x18, 0x60, 0xe0, 0x02,
            ],
            Core::DivMod64 => [
                0x8d, 0xb4, 0x66, 0x4d, 0x99, 0xe8, 0xc8, 0x78, 0xef, 0x31, 0xeb, 0xf1, 0xac, 0xa6,
                0xa8, 0x76, 0x4b, 0x58, 0x49, 0x5d, 0xa0, 0xec, 0xc0, 0x32, 0xb3, 0xc9, 0xb4, 0x89,
                0xaa, 0xa0, 0xff, 0x90,
            ],
            Core::DivMod8 => [
                0x25, 0xf3, 0x69, 0x4b, 0x75, 0x2e, 0x4d, 0xf1, 0xb0, 0xcc, 0x8c, 0x51, 0xbf, 0x99,
                0x17, 0xef, 0x6e, 0x11, 0xc5, 0x2e, 0xa7, 0x77, 0x35, 0x3e, 0x95, 0x07, 0xe8, 0x19,
                0xa8, 0x06, 0x62, 0x95,
            ],
            Core::Divide16 => [
                0x2d, 0xc3, 0xe9, 0xa9, 0xc6, 0x1d, 0xc8, 0x8d, 0xd7, 0x44, 0xf1, 0x6b, 0xe1, 0xae,
                0xd4, 0xb7, 0x2a, 0xb4, 0x79, 0x49, 0x0e, 0x41, 0x42, 0x26, 0x58, 0xb9, 0x72, 0x19,
                0xdf, 0x49, 0x95, 0x7c,
            ],
            Core::Divide32 => [
                0xf6, 0xdc, 0xf4, 0xbd, 0x8d, 0x9b, 0x5e, 0xff, 0x75, 0x1e, 0x9b, 0x01, 0x47, 0xa9,
                0x8e, 0x0f, 0xc4, 0xc8, 0x87, 0x2c, 0x1a, 0xb7, 0x82, 0xd1, 0xeb, 0xec, 0x63, 0x83,
                0xa0, 0x9b, 0x78, 0x0a,
            ],
            Core::Divide64 => [
                0x8a, 0x94, 0x51, 0x8b, 0x9b, 0x76, 0x17, 0x9a, 0x9d, 0x5c, 0x1e, 0x31, 0x6e, 0x69,
                0x00, 0x62, 0x7f, 0x5b, 0x6e, 0x3e, 0xcc, 0x66, 0x8f, 0xe2, 0xd0, 0xc5, 0xd5, 0x62,
                0x38, 0xe3, 0x3e, 0xa3,
            ],
            Core::Divide8 => [
                0xd7, 0x16, 0x00, 0x0c, 0xe7, 0xe5, 0x72, 0x78, 0x84, 0xa6, 0xd7, 0x97, 0x6a, 0x32,
                0xc0, 0xf6, 0xbd, 0xb8, 0x81, 0xd0, 0x0f, 0xe8, 0xdb, 0xb2, 0x8d, 0xea, 0x37, 0x01,
                0x03, 0x76, 0x9e, 0x2d,
            ],
            Core::Divides16 => [
                0x8f, 0x41, 0x84, 0xa5, 0x82, 0x67, 0x4e, 0x00, 0x3b, 0x37, 0x40, 0x1b, 0x13, 0xc9,
                0xb8, 0xc3, 0x7c, 0x68, 0x3b, 0x88, 0xdf, 0x63, 0x3a, 0x22, 0x71, 0x9a, 0x23, 0xe1,
                0x94, 0xbd, 0xa6, 0xf3,
            ],
            Core::Divides32 => [
                0xe8, 0x9e, 0xcd, 0xc3, 0x7c, 0xd1, 0x04, 0x66, 0xb0, 0x0f, 0x01, 0x37, 0xe2, 0x0b,
                0x09, 0xb9, 0xc5, 0x94, 0x8d, 0x07, 0x31, 0x2d, 0x05, 0x3f, 0xed, 0x63, 0xc4, 0x91,
                0xbb, 0x5c, 0x4c, 0x6a,
            ],
            Core::Divides64 => [
                0xb4, 0x31, 0x98, 0xd5, 0x74, 0x41, 0x90, 0x57, 0xe7, 0x6b, 0xc0, 0x69, 0xd7, 0xd3,
                0xe2, 0xcb, 0x04, 0xf7, 0x07, 0x2b, 0x80, 0x69, 0x6d, 0x3d, 0xb9, 0x2c, 0x8a, 0x72,
                0xc7, 0x8a, 0x5c, 0xfc,
            ],
            Core::Divides8 => [
                0x3f, 0xc3, 0xb9, 0xb4, 0x96, 0x8e, 0x16, 0x74, 0xda, 0x9e, 0xc4, 0x57, 0xb1, 0xca,
                0x8a, 0x66, 0x29, 0xa3, 0x7b, 0x4b, 0xef, 0xdc, 0xd7, 0xcc, 0x55, 0x88, 0x60, 0x80,
                0xae, 0xf6, 0x8a, 0xf8,
            ],
            Core::Eq16 => [
                0xab, 0x31, 0xa7, 0x97, 0x4a, 0xcb, 0xf7, 0x2a, 0xb2, 0xf2, 0x1b, 0xf5, 0x3f, 0xec,
                0x34, 0x6a, 0x28, 0xe6, 0xe6, 0x5e, 0x4c, 0x05, 0xe3, 0xe7, 0x84, 0x3e, 0x14, 0x73,
                0xb1, 0xfd, 0x3f, 0xf9,
            ],
            Core::Eq256 => [
                0xf7, 0xdf, 0x29, 0xed, 0x2d, 0x00, 0x78, 0x22, 0x7e, 0x71, 0x9b, 0x7d, 0x13, 0x37,
                0xdb, 0xea, 0x3a, 0x71, 0x0f, 0x58, 0x4a, 0x4b, 0x07, 0xf0, 0xf2, 0x65, 0x4a, 0x44,
                0x65, 0x54, 0xf2, 0xbe,
            ],
            Core::Eq32 => [
                0x76, 0x79, 0x1f, 0x0d, 0xdd, 0xab, 0x9d, 0xc7, 0x7a, 0x14, 0xa6, 0x8c, 0xc6, 0x77,
                0x87, 0xc9, 0x84, 0x49, 0x3f, 0xae, 0xbb, 0xdb, 0xb6, 0xd6, 0xd4, 0x93, 0x35, 0x2e,
                0x11, 0x47, 0x20, 0xd5,
            ],
            Core::Eq64 => [
                0x01, 0x1b, 0x5f, 0x4c, 0xb3, 0x1f, 0x34, 0xba, 0x15, 0x8a, 0x68, 0x22, 0x3b, 0x34,
                0x44, 0x5e, 0x93, 0xf3, 0xdc, 0x18, 0x91, 0xb5, 0x33, 0x90, 0x04, 0x92, 0x3d, 0xcf,
                0xab, 0xe0, 0x34, 0xc1,
            ],
            Core::Eq8 => [
                0x0f, 0x8b, 0x1f, 0xde, 0x19, 0x49, 0xbe, 0xbd, 0x47, 0xd2, 0x55, 0x4a, 0x48, 0x07,
                0xfd, 0x8d, 0xca, 0xef, 0x87, 0x30, 0x18, 0x38, 0xd6, 0x0e, 0x84, 0xc7, 0x38, 0x14,
                0x42, 0xda, 0x7e, 0xce,
            ],
            Core::FeAdd => [
                0xa5, 0xe9, 0x21, 0xd4, 0xd2, 0x95, 0x9a, 0xf7, 0x83, 0x3d, 0xf9, 0x30, 0x83, 0x68,
                0x98, 0x57, 0x32, 0xab, 0x0f, 0x33, 0xba, 0xd8, 0xaa, 0x34, 0x5b, 0x39, 0xf3, 0x03,
                0x03, 0x2b, 0xe7, 0xae,
            ],
            Core::FeInvert => [
                0x21, 0xf4, 0x29, 0x56, 0x9d, 0x78, 0xe0, 0x9f, 0xb2, 0x90, 0x13, 0xef, 0x8a, 0x16,
                0xc7, 0x55, 0x0e, 0xdc, 0x4e, 0x19, 0x05, 0x25, 0x71, 0x5a, 0xbf, 0xb0, 0xc1, 0x9b,
                0x21, 0xcf, 0xd9, 0xc2,
            ],
            Core::FeIsOdd => [
                0x21, 0xd3, 0x5d, 0x0a, 0x65, 0x7f, 0x5d, 0xef, 0xac, 0xde, 0x6d, 0xa3, 0xfd, 0xb4,
                0x8d, 0x67, 0x56, 0xf3, 0x06, 0x90, 0xb4, 0x02, 0x70, 0xf0, 0x10, 0x17, 0xe7, 0xe3,
                0x48, 0xd7, 0x58, 0x5d,
            ],
            Core::FeIsZero => [
                0xf5, 0x71, 0x62, 0x82, 0xba, 0x0d, 0x26, 0xbd, 0x06, 0x71, 0xcd, 0xfe, 0x1f, 0x11,
                0x8f, 0x45, 0xba, 0xe7, 0x9d, 0x10, 0x9a, 0x6d, 0x9f, 0x39, 0x7a, 0x4c, 0x45, 0x54,
                0x6f, 0x62, 0xaf, 0x00,
            ],
            Core::FeMultiply => [
                0xc1, 0x76, 0x96, 0x9c, 0x33, 0x7f, 0xbc, 0x6b, 0x5a, 0xc8, 0xed, 0x3e, 0xa2, 0x8a,
                0xb3, 0xfd, 0x94, 0x78, 0x20, 0x64, 0x31, 0x95, 0xd7, 0x3b, 0x86, 0x20, 0x18, 0xf7,
                0x08, 0x48, 0x7d, 0x23,
            ],
            Core::FeMultiplyBeta => [
                0xc0, 0xfc, 0x76, 0x66, 0x8d, 0x99, 0xd4, 0x07, 0x3f, 0x39, 0x93, 0x44, 0x3a, 0x68,
                0x98, 0x0b, 0xa9, 0x6e, 0x97, 0x60, 0x1e, 0xd1, 0xa9, 0x19, 0xc4, 0xa2, 0x33, 0x05,
                0x7b, 0x66, 0xfe, 0x23,
            ],
            Core::FeNegate => [
                0xd6, 0xef, 0x8f, 0xc2, 0x27, 0x5d, 0xdf, 0x70, 0x58, 0xb5, 0xe4, 0x99, 0xf1, 0x0d,
                0xd2, 0x84, 0x66, 0xf5, 0x7b, 0x72, 0x53, 0xa5, 0x44, 0x07, 0x79, 0x8a, 0x47, 0x72,
                0x59, 0x21, 0x84, 0xb9,
            ],
            Core::FeNormalize => [
                0xf4, 0x98, 0xbc, 0xdf, 0xf3, 0x50, 0x67, 0x88, 0x1b, 0xd9, 0x6e, 0xe1, 0xc9, 0x1f,
                0x43, 0x67, 0x1e, 0x79, 0x18, 0x6e, 0x70, 0x0e, 0xc4, 0xda, 0x2d, 0xc0, 0x29, 0x6a,
                0x2d, 0xea, 0x91, 0xc7,
            ],
            Core::FeSquare => [
                0x16, 0x8a, 0x11, 0xb1, 0x9c, 0x0f, 0xb9, 0x7f, 0x10, 0x99, 0xf8, 0x63, 0xa4, 0xc8,
                0xe2, 0xf8, 0x12, 0x42, 0x5f, 0xbd, 0x3d, 0x01, 0xbd, 0xed, 0x3f, 0x46, 0x0c, 0xb3,
                0x59, 0xbf, 0xdd, 0x89,
            ],
            Core::FeSquareRoot => [
                0x3b, 0x46, 0xcd, 0xe3, 0x06, 0x74, 0xb7, 0x77, 0x16, 0xf2, 0xc8, 0xf6, 0x14, 0xad,
                0xbd, 0x16, 0x61, 0x98, 0x5d, 0x82, 0xc5, 0x53, 0x04, 0xd5, 0x6e, 0x53, 0xef, 0x5c,
                0x74, 0xc8, 0x3e, 0x83,
            ],
            Core::FullAdd16 => [
                0xf4, 0x54, 0xa6, 0x32, 0xfd, 0x19, 0x28, 0xcd, 0x6f, 0x07, 0x0d, 0xf6, 0x80, 0x14,
                0x28, 0x8e, 0x97, 0xb3, 0xe5, 0x82, 0xe7, 0xf0, 0x3e, 0x98, 0xed, 0x32, 0x34, 0xb6,
                0x28, 0x2d, 0x02, 0x94,
            ],
            Core::FullAdd32 => [
                0x96, 0x06, 0xb9, 0xe6, 0x54, 0x1e, 0x03, 0x56, 0xed, 0x76, 0x08, 0xd5, 0xbc, 0x6f,
                0x9d, 0x4e, 0xf6, 0x8e, 0x0a, 0x3d, 0x23, 0x51, 0x68, 0xd9, 0xe7, 0x5c, 0xae, 0x66,
                0x52, 0xdd, 0x06, 0x34,
            ],
            Core::FullAdd64 => [
                0xf0, 0xf7, 0x95, 0x32, 0x00, 0x1c, 0x02, 0xaf, 0xac, 0x65, 0x0a, 0x62, 0x80, 0x19,
                0x32, 0xaf, 0xc2, 0xc1, 0xb2, 0x86, 0xa3, 0x1f, 0xef, 0x7a, 0xe0, 0x8a, 0x47, 0xea,
                0x1e, 0x71, 0x38, 0x7d,
            ],
            Core::FullAdd8 => [
                0xcd, 0xd0, 0x80, 0xfd, 0x86, 0x12, 0xe7, 0xd1, 0x4a, 0xda, 0x34, 0x91, 0x64, 0xa6,
                0xd5, 0xaf, 0x60, 0x54, 0xd6, 0x73, 0x77, 0xe1, 0x76, 0x65, 0xc4, 0x72, 0x20, 0x40,
                0x28, 0xbc, 0x21, 0x25,
            ],
            Core::FullDecrement16 => [
                0xa7, 0xd1, 0xb9, 0xa6, 0x09, 0xdc, 0xd3, 0x67, 0x66, 0xa1, 0x67, 0xa3, 0x7a, 0xa0,
                0xe6, 0x49, 0xfb, 0xdd, 0x75, 0xf6, 0x46, 0x91, 0xa9, 0xfb, 0x6b, 0xf1, 0x66, 0x3d,
                0x32, 0x92, 0x82, 0xa8,
            ],
            Core::FullDecrement32 => [
                0xcb, 0xcf, 0xd7, 0xca, 0x73, 0x72, 0xb2, 0xb1, 0x80, 0x00, 0x8e, 0x51, 0x58, 0x98,
                0x80, 0xd3, 0xb5, 0x39, 0x9b, 0xd9, 0xa8, 0xdd, 0xb9, 0xbc, 0xda, 0x61, 0x32, 0xc2,
                0x2c, 0x77, 0x7e, 0x0f,
            ],
            Core::FullDecrement64 => [
                0x42, 0xdf, 0x92, 0xd2, 0xe7, 0xf4, 0x2e, 0x57, 0x9b, 0xd4, 0xc4, 0x30, 0x8c, 0xd2,
                0xdb, 0x6e, 0x76, 0x77, 0x46, 0x95, 0xd6, 0xa5, 0x69, 0x50, 0xc5, 0xcf, 0x92, 0x03,
                0x00, 0x2d, 0x41, 0xaa,
            ],
            Core::FullDecrement8 => [
                0xff, 0x58, 0xdd, 0x01, 0x62, 0xbc, 0xe9, 0xfb, 0x73, 0xed, 0x01, 0x5b, 0x7e, 0x5d,
                0x3d, 0x1d, 0xc1, 0x1b, 0x50, 0x5c, 0x93, 0x2d, 0x49, 0x0c, 0xaa, 0xdc, 0x1b, 0x5d,
                0x89, 0x3d, 0x1f, 0xad,
            ],
            Core::FullIncrement16 => [
                0x54, 0x53, 0x60, 0x8e, 0xef, 0xf9, 0x67, 0xaf, 0xef, 0x4a, 0xe5, 0x83, 0x1a, 0xa7,
                0xa9, 0xfc, 0x75, 0xc2, 0xba, 0x57, 0xe1, 0xac, 0x41, 0x2a, 0xb5, 0x46, 0x4d, 0x4d,
                0xe9, 0x3a, 0xf4, 0xf9,
            ],
            Core::FullIncrement32 => [
                0x36, 0x32, 0x33, 0x21, 0x58, 0xf9, 0x0b, 0xa4, 0xdf, 0x62, 0x3f, 0x08, 0x68, 0x08,
                0xd1, 0xd3, 0x39, 0xf3, 0x6d, 0x9a, 0x6f, 0x92, 0x27, 0x23, 0x34, 0x16, 0xf2, 0x34,
                0xcd, 0xd2, 0x5d, 0x98,
            ],
            Core::FullIncrement64 => [
                0x06, 0x37, 0xfc, 0xc5, 0x52, 0xf6, 0x2a, 0x8a, 0x08, 0xc6, 0xd0, 0x70, 0x66, 0x7e,
                0xd8, 0x0e, 0x47, 0xba, 0x70, 0x7c, 0x12, 0x0c, 0x4d, 0x65, 0x66, 0x04, 0xa3, 0x43,
                0x3b, 0xfc, 0x40, 0xa6,
            ],
            Core::FullIncrement8 => [
                0x29, 0x15, 0x24, 0xe1, 0x70, 0xd5, 0x23, 0xfd, 0x35, 0xc2, 0x57, 0x7b, 0x8e, 0xa3,
                0xf4, 0x47, 0x8e, 0xc6, 0x32, 0x06, 0xc9, 0x8f, 0xb5, 0xcc, 0xcf, 0x77, 0xc2, 0x09,
                0xbe, 0xbd, 0x29, 0xef,
            ],
            Core::FullMultiply16 => [
                0x32, 0xcf, 0x7f, 0x50, 0x89, 0x4e, 0xa2, 0xc4, 0x61, 0xa0, 0x54, 0x66, 0xbb, 0xfa,
                0x1e, 0x4e, 0x1b, 0x04, 0x99, 0x57, 0x52, 0x3f, 0x64, 0x93, 0x7a, 0x8b, 0x54, 0x27,
                0x3d, 0xd3, 0x1b, 0x37,
            ],
            Core::FullMultiply32 => [
                0xde, 0xa1, 0xaf, 0xc6, 0xfd, 0x54, 0x6c, 0x75, 0xe0, 0xb2, 0xd8, 0xe4, 0x18, 0xf2,
                0x61, 0x79, 0xd6, 0xdb, 0xe9, 0x05, 0x8b, 0x07, 0x9a, 0xa9, 0xab, 0x80, 0xea, 0xa6,
                0xc0, 0x5f, 0x39, 0xcb,
            ],
            Core::FullMultiply64 => [
                0x81, 0x3d, 0x74, 0xd6, 0xc3, 0x06, 0x4c, 0xf7, 0xc5, 0xdb, 0x2d, 0xda, 0x96, 0x4e,
                0xd0, 0xe2, 0xd5, 0xa2, 0x49, 0x1b, 0x89, 0x43, 0x29, 0x21, 0x92, 0x37, 0xcb, 0x1a,
                0x91, 0xee, 0x09, 0x34,
            ],
            Core::FullMultiply8 => [
                0xf7, 0xf3, 0x9d, 0x95, 0xda, 0xb5, 0x73, 0x08, 0x52, 0xe9, 0xcc, 0x7e, 0x74, 0xc0,
                0x74, 0x3b, 0x8f, 0xb3, 0xf7, 0x54, 0x87, 0x12, 0x0b, 0xa3, 0x26, 0xff, 0x60, 0x0a,
                0xd8, 0xb1, 0xf3, 0xe6,
            ],
            Core::FullSubtract16 => [
                0x95, 0xea, 0x5e, 0x54, 0xc5, 0x60, 0x3f, 0x2f, 0x78, 0xac, 0xf6, 0xb8, 0xa8, 0x7a,
                0x63, 0xb3, 0xac, 0xc7, 0xb6, 0x5f, 0x2b, 0x87, 0xb6, 0x90, 0x4b, 0x98, 0x30, 0xfa,
                0x91, 0x21, 0x2c, 0x8c,
            ],
            Core::FullSubtract32 => [
                0x32, 0x96, 0x35, 0x26, 0xfc, 0x60, 0x89, 0xf5, 0xec, 0x7d, 0xa5, 0x84, 0xfb, 0xee,
                0x32, 0x37, 0x63, 0x1c, 0x9b, 0x12, 0x81, 0xf6, 0xf4, 0x46, 0xd9, 0x9c, 0x9b, 0x50,
                0xc8, 0x0c, 0x76, 0xdb,
            ],
            Core::FullSubtract64 => [
                0x15, 0xd4, 0x8a, 0x43, 0x24, 0x79, 0xb4, 0x51, 0xa6, 0xe2, 0xc1, 0x8f, 0x43, 0xfb,
                0x0d, 0xfc, 0x9a, 0xeb, 0x6f, 0xce, 0x04, 0x03, 0x50, 0x27, 0xb0, 0x33, 0xaa, 0x99,
                0xfb, 0x14, 0x34, 0x47,
            ],
            Core::FullSubtract8 => [
                0x2b, 0xd4, 0xf0, 0xb3, 0xa0, 0xa7, 0x58, 0xac, 0x39, 0xf1, 0x58, 0x1b, 0x2c, 0x34,
                0xd7, 0xf4, 0x14, 0xdb, 0x4d, 0x8e, 0x1b, 0xc1, 0x19, 0xd7, 0xf8, 0x92, 0x91, 0x35,
                0x48, 0x0e, 0x9a, 0xff,
            ],
            Core::GeIsOnCurve => [
                0x05, 0x72, 0xae, 0xf3, 0x63, 0x00, 0x50, 0x8c, 0x96, 0xed, 0xa4, 0xa4, 0xfe, 0xc8,
                0x57, 0x9d, 0x8d, 0x8a, 0x43, 0xbd, 0x54, 0x0c, 0xb6, 0xdd, 0xec, 0xc8, 0x2a, 0x49,
                0xe6, 0x68, 0xf7, 0xde,
            ],
            Core::GeNegate => [
                0x90, 0xf3, 0x3d, 0x91, 0x0f, 0x85, 0x8f, 0x9b, 0x3b, 0x84, 0x0d, 0xdb, 0xc0, 0x3d,
                0x8a, 0x39, 0xaf, 0x81, 0x02, 0x14, 0x85, 0x3b, 0xad, 0x3c, 0x90, 0x5f, 0x18, 0x29,
                0x6f, 0xdf, 0xcb, 0x58,
            ],
            Core::GejAdd => [
                0x00, 0x87, 0x1c, 0x07, 0x66, 0x28, 0x6a, 0x50, 0x0a, 0xce, 0xa0, 0x5f, 0x7a, 0x1f,
                0xd9, 0x8c, 0x3e, 0x52, 0x46, 0xc1, 0x94, 0x67, 0x71, 0xe4, 0x73, 0x8e, 0x75, 0x82,
                0x09, 0x1c, 0xca, 0x0d,
            ],
            Core::GejDouble => [
                0x5f, 0xc4, 0x80, 0xc5, 0x05, 0xb6, 0x7d, 0xdd, 0xb9, 0xce, 0x84, 0xb0, 0xa8, 0xee,
                0x5a, 0x7e, 0x4e, 0xf0, 0xe9, 0x36, 0x59, 0xd4, 0x45, 0x21, 0xe6, 0xa6, 0x64, 0xac,
                0x5d, 0x65, 0x56, 0xf8,
            ],
            Core::GejGeAdd => [
                0xa2, 0x4d, 0x04, 0x9f, 0xf1, 0xe4, 0xcf, 0x37, 0x84, 0xc1, 0xb7, 0xd6, 0xd1, 0xba,
                0x09, 0xfd, 0x17, 0xfe, 0xcf, 0xeb, 0x55, 0x80, 0xea, 0xb5, 0xf1, 0x1e, 0x8f, 0x8e,
                0xb9, 0xd9, 0xad, 0xef,
            ],
            Core::GejGeAddEx => [
                0x1e, 0x0e, 0x26, 0xd0, 0xb2, 0x9a, 0xe6, 0x3d, 0x41, 0xe6, 0x76, 0x7e, 0x01, 0x6e,
                0x7e, 0x24, 0x86, 0xe4, 0xf5, 0xd8, 0xdc, 0x2c, 0xf6, 0x65, 0x02, 0x22, 0x03, 0x16,
                0x40, 0xf1, 0x73, 0x3a,
            ],
            Core::GejInfinity => [
                0x88, 0xd4, 0x64, 0x2c, 0xfc, 0x2b, 0x52, 0xd0, 0x90, 0xce, 0x6e, 0x89, 0x5c, 0x20,
                0xda, 0x2e, 0xfb, 0x0d, 0xf6, 0xfe, 0x84, 0xf2, 0x27, 0x22, 0xbc, 0x46, 0x11, 0x1c,
                0xc6, 0xbe, 0x5c, 0xda,
            ],
            Core::GejIsInfinity => [
                0x80, 0xf5, 0x28, 0xe5, 0xd8, 0x56, 0x72, 0xdc, 0x8d, 0x9c, 0x26, 0x4f, 0x67, 0xc7,
                0xb7, 0x27, 0x00, 0xfa, 0xad, 0x89, 0x97, 0x2a, 0x7e, 0x1d, 0x27, 0xd0, 0x49, 0xc9,
                0x47, 0x4b, 0x6c, 0xd9,
            ],
            Core::GejIsOnCurve => [
                0x70, 0xce, 0x4f, 0xfc, 0xe2, 0x49, 0x7c, 0xc6, 0x2f, 0x17, 0x0c, 0x57, 0x14, 0xff,
                0x2c, 0xfe, 0xce, 0x90, 0xb4, 0xcb, 0x89, 0xa6, 0xa2, 0x2f, 0xac, 0x26, 0xb1, 0xb5,
                0xc6, 0x6f, 0xaa, 0x10,
            ],
            Core::GejNegate => [
                0xad, 0x53, 0xf1, 0x79, 0x3f, 0xe0, 0x7b, 0x8d, 0x67, 0x2d, 0x9f, 0x7b, 0x07, 0x41,
                0xe9, 0xed, 0x61, 0x55, 0x7e, 0xff, 0x5b, 0x72, 0x96, 0xc5, 0x68, 0xe0, 0x9b, 0x3d,
                0x19, 0xcf, 0x71, 0x24,
            ],
            Core::GejNormalize => [
                0x03, 0x33, 0xdf, 0x98, 0xa8, 0x06, 0x0c, 0x93, 0x15, 0xc5, 0xfd, 0xb8, 0x3e, 0xbf,
                0xfe, 0x34, 0x07, 0xbf, 0x9c, 0x33, 0x6b, 0xf4, 0xbb, 0x92, 0x2d, 0xf5, 0x85, 0x88,
                0x7e, 0x1a, 0xcd, 0xc7,
            ],
            Core::GejRescale => [
                0xf0, 0x0a, 0xd1, 0x8f, 0xb4, 0x92, 0x8c, 0xfa, 0xd0, 0x2c, 0x5b, 0x9d, 0x8b, 0x6f,
                0xd4, 0xb0, 0x5d, 0x7c, 0xb5, 0x49, 0xee, 0x65, 0x98, 0x4d, 0x02, 0x2a, 0x6d, 0xf9,
                0x87, 0x12, 0xb6, 0xd9,
            ],
            Core::GejXEquiv => [
                0xe3, 0x1c, 0x0f, 0x2c, 0x5d, 0x08, 0x13, 0x9b, 0x4f, 0xeb, 0x09, 0x85, 0x2c, 0x06,
                0xb6, 0xaa, 0x00, 0xb1, 0xd1, 0x3e, 0x62, 0xba, 0xbd, 0x99, 0x82, 0x82, 0x12, 0xdc,
                0xff, 0x82, 0x21, 0x7c,
            ],
            Core::GejYIsOdd => [
                0xe3, 0x4c, 0x86, 0x7d, 0xe1, 0x6b, 0x2f, 0x65, 0x61, 0x09, 0xa7, 0x38, 0x72, 0xb0,
                0xb5, 0xba, 0x55, 0xca, 0x3c, 0x2d, 0xbe, 0xa9, 0xc2, 0xc6, 0xe4, 0xcb, 0x19, 0xad,
                0x18, 0xc0, 0x6f, 0x56,
            ],
            Core::Generate => [
                0xa3, 0xc5, 0x5b, 0xef, 0x32, 0xa3, 0x50, 0xd9, 0x0d, 0x5c, 0x3d, 0xac, 0x24, 0x76,
                0x7a, 0x03, 0x86, 0x7f, 0xaf, 0x7a, 0x73, 0x27, 0x77, 0x03, 0x89, 0x5a, 0x27, 0xcb,
                0x6b, 0x44, 0x25, 0x2d,
            ],
            Core::High16 => [
                0x62, 0x10, 0xac, 0x71, 0x36, 0x58, 0x6c, 0x73, 0xa0, 0x9c, 0x94, 0x21, 0xa4, 0x0e,
                0x30, 0x8c, 0x44, 0x91, 0xea, 0xce, 0x9b, 0x5b, 0x36, 0x95, 0xd6, 0x1f, 0x4c, 0x81,
                0x96, 0xa6, 0x9d, 0xc8,
            ],
            Core::High32 => [
                0x71, 0x94, 0x24, 0xb1, 0xac, 0xd3, 0x5b, 0x13, 0x73, 0x58, 0x06, 0x90, 0xa7, 0xec,
                0x0b, 0x8f, 0xb4, 0x86, 0x14, 0x5c, 0x9c, 0xde, 0x72, 0x8d, 0xa7, 0x98, 0x46, 0x93,
                0xe9, 0x5f, 0xc7, 0xc0,
            ],
            Core::High64 => [
                0x8c, 0x5d, 0x44, 0x09, 0x34, 0xdf, 0xdc, 0xf2, 0x75, 0xa2, 0x1c, 0xf0, 0x87, 0xac,
                0x12, 0x7d, 0xa7, 0x57, 0x15, 0xb5, 0xda, 0xc9, 0xc6, 0x93, 0xc2, 0xaf, 0xd7, 0xc7,
                0x18, 0xdc, 0x0f, 0xfb,
            ],
            Core::High8 => [
                0x3a, 0x5c, 0xe0, 0x0e, 0x15, 0xe3, 0x18, 0x08, 0x51, 0xc2, 0x00, 0x21, 0x1f, 0x1c,
                0x82, 0xda, 0xa3, 0x3e, 0xc8, 0x76, 0x38, 0x24, 0x8a, 0x4b, 0xf1, 0x13, 0x40, 0x7c,
                0x6b, 0x16, 0xac, 0x4f,
            ],
            Core::Increment16 => [
                0x80, 0xf6, 0xcb, 0xbc, 0x09, 0xb7, 0x8c, 0xea, 0x76, 0xc8, 0x13, 0x90, 0xd3, 0xed,
                0x98, 0x9b, 0x70, 0xe4, 0x39, 0x16, 0x1e, 0xff, 0xaf, 0x9a, 0x62, 0xc6, 0x4b, 0x1b,
                0x95, 0x9c, 0xd0, 0x30,
            ],
            Core::Increment32 => [
                0x5a, 0x96, 0x3c, 0xa4, 0xad, 0xa6, 0x61, 0x9a, 0x80, 0x53, 0x46, 0xe8, 0x09, 0x95,
                0x03, 0xe4, 0x78, 0x25, 0xbe, 0x5c, 0xf3, 0xc9, 0xa8, 0x9b, 0xfd, 0xbe, 0x2f, 0x19,
                0x1c, 0x32, 0xa2, 0xe0,
            ],
            Core::Increment64 => [
                0x86, 0x57, 0x52, 0x36, 0x83, 0xc2, 0xa0, 0x5c, 0x0e, 0x09, 0x98, 0xcd, 0xa7, 0xb9,
                0x4e, 0x8b, 0x03, 0x65, 0xee, 0x83, 0x22, 0xa1, 0x4c, 0x5a, 0x37, 0x22, 0xb5, 0x15,
                0xbb, 0x68, 0x1f, 0x74,
            ],
            Core::Increment8 => [
                0xd1, 0xdb, 0x8a, 0x9e, 0xce, 0xd1, 0x7e, 0x21, 0x99, 0x65, 0x26, 0xbc, 0x73, 0xbe,
                0x8e, 0x57, 0x98, 0x68, 0x8d, 0xa3, 0xc2, 0xef, 0x8b, 0x8d, 0x5b, 0xa2, 0x55, 0xd1,
                0x1e, 0xd8, 0x18, 0x3e,
            ],
            Core::IsOne16 => [
                0x87, 0x7c, 0xd0, 0x01, 0xd5, 0xe7, 0xb7, 0x4e, 0xdd, 0x1a, 0x4a, 0x5c, 0x96, 0x56,
                0x41, 0xd4, 0xfb, 0x53, 0x68, 0x2d, 0x7f, 0xef, 0xae, 0x50, 0xb5, 0x14, 0x12, 0x18,
                0xc0, 0x4b, 0xe5, 0xaf,
            ],
            Core::IsOne32 => [
                0x83, 0x35, 0x4e, 0x97, 0xd6, 0x14, 0x60, 0x0a, 0x49, 0x89, 0x4e, 0xc2, 0xc9, 0xd1,
                0x98, 0x0f, 0x9c, 0x4c, 0x92, 0x8c, 0x15, 0x61, 0xee, 0xca, 0xc9, 0x9c, 0x16, 0x81,
                0x9c, 0x2b, 0x07, 0x91,
            ],
            Core::IsOne64 => [
                0xb3, 0x55, 0xf6, 0x02, 0xec, 0x76, 0xc4, 0xc4, 0xce, 0x70, 0x77, 0x20, 0xfd, 0x54,
                0x34, 0x32, 0x22, 0xa7, 0xc8, 0xcf, 0xca, 0x43, 0x94, 0x51, 0xf2, 0x3f, 0x98, 0x54,
                0x39, 0x50, 0x78, 0xba,
            ],
            Core::IsOne8 => [
                0xf2, 0x45, 0x9c, 0xf3, 0x5b, 0x97, 0x20, 0x29, 0xfb, 0xb2, 0x2e, 0x82, 0x19, 0x1a,
                0xce, 0x11, 0x73, 0x2b, 0x1e, 0x08, 0x34, 0x6e, 0x21, 0xcf, 0x0d, 0x30, 0x5b, 0x41,
                0xfa, 0xe8, 0x79, 0x90,
            ],
            Core::IsZero16 => [
                0x6e, 0xd0, 0x33, 0x25, 0x6f, 0xe6, 0x45, 0xb5, 0xec, 0x3f, 0x59, 0x50, 0x8c, 0x60,
                0x19, 0x2a, 0xc8, 0x76, 0x30, 0x08, 0x91, 0x5c, 0x81, 0x15, 0xe9, 0x29, 0x33, 0x0c,
                0x0f, 0xcd, 0x48, 0x38,
            ],
            Core::IsZero32 => [
                0x1a, 0xf3, 0x58, 0xa0, 0x6b, 0xe9, 0x3a, 0xc6, 0xf3, 0x7c, 0xbb, 0x7d, 0x25, 0x4d,
                0x7b, 0xf9, 0xd7, 0x18, 0x77, 0x38, 0xba, 0xf1, 0xf2, 0x5d, 0x0b, 0x67, 0xc9, 0xff,
                0xe0, 0x0f, 0x6a, 0x62,
            ],
            Core::IsZero64 => [
                0xc1, 0x10, 0x0c, 0xfc, 0x16, 0x20, 0x3c, 0xa4, 0x44, 0xf1, 0x60, 0x82, 0x64, 0x5b,
                0x72, 0x4d, 0x3e, 0xcd, 0x23, 0xdf, 0x5d, 0x0c, 0xcf, 0x91, 0xf3, 0x5c, 0x5d, 0x9b,
                0x5a, 0x02, 0xa2, 0xf2,
            ],
            Core::IsZero8 => [
                0x4d, 0x58, 0x68, 0x0d, 0x8e, 0x1e, 0x86, 0x18, 0xc4, 0x3c, 0xe5, 0x25, 0xf7, 0x86,
                0x61, 0xa3, 0x01, 0x48, 0x67, 0x58, 0x61, 0xa6, 0x12, 0xd1, 0xbf, 0xcf, 0xe3, 0xbb,
                0x5f, 0xa4, 0xca, 0x95,
            ],
            Core::Le16 => [
                0xd6, 0x17, 0xfe, 0xea, 0xfd, 0x6f, 0xfc, 0x23, 0xfe, 0xff, 0xbe, 0x70, 0x12, 0xce,
                0x3a, 0x03, 0x02, 0xd4, 0xd1, 0x11, 0x66, 0x58, 0x22, 0xb3, 0x04, 0xb7, 0x9a, 0xdb,
                0xcc, 0x9a, 0x16, 0xd7,
            ],
            Core::Le32 => [
                0x45, 0x74, 0x5c, 0x5b, 0xc7, 0xf8, 0x97, 0x8f, 0x85, 0xb1, 0xb1, 0x4d, 0x49, 0x4a,
                0xf2, 0x1a, 0x8a, 0x51, 0xcc, 0xd8, 0x7f, 0x3f, 0xed, 0xe9, 0x59, 0x74, 0x95, 0x91,
                0x32, 0xae, 0xf9, 0xce,
            ],
            Core::Le64 => [
                0x3a, 0x8f, 0x08, 0x70, 0x90, 0x0a, 0xf7, 0x1d, 0x42, 0x42, 0xfe, 0x26, 0xad, 0x4d,
                0xfb, 0xed, 0x92, 0xf3, 0x0b, 0xb7, 0x9b, 0x72, 0x73, 0x7d, 0xbc, 0xab, 0x9a, 0xc5,
                0xc0, 0x70, 0xab, 0xe5,
            ],
            Core::Le8 => [
                0x5e, 0xcc, 0x9b, 0x33, 0x25, 0x44, 0x67, 0x49, 0xaf, 0xa4, 0x09, 0x65, 0xea, 0x21,
                0xe0, 0x11, 0x18, 0xfb, 0x8c, 0x1a, 0xdc, 0xdc, 0x11, 0x21, 0x97, 0x93, 0xcb, 0x2a,
                0xfc, 0x4a, 0x7e, 0x8c,
            ],
            Core::LinearCombination1 => [
                0xd8, 0x83, 0x20, 0xf4, 0x71, 0xf3, 0xbe, 0xee, 0xa1, 0x31, 0x3d, 0x55, 0x1e, 0x41,
                0x9b, 0xe0, 0x57, 0x27, 0xae, 0x5f, 0x4d, 0xe6, 0xa2, 0xf2, 0xf2, 0x6f, 0x3c, 0xb5,
                0xe8, 0xdd, 0xdd, 0x3f,
            ],
            Core::LinearVerify1 => [
                0x0c, 0x9f, 0xae, 0x64, 0xa6, 0x4c, 0xd6, 0x3c, 0xa8, 0x5b, 0xeb, 0xa7, 0x5c, 0x9f,
                0x2c, 0x6e, 0x85, 0x34, 0x3b, 0x74, 0xf2, 0x86, 0x34, 0xea, 0x85, 0xf7, 0x0f, 0xc3,
                0x41, 0xcc, 0xaf, 0xf3,
            ],
            Core::Low16 => [
                0xa1, 0x14, 0xe9, 0x58, 0x0d, 0xe0, 0x7d, 0x8b, 0x07, 0x7e, 0xb8, 0x89, 0x98, 0x75,
                0x5a, 0x0a, 0x62, 0xbf, 0xe0, 0x85, 0xfb, 0x23, 0x40, 0x4c, 0xd1, 0xe8, 0x78, 0x68,
                0xcd, 0x56, 0xd5, 0xbd,
            ],
            Core::Low32 => [
                0xa7, 0x17, 0x61, 0x0e, 0x8c, 0x57, 0x71, 0x25, 0x51, 0x40, 0xd6, 0x20, 0x7f, 0xff,
                0x3b, 0xdd, 0x34, 0x64, 0xac, 0xff, 0x59, 0x98, 0xe1, 0x29, 0xaf, 0x8b, 0x9f, 0x4c,
                0x0e, 0x21, 0xb2, 0x3d,
            ],
            Core::Low64 => [
                0x9a, 0x6f, 0x4a, 0x0a, 0xd2, 0xfb, 0x72, 0xc5, 0x79, 0x42, 0x4c, 0x72, 0xe1, 0x7d,
                0xaa, 0xe6, 0x2b, 0x36, 0x66, 0x95, 0xcd, 0x1a, 0x26, 0x85, 0xfb, 0x0a, 0xb0, 0x2e,
                0x4c, 0x2c, 0xc0, 0xf5,
            ],
            Core::Low8 => [
                0x69, 0xc9, 0xb9, 0xca, 0xb9, 0xf4, 0x4c, 0xff, 0xae, 0xdf, 0x1c, 0x84, 0x31, 0x46,
                0xbc, 0xc0, 0xb0, 0x3b, 0x0f, 0x0c, 0x13, 0x48, 0x66, 0xaf, 0xd2, 0x3a, 0x61, 0x4f,
                0x01, 0xa7, 0x0a, 0x24,
            ],
            Core::Lt16 => [
                0x4f, 0x46, 0x5a, 0x49, 0xa9, 0x63, 0xac, 0xce, 0x93, 0xf6, 0xb6, 0xf8, 0x23, 0xeb,
                0x94, 0x72, 0xb4, 0xcc, 0x21, 0xf6, 0xe5, 0x8b, 0x76, 0x57, 0x05, 0x08, 0xba, 0xbd,
                0xf4, 0x4a, 0x8c, 0x97,
            ],
            Core::Lt32 => [
                0xd3, 0x42, 0x1d, 0xfc, 0x84, 0x67, 0x1c, 0xd7, 0x44, 0x50, 0x82, 0xa9, 0x86, 0xdf,
                0x16, 0x28, 0xed, 0xca, 0xcb, 0xf4, 0x29, 0xa3, 0xc8, 0x09, 0xd2, 0x36, 0x54, 0x97,
                0xe1, 0x17, 0x0b, 0xfd,
            ],
            Core::Lt64 => [
                0x7c, 0x67, 0xaa, 0x89, 0x93, 0xfc, 0xda, 0x59, 0x30, 0xf7, 0x9d, 0xb2, 0x00, 0x8a,
                0xa6, 0xd7, 0xbe, 0x5f, 0x3b, 0xed, 0x5a, 0xca, 0x5e, 0x03, 0x2d, 0xcc, 0xcf, 0x0b,
                0xe8, 0x4f, 0x62, 0xd1,
            ],
            Core::Lt8 => [
                0x98, 0x76, 0x3f, 0x78, 0x21, 0x69, 0x09, 0x54, 0xcf, 0x50, 0x81, 0x02, 0x09, 0xdf,
                0x6e, 0x15, 0x57, 0x03, 0x16, 0xbb, 0xa8, 0x9f, 0xfa, 0x9a, 0xe5, 0x56, 0xa9, 0x15,
                0xf3, 0x7b, 0x64, 0x0b,
            ],
            Core::Maj16 => [
                0xf5, 0xa4, 0x1d, 0xa0, 0x37, 0x7f, 0xe6, 0x88, 0xac, 0x2f, 0xcd, 0xf3, 0x5b, 0x6b,
                0x7a, 0x47, 0x2e, 0x78, 0xea, 0x69, 0xfd, 0x2b, 0x17, 0xf7, 0x56, 0x34, 0x2b, 0xaa,
                0x1f, 0x8b, 0x9f, 0xdd,
            ],
            Core::Maj32 => [
                0xf1, 0xd9, 0x3b, 0x04, 0x6b, 0x02, 0x85, 0xf6, 0xe5, 0x20, 0x46, 0x7f, 0x26, 0x5f,
                0x6a, 0x6a, 0xe3, 0x49, 0x1f, 0x16, 0x78, 0xc8, 0xa8, 0x26, 0xa0, 0x09, 0x9b, 0x9c,
                0xd4, 0x15, 0x99, 0xd4,
            ],
            Core::Maj64 => [
                0x5e, 0x99, 0x6c, 0x51, 0x51, 0xd7, 0xac, 0xee, 0x4a, 0x7d, 0x9e, 0x22, 0x95, 0xef,
                0x8f, 0x2c, 0x75, 0x54, 0x88, 0x84, 0x4b, 0xfd, 0xe2, 0x5d, 0x5a, 0xcd, 0xfe, 0xa2,
                0x98, 0x44, 0x43, 0x5a,
            ],
            Core::Maj8 => [
                0xab, 0x6c, 0x4c, 0x41, 0xab, 0x67, 0xeb, 0xf7, 0xea, 0x60, 0x87, 0x0c, 0x9b, 0x2d,
                0x93, 0x10, 0x01, 0x74, 0x29, 0x61, 0x8d, 0x2a, 0xb9, 0x02, 0x68, 0xb5, 0xc5, 0xc2,
                0xb5, 0x37, 0x7f, 0x3b,
            ],
            Core::Max16 => [
                0xcd, 0xdd, 0x81, 0x69, 0x55, 0x22, 0x97, 0x1b, 0x52, 0x9e, 0xb0, 0xbb, 0x53, 0x81,
                0x8f, 0xe2, 0x28, 0x58, 0xac, 0x6b, 0x03, 0x7c, 0x38, 0x10, 0xec, 0x26, 0x8b, 0x53,
                0xb8, 0x17, 0xa3, 0x8d,
            ],
            Core::Max32 => [
                0xd9, 0x16, 0xf3, 0x4b, 0xd7, 0x85, 0xb9, 0x38, 0x08, 0xb6, 0x7c, 0x62, 0x28, 0x4f,
                0x94, 0xaa, 0x5d, 0xf4, 0x79, 0x19, 0x59, 0xb4, 0x9b, 0x85, 0x82, 0xb0, 0x58, 0x2b,
                0x82, 0x85, 0x38, 0x83,
            ],
            Core::Max64 => [
                0x99, 0xef, 0x5a, 0xbf, 0xf7, 0x87, 0x00, 0xbd, 0x93, 0xd1, 0xc9, 0x77, 0xfd, 0x68,
                0x6f, 0x31, 0x21, 0x0f, 0xd1, 0xce, 0x1d, 0x09, 0x4d, 0x23, 0x04, 0x8c, 0x02, 0x6c,
                0x19, 0xfc, 0x98, 0x4a,
            ],
            Core::Max8 => [
                0x3a, 0xcb, 0xf7, 0x73, 0x3f, 0xd8, 0xef, 0x4f, 0x51, 0x96, 0x7a, 0xa2, 0x45, 0x55,
                0x8b, 0x28, 0x37, 0x4d, 0x54, 0xc6, 0x32, 0xa9, 0xf4, 0x6a, 0xab, 0x6b, 0x2b, 0x1e,
                0xc2, 0xd5, 0x13, 0x9a,
            ],
            Core::Median16 => [
                0xd6, 0x46, 0x3e, 0x89, 0xdf, 0xe2, 0xc3, 0x34, 0xb0, 0x1f, 0x90, 0x97, 0xb0, 0x1a,
                0x75, 0xc4, 0x75, 0xfb, 0x0f, 0x0f, 0x16, 0x7c, 0xb6, 0xb7, 0x49, 0x76, 0xf1, 0x97,
                0xbd, 0x4c, 0x76, 0xe1,
            ],
            Core::Median32 => [
                0x01, 0xdc, 0xa1, 0xed, 0xe5, 0xb2, 0x9d, 0x88, 0x48, 0x0b, 0xc4, 0xdc, 0x43, 0xbe,
                0x4e, 0x04, 0xf2, 0xd2, 0x09, 0xee, 0x32, 0xf6, 0xcf, 0x3e, 0xc7, 0x05, 0x0d, 0xa1,
                0x07, 0x35, 0x90, 0x63,
            ],
            Core::Median64 => [
                0x70, 0x7a, 0x6f, 0xa8, 0xc1, 0x54, 0x95, 0xa2, 0xc4, 0x82, 0x22, 0x63, 0xdd, 0xce,
                0xa5, 0xe0, 0xf0, 0xe8, 0x50, 0xa9, 0x4b, 0x1c, 0xf4, 0xec, 0xc0, 0x65, 0x2f, 0xaf,
                0xee, 0xf1, 0x30, 0xe9,
            ],
            Core::Median8 => [
                0x26, 0xbf, 0xcb, 0x7b, 0xab, 0xcf, 0x26, 0xc9, 0x7a, 0x84, 0x16, 0x75, 0x45, 0x74,
                0x62, 0x76, 0x59, 0x27, 0x9a, 0xf8, 0x09, 0x55, 0x48, 0x82, 0x40, 0x0e, 0xf8, 0x74,
                0xdf, 0x1c, 0x1a, 0x31,
            ],
            Core::Min16 => [
                0x63, 0xbb, 0xc0, 0xe5, 0xdb, 0x1c, 0xe0, 0x80, 0x97, 0xd3, 0xf5, 0xbc, 0xc5, 0xbe,
                0x1d, 0x72, 0x09, 0x0b, 0xab, 0x4a, 0x87, 0xc1, 0x0c, 0x80, 0x05, 0x49, 0xca, 0x84,
                0xef, 0xbf, 0xe5, 0xe3,
            ],
            Core::Min32 => [
                0xaf, 0x4d, 0x61, 0x84, 0x2f, 0x47, 0x90, 0x32, 0x5c, 0x61, 0x45, 0x56, 0xc7, 0x77,
                0x6e, 0xea, 0x45, 0x75, 0x13, 0xca, 0x42, 0x2f, 0x30, 0x5b, 0x46, 0x3f, 0x8f, 0x7d,
                0x8b, 0xda, 0x0c, 0x3c,
            ],
            Core::Min64 => [
                0xf0, 0xa3, 0x17, 0x0f, 0xb3, 0x06, 0x50, 0x90, 0x71, 0xb2, 0x14, 0x32, 0x30, 0x9c,
                0x00, 0x86, 0x5d, 0x8f, 0xb6, 0x75, 0x2d, 0xe6, 0xec, 0x41, 0x0a, 0xbb, 0x6c, 0xe3,
                0x5b, 0x4f, 0xfa, 0x9c,
            ],
            Core::Min8 => [
                0x8a, 0xa7, 0x1d, 0x09, 0x3e, 0x83, 0x8a, 0xd3, 0x6a, 0xa6, 0xfe, 0x16, 0xfd, 0x1d,
                0xde, 0x9f, 0xb5, 0x77, 0x2a, 0xc4, 0x5d, 0x6b, 0xb8, 0xe4, 0x2c, 0x88, 0x83, 0xa2,
                0xa9, 0xf1, 0x00, 0xe2,
            ],
            Core::Modulo16 => [
                0x0e, 0xa8, 0x98, 0x59, 0x79, 0x20, 0x9d, 0x31, 0x94, 0xc5, 0x59, 0x12, 0xf2, 0x1a,
                0x65, 0x77, 0xda, 0x30, 0xc5, 0x17, 0xe0, 0x08, 0xf8, 0x51, 0xc3, 0x52, 0x08, 0x5b,
                0xb0, 0x67, 0xa0, 0xf1,
            ],
            Core::Modulo32 => [
                0x22, 0x3f, 0xb5, 0x83, 0x2b, 0x95, 0x59, 0xca, 0x86, 0x51, 0x72, 0x6a, 0xb7, 0x95,
                0x6e, 0x58, 0xe5, 0x3a, 0x40, 0x2d, 0xf9, 0x60, 0x34, 0x86, 0x18, 0x18, 0x07, 0xa2,
                0xfc, 0xae, 0xd4, 0x87,
            ],
            Core::Modulo64 => [
                0x82, 0x59, 0xa2, 0xe3, 0xb0, 0x78, 0x9c, 0x9d, 0x31, 0xea, 0xbc, 0x49, 0x39, 0xd3,
                0x1b, 0xc5, 0x25, 0x4c, 0x56, 0x65, 0xc4, 0x12, 0x05, 0x2c, 0x12, 0xd7, 0x2e, 0xcd,
                0xfb, 0xed, 0x52, 0x25,
            ],
            Core::Modulo8 => [
                0xf8, 0xfd, 0x7c, 0x21, 0xc1, 0x5e, 0xbc, 0x5a, 0x04, 0xb3, 0xd3, 0xbc, 0x0a, 0xd6,
                0x47, 0xe2, 0x1d, 0x89, 0xf8, 0x00, 0x8d, 0xdd, 0xdf, 0xfe, 0xbb, 0x00, 0x3e, 0x35,
                0x52, 0xbd, 0xb5, 0x70,
            ],
            Core::Multiply16 => [
                0xd6, 0x6b, 0x0a, 0xd2, 0x1c, 0xdb, 0xe6, 0x01, 0x4f, 0x26, 0x19, 0xf4, 0x40, 0xd9,
                0x35, 0xf6, 0x42, 0xe9, 0x7c, 0x59, 0x4e, 0x3b, 0x44, 0x1f, 0x01, 0x98, 0x0e, 0xea,
                0x08, 0xc4, 0x85, 0x91,
            ],
            Core::Multiply32 => [
                0xc6, 0x7f, 0x65, 0xaf, 0xae, 0x79, 0x10, 0x8c, 0xe5, 0xf7, 0x24, 0xf4, 0x01, 0x81,
                0x7a, 0xa1, 0xb4, 0xa7, 0x60, 0x8a, 0x4e, 0x18, 0xee, 0x74, 0x82, 0x6b, 0x37, 0xf7,
                0x94, 0x9c, 0x77, 0xd6,
            ],
            Core::Multiply64 => [
                0x16, 0xac, 0x08, 0xfe, 0xc9, 0xd9, 0x66, 0x52, 0x7f, 0x80, 0xc3, 0x9c, 0x64, 0xa4,
                0xfc, 0x10, 0x82, 0xb1, 0x50, 0xbd, 0xbb, 0x05, 0xf7, 0xf6, 0xc4, 0xb9, 0xa0, 0x0e,
                0xbd, 0x89, 0x4d, 0xba,
            ],
            Core::Multiply8 => [
                0xce, 0xc3, 0xd5, 0xc3, 0x2d, 0xee, 0x35, 0x2e, 0x75, 0x4f, 0x14, 0x1c, 0x02, 0xef,
                0x2b, 0x60, 0xf8, 0x6e, 0x27, 0xd2, 0xcb, 0xe2, 0x73, 0x26, 0x06, 0x0d, 0xec, 0x9e,
                0x7b, 0xcc, 0x20, 0x6b,
            ],
            Core::Negate16 => [
                0x81, 0xf3, 0xef, 0x08, 0x2b, 0x19, 0x9d, 0x2e, 0x47, 0x45, 0x35, 0xfb, 0x84, 0x28,
                0x2f, 0x21, 0x11, 0x1c, 0x1a, 0x04, 0x97, 0xa9, 0x16, 0x95, 0xe7, 0x2c, 0x4d, 0x51,
                0xaf, 0x1c, 0xb0, 0x57,
            ],
            Core::Negate32 => [
                0xba, 0x17, 0x3b, 0xde, 0xf2, 0x4e, 0x63, 0x7f, 0x4a, 0x5a, 0x0c, 0x6f, 0xed, 0x4b,
                0xbc, 0x22, 0xc7, 0xd0, 0x8b, 0xcc, 0x20, 0xfd, 0xca, 0x61, 0x70, 0x36, 0x3d, 0x25,
                0x26, 0x95, 0x58, 0x36,
            ],
            Core::Negate64 => [
                0xe7, 0x13, 0x27, 0x2d, 0x6d, 0x9a, 0x6d, 0x69, 0x6c, 0x46, 0x88, 0x9d, 0xfb, 0x27,
                0x04, 0x75, 0xe9, 0x23, 0x8d, 0x17, 0x72, 0xab, 0xf9, 0x16, 0x5b, 0x09, 0x3c, 0x79,
                0x98, 0xee, 0x80, 0x70,
            ],
            Core::Negate8 => [
                0x8b, 0xe8, 0x71, 0x39, 0x3e, 0x18, 0xc0, 0x63, 0x69, 0x16, 0x1f, 0xb1, 0xc0, 0xad,
                0x2b, 0x92, 0x06, 0x22, 0x97, 0x4e, 0x3f, 0xdf, 0xca, 0x90, 0x56, 0x7e, 0x6d, 0xa4,
                0x29, 0xfd, 0x98, 0x42,
            ],
            Core::One16 => [
                0x76, 0x29, 0x79, 0x2b, 0xa1, 0xa4, 0x41, 0x14, 0xe5, 0x9e, 0x64, 0xb5, 0x1d, 0x18,
                0x7a, 0xf6, 0xd9, 0x65, 0xd9, 0x66, 0x20, 0xdc, 0xa2, 0x62, 0x75, 0x07, 0xb7, 0xd3,
                0x88, 0xd9, 0x35, 0x3c,
            ],
            Core::One32 => [
                0x0b, 0xa0, 0x04, 0xce, 0xa2, 0x50, 0xfe, 0x95, 0x3a, 0xc7, 0x4e, 0x6e, 0xcd, 0x36,
                0x20, 0xe8, 0x02, 0x84, 0x1f, 0xda, 0x79, 0x52, 0x08, 0xde, 0xc6, 0x6d, 0x62, 0x6e,
                0x06, 0xaa, 0x29, 0xb2,
            ],
            Core::One64 => [
                0xba, 0x34, 0x78, 0xc1, 0x08, 0x74, 0x0a, 0x83, 0xf0, 0xca, 0x0e, 0xae, 0x86, 0xc3,
                0x1b, 0x4a, 0xfa, 0xc8, 0x30, 0xdf, 0x09, 0x34, 0x67, 0xcc, 0x08, 0xea, 0x1c, 0x04,
                0x15, 0xef, 0xef, 0x6d,
            ],
            Core::One8 => [
                0x15, 0xca, 0x77, 0xa4, 0xb7, 0x02, 0x25, 0x68, 0x37, 0xf9, 0x0f, 0xf7, 0x8c, 0xa7,
                0x74, 0x0a, 0x40, 0xfe, 0xce, 0x71, 0x91, 0x30, 0x1d, 0x00, 0xe5, 0x17, 0xd8, 0xd3,
                0x4f, 0x46, 0xc2, 0x50,
            ],
            Core::Or16 => [
                0xdf, 0xea, 0xd0, 0xba, 0x93, 0xe4, 0x91, 0x55, 0xc4, 0x0c, 0xb3, 0x72, 0xca, 0x5e,
                0xf6, 0x17, 0x97, 0x41, 0xc6, 0x1f, 0x2b, 0x3c, 0xc2, 0x79, 0x7e, 0xf1, 0x62, 0xc8,
                0xd2, 0xfc, 0x1f, 0x9a,
            ],
            Core::Or32 => [
                0x67, 0xac, 0x69, 0x45, 0xcd, 0xc0, 0x06, 0xd2, 0x5e, 0x5b, 0xbe, 0x6c, 0x4f, 0xe8,
                0x1c, 0xa1, 0x67, 0x41, 0xff, 0xab, 0x3d, 0x23, 0x96, 0x0e, 0xeb, 0x49, 0x85, 0x0f,
                0x92, 0x73, 0x2c, 0xbb,
            ],
            Core::Or64 => [
                0x1e, 0xb9, 0x52, 0xe4, 0x61, 0x16, 0xe2, 0x71, 0xdc, 0x48, 0x9e, 0x67, 0x22, 0xa9,
                0x01, 0x85, 0xeb, 0xf5, 0xfb, 0x77, 0x5b, 0x77, 0x81, 0x60, 0x6d, 0xbf, 0x5e, 0x89,
                0xcc, 0xd7, 0xc2, 0x50,
            ],
            Core::Or8 => [
                0x37, 0x8f, 0x7a, 0xbe, 0x8b, 0x08, 0x1f, 0xaf, 0x5b, 0x3a, 0x25, 0x78, 0xef, 0x19,
                0x79, 0xfe, 0x80, 0xbc, 0xcb, 0x07, 0x91, 0x15, 0x6a, 0x49, 0x3f, 0x8a, 0x3f, 0x6e,
                0x0b, 0xb2, 0xfc, 0x84,
            ],
            Core::ParseLock => [
                0x45, 0x71, 0xca, 0x04, 0x52, 0x57, 0x7c, 0xfa, 0xf8, 0xa8, 0x5f, 0x5e, 0x32, 0x9d,
                0x60, 0x2c, 0x8d, 0xca, 0xd1, 0x27, 0x6a, 0x97, 0x4e, 0x2c, 0x75, 0xd2, 0xec, 0x04,
                0x65, 0x7b, 0xc4, 0x25,
            ],
            Core::ParseSequence => [
                0x55, 0xc9, 0x03, 0x66, 0x0c, 0x8c, 0x92, 0xb1, 0x42, 0x9f, 0x63, 0xe2, 0x74, 0x71,
                0xac, 0xd5, 0x9c, 0x3a, 0x26, 0xaa, 0x7a, 0x59, 0x22, 0x1d, 0x7a, 0x6d, 0x52, 0xb6,
                0x2e, 0xba, 0xf9, 0x4a,
            ],
            Core::PointVerify1 => [
                0x02, 0xb8, 0x9a, 0x1c, 0xa7, 0xd2, 0x4f, 0x82, 0x35, 0x3e, 0x97, 0x32, 0xfa, 0x39,
                0xce, 0x65, 0x50, 0x9b, 0x39, 0xae, 0xaf, 0x43, 0xd2, 0xe5, 0xf8, 0x0c, 0xa9, 0x03,
                0xfc, 0x81, 0xc8, 0x46,
            ],
            Core::ScalarAdd => [
                0x4e, 0xe9, 0xa9, 0x6c, 0xef, 0x49, 0x6c, 0xf4, 0xa8, 0xfc, 0x4e, 0x8a, 0x8b, 0xc0,
                0xd1, 0x59, 0xca, 0x5f, 0xfb, 0x87, 0x53, 0x64, 0x3a, 0x8a, 0xdf, 0x63, 0x8a, 0xe8,
                0x9b, 0xbb, 0xb3, 0x45,
            ],
            Core::ScalarInvert => [
                0x12, 0xb8, 0x55, 0xe5, 0xeb, 0xaa, 0x7f, 0x8b, 0xb4, 0x4f, 0xee, 0x26, 0x16, 0xa0,
                0x51, 0xad, 0x00, 0x49, 0x9f, 0x9d, 0xf2, 0xa2, 0xad, 0xf7, 0x99, 0x73, 0xe9, 0xdb,
                0x81, 0x85, 0x75, 0x9c,
            ],
            Core::ScalarIsZero => [
                0x4d, 0x25, 0x28, 0x03, 0x45, 0x7b, 0x83, 0xb8, 0x5b, 0x98, 0x7f, 0x04, 0x87, 0x33,
                0xfb, 0xee, 0xde, 0xaa, 0x8d, 0x25, 0x9d, 0x32, 0x05, 0x07, 0x45, 0x00, 0x19, 0xc6,
                0x22, 0x03, 0x4f, 0x26,
            ],
            Core::ScalarMultiply => [
                0x87, 0x56, 0xf2, 0xdc, 0x31, 0x0c, 0xde, 0xb6, 0x40, 0x45, 0xc4, 0x4c, 0x23, 0x66,
                0xe1, 0x4b, 0xc1, 0xfa, 0xfa, 0x17, 0x15, 0x9f, 0x2d, 0x7b, 0x48, 0x9b, 0xd9, 0x45,
                0x3e, 0xe3, 0x7e, 0xa0,
            ],
            Core::ScalarMultiplyLambda => [
                0x2b, 0x31, 0xd3, 0x9e, 0xc4, 0xff, 0x37, 0x23, 0x1a, 0x1b, 0x3e, 0xbe, 0x75, 0x9d,
                0x41, 0xe0, 0xf5, 0xce, 0x34, 0x49, 0x2d, 0x4b, 0xd3, 0xc2, 0x09, 0x88, 0xc3, 0xf2,
                0xf7, 0xc5, 0x3e, 0xdc,
            ],
            Core::ScalarNegate => [
                0xfc, 0x2e, 0xd1, 0x87, 0x50, 0xa2, 0x21, 0x81, 0xaf, 0x5b, 0x81, 0x41, 0x96, 0x92,
                0x73, 0xca, 0xaf, 0x72, 0xcc, 0x11, 0x31, 0xe1, 0x08, 0x2c, 0xf0, 0x08, 0xf5, 0xca,
                0x09, 0x09, 0xc2, 0x16,
            ],
            Core::ScalarNormalize => [
                0x79, 0x7c, 0xff, 0xa8, 0x08, 0x59, 0xca, 0xb7, 0xcd, 0xbf, 0x3b, 0x9d, 0xe6, 0xe0,
                0xa8, 0xb7, 0x91, 0x48, 0x4e, 0xaa, 0xba, 0xcb, 0xdf, 0xba, 0xeb, 0x01, 0xe2, 0x38,
                0x95, 0xe9, 0x61, 0x99,
            ],
            Core::ScalarSquare => [
                0x77, 0xb4, 0x3c, 0x60, 0x38, 0xad, 0x80, 0xb4, 0x6d, 0x3a, 0x76, 0xe2, 0x12, 0xb3,
                0xa8, 0xc0, 0xd2, 0xf0, 0x63, 0x07, 0xbc, 0x45, 0x6e, 0x40, 0xb5, 0xd6, 0xf4, 0xa3,
                0xa5, 0x0e, 0x26, 0x4d,
            ],
            Core::Scale => [
                0xb8, 0xa8, 0x0c, 0x64, 0x95, 0x49, 0x12, 0xe5, 0x43, 0xd4, 0x2c, 0x1a, 0xf3, 0x15,
                0x32, 0x1b, 0xcf, 0xfb, 0x66, 0x68, 0x00, 0x50, 0xfc, 0x09, 0xa5, 0xaf, 0xd7, 0x56,
                0x01, 0x1b, 0x82, 0x84,
            ],
            Core::Sha256Block => [
                0x94, 0xa3, 0x6a, 0x40, 0x83, 0x30, 0x9e, 0x0b, 0x86, 0xde, 0x77, 0xd0, 0xfb, 0x48,
                0xd9, 0xd3, 0x31, 0xe2, 0xd2, 0xf1, 0x67, 0x74, 0x0b, 0x60, 0x6e, 0x60, 0x57, 0x4a,
                0xf4, 0x38, 0xcd, 0x86,
            ],
            Core::Sha256Ctx8Add1 => [
                0x8a, 0x1d, 0x25, 0x70, 0x87, 0xb3, 0x2c, 0xcd, 0xc3, 0x32, 0x00, 0x37, 0x4e, 0x6e,
                0x95, 0xc8, 0x75, 0xa0, 0x5e, 0x54, 0x81, 0x22, 0x32, 0x3f, 0x6b, 0x7a, 0xb9, 0xc0,
                0x7e, 0xb9, 0xb6, 0xee,
            ],
            Core::Sha256Ctx8Add128 => [
                0xe7, 0x77, 0x2c, 0xb9, 0xc0, 0xec, 0x42, 0x1a, 0xb8, 0xa7, 0x89, 0xd4, 0x5c, 0xd6,
                0x46, 0x61, 0xf4, 0x33, 0xdd, 0x7d, 0x3d, 0x2c, 0x94, 0xdc, 0x1f, 0x1c, 0x4f, 0x3a,
                0xf8, 0xc7, 0x80, 0xab,
            ],
            Core::Sha256Ctx8Add16 => [
                0xf0, 0xf4, 0xcf, 0x99, 0xad, 0x58, 0xa0, 0x38, 0x5e, 0x16, 0xb8, 0x7d, 0xbf, 0x32,
                0x71, 0xc4, 0x75, 0x24, 0xde, 0xfd, 0x78, 0xf1, 0x1b, 0xbc, 0x74, 0x71, 0xfe, 0x4d,
                0xa9, 0x4a, 0xeb, 0xad,
            ],
            Core::Sha256Ctx8Add2 => [
                0x79, 0x98, 0xc7, 0xc2, 0xbd, 0x81, 0x4b, 0x0c, 0x0f, 0x40, 0x3f, 0x58, 0xc5, 0x76,
                0xea, 0x56, 0x40, 0x7d, 0x25, 0xee, 0x22, 0x9f, 0xae, 0x62, 0x5b, 0xca, 0xb0, 0xc6,
                0x20, 0xa2, 0xa2, 0x9c,
            ],
            Core::Sha256Ctx8Add256 => [
                0xa8, 0x74, 0x99, 0x61, 0xb3, 0x1a, 0xfe, 0x2f, 0xb5, 0x53, 0xb7, 0x0b, 0x4c, 0xea,
                0x78, 0x7d, 0xca, 0x47, 0x25, 0x84, 0x54, 0xd5, 0x83, 0xdc, 0x45, 0xa0, 0x78, 0x0d,
                0x5e, 0x2a, 0x2a, 0x50,
            ],
            Core::Sha256Ctx8Add32 => [
                0xf2, 0x20, 0x68, 0xb7, 0x76, 0xa3, 0x78, 0x7f, 0x9d, 0x52, 0xec, 0x2a, 0x95, 0x91,
                0x95, 0x1f, 0x8a, 0x73, 0xf0, 0x09, 0x60, 0x09, 0x12, 0x35, 0x8a, 0x2a, 0x1e, 0x15,
                0x86, 0x4e, 0x80, 0xb2,
            ],
            Core::Sha256Ctx8Add4 => [
                0x40, 0xe6, 0x6d, 0xec, 0xa1, 0x32, 0xf5, 0xae, 0x0c, 0x54, 0x93, 0x7b, 0x95, 0xcc,
                0xac, 0xa1, 0x12, 0x67, 0xa4, 0xad, 0xca, 0x70, 0x28, 0x87, 0xb6, 0xe4, 0x08, 0xed,
                0x76, 0x15, 0x30, 0xbc,
            ],
            Core::Sha256Ctx8Add512 => [
                0x5a, 0x96, 0xee, 0x66, 0x8b, 0x52, 0xf6, 0x3f, 0x7b, 0xdb, 0xaa, 0xe3, 0xcc, 0x38,
                0xe3, 0x93, 0xef, 0x24, 0xb3, 0xa9, 0x67, 0x85, 0x6f, 0x2c, 0xd8, 0x9d, 0x83, 0xba,
                0x4c, 0xcb, 0xda, 0x8d,
            ],
            Core::Sha256Ctx8Add64 => [
                0x36, 0xed, 0xe6, 0xb6, 0x8a, 0xb6, 0xe4, 0xdb, 0x31, 0xef, 0xc7, 0xa7, 0xea, 0xe7,
                0xec, 0x3f, 0x7c, 0xee, 0xad, 0x93, 0x91, 0x10, 0x3b, 0x8d, 0xcb, 0x40, 0x60, 0x91,
                0x57, 0xfb, 0x87, 0x02,
            ],
            Core::Sha256Ctx8Add8 => [
                0xac, 0x1f, 0xf0, 0xfb, 0x0c, 0xf7, 0x1d, 0x71, 0x13, 0xdb, 0x42, 0xaa, 0x1d, 0xdb,
                0x89, 0x55, 0x96, 0xe6, 0x45, 0xd7, 0x63, 0xb8, 0xdf, 0x5e, 0x87, 0x25, 0xf5, 0x14,
                0x4c, 0x39, 0x95, 0x88,
            ],
            Core::Sha256Ctx8AddBuffer511 => [
                0x69, 0x01, 0xac, 0x0e, 0x30, 0xfd, 0x59, 0xce, 0xc8, 0x79, 0xd1, 0x69, 0x3b, 0x26,
                0x79, 0x59, 0x35, 0x69, 0x07, 0x84, 0x4b, 0x57, 0x7e, 0xdc, 0x3c, 0xe0, 0xe3, 0xf7,
                0x14, 0xa8, 0xef, 0x31,
            ],
            Core::Sha256Ctx8Finalize => [
                0x84, 0x61, 0x59, 0x54, 0x00, 0x7e, 0xd8, 0x23, 0xd6, 0x05, 0x46, 0xd7, 0x5b, 0x04,
                0xb9, 0x09, 0xbc, 0x90, 0x92, 0x06, 0x37, 0x14, 0x73, 0xda, 0xc7, 0x0e, 0x12, 0x68,
                0x04, 0x77, 0x08, 0xcb,
            ],
            Core::Sha256Ctx8Init => [
                0x6a, 0xdd, 0xa7, 0xd7, 0x33, 0x9f, 0x7d, 0xce, 0x4d, 0x62, 0xff, 0x82, 0x28, 0x16,
                0xda, 0x8d, 0xf5, 0x6a, 0xf6, 0x24, 0x3f, 0xa0, 0x73, 0xa2, 0x5c, 0x4c, 0x7c, 0xb5,
                0x7e, 0x01, 0x12, 0xb3,
            ],
            Core::Sha256Iv => [
                0x6a, 0x18, 0xe6, 0xe7, 0x64, 0xaf, 0x80, 0x0d, 0xc6, 0xfa, 0xeb, 0x07, 0x54, 0xbf,
                0x0b, 0x17, 0x32, 0x9f, 0x98, 0x28, 0x1d, 0x13, 0xf5, 0x15, 0x77, 0x00, 0xfa, 0x6a,
                0x1d, 0x6d, 0x5d, 0x42,
            ],
            Core::Some16 => [
                0x30, 0xd8, 0x14, 0xff, 0xb4, 0x92, 0x78, 0xb4, 0x25, 0x00, 0x7b, 0x9d, 0xe2, 0x79,
                0xf7, 0x6f, 0x4a, 0x6d, 0xa4, 0xc0, 0x34, 0x63, 0x4a, 0xbb, 0x87, 0x11, 0x0e, 0xcb,
                0xea, 0x2c, 0xe4, 0x29,
            ],
            Core::Some32 => [
                0x69, 0x27, 0x91, 0x90, 0x3b, 0xe7, 0xd9, 0xe4, 0xc5, 0x47, 0x72, 0xba, 0x88, 0xa4,
                0x86, 0x15, 0x46, 0x36, 0x12, 0x18, 0xdd, 0x8a, 0x26, 0xce, 0xed, 0x69, 0x9b, 0xcf,
                0x77, 0xc0, 0x99, 0x09,
            ],
            Core::Some64 => [
                0xfa, 0x9b, 0x01, 0x60, 0xc7, 0x27, 0x15, 0xff, 0xfd, 0x1d, 0x94, 0xda, 0x19, 0x97,
                0x88, 0x28, 0x09, 0xb4, 0x62, 0xbb, 0x14, 0x4a, 0xce, 0xcb, 0x43, 0x05, 0x44, 0x68,
                0xe6, 0x37, 0x86, 0xb5,
            ],
            Core::Some8 => [
                0x0b, 0xd7, 0xab, 0x43, 0x80, 0xc8, 0xf4, 0x5a, 0xca, 0x7c, 0xac, 0x97, 0x28, 0x86,
                0xce, 0xef, 0x23, 0xba, 0x84, 0x21, 0x0c, 0x5c, 0x4d, 0x96, 0x9b, 0x1f, 0x59, 0xb1,
                0x83, 0x2c, 0x1d, 0x36,
            ],
            Core::Subtract16 => [
                0x0c, 0xd3, 0xab, 0x73, 0xe5, 0xce, 0x2a, 0x44, 0xf2, 0xd1, 0xc3, 0x32, 0xa1, 0xed,
                0x5a, 0xef, 0x5b, 0xcb, 0x60, 0x4f, 0x72, 0x1b, 0x15, 0xb8, 0x01, 0x0d, 0xd5, 0x4f,
                0x40, 0xc6, 0xac, 0xa7,
            ],
            Core::Subtract32 => [
                0xe0, 0xde, 0x68, 0x76, 0x25, 0x9d, 0x5a, 0x00, 0x4b, 0x30, 0x16, 0xd3, 0x58, 0x15,
                0xdb, 0x41, 0xbc, 0xec, 0xdb, 0xfa, 0x18, 0xd3, 0x7d, 0x99, 0x20, 0x4e, 0x49, 0x57,
                0xad, 0x2c, 0x4d, 0x0e,
            ],
            Core::Subtract64 => [
                0xff, 0xe6, 0x73, 0xee, 0x8e, 0xdc, 0x72, 0x9a, 0x47, 0xec, 0xed, 0x3a, 0x67, 0x7f,
                0x85, 0xb1, 0xda, 0xe7, 0x14, 0xa5, 0x10, 0x0c, 0x49, 0x49, 0x73, 0xab, 0xcb, 0x32,
                0x59, 0xa3, 0xc7, 0x56,
            ],
            Core::Subtract8 => [
                0xbd, 0xa5, 0x98, 0x13, 0x96, 0x47, 0x8e, 0xb3, 0x6f, 0x85, 0x92, 0xa7, 0x50, 0x9f,
                0xa4, 0x87, 0x7c, 0x50, 0xb2, 0xbf, 0x91, 0x65, 0xe5, 0xb7, 0x96, 0x35, 0xbf, 0x8b,
                0xcb, 0x84, 0xd4, 0x42,
            ],
            Core::Verify => [
                0xf1, 0x1c, 0x94, 0x81, 0xe7, 0x18, 0x63, 0xa2, 0x44, 0x53, 0xc3, 0xe2, 0x13, 0x04,
                0x64, 0x99, 0xa0, 0x3a, 0x9a, 0x0f, 0x99, 0x3b, 0xe3, 0xc4, 0x8e, 0x9d, 0x1f, 0x40,
                0x5d, 0x97, 0x94, 0x7c,
            ],
            Core::Xor16 => [
                0xca, 0x36, 0x35, 0x51, 0x35, 0xa8, 0x6a, 0x11, 0x68, 0x6c, 0x01, 0xaa, 0x35, 0xf2,
                0x5b, 0x97, 0xfa, 0xee, 0xda, 0xbf, 0xde, 0xc8, 0xdf, 0x08, 0xd2, 0xc0, 0xf6, 0x65,
                0x08, 0x33, 0xf9, 0x3f,
            ],
            Core::Xor32 => [
                0xff, 0xe2, 0xc8, 0xee, 0x96, 0xd5, 0x57, 0x97, 0x81, 0xc4, 0x36, 0x62, 0x88, 0xd9,
                0x50, 0x71, 0x85, 0xe4, 0x61, 0xcc, 0xc4, 0x0a, 0x45, 0xbb, 0xcc, 0x55, 0x94, 0x89,
                0xd3, 0xc6, 0x96, 0x12,
            ],
            Core::Xor64 => [
                0x5b, 0x3b, 0xb7, 0xb1, 0x8c, 0x70, 0x98, 0xf4, 0xe5, 0xcd, 0x14, 0x97, 0x22, 0xac,
                0x73, 0x09, 0xce, 0x66, 0xa3, 0xe2, 0x19, 0xc6, 0x1b, 0x33, 0x3f, 0x31, 0x33, 0x58,
                0x3a, 0x7b, 0x57, 0x2d,
            ],
            Core::Xor8 => [
                0xff, 0x3e, 0x52, 0x62, 0x02, 0xff, 0x89, 0xcf, 0xf7, 0xbb, 0xe7, 0x0f, 0xdb, 0xf9,
                0xf9, 0x7d, 0x23, 0xc1, 0x2f, 0x6e, 0x2b, 0xb8, 0xbb, 0xe8, 0x30, 0x4a, 0xc7, 0x0f,
                0x61, 0xc1, 0xcf, 0x2c,
            ],
            Core::XorXor16 => [
                0xa1, 0xf2, 0xd6, 0x33, 0xbf, 0x98, 0x89, 0xa0, 0x8a, 0x42, 0x51, 0x2a, 0x78, 0x93,
                0xa6, 0x79, 0x9d, 0xc4, 0x7a, 0xa8, 0x29, 0xff, 0x8f, 0x57, 0x7c, 0x5b, 0xc9, 0x75,
                0x66, 0xc4, 0xd3, 0xfe,
            ],
            Core::XorXor32 => [
                0x0d, 0x5f, 0xf5, 0x81, 0x23, 0xba, 0x0d, 0xae, 0x3b, 0x32, 0x40, 0xa6, 0x31, 0x05,
                0x2a, 0xf2, 0xe8, 0x7b, 0x52, 0xb6, 0x37, 0xa2, 0xcb, 0xd3, 0x37, 0xd2, 0x25, 0x93,
                0x70, 0x62, 0x87, 0x41,
            ],
            Core::XorXor64 => [
                0x78, 0x3f, 0x49, 0xa1, 0x9d, 0x4f, 0x4a, 0xae, 0x4d, 0x3c, 0x1d, 0x6c, 0xcf, 0x83,
                0x15, 0x6d, 0xc5, 0x5d, 0x0b, 0x5c, 0x08, 0xcf, 0x59, 0x23, 0x36, 0x58, 0x4c, 0xb1,
                0x31, 0x67, 0xc6, 0xca,
            ],
            Core::XorXor8 => [
                0x83, 0xa9, 0x80, 0xcc, 0x61, 0x06, 0x85, 0x24, 0x88, 0x10, 0x5d, 0x3c, 0xee, 0x10,
                0xf3, 0x51, 0x13, 0xb8, 0xc9, 0xf7, 0x46, 0x64, 0xe7, 0xce, 0x6d, 0x4e, 0xc0, 0x91,
                0x2b, 0xc2, 0x9b, 0xc7,
            ],
        };

        Cmr(Midstate(bytes))
    }

    fn source_ty(&self) -> TypeName {
        let name: &'static [u8] = match self {
            Core::Add16 => b"i",
            Core::Add32 => b"l",
            Core::Add64 => b"*ll",
            Core::Add8 => b"****22*22**22*22***22*22**22*22",
            Core::All16 => b"****22*22**22*22***22*22**22*22",
            Core::All32 => b"i",
            Core::All64 => b"l",
            Core::All8 => b"***22*22**22*22",
            Core::And16 => b"i",
            Core::And32 => b"l",
            Core::And64 => b"*ll",
            Core::And8 => b"****22*22**22*22***22*22**22*22",
            Core::Bip0340Verify => b"**hh*hh",
            Core::Ch16 => b"*****22*22**22*22***22*22**22*22i",
            Core::Ch32 => b"*il",
            Core::Ch64 => b"*l*ll",
            Core::Ch8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Core::CheckSigVerify => b"**h*hh*hh",
            Core::Complement16 => b"****22*22**22*22***22*22**22*22",
            Core::Complement32 => b"i",
            Core::Complement64 => b"l",
            Core::Complement8 => b"***22*22**22*22",
            Core::Decompress => b"*2h",
            Core::Decrement16 => b"****22*22**22*22***22*22**22*22",
            Core::Decrement32 => b"i",
            Core::Decrement64 => b"l",
            Core::Decrement8 => b"***22*22**22*22",
            Core::DivMod16 => b"i",
            Core::DivMod32 => b"l",
            Core::DivMod64 => b"*ll",
            Core::DivMod8 => b"****22*22**22*22***22*22**22*22",
            Core::Divide16 => b"i",
            Core::Divide32 => b"l",
            Core::Divide64 => b"*ll",
            Core::Divide8 => b"****22*22**22*22***22*22**22*22",
            Core::Divides16 => b"i",
            Core::Divides32 => b"l",
            Core::Divides64 => b"*ll",
            Core::Divides8 => b"****22*22**22*22***22*22**22*22",
            Core::Eq16 => b"i",
            Core::Eq256 => b"*hh",
            Core::Eq32 => b"l",
            Core::Eq64 => b"*ll",
            Core::Eq8 => b"****22*22**22*22***22*22**22*22",
            Core::FeAdd => b"*hh",
            Core::FeInvert => b"h",
            Core::FeIsOdd => b"h",
            Core::FeIsZero => b"h",
            Core::FeMultiply => b"*hh",
            Core::FeMultiplyBeta => b"h",
            Core::FeNegate => b"h",
            Core::FeNormalize => b"h",
            Core::FeSquare => b"h",
            Core::FeSquareRoot => b"h",
            Core::FullAdd16 => b"*2i",
            Core::FullAdd32 => b"*2l",
            Core::FullAdd64 => b"*2*ll",
            Core::FullAdd8 => b"*2****22*22**22*22***22*22**22*22",
            Core::FullDecrement16 => b"*2****22*22**22*22***22*22**22*22",
            Core::FullDecrement32 => b"*2i",
            Core::FullDecrement64 => b"*2l",
            Core::FullDecrement8 => b"*2***22*22**22*22",
            Core::FullIncrement16 => b"*2****22*22**22*22***22*22**22*22",
            Core::FullIncrement32 => b"*2i",
            Core::FullIncrement64 => b"*2l",
            Core::FullIncrement8 => b"*2***22*22**22*22",
            Core::FullMultiply16 => b"l",
            Core::FullMultiply32 => b"*ll",
            Core::FullMultiply64 => b"h",
            Core::FullMultiply8 => b"i",
            Core::FullSubtract16 => b"*2i",
            Core::FullSubtract32 => b"*2l",
            Core::FullSubtract64 => b"*2*ll",
            Core::FullSubtract8 => b"*2****22*22**22*22***22*22**22*22",
            Core::GeIsOnCurve => b"*hh",
            Core::GeNegate => b"*hh",
            Core::GejAdd => b"***hhh**hhh",
            Core::GejDouble => b"**hhh",
            Core::GejGeAdd => b"***hhh*hh",
            Core::GejGeAddEx => b"***hhh*hh",
            Core::GejInfinity => b"1",
            Core::GejIsInfinity => b"**hhh",
            Core::GejIsOnCurve => b"**hhh",
            Core::GejNegate => b"**hhh",
            Core::GejNormalize => b"**hhh",
            Core::GejRescale => b"***hhhh",
            Core::GejXEquiv => b"*h**hhh",
            Core::GejYIsOdd => b"**hhh",
            Core::Generate => b"h",
            Core::High16 => b"1",
            Core::High32 => b"1",
            Core::High64 => b"1",
            Core::High8 => b"1",
            Core::Increment16 => b"****22*22**22*22***22*22**22*22",
            Core::Increment32 => b"i",
            Core::Increment64 => b"l",
            Core::Increment8 => b"***22*22**22*22",
            Core::IsOne16 => b"****22*22**22*22***22*22**22*22",
            Core::IsOne32 => b"i",
            Core::IsOne64 => b"l",
            Core::IsOne8 => b"***22*22**22*22",
            Core::IsZero16 => b"****22*22**22*22***22*22**22*22",
            Core::IsZero32 => b"i",
            Core::IsZero64 => b"l",
            Core::IsZero8 => b"***22*22**22*22",
            Core::Le16 => b"i",
            Core::Le32 => b"l",
            Core::Le64 => b"*ll",
            Core::Le8 => b"****22*22**22*22***22*22**22*22",
            Core::LinearCombination1 => b"**h**hhhh",
            Core::LinearVerify1 => b"***h*hhh*hh",
            Core::Low16 => b"1",
            Core::Low32 => b"1",
            Core::Low64 => b"1",
            Core::Low8 => b"1",
            Core::Lt16 => b"i",
            Core::Lt32 => b"l",
            Core::Lt64 => b"*ll",
            Core::Lt8 => b"****22*22**22*22***22*22**22*22",
            Core::Maj16 => b"*****22*22**22*22***22*22**22*22i",
            Core::Maj32 => b"*il",
            Core::Maj64 => b"*l*ll",
            Core::Maj8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Core::Max16 => b"i",
            Core::Max32 => b"l",
            Core::Max64 => b"*ll",
            Core::Max8 => b"****22*22**22*22***22*22**22*22",
            Core::Median16 => b"*****22*22**22*22***22*22**22*22i",
            Core::Median32 => b"*il",
            Core::Median64 => b"*l*ll",
            Core::Median8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Core::Min16 => b"i",
            Core::Min32 => b"l",
            Core::Min64 => b"*ll",
            Core::Min8 => b"****22*22**22*22***22*22**22*22",
            Core::Modulo16 => b"i",
            Core::Modulo32 => b"l",
            Core::Modulo64 => b"*ll",
            Core::Modulo8 => b"****22*22**22*22***22*22**22*22",
            Core::Multiply16 => b"i",
            Core::Multiply32 => b"l",
            Core::Multiply64 => b"*ll",
            Core::Multiply8 => b"****22*22**22*22***22*22**22*22",
            Core::Negate16 => b"****22*22**22*22***22*22**22*22",
            Core::Negate32 => b"i",
            Core::Negate64 => b"l",
            Core::Negate8 => b"***22*22**22*22",
            Core::One16 => b"1",
            Core::One32 => b"1",
            Core::One64 => b"1",
            Core::One8 => b"1",
            Core::Or16 => b"i",
            Core::Or32 => b"l",
            Core::Or64 => b"*ll",
            Core::Or8 => b"****22*22**22*22***22*22**22*22",
            Core::ParseLock => b"i",
            Core::ParseSequence => b"i",
            Core::PointVerify1 => b"***h*2hh*2h",
            Core::ScalarAdd => b"*hh",
            Core::ScalarInvert => b"h",
            Core::ScalarIsZero => b"h",
            Core::ScalarMultiply => b"*hh",
            Core::ScalarMultiplyLambda => b"h",
            Core::ScalarNegate => b"h",
            Core::ScalarNormalize => b"h",
            Core::ScalarSquare => b"h",
            Core::Scale => b"*h**hhh",
            Core::Sha256Block => b"*h*hh",
            Core::Sha256Ctx8Add1 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh***22*22**22*22",
            Core::Sha256Ctx8Add128 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh**hh*hh",
            Core::Sha256Ctx8Add16 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*ll",
            Core::Sha256Ctx8Add2 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh****22*22**22*22***22*22**22*22",
            Core::Sha256Ctx8Add256 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh***hh*hh**hh*hh",
            Core::Sha256Ctx8Add32 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lhh",
            Core::Sha256Ctx8Add4 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lhi",
            Core::Sha256Ctx8Add512 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh****hh*hh**hh*hh***hh*hh**hh*hh",
            Core::Sha256Ctx8Add64 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*hh",
            Core::Sha256Ctx8Add8 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lhl",
            Core::Sha256Ctx8AddBuffer511 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*+1***hh*hh**hh*hh*+1**hh*hh*+1*hh*+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22",
            Core::Sha256Ctx8Finalize => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Ctx8Init => b"1",
            Core::Sha256Iv => b"1",
            Core::Some16 => b"****22*22**22*22***22*22**22*22",
            Core::Some32 => b"i",
            Core::Some64 => b"l",
            Core::Some8 => b"***22*22**22*22",
            Core::Subtract16 => b"i",
            Core::Subtract32 => b"l",
            Core::Subtract64 => b"*ll",
            Core::Subtract8 => b"****22*22**22*22***22*22**22*22",
            Core::Verify => b"2",
            Core::Xor16 => b"i",
            Core::Xor32 => b"l",
            Core::Xor64 => b"*ll",
            Core::Xor8 => b"****22*22**22*22***22*22**22*22",
            Core::XorXor16 => b"*****22*22**22*22***22*22**22*22i",
            Core::XorXor32 => b"*il",
            Core::XorXor64 => b"*l*ll",
            Core::XorXor8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
        };

        TypeName(name)
    }

    fn target_ty(&self) -> TypeName {
        let name: &'static [u8] = match self {
            Core::Add16 => b"*2****22*22**22*22***22*22**22*22",
            Core::Add32 => b"*2i",
            Core::Add64 => b"*2l",
            Core::Add8 => b"*2***22*22**22*22",
            Core::All16 => b"2",
            Core::All32 => b"2",
            Core::All64 => b"2",
            Core::All8 => b"2",
            Core::And16 => b"****22*22**22*22***22*22**22*22",
            Core::And32 => b"i",
            Core::And64 => b"l",
            Core::And8 => b"***22*22**22*22",
            Core::Bip0340Verify => b"1",
            Core::Ch16 => b"****22*22**22*22***22*22**22*22",
            Core::Ch32 => b"i",
            Core::Ch64 => b"l",
            Core::Ch8 => b"***22*22**22*22",
            Core::CheckSigVerify => b"1",
            Core::Complement16 => b"****22*22**22*22***22*22**22*22",
            Core::Complement32 => b"i",
            Core::Complement64 => b"l",
            Core::Complement8 => b"***22*22**22*22",
            Core::Decompress => b"+1*hh",
            Core::Decrement16 => b"*2****22*22**22*22***22*22**22*22",
            Core::Decrement32 => b"*2i",
            Core::Decrement64 => b"*2l",
            Core::Decrement8 => b"*2***22*22**22*22",
            Core::DivMod16 => b"i",
            Core::DivMod32 => b"l",
            Core::DivMod64 => b"*ll",
            Core::DivMod8 => b"****22*22**22*22***22*22**22*22",
            Core::Divide16 => b"****22*22**22*22***22*22**22*22",
            Core::Divide32 => b"i",
            Core::Divide64 => b"l",
            Core::Divide8 => b"***22*22**22*22",
            Core::Divides16 => b"2",
            Core::Divides32 => b"2",
            Core::Divides64 => b"2",
            Core::Divides8 => b"2",
            Core::Eq16 => b"2",
            Core::Eq256 => b"2",
            Core::Eq32 => b"2",
            Core::Eq64 => b"2",
            Core::Eq8 => b"2",
            Core::FeAdd => b"h",
            Core::FeInvert => b"h",
            Core::FeIsOdd => b"2",
            Core::FeIsZero => b"2",
            Core::FeMultiply => b"h",
            Core::FeMultiplyBeta => b"h",
            Core::FeNegate => b"h",
            Core::FeNormalize => b"h",
            Core::FeSquare => b"h",
            Core::FeSquareRoot => b"+1h",
            Core::FullAdd16 => b"*2****22*22**22*22***22*22**22*22",
            Core::FullAdd32 => b"*2i",
            Core::FullAdd64 => b"*2l",
            Core::FullAdd8 => b"*2***22*22**22*22",
            Core::FullDecrement16 => b"*2****22*22**22*22***22*22**22*22",
            Core::FullDecrement32 => b"*2i",
            Core::FullDecrement64 => b"*2l",
            Core::FullDecrement8 => b"*2***22*22**22*22",
            Core::FullIncrement16 => b"*2****22*22**22*22***22*22**22*22",
            Core::FullIncrement32 => b"*2i",
            Core::FullIncrement64 => b"*2l",
            Core::FullIncrement8 => b"*2***22*22**22*22",
            Core::FullMultiply16 => b"i",
            Core::FullMultiply32 => b"l",
            Core::FullMultiply64 => b"*ll",
            Core::FullMultiply8 => b"****22*22**22*22***22*22**22*22",
            Core::FullSubtract16 => b"*2****22*22**22*22***22*22**22*22",
            Core::FullSubtract32 => b"*2i",
            Core::FullSubtract64 => b"*2l",
            Core::FullSubtract8 => b"*2***22*22**22*22",
            Core::GeIsOnCurve => b"2",
            Core::GeNegate => b"*hh",
            Core::GejAdd => b"**hhh",
            Core::GejDouble => b"**hhh",
            Core::GejGeAdd => b"**hhh",
            Core::GejGeAddEx => b"*h**hhh",
            Core::GejInfinity => b"**hhh",
            Core::GejIsInfinity => b"2",
            Core::GejIsOnCurve => b"2",
            Core::GejNegate => b"**hhh",
            Core::GejNormalize => b"+1*hh",
            Core::GejRescale => b"**hhh",
            Core::GejXEquiv => b"2",
            Core::GejYIsOdd => b"2",
            Core::Generate => b"**hhh",
            Core::High16 => b"****22*22**22*22***22*22**22*22",
            Core::High32 => b"i",
            Core::High64 => b"l",
            Core::High8 => b"***22*22**22*22",
            Core::Increment16 => b"*2****22*22**22*22***22*22**22*22",
            Core::Increment32 => b"*2i",
            Core::Increment64 => b"*2l",
            Core::Increment8 => b"*2***22*22**22*22",
            Core::IsOne16 => b"2",
            Core::IsOne32 => b"2",
            Core::IsOne64 => b"2",
            Core::IsOne8 => b"2",
            Core::IsZero16 => b"2",
            Core::IsZero32 => b"2",
            Core::IsZero64 => b"2",
            Core::IsZero8 => b"2",
            Core::Le16 => b"2",
            Core::Le32 => b"2",
            Core::Le64 => b"2",
            Core::Le8 => b"2",
            Core::LinearCombination1 => b"**hhh",
            Core::LinearVerify1 => b"1",
            Core::Low16 => b"****22*22**22*22***22*22**22*22",
            Core::Low32 => b"i",
            Core::Low64 => b"l",
            Core::Low8 => b"***22*22**22*22",
            Core::Lt16 => b"2",
            Core::Lt32 => b"2",
            Core::Lt64 => b"2",
            Core::Lt8 => b"2",
            Core::Maj16 => b"****22*22**22*22***22*22**22*22",
            Core::Maj32 => b"i",
            Core::Maj64 => b"l",
            Core::Maj8 => b"***22*22**22*22",
            Core::Max16 => b"****22*22**22*22***22*22**22*22",
            Core::Max32 => b"i",
            Core::Max64 => b"l",
            Core::Max8 => b"***22*22**22*22",
            Core::Median16 => b"****22*22**22*22***22*22**22*22",
            Core::Median32 => b"i",
            Core::Median64 => b"l",
            Core::Median8 => b"***22*22**22*22",
            Core::Min16 => b"****22*22**22*22***22*22**22*22",
            Core::Min32 => b"i",
            Core::Min64 => b"l",
            Core::Min8 => b"***22*22**22*22",
            Core::Modulo16 => b"****22*22**22*22***22*22**22*22",
            Core::Modulo32 => b"i",
            Core::Modulo64 => b"l",
            Core::Modulo8 => b"***22*22**22*22",
            Core::Multiply16 => b"i",
            Core::Multiply32 => b"l",
            Core::Multiply64 => b"*ll",
            Core::Multiply8 => b"****22*22**22*22***22*22**22*22",
            Core::Negate16 => b"*2****22*22**22*22***22*22**22*22",
            Core::Negate32 => b"*2i",
            Core::Negate64 => b"*2l",
            Core::Negate8 => b"*2***22*22**22*22",
            Core::One16 => b"****22*22**22*22***22*22**22*22",
            Core::One32 => b"i",
            Core::One64 => b"l",
            Core::One8 => b"***22*22**22*22",
            Core::Or16 => b"****22*22**22*22***22*22**22*22",
            Core::Or32 => b"i",
            Core::Or64 => b"l",
            Core::Or8 => b"***22*22**22*22",
            Core::ParseLock => b"+ii",
            Core::ParseSequence => b"+1+****22*22**22*22***22*22**22*22****22*22**22*22***22*22**22*22",
            Core::PointVerify1 => b"1",
            Core::ScalarAdd => b"h",
            Core::ScalarInvert => b"h",
            Core::ScalarIsZero => b"2",
            Core::ScalarMultiply => b"h",
            Core::ScalarMultiplyLambda => b"h",
            Core::ScalarNegate => b"h",
            Core::ScalarNormalize => b"h",
            Core::ScalarSquare => b"h",
            Core::Scale => b"**hhh",
            Core::Sha256Block => b"h",
            Core::Sha256Ctx8Add1 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Ctx8Add128 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Ctx8Add16 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Ctx8Add2 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Ctx8Add256 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Ctx8Add32 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Ctx8Add4 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Ctx8Add512 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Ctx8Add64 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Ctx8Add8 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Ctx8AddBuffer511 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Ctx8Finalize => b"h",
            Core::Sha256Ctx8Init => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Iv => b"h",
            Core::Some16 => b"2",
            Core::Some32 => b"2",
            Core::Some64 => b"2",
            Core::Some8 => b"2",
            Core::Subtract16 => b"*2****22*22**22*22***22*22**22*22",
            Core::Subtract32 => b"*2i",
            Core::Subtract64 => b"*2l",
            Core::Subtract8 => b"*2***22*22**22*22",
            Core::Verify => b"1",
            Core::Xor16 => b"****22*22**22*22***22*22**22*22",
            Core::Xor32 => b"i",
            Core::Xor64 => b"l",
            Core::Xor8 => b"***22*22**22*22",
            Core::XorXor16 => b"****22*22**22*22***22*22**22*22",
            Core::XorXor32 => b"i",
            Core::XorXor64 => b"l",
            Core::XorXor8 => b"***22*22**22*22",
        };

        TypeName(name)
    }

    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> std::io::Result<usize> {
        let (n, len) = match self {
            Core::Verify => (0, 2),
            Core::Low8 => (37, 7),
            Core::Low16 => (304, 10),
            Core::Low32 => (305, 10),
            Core::Low64 => (306, 10),
            Core::High8 => (45, 7),
            Core::High16 => (368, 10),
            Core::High32 => (369, 10),
            Core::High64 => (370, 10),
            Core::Complement8 => (389, 10),
            Core::Complement16 => (3120, 13),
            Core::Complement32 => (3121, 13),
            Core::Complement64 => (3122, 13),
            Core::And8 => (397, 10),
            Core::And16 => (3184, 13),
            Core::And32 => (3185, 13),
            Core::And64 => (3186, 13),
            Core::Or8 => (405, 10),
            Core::Or16 => (3248, 13),
            Core::Or32 => (3249, 13),
            Core::Or64 => (3250, 13),
            Core::Xor8 => (413, 10),
            Core::Xor16 => (3312, 13),
            Core::Xor32 => (3313, 13),
            Core::Xor64 => (3314, 13),
            Core::Maj8 => (837, 11),
            Core::Maj16 => (6704, 14),
            Core::Maj32 => (6705, 14),
            Core::Maj64 => (6706, 14),
            Core::XorXor8 => (845, 11),
            Core::XorXor16 => (6768, 14),
            Core::XorXor32 => (6769, 14),
            Core::XorXor64 => (6770, 14),
            Core::Ch8 => (853, 11),
            Core::Ch16 => (6832, 14),
            Core::Ch32 => (6833, 14),
            Core::Ch64 => (6834, 14),
            Core::Some8 => (861, 11),
            Core::Some16 => (6896, 14),
            Core::Some32 => (6897, 14),
            Core::Some64 => (6898, 14),
            Core::All8 => (869, 11),
            Core::All16 => (6960, 14),
            Core::All32 => (6961, 14),
            Core::All64 => (6962, 14),
            Core::Eq8 => (877, 11),
            Core::Eq16 => (7024, 14),
            Core::Eq32 => (7025, 14),
            Core::Eq64 => (7026, 14),
            Core::Eq256 => (14056, 15),
            Core::One8 => (69, 7),
            Core::One16 => (560, 10),
            Core::One32 => (561, 10),
            Core::One64 => (562, 10),
            Core::FullAdd8 => (293, 9),
            Core::FullAdd16 => (2352, 12),
            Core::FullAdd32 => (2353, 12),
            Core::FullAdd64 => (2354, 12),
            Core::Add8 => (301, 9),
            Core::Add16 => (2416, 12),
            Core::Add32 => (2417, 12),
            Core::Add64 => (2418, 12),
            Core::FullIncrement8 => (2437, 12),
            Core::FullIncrement16 => (19504, 15),
            Core::FullIncrement32 => (19505, 15),
            Core::FullIncrement64 => (19506, 15),
            Core::Increment8 => (2445, 12),
            Core::Increment16 => (19568, 15),
            Core::Increment32 => (19569, 15),
            Core::Increment64 => (19570, 15),
            Core::FullSubtract8 => (2461, 12),
            Core::FullSubtract16 => (19696, 15),
            Core::FullSubtract32 => (19697, 15),
            Core::FullSubtract64 => (19698, 15),
            Core::Subtract8 => (4933, 13),
            Core::Subtract16 => (39472, 16),
            Core::Subtract32 => (39473, 16),
            Core::Subtract64 => (39474, 16),
            Core::Negate8 => (4941, 13),
            Core::Negate16 => (39536, 16),
            Core::Negate32 => (39537, 16),
            Core::Negate64 => (39538, 16),
            Core::FullDecrement8 => (4949, 13),
            Core::FullDecrement16 => (39600, 16),
            Core::FullDecrement32 => (39601, 16),
            Core::FullDecrement64 => (39602, 16),
            Core::Decrement8 => (4957, 13),
            Core::Decrement16 => (39664, 16),
            Core::Decrement32 => (39665, 16),
            Core::Decrement64 => (39666, 16),
            Core::FullMultiply8 => (4965, 13),
            Core::FullMultiply16 => (39728, 16),
            Core::FullMultiply32 => (39729, 16),
            Core::FullMultiply64 => (39730, 16),
            Core::Multiply8 => (4973, 13),
            Core::Multiply16 => (39792, 16),
            Core::Multiply32 => (39793, 16),
            Core::Multiply64 => (39794, 16),
            Core::IsZero8 => (4981, 13),
            Core::IsZero16 => (39856, 16),
            Core::IsZero32 => (39857, 16),
            Core::IsZero64 => (39858, 16),
            Core::IsOne8 => (4989, 13),
            Core::IsOne16 => (39920, 16),
            Core::IsOne32 => (39921, 16),
            Core::IsOne64 => (39922, 16),
            Core::Le8 => (79877, 17),
            Core::Le16 => (639024, 20),
            Core::Le32 => (639025, 20),
            Core::Le64 => (639026, 20),
            Core::Lt8 => (79885, 17),
            Core::Lt16 => (639088, 20),
            Core::Lt32 => (639089, 20),
            Core::Lt64 => (639090, 20),
            Core::Min8 => (79893, 17),
            Core::Min16 => (639152, 20),
            Core::Min32 => (639153, 20),
            Core::Min64 => (639154, 20),
            Core::Max8 => (79901, 17),
            Core::Max16 => (639216, 20),
            Core::Max32 => (639217, 20),
            Core::Max64 => (639218, 20),
            Core::Median8 => (79909, 17),
            Core::Median16 => (639280, 20),
            Core::Median32 => (639281, 20),
            Core::Median64 => (639282, 20),
            Core::DivMod8 => (79925, 17),
            Core::DivMod16 => (639408, 20),
            Core::DivMod32 => (639409, 20),
            Core::DivMod64 => (639410, 20),
            Core::Divide8 => (79933, 17),
            Core::Divide16 => (639472, 20),
            Core::Divide32 => (639473, 20),
            Core::Divide64 => (639474, 20),
            Core::Modulo8 => (79941, 17),
            Core::Modulo16 => (639536, 20),
            Core::Modulo32 => (639537, 20),
            Core::Modulo64 => (639538, 20),
            Core::Divides8 => (79949, 17),
            Core::Divides16 => (639600, 20),
            Core::Divides32 => (639601, 20),
            Core::Divides64 => (639602, 20),
            Core::Sha256Block => (20, 5),
            Core::Sha256Iv => (84, 7),
            Core::Sha256Ctx8Add1 => (170, 8),
            Core::Sha256Ctx8Add2 => (684, 10),
            Core::Sha256Ctx8Add4 => (685, 10),
            Core::Sha256Ctx8Add8 => (5488, 13),
            Core::Sha256Ctx8Add16 => (5489, 13),
            Core::Sha256Ctx8Add32 => (5490, 13),
            Core::Sha256Ctx8Add64 => (5491, 13),
            Core::Sha256Ctx8Add128 => (10984, 14),
            Core::Sha256Ctx8Add256 => (10985, 14),
            Core::Sha256Ctx8Add512 => (10986, 14),
            Core::Sha256Ctx8AddBuffer511 => (688, 10),
            Core::Sha256Ctx8Finalize => (689, 10),
            Core::Sha256Ctx8Init => (690, 10),
            Core::PointVerify1 => (192, 8),
            Core::Decompress => (388, 9),
            Core::LinearVerify1 => (778, 10),
            Core::LinearCombination1 => (6240, 13),
            Core::Scale => (3121, 12),
            Core::Generate => (3122, 12),
            Core::GejInfinity => (3123, 12),
            Core::GejNormalize => (6248, 13),
            Core::GejNegate => (6249, 13),
            Core::GeNegate => (6250, 13),
            Core::GejDouble => (6251, 13),
            Core::GejAdd => (6252, 13),
            Core::GejGeAddEx => (6253, 13),
            Core::GejGeAdd => (6254, 13),
            Core::GejRescale => (6255, 13),
            Core::GejIsInfinity => (100096, 17),
            Core::GejXEquiv => (100099, 17),
            Core::GejYIsOdd => (100100, 17),
            Core::GejIsOnCurve => (100101, 17),
            Core::GeIsOnCurve => (100102, 17),
            Core::ScalarNormalize => (100103, 17),
            Core::ScalarNegate => (100104, 17),
            Core::ScalarAdd => (100105, 17),
            Core::ScalarSquare => (100106, 17),
            Core::ScalarMultiply => (100107, 17),
            Core::ScalarMultiplyLambda => (100108, 17),
            Core::ScalarInvert => (100109, 17),
            Core::ScalarIsZero => (100110, 17),
            Core::FeNormalize => (200227, 18),
            Core::FeNegate => (200228, 18),
            Core::FeAdd => (200229, 18),
            Core::FeSquare => (200230, 18),
            Core::FeMultiply => (200231, 18),
            Core::FeMultiplyBeta => (200232, 18),
            Core::FeInvert => (200233, 18),
            Core::FeSquareRoot => (200234, 18),
            Core::FeIsZero => (200235, 18),
            Core::FeIsOdd => (200236, 18),
            Core::CheckSigVerify => (98, 7),
            Core::Bip0340Verify => (396, 9),
            Core::ParseLock => (102, 7),
            Core::ParseSequence => (412, 9),
        };

        w.write_bits_be(n, len)
    }

    fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Self, decode::Error> {
        decode_bits!(bits, {
            0 => {
                0 => {Core::Verify},
                1 => {
                    0 => {
                        0 => {
                            0 => {},
                            1 => {
                                0 => {
                                    0 => {},
                                    1 => {Core::Low8}
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {
                                                0 => {Core::Low16},
                                                1 => {Core::Low32}
                                            },
                                            1 => {
                                                0 => {Core::Low64},
                                                1 => {}
                                            }
                                        },
                                        1 => {}
                                    },
                                    1 => {}
                                }
                            }
                        },
                        1 => {
                            0 => {},
                            1 => {
                                0 => {
                                    0 => {},
                                    1 => {Core::High8}
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {
                                                0 => {Core::High16},
                                                1 => {Core::High32}
                                            },
                                            1 => {
                                                0 => {Core::High64},
                                                1 => {}
                                            }
                                        },
                                        1 => {}
                                    },
                                    1 => {}
                                }
                            }
                        }
                    },
                    1 => {
                        0 => {
                            0 => {
                                0 => {
                                    0 => {
                                        0 => {},
                                        1 => {
                                            0 => {
                                                0 => {},
                                                1 => {Core::Complement8}
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Core::Complement16},
                                                            1 => {Core::Complement32}
                                                        },
                                                        1 => {
                                                            0 => {Core::Complement64},
                                                            1 => {}
                                                        }
                                                    },
                                                    1 => {}
                                                },
                                                1 => {}
                                            }
                                        }
                                    },
                                    1 => {
                                        0 => {},
                                        1 => {
                                            0 => {
                                                0 => {},
                                                1 => {Core::And8}
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Core::And16},
                                                            1 => {Core::And32}
                                                        },
                                                        1 => {
                                                            0 => {Core::And64},
                                                            1 => {}
                                                        }
                                                    },
                                                    1 => {}
                                                },
                                                1 => {}
                                            }
                                        }
                                    }
                                },
                                1 => {
                                    0 => {
                                        0 => {},
                                        1 => {
                                            0 => {
                                                0 => {},
                                                1 => {Core::Or8}
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Core::Or16},
                                                            1 => {Core::Or32}
                                                        },
                                                        1 => {
                                                            0 => {Core::Or64},
                                                            1 => {}
                                                        }
                                                    },
                                                    1 => {}
                                                },
                                                1 => {}
                                            }
                                        }
                                    },
                                    1 => {
                                        0 => {},
                                        1 => {
                                            0 => {
                                                0 => {},
                                                1 => {Core::Xor8}
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Core::Xor16},
                                                            1 => {Core::Xor32}
                                                        },
                                                        1 => {
                                                            0 => {Core::Xor64},
                                                            1 => {}
                                                        }
                                                    },
                                                    1 => {}
                                                },
                                                1 => {}
                                            }
                                        }
                                    }
                                }
                            },
                            1 => {
                                0 => {
                                    0 => {
                                        0 => {
                                            0 => {},
                                            1 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {Core::Maj8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Core::Maj16},
                                                                1 => {Core::Maj32}
                                                            },
                                                            1 => {
                                                                0 => {Core::Maj64},
                                                                1 => {}
                                                            }
                                                        },
                                                        1 => {}
                                                    },
                                                    1 => {}
                                                }
                                            }
                                        },
                                        1 => {
                                            0 => {},
                                            1 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {Core::XorXor8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Core::XorXor16},
                                                                1 => {Core::XorXor32}
                                                            },
                                                            1 => {
                                                                0 => {Core::XorXor64},
                                                                1 => {}
                                                            }
                                                        },
                                                        1 => {}
                                                    },
                                                    1 => {}
                                                }
                                            }
                                        }
                                    },
                                    1 => {
                                        0 => {
                                            0 => {},
                                            1 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {Core::Ch8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Core::Ch16},
                                                                1 => {Core::Ch32}
                                                            },
                                                            1 => {
                                                                0 => {Core::Ch64},
                                                                1 => {}
                                                            }
                                                        },
                                                        1 => {}
                                                    },
                                                    1 => {}
                                                }
                                            }
                                        },
                                        1 => {
                                            0 => {},
                                            1 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {Core::Some8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Core::Some16},
                                                                1 => {Core::Some32}
                                                            },
                                                            1 => {
                                                                0 => {Core::Some64},
                                                                1 => {}
                                                            }
                                                        },
                                                        1 => {}
                                                    },
                                                    1 => {}
                                                }
                                            }
                                        }
                                    }
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {},
                                            1 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {Core::All8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Core::All16},
                                                                1 => {Core::All32}
                                                            },
                                                            1 => {
                                                                0 => {Core::All64},
                                                                1 => {}
                                                            }
                                                        },
                                                        1 => {}
                                                    },
                                                    1 => {}
                                                }
                                            }
                                        },
                                        1 => {
                                            0 => {},
                                            1 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {Core::Eq8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Core::Eq16},
                                                                1 => {Core::Eq32}
                                                            },
                                                            1 => {
                                                                0 => {Core::Eq64},
                                                                1 => {}
                                                            }
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Core::Eq256},
                                                                    1 => {}
                                                                },
                                                                1 => {}
                                                            },
                                                            1 => {}
                                                        }
                                                    },
                                                    1 => {}
                                                }
                                            }
                                        }
                                    },
                                    1 => {}
                                }
                            }
                        },
                        1 => {}
                    }
                }
            },
            1 => {
                0 => {
                    0 => {
                        0 => {
                            0 => {},
                            1 => {
                                0 => {
                                    0 => {},
                                    1 => {Core::One8}
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {
                                                0 => {Core::One16},
                                                1 => {Core::One32}
                                            },
                                            1 => {
                                                0 => {Core::One64},
                                                1 => {}
                                            }
                                        },
                                        1 => {}
                                    },
                                    1 => {}
                                }
                            }
                        },
                        1 => {
                            0 => {
                                0 => {
                                    0 => {},
                                    1 => {
                                        0 => {
                                            0 => {},
                                            1 => {Core::FullAdd8}
                                        },
                                        1 => {
                                            0 => {
                                                0 => {
                                                    0 => {
                                                        0 => {Core::FullAdd16},
                                                        1 => {Core::FullAdd32}
                                                    },
                                                    1 => {
                                                        0 => {Core::FullAdd64},
                                                        1 => {}
                                                    }
                                                },
                                                1 => {}
                                            },
                                            1 => {}
                                        }
                                    }
                                },
                                1 => {
                                    0 => {},
                                    1 => {
                                        0 => {
                                            0 => {},
                                            1 => {Core::Add8}
                                        },
                                        1 => {
                                            0 => {
                                                0 => {
                                                    0 => {
                                                        0 => {Core::Add16},
                                                        1 => {Core::Add32}
                                                    },
                                                    1 => {
                                                        0 => {Core::Add64},
                                                        1 => {}
                                                    }
                                                },
                                                1 => {}
                                            },
                                            1 => {}
                                        }
                                    }
                                }
                            },
                            1 => {
                                0 => {
                                    0 => {
                                        0 => {
                                            0 => {
                                                0 => {},
                                                1 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {Core::FullIncrement8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Core::FullIncrement16},
                                                                    1 => {Core::FullIncrement32}
                                                                },
                                                                1 => {
                                                                    0 => {Core::FullIncrement64},
                                                                    1 => {}
                                                                }
                                                            },
                                                            1 => {}
                                                        },
                                                        1 => {}
                                                    }
                                                }
                                            },
                                            1 => {
                                                0 => {},
                                                1 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {Core::Increment8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Core::Increment16},
                                                                    1 => {Core::Increment32}
                                                                },
                                                                1 => {
                                                                    0 => {Core::Increment64},
                                                                    1 => {}
                                                                }
                                                            },
                                                            1 => {}
                                                        },
                                                        1 => {}
                                                    }
                                                }
                                            }
                                        },
                                        1 => {
                                            0 => {},
                                            1 => {
                                                0 => {},
                                                1 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {Core::FullSubtract8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Core::FullSubtract16},
                                                                    1 => {Core::FullSubtract32}
                                                                },
                                                                1 => {
                                                                    0 => {Core::FullSubtract64},
                                                                    1 => {}
                                                                }
                                                            },
                                                            1 => {}
                                                        },
                                                        1 => {}
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {
                                                        0 => {
                                                            0 => {},
                                                            1 => {Core::Subtract8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Core::Subtract16},
                                                                        1 => {Core::Subtract32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Core::Subtract64},
                                                                        1 => {}
                                                                    }
                                                                },
                                                                1 => {}
                                                            },
                                                            1 => {}
                                                        }
                                                    }
                                                },
                                                1 => {
                                                    0 => {},
                                                    1 => {
                                                        0 => {
                                                            0 => {},
                                                            1 => {Core::Negate8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Core::Negate16},
                                                                        1 => {Core::Negate32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Core::Negate64},
                                                                        1 => {}
                                                                    }
                                                                },
                                                                1 => {}
                                                            },
                                                            1 => {}
                                                        }
                                                    }
                                                }
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {
                                                        0 => {
                                                            0 => {},
                                                            1 => {Core::FullDecrement8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Core::FullDecrement16},
                                                                        1 => {Core::FullDecrement32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Core::FullDecrement64},
                                                                        1 => {}
                                                                    }
                                                                },
                                                                1 => {}
                                                            },
                                                            1 => {}
                                                        }
                                                    }
                                                },
                                                1 => {
                                                    0 => {},
                                                    1 => {
                                                        0 => {
                                                            0 => {},
                                                            1 => {Core::Decrement8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Core::Decrement16},
                                                                        1 => {Core::Decrement32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Core::Decrement64},
                                                                        1 => {}
                                                                    }
                                                                },
                                                                1 => {}
                                                            },
                                                            1 => {}
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                        1 => {
                                            0 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {
                                                        0 => {
                                                            0 => {},
                                                            1 => {Core::FullMultiply8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Core::FullMultiply16},
                                                                        1 => {Core::FullMultiply32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Core::FullMultiply64},
                                                                        1 => {}
                                                                    }
                                                                },
                                                                1 => {}
                                                            },
                                                            1 => {}
                                                        }
                                                    }
                                                },
                                                1 => {
                                                    0 => {},
                                                    1 => {
                                                        0 => {
                                                            0 => {},
                                                            1 => {Core::Multiply8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Core::Multiply16},
                                                                        1 => {Core::Multiply32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Core::Multiply64},
                                                                        1 => {}
                                                                    }
                                                                },
                                                                1 => {}
                                                            },
                                                            1 => {}
                                                        }
                                                    }
                                                }
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {
                                                        0 => {
                                                            0 => {},
                                                            1 => {Core::IsZero8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Core::IsZero16},
                                                                        1 => {Core::IsZero32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Core::IsZero64},
                                                                        1 => {}
                                                                    }
                                                                },
                                                                1 => {}
                                                            },
                                                            1 => {}
                                                        }
                                                    }
                                                },
                                                1 => {
                                                    0 => {},
                                                    1 => {
                                                        0 => {
                                                            0 => {},
                                                            1 => {Core::IsOne8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Core::IsOne16},
                                                                        1 => {Core::IsOne32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Core::IsOne64},
                                                                        1 => {}
                                                                    }
                                                                },
                                                                1 => {}
                                                            },
                                                            1 => {}
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {Core::Le8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Core::Le16},
                                                                                        1 => {Core::Le32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Core::Le64},
                                                                                        1 => {}
                                                                                    }
                                                                                },
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {Core::Lt8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Core::Lt16},
                                                                                        1 => {Core::Lt32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Core::Lt64},
                                                                                        1 => {}
                                                                                    }
                                                                                },
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                }
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {Core::Min8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Core::Min16},
                                                                                        1 => {Core::Min32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Core::Min64},
                                                                                        1 => {}
                                                                                    }
                                                                                },
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {Core::Max8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Core::Max16},
                                                                                        1 => {Core::Max32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Core::Max64},
                                                                                        1 => {}
                                                                                    }
                                                                                },
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {Core::Median8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Core::Median16},
                                                                                        1 => {Core::Median32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Core::Median64},
                                                                                        1 => {}
                                                                                    }
                                                                                },
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                },
                                                                1 => {}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {Core::DivMod8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Core::DivMod16},
                                                                                        1 => {Core::DivMod32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Core::DivMod64},
                                                                                        1 => {}
                                                                                    }
                                                                                },
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {Core::Divide8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Core::Divide16},
                                                                                        1 => {Core::Divide32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Core::Divide64},
                                                                                        1 => {}
                                                                                    }
                                                                                },
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {Core::Modulo8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Core::Modulo16},
                                                                                        1 => {Core::Modulo32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Core::Modulo64},
                                                                                        1 => {}
                                                                                    }
                                                                                },
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {Core::Divides8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Core::Divides16},
                                                                                        1 => {Core::Divides32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Core::Divides64},
                                                                                        1 => {}
                                                                                    }
                                                                                },
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                }
                                                            },
                                                            1 => {}
                                                        },
                                                        1 => {}
                                                    }
                                                },
                                                1 => {}
                                            },
                                            1 => {}
                                        },
                                        1 => {}
                                    },
                                    1 => {}
                                }
                            }
                        }
                    },
                    1 => {
                        0 => {
                            0 => {Core::Sha256Block},
                            1 => {
                                0 => {
                                    0 => {Core::Sha256Iv},
                                    1 => {
                                        0 => {Core::Sha256Ctx8Add1},
                                        1 => {
                                            0 => {
                                                0 => {Core::Sha256Ctx8Add2},
                                                1 => {Core::Sha256Ctx8Add4}
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Core::Sha256Ctx8Add8},
                                                            1 => {Core::Sha256Ctx8Add16}
                                                        },
                                                        1 => {
                                                            0 => {Core::Sha256Ctx8Add32},
                                                            1 => {Core::Sha256Ctx8Add64}
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Core::Sha256Ctx8Add128},
                                                                1 => {Core::Sha256Ctx8Add256}
                                                            },
                                                            1 => {
                                                                0 => {Core::Sha256Ctx8Add512},
                                                                1 => {}
                                                            }
                                                        },
                                                        1 => {}
                                                    }
                                                },
                                                1 => {}
                                            }
                                        }
                                    }
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {
                                                0 => {Core::Sha256Ctx8AddBuffer511},
                                                1 => {Core::Sha256Ctx8Finalize}
                                            },
                                            1 => {
                                                0 => {Core::Sha256Ctx8Init},
                                                1 => {}
                                            }
                                        },
                                        1 => {}
                                    },
                                    1 => {}
                                }
                            }
                        },
                        1 => {}
                    }
                },
                1 => {
                    0 => {
                        0 => {
                            0 => {
                                0 => {
                                    0 => {
                                        0 => {Core::PointVerify1},
                                        1 => {}
                                    },
                                    1 => {
                                        0 => {
                                            0 => {Core::Decompress},
                                            1 => {
                                                0 => {Core::LinearVerify1},
                                                1 => {}
                                            }
                                        },
                                        1 => {
                                            0 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Core::LinearCombination1},
                                                            1 => {}
                                                        },
                                                        1 => {Core::Scale}
                                                    },
                                                    1 => {
                                                        0 => {Core::Generate},
                                                        1 => {Core::GejInfinity}
                                                    }
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Core::GejNormalize},
                                                            1 => {Core::GejNegate}
                                                        },
                                                        1 => {
                                                            0 => {Core::GeNegate},
                                                            1 => {Core::GejDouble}
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {Core::GejAdd},
                                                            1 => {Core::GejGeAddEx}
                                                        },
                                                        1 => {
                                                            0 => {Core::GejGeAdd},
                                                            1 => {Core::GejRescale}
                                                        }
                                                    }
                                                }
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Core::GejIsInfinity},
                                                                            1 => {}
                                                                        },
                                                                        1 => {
                                                                            0 => {},
                                                                            1 => {Core::GejXEquiv}
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {Core::GejYIsOdd},
                                                                            1 => {Core::GejIsOnCurve}
                                                                        },
                                                                        1 => {
                                                                            0 => {Core::GeIsOnCurve},
                                                                            1 => {Core::ScalarNormalize}
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Core::ScalarNegate},
                                                                            1 => {Core::ScalarAdd}
                                                                        },
                                                                        1 => {
                                                                            0 => {Core::ScalarSquare},
                                                                            1 => {Core::ScalarMultiply}
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {Core::ScalarMultiplyLambda},
                                                                            1 => {Core::ScalarInvert}
                                                                        },
                                                                        1 => {
                                                                            0 => {Core::ScalarIsZero},
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                }
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {
                                                                                0 => {},
                                                                                1 => {Core::FeNormalize}
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Core::FeNegate},
                                                                                1 => {Core::FeAdd}
                                                                            },
                                                                            1 => {
                                                                                0 => {Core::FeSquare},
                                                                                1 => {Core::FeMultiply}
                                                                            }
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Core::FeMultiplyBeta},
                                                                                1 => {Core::FeInvert}
                                                                            },
                                                                            1 => {
                                                                                0 => {Core::FeSquareRoot},
                                                                                1 => {Core::FeIsZero}
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Core::FeIsOdd},
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                },
                                                                1 => {}
                                                            }
                                                        },
                                                        1 => {}
                                                    },
                                                    1 => {}
                                                },
                                                1 => {}
                                            }
                                        }
                                    }
                                },
                                1 => {
                                    0 => {Core::CheckSigVerify},
                                    1 => {
                                        0 => {
                                            0 => {Core::Bip0340Verify},
                                            1 => {}
                                        },
                                        1 => {}
                                    }
                                }
                            },
                            1 => {
                                0 => {},
                                1 => {
                                    0 => {Core::ParseLock},
                                    1 => {
                                        0 => {
                                            0 => {Core::ParseSequence},
                                            1 => {}
                                        },
                                        1 => {}
                                    }
                                }
                            }
                        },
                        1 => {}
                    },
                    1 => {}
                }
            }
        })
    }

    fn c_jet_ptr(&self) -> &dyn Fn(&mut CFrameItem, CFrameItem, &Self::CJetEnvironment) -> bool {
        match self {
            Core::Add16 => &simplicity_sys::c_jets::jets_wrapper::add_16,
            Core::Add32 => &simplicity_sys::c_jets::jets_wrapper::add_32,
            Core::Add64 => &simplicity_sys::c_jets::jets_wrapper::add_64,
            Core::Add8 => &simplicity_sys::c_jets::jets_wrapper::add_8,
            Core::All16 => &simplicity_sys::c_jets::jets_wrapper::all_16,
            Core::All32 => &simplicity_sys::c_jets::jets_wrapper::all_32,
            Core::All64 => &simplicity_sys::c_jets::jets_wrapper::all_64,
            Core::All8 => &simplicity_sys::c_jets::jets_wrapper::all_8,
            Core::And16 => &simplicity_sys::c_jets::jets_wrapper::and_16,
            Core::And32 => &simplicity_sys::c_jets::jets_wrapper::and_32,
            Core::And64 => &simplicity_sys::c_jets::jets_wrapper::and_64,
            Core::And8 => &simplicity_sys::c_jets::jets_wrapper::and_8,
            Core::Bip0340Verify => &simplicity_sys::c_jets::jets_wrapper::bip_0340_verify,
            Core::Ch16 => &simplicity_sys::c_jets::jets_wrapper::ch_16,
            Core::Ch32 => &simplicity_sys::c_jets::jets_wrapper::ch_32,
            Core::Ch64 => &simplicity_sys::c_jets::jets_wrapper::ch_64,
            Core::Ch8 => &simplicity_sys::c_jets::jets_wrapper::ch_8,
            Core::CheckSigVerify => &simplicity_sys::c_jets::jets_wrapper::check_sig_verify,
            Core::Complement16 => &simplicity_sys::c_jets::jets_wrapper::complement_16,
            Core::Complement32 => &simplicity_sys::c_jets::jets_wrapper::complement_32,
            Core::Complement64 => &simplicity_sys::c_jets::jets_wrapper::complement_64,
            Core::Complement8 => &simplicity_sys::c_jets::jets_wrapper::complement_8,
            Core::Decompress => &simplicity_sys::c_jets::jets_wrapper::decompress,
            Core::Decrement16 => &simplicity_sys::c_jets::jets_wrapper::decrement_16,
            Core::Decrement32 => &simplicity_sys::c_jets::jets_wrapper::decrement_32,
            Core::Decrement64 => &simplicity_sys::c_jets::jets_wrapper::decrement_64,
            Core::Decrement8 => &simplicity_sys::c_jets::jets_wrapper::decrement_8,
            Core::DivMod16 => &simplicity_sys::c_jets::jets_wrapper::div_mod_16,
            Core::DivMod32 => &simplicity_sys::c_jets::jets_wrapper::div_mod_32,
            Core::DivMod64 => &simplicity_sys::c_jets::jets_wrapper::div_mod_64,
            Core::DivMod8 => &simplicity_sys::c_jets::jets_wrapper::div_mod_8,
            Core::Divide16 => &simplicity_sys::c_jets::jets_wrapper::divide_16,
            Core::Divide32 => &simplicity_sys::c_jets::jets_wrapper::divide_32,
            Core::Divide64 => &simplicity_sys::c_jets::jets_wrapper::divide_64,
            Core::Divide8 => &simplicity_sys::c_jets::jets_wrapper::divide_8,
            Core::Divides16 => &simplicity_sys::c_jets::jets_wrapper::divides_16,
            Core::Divides32 => &simplicity_sys::c_jets::jets_wrapper::divides_32,
            Core::Divides64 => &simplicity_sys::c_jets::jets_wrapper::divides_64,
            Core::Divides8 => &simplicity_sys::c_jets::jets_wrapper::divides_8,
            Core::Eq16 => &simplicity_sys::c_jets::jets_wrapper::eq_16,
            Core::Eq256 => &simplicity_sys::c_jets::jets_wrapper::eq_256,
            Core::Eq32 => &simplicity_sys::c_jets::jets_wrapper::eq_32,
            Core::Eq64 => &simplicity_sys::c_jets::jets_wrapper::eq_64,
            Core::Eq8 => &simplicity_sys::c_jets::jets_wrapper::eq_8,
            Core::FeAdd => &simplicity_sys::c_jets::jets_wrapper::fe_add,
            Core::FeInvert => &simplicity_sys::c_jets::jets_wrapper::fe_invert,
            Core::FeIsOdd => &simplicity_sys::c_jets::jets_wrapper::fe_is_odd,
            Core::FeIsZero => &simplicity_sys::c_jets::jets_wrapper::fe_is_zero,
            Core::FeMultiply => &simplicity_sys::c_jets::jets_wrapper::fe_multiply,
            Core::FeMultiplyBeta => &simplicity_sys::c_jets::jets_wrapper::fe_multiply_beta,
            Core::FeNegate => &simplicity_sys::c_jets::jets_wrapper::fe_negate,
            Core::FeNormalize => &simplicity_sys::c_jets::jets_wrapper::fe_normalize,
            Core::FeSquare => &simplicity_sys::c_jets::jets_wrapper::fe_square,
            Core::FeSquareRoot => &simplicity_sys::c_jets::jets_wrapper::fe_square_root,
            Core::FullAdd16 => &simplicity_sys::c_jets::jets_wrapper::full_add_16,
            Core::FullAdd32 => &simplicity_sys::c_jets::jets_wrapper::full_add_32,
            Core::FullAdd64 => &simplicity_sys::c_jets::jets_wrapper::full_add_64,
            Core::FullAdd8 => &simplicity_sys::c_jets::jets_wrapper::full_add_8,
            Core::FullDecrement16 => &simplicity_sys::c_jets::jets_wrapper::full_decrement_16,
            Core::FullDecrement32 => &simplicity_sys::c_jets::jets_wrapper::full_decrement_32,
            Core::FullDecrement64 => &simplicity_sys::c_jets::jets_wrapper::full_decrement_64,
            Core::FullDecrement8 => &simplicity_sys::c_jets::jets_wrapper::full_decrement_8,
            Core::FullIncrement16 => &simplicity_sys::c_jets::jets_wrapper::full_increment_16,
            Core::FullIncrement32 => &simplicity_sys::c_jets::jets_wrapper::full_increment_32,
            Core::FullIncrement64 => &simplicity_sys::c_jets::jets_wrapper::full_increment_64,
            Core::FullIncrement8 => &simplicity_sys::c_jets::jets_wrapper::full_increment_8,
            Core::FullMultiply16 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_16,
            Core::FullMultiply32 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_32,
            Core::FullMultiply64 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_64,
            Core::FullMultiply8 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_8,
            Core::FullSubtract16 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_16,
            Core::FullSubtract32 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_32,
            Core::FullSubtract64 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_64,
            Core::FullSubtract8 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_8,
            Core::GeIsOnCurve => &simplicity_sys::c_jets::jets_wrapper::ge_is_on_curve,
            Core::GeNegate => &simplicity_sys::c_jets::jets_wrapper::ge_negate,
            Core::GejAdd => &simplicity_sys::c_jets::jets_wrapper::gej_add,
            Core::GejDouble => &simplicity_sys::c_jets::jets_wrapper::gej_double,
            Core::GejGeAdd => &simplicity_sys::c_jets::jets_wrapper::gej_ge_add,
            Core::GejGeAddEx => &simplicity_sys::c_jets::jets_wrapper::gej_ge_add_ex,
            Core::GejInfinity => &simplicity_sys::c_jets::jets_wrapper::gej_infinity,
            Core::GejIsInfinity => &simplicity_sys::c_jets::jets_wrapper::gej_is_infinity,
            Core::GejIsOnCurve => &simplicity_sys::c_jets::jets_wrapper::gej_is_on_curve,
            Core::GejNegate => &simplicity_sys::c_jets::jets_wrapper::gej_negate,
            Core::GejNormalize => &simplicity_sys::c_jets::jets_wrapper::gej_normalize,
            Core::GejRescale => &simplicity_sys::c_jets::jets_wrapper::gej_rescale,
            Core::GejXEquiv => &simplicity_sys::c_jets::jets_wrapper::gej_x_equiv,
            Core::GejYIsOdd => &simplicity_sys::c_jets::jets_wrapper::gej_y_is_odd,
            Core::Generate => &simplicity_sys::c_jets::jets_wrapper::generate,
            Core::High16 => &simplicity_sys::c_jets::jets_wrapper::high_16,
            Core::High32 => &simplicity_sys::c_jets::jets_wrapper::high_32,
            Core::High64 => &simplicity_sys::c_jets::jets_wrapper::high_64,
            Core::High8 => &simplicity_sys::c_jets::jets_wrapper::high_8,
            Core::Increment16 => &simplicity_sys::c_jets::jets_wrapper::increment_16,
            Core::Increment32 => &simplicity_sys::c_jets::jets_wrapper::increment_32,
            Core::Increment64 => &simplicity_sys::c_jets::jets_wrapper::increment_64,
            Core::Increment8 => &simplicity_sys::c_jets::jets_wrapper::increment_8,
            Core::IsOne16 => &simplicity_sys::c_jets::jets_wrapper::is_one_16,
            Core::IsOne32 => &simplicity_sys::c_jets::jets_wrapper::is_one_32,
            Core::IsOne64 => &simplicity_sys::c_jets::jets_wrapper::is_one_64,
            Core::IsOne8 => &simplicity_sys::c_jets::jets_wrapper::is_one_8,
            Core::IsZero16 => &simplicity_sys::c_jets::jets_wrapper::is_zero_16,
            Core::IsZero32 => &simplicity_sys::c_jets::jets_wrapper::is_zero_32,
            Core::IsZero64 => &simplicity_sys::c_jets::jets_wrapper::is_zero_64,
            Core::IsZero8 => &simplicity_sys::c_jets::jets_wrapper::is_zero_8,
            Core::Le16 => &simplicity_sys::c_jets::jets_wrapper::le_16,
            Core::Le32 => &simplicity_sys::c_jets::jets_wrapper::le_32,
            Core::Le64 => &simplicity_sys::c_jets::jets_wrapper::le_64,
            Core::Le8 => &simplicity_sys::c_jets::jets_wrapper::le_8,
            Core::LinearCombination1 => &simplicity_sys::c_jets::jets_wrapper::linear_combination_1,
            Core::LinearVerify1 => &simplicity_sys::c_jets::jets_wrapper::linear_verify_1,
            Core::Low16 => &simplicity_sys::c_jets::jets_wrapper::low_16,
            Core::Low32 => &simplicity_sys::c_jets::jets_wrapper::low_32,
            Core::Low64 => &simplicity_sys::c_jets::jets_wrapper::low_64,
            Core::Low8 => &simplicity_sys::c_jets::jets_wrapper::low_8,
            Core::Lt16 => &simplicity_sys::c_jets::jets_wrapper::lt_16,
            Core::Lt32 => &simplicity_sys::c_jets::jets_wrapper::lt_32,
            Core::Lt64 => &simplicity_sys::c_jets::jets_wrapper::lt_64,
            Core::Lt8 => &simplicity_sys::c_jets::jets_wrapper::lt_8,
            Core::Maj16 => &simplicity_sys::c_jets::jets_wrapper::maj_16,
            Core::Maj32 => &simplicity_sys::c_jets::jets_wrapper::maj_32,
            Core::Maj64 => &simplicity_sys::c_jets::jets_wrapper::maj_64,
            Core::Maj8 => &simplicity_sys::c_jets::jets_wrapper::maj_8,
            Core::Max16 => &simplicity_sys::c_jets::jets_wrapper::max_16,
            Core::Max32 => &simplicity_sys::c_jets::jets_wrapper::max_32,
            Core::Max64 => &simplicity_sys::c_jets::jets_wrapper::max_64,
            Core::Max8 => &simplicity_sys::c_jets::jets_wrapper::max_8,
            Core::Median16 => &simplicity_sys::c_jets::jets_wrapper::median_16,
            Core::Median32 => &simplicity_sys::c_jets::jets_wrapper::median_32,
            Core::Median64 => &simplicity_sys::c_jets::jets_wrapper::median_64,
            Core::Median8 => &simplicity_sys::c_jets::jets_wrapper::median_8,
            Core::Min16 => &simplicity_sys::c_jets::jets_wrapper::min_16,
            Core::Min32 => &simplicity_sys::c_jets::jets_wrapper::min_32,
            Core::Min64 => &simplicity_sys::c_jets::jets_wrapper::min_64,
            Core::Min8 => &simplicity_sys::c_jets::jets_wrapper::min_8,
            Core::Modulo16 => &simplicity_sys::c_jets::jets_wrapper::modulo_16,
            Core::Modulo32 => &simplicity_sys::c_jets::jets_wrapper::modulo_32,
            Core::Modulo64 => &simplicity_sys::c_jets::jets_wrapper::modulo_64,
            Core::Modulo8 => &simplicity_sys::c_jets::jets_wrapper::modulo_8,
            Core::Multiply16 => &simplicity_sys::c_jets::jets_wrapper::multiply_16,
            Core::Multiply32 => &simplicity_sys::c_jets::jets_wrapper::multiply_32,
            Core::Multiply64 => &simplicity_sys::c_jets::jets_wrapper::multiply_64,
            Core::Multiply8 => &simplicity_sys::c_jets::jets_wrapper::multiply_8,
            Core::Negate16 => &simplicity_sys::c_jets::jets_wrapper::negate_16,
            Core::Negate32 => &simplicity_sys::c_jets::jets_wrapper::negate_32,
            Core::Negate64 => &simplicity_sys::c_jets::jets_wrapper::negate_64,
            Core::Negate8 => &simplicity_sys::c_jets::jets_wrapper::negate_8,
            Core::One16 => &simplicity_sys::c_jets::jets_wrapper::one_16,
            Core::One32 => &simplicity_sys::c_jets::jets_wrapper::one_32,
            Core::One64 => &simplicity_sys::c_jets::jets_wrapper::one_64,
            Core::One8 => &simplicity_sys::c_jets::jets_wrapper::one_8,
            Core::Or16 => &simplicity_sys::c_jets::jets_wrapper::or_16,
            Core::Or32 => &simplicity_sys::c_jets::jets_wrapper::or_32,
            Core::Or64 => &simplicity_sys::c_jets::jets_wrapper::or_64,
            Core::Or8 => &simplicity_sys::c_jets::jets_wrapper::or_8,
            Core::ParseLock => &simplicity_sys::c_jets::jets_wrapper::parse_lock,
            Core::ParseSequence => &simplicity_sys::c_jets::jets_wrapper::parse_sequence,
            Core::PointVerify1 => &simplicity_sys::c_jets::jets_wrapper::point_verify_1,
            Core::ScalarAdd => &simplicity_sys::c_jets::jets_wrapper::scalar_add,
            Core::ScalarInvert => &simplicity_sys::c_jets::jets_wrapper::scalar_invert,
            Core::ScalarIsZero => &simplicity_sys::c_jets::jets_wrapper::scalar_is_zero,
            Core::ScalarMultiply => &simplicity_sys::c_jets::jets_wrapper::scalar_multiply,
            Core::ScalarMultiplyLambda => &simplicity_sys::c_jets::jets_wrapper::scalar_multiply_lambda,
            Core::ScalarNegate => &simplicity_sys::c_jets::jets_wrapper::scalar_negate,
            Core::ScalarNormalize => &simplicity_sys::c_jets::jets_wrapper::scalar_normalize,
            Core::ScalarSquare => &simplicity_sys::c_jets::jets_wrapper::scalar_square,
            Core::Scale => &simplicity_sys::c_jets::jets_wrapper::scale,
            Core::Sha256Block => &simplicity_sys::c_jets::jets_wrapper::sha_256_block,
            Core::Sha256Ctx8Add1 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_1,
            Core::Sha256Ctx8Add128 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_128,
            Core::Sha256Ctx8Add16 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_16,
            Core::Sha256Ctx8Add2 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_2,
            Core::Sha256Ctx8Add256 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_256,
            Core::Sha256Ctx8Add32 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_32,
            Core::Sha256Ctx8Add4 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_4,
            Core::Sha256Ctx8Add512 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_512,
            Core::Sha256Ctx8Add64 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_64,
            Core::Sha256Ctx8Add8 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_8,
            Core::Sha256Ctx8AddBuffer511 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_buffer_511,
            Core::Sha256Ctx8Finalize => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_finalize,
            Core::Sha256Ctx8Init => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_init,
            Core::Sha256Iv => &simplicity_sys::c_jets::jets_wrapper::sha_256_iv,
            Core::Some16 => &simplicity_sys::c_jets::jets_wrapper::some_16,
            Core::Some32 => &simplicity_sys::c_jets::jets_wrapper::some_32,
            Core::Some64 => &simplicity_sys::c_jets::jets_wrapper::some_64,
            Core::Some8 => &simplicity_sys::c_jets::jets_wrapper::some_8,
            Core::Subtract16 => &simplicity_sys::c_jets::jets_wrapper::subtract_16,
            Core::Subtract32 => &simplicity_sys::c_jets::jets_wrapper::subtract_32,
            Core::Subtract64 => &simplicity_sys::c_jets::jets_wrapper::subtract_64,
            Core::Subtract8 => &simplicity_sys::c_jets::jets_wrapper::subtract_8,
            Core::Verify => &simplicity_sys::c_jets::jets_wrapper::verify,
            Core::Xor16 => &simplicity_sys::c_jets::jets_wrapper::xor_16,
            Core::Xor32 => &simplicity_sys::c_jets::jets_wrapper::xor_32,
            Core::Xor64 => &simplicity_sys::c_jets::jets_wrapper::xor_64,
            Core::Xor8 => &simplicity_sys::c_jets::jets_wrapper::xor_8,
            Core::XorXor16 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_16,
            Core::XorXor32 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_32,
            Core::XorXor64 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_64,
            Core::XorXor8 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_8,
        }
    }

    fn cost(&self) -> Cost {
        match self {
            Core::Add16 => Cost::from_milliweight(226),
            Core::Add32 => Cost::from_milliweight(183),
            Core::Add64 => Cost::from_milliweight(221),
            Core::Add8 => Cost::from_milliweight(150),
            Core::All16 => Cost::from_milliweight(110),
            Core::All32 => Cost::from_milliweight(136),
            Core::All64 => Cost::from_milliweight(165),
            Core::All8 => Cost::from_milliweight(113),
            Core::And16 => Cost::from_milliweight(195),
            Core::And32 => Cost::from_milliweight(175),
            Core::And64 => Cost::from_milliweight(221),
            Core::And8 => Cost::from_milliweight(159),
            Core::Bip0340Verify => Cost::from_milliweight(49671),
            Core::Ch16 => Cost::from_milliweight(245),
            Core::Ch32 => Cost::from_milliweight(238),
            Core::Ch64 => Cost::from_milliweight(274),
            Core::Ch8 => Cost::from_milliweight(240),
            Core::CheckSigVerify => Cost::from_milliweight(50000),
            Core::Complement16 => Cost::from_milliweight(146),
            Core::Complement32 => Cost::from_milliweight(161),
            Core::Complement64 => Cost::from_milliweight(174),
            Core::Complement8 => Cost::from_milliweight(139),
            Core::Decompress => Cost::from_milliweight(10956),
            Core::Decrement16 => Cost::from_milliweight(116),
            Core::Decrement32 => Cost::from_milliweight(159),
            Core::Decrement64 => Cost::from_milliweight(160),
            Core::Decrement8 => Cost::from_milliweight(195),
            Core::DivMod16 => Cost::from_milliweight(223),
            Core::DivMod32 => Cost::from_milliweight(198),
            Core::DivMod64 => Cost::from_milliweight(220),
            Core::DivMod8 => Cost::from_milliweight(141),
            Core::Divide16 => Cost::from_milliweight(188),
            Core::Divide32 => Cost::from_milliweight(225),
            Core::Divide64 => Cost::from_milliweight(202),
            Core::Divide8 => Cost::from_milliweight(125),
            Core::Divides16 => Cost::from_milliweight(173),
            Core::Divides32 => Cost::from_milliweight(175),
            Core::Divides64 => Cost::from_milliweight(246),
            Core::Divides8 => Cost::from_milliweight(142),
            Core::Eq16 => Cost::from_milliweight(174),
            Core::Eq256 => Cost::from_milliweight(431),
            Core::Eq32 => Cost::from_milliweight(233),
            Core::Eq64 => Cost::from_milliweight(202),
            Core::Eq8 => Cost::from_milliweight(120),
            Core::FeAdd => Cost::from_milliweight(908),
            Core::FeInvert => Cost::from_milliweight(3375),
            Core::FeIsOdd => Cost::from_milliweight(544),
            Core::FeIsZero => Cost::from_milliweight(521),
            Core::FeMultiply => Cost::from_milliweight(975),
            Core::FeMultiplyBeta => Cost::from_milliweight(824),
            Core::FeNegate => Cost::from_milliweight(846),
            Core::FeNormalize => Cost::from_milliweight(813),
            Core::FeSquare => Cost::from_milliweight(829),
            Core::FeSquareRoot => Cost::from_milliweight(10698),
            Core::FullAdd16 => Cost::from_milliweight(193),
            Core::FullAdd32 => Cost::from_milliweight(197),
            Core::FullAdd64 => Cost::from_milliweight(225),
            Core::FullAdd8 => Cost::from_milliweight(190),
            Core::FullDecrement16 => Cost::from_milliweight(107),
            Core::FullDecrement32 => Cost::from_milliweight(153),
            Core::FullDecrement64 => Cost::from_milliweight(151),
            Core::FullDecrement8 => Cost::from_milliweight(218),
            Core::FullIncrement16 => Cost::from_milliweight(108),
            Core::FullIncrement32 => Cost::from_milliweight(171),
            Core::FullIncrement64 => Cost::from_milliweight(161),
            Core::FullIncrement8 => Cost::from_milliweight(204),
            Core::FullMultiply16 => Cost::from_milliweight(208),
            Core::FullMultiply32 => Cost::from_milliweight(213),
            Core::FullMultiply64 => Cost::from_milliweight(209),
            Core::FullMultiply8 => Cost::from_milliweight(190),
            Core::FullSubtract16 => Cost::from_milliweight(201),
            Core::FullSubtract32 => Cost::from_milliweight(170),
            Core::FullSubtract64 => Cost::from_milliweight(231),
            Core::FullSubtract8 => Cost::from_milliweight(141),
            Core::GeIsOnCurve => Cost::from_milliweight(763),
            Core::GeNegate => Cost::from_milliweight(1278),
            Core::GejAdd => Cost::from_milliweight(3292),
            Core::GejDouble => Cost::from_milliweight(2103),
            Core::GejGeAdd => Cost::from_milliweight(2890),
            Core::GejGeAddEx => Cost::from_milliweight(3114),
            Core::GejInfinity => Cost::from_milliweight(971),
            Core::GejIsInfinity => Cost::from_milliweight(923),
            Core::GejIsOnCurve => Cost::from_milliweight(1106),
            Core::GejNegate => Cost::from_milliweight(1823),
            Core::GejNormalize => Cost::from_milliweight(4337),
            Core::GejRescale => Cost::from_milliweight(2315),
            Core::GejXEquiv => Cost::from_milliweight(1270),
            Core::GejYIsOdd => Cost::from_milliweight(3665),
            Core::Generate => Cost::from_milliweight(51706),
            Core::High16 => Cost::from_milliweight(159),
            Core::High32 => Cost::from_milliweight(121),
            Core::High64 => Cost::from_milliweight(110),
            Core::High8 => Cost::from_milliweight(169),
            Core::Increment16 => Cost::from_milliweight(129),
            Core::Increment32 => Cost::from_milliweight(195),
            Core::Increment64 => Cost::from_milliweight(187),
            Core::Increment8 => Cost::from_milliweight(155),
            Core::IsOne16 => Cost::from_milliweight(117),
            Core::IsOne32 => Cost::from_milliweight(136),
            Core::IsOne64 => Cost::from_milliweight(163),
            Core::IsOne8 => Cost::from_milliweight(160),
            Core::IsZero16 => Cost::from_milliweight(143),
            Core::IsZero32 => Cost::from_milliweight(135),
            Core::IsZero64 => Cost::from_milliweight(136),
            Core::IsZero8 => Cost::from_milliweight(163),
            Core::Le16 => Cost::from_milliweight(166),
            Core::Le32 => Cost::from_milliweight(216),
            Core::Le64 => Cost::from_milliweight(173),
            Core::Le8 => Cost::from_milliweight(143),
            Core::LinearCombination1 => Cost::from_milliweight(86722),
            Core::LinearVerify1 => Cost::from_milliweight(43063),
            Core::Low16 => Cost::from_milliweight(172),
            Core::Low32 => Cost::from_milliweight(170),
            Core::Low64 => Cost::from_milliweight(162),
            Core::Low8 => Cost::from_milliweight(173),
            Core::Lt16 => Cost::from_milliweight(188),
            Core::Lt32 => Cost::from_milliweight(215),
            Core::Lt64 => Cost::from_milliweight(195),
            Core::Lt8 => Cost::from_milliweight(130),
            Core::Maj16 => Cost::from_milliweight(273),
            Core::Maj32 => Cost::from_milliweight(289),
            Core::Maj64 => Cost::from_milliweight(293),
            Core::Maj8 => Cost::from_milliweight(241),
            Core::Max16 => Cost::from_milliweight(164),
            Core::Max32 => Cost::from_milliweight(162),
            Core::Max64 => Cost::from_milliweight(193),
            Core::Max8 => Cost::from_milliweight(142),
            Core::Median16 => Cost::from_milliweight(270),
            Core::Median32 => Cost::from_milliweight(256),
            Core::Median64 => Cost::from_milliweight(336),
            Core::Median8 => Cost::from_milliweight(256),
            Core::Min16 => Cost::from_milliweight(164),
            Core::Min32 => Cost::from_milliweight(181),
            Core::Min64 => Cost::from_milliweight(150),
            Core::Min8 => Cost::from_milliweight(135),
            Core::Modulo16 => Cost::from_milliweight(188),
            Core::Modulo32 => Cost::from_milliweight(207),
            Core::Modulo64 => Cost::from_milliweight(191),
            Core::Modulo8 => Cost::from_milliweight(158),
            Core::Multiply16 => Cost::from_milliweight(154),
            Core::Multiply32 => Cost::from_milliweight(165),
            Core::Multiply64 => Cost::from_milliweight(185),
            Core::Multiply8 => Cost::from_milliweight(126),
            Core::Negate16 => Cost::from_milliweight(121),
            Core::Negate32 => Cost::from_milliweight(185),
            Core::Negate64 => Cost::from_milliweight(162),
            Core::Negate8 => Cost::from_milliweight(152),
            Core::One16 => Cost::from_milliweight(126),
            Core::One32 => Cost::from_milliweight(122),
            Core::One64 => Cost::from_milliweight(123),
            Core::One8 => Cost::from_milliweight(127),
            Core::Or16 => Cost::from_milliweight(204),
            Core::Or32 => Cost::from_milliweight(197),
            Core::Or64 => Cost::from_milliweight(214),
            Core::Or8 => Cost::from_milliweight(147),
            Core::ParseLock => Cost::from_milliweight(177),
            Core::ParseSequence => Cost::from_milliweight(261),
            Core::PointVerify1 => Cost::from_milliweight(50604),
            Core::ScalarAdd => Cost::from_milliweight(962),
            Core::ScalarInvert => Cost::from_milliweight(4025),
            Core::ScalarIsZero => Cost::from_milliweight(569),
            Core::ScalarMultiply => Cost::from_milliweight(1230),
            Core::ScalarMultiplyLambda => Cost::from_milliweight(984),
            Core::ScalarNegate => Cost::from_milliweight(851),
            Core::ScalarNormalize => Cost::from_milliweight(808),
            Core::ScalarSquare => Cost::from_milliweight(947),
            Core::Scale => Cost::from_milliweight(75377),
            Core::Sha256Block => Cost::from_milliweight(986),
            Core::Sha256Ctx8Add1 => Cost::from_milliweight(1600),
            Core::Sha256Ctx8Add128 => Cost::from_milliweight(3921),
            Core::Sha256Ctx8Add16 => Cost::from_milliweight(2275),
            Core::Sha256Ctx8Add2 => Cost::from_milliweight(3772),
            Core::Sha256Ctx8Add256 => Cost::from_milliweight(6211),
            Core::Sha256Ctx8Add32 => Cost::from_milliweight(4599),
            Core::Sha256Ctx8Add4 => Cost::from_milliweight(3515),
            Core::Sha256Ctx8Add512 => Cost::from_milliweight(10936),
            Core::Sha256Ctx8Add64 => Cost::from_milliweight(3111),
            Core::Sha256Ctx8Add8 => Cost::from_milliweight(1625),
            Core::Sha256Ctx8AddBuffer511 => Cost::from_milliweight(14290),
            Core::Sha256Ctx8Finalize => Cost::from_milliweight(2111),
            Core::Sha256Ctx8Init => Cost::from_milliweight(184),
            Core::Sha256Iv => Cost::from_milliweight(129),
            Core::Some16 => Cost::from_milliweight(129),
            Core::Some32 => Cost::from_milliweight(183),
            Core::Some64 => Cost::from_milliweight(139),
            Core::Some8 => Cost::from_milliweight(104),
            Core::Subtract16 => Cost::from_milliweight(237),
            Core::Subtract32 => Cost::from_milliweight(186),
            Core::Subtract64 => Cost::from_milliweight(315),
            Core::Subtract8 => Cost::from_milliweight(149),
            Core::Verify => Cost::from_milliweight(144),
            Core::Xor16 => Cost::from_milliweight(188),
            Core::Xor32 => Cost::from_milliweight(204),
            Core::Xor64 => Cost::from_milliweight(207),
            Core::Xor8 => Cost::from_milliweight(135),
            Core::XorXor16 => Cost::from_milliweight(235),
            Core::XorXor32 => Cost::from_milliweight(251),
            Core::XorXor64 => Cost::from_milliweight(285),
            Core::XorXor8 => Cost::from_milliweight(258),
        }
    }
}

impl fmt::Display for Core {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Core::Add16 => f.write_str("add_16"),
            Core::Add32 => f.write_str("add_32"),
            Core::Add64 => f.write_str("add_64"),
            Core::Add8 => f.write_str("add_8"),
            Core::All16 => f.write_str("all_16"),
            Core::All32 => f.write_str("all_32"),
            Core::All64 => f.write_str("all_64"),
            Core::All8 => f.write_str("all_8"),
            Core::And16 => f.write_str("and_16"),
            Core::And32 => f.write_str("and_32"),
            Core::And64 => f.write_str("and_64"),
            Core::And8 => f.write_str("and_8"),
            Core::Bip0340Verify => f.write_str("bip_0340_verify"),
            Core::Ch16 => f.write_str("ch_16"),
            Core::Ch32 => f.write_str("ch_32"),
            Core::Ch64 => f.write_str("ch_64"),
            Core::Ch8 => f.write_str("ch_8"),
            Core::CheckSigVerify => f.write_str("check_sig_verify"),
            Core::Complement16 => f.write_str("complement_16"),
            Core::Complement32 => f.write_str("complement_32"),
            Core::Complement64 => f.write_str("complement_64"),
            Core::Complement8 => f.write_str("complement_8"),
            Core::Decompress => f.write_str("decompress"),
            Core::Decrement16 => f.write_str("decrement_16"),
            Core::Decrement32 => f.write_str("decrement_32"),
            Core::Decrement64 => f.write_str("decrement_64"),
            Core::Decrement8 => f.write_str("decrement_8"),
            Core::DivMod16 => f.write_str("div_mod_16"),
            Core::DivMod32 => f.write_str("div_mod_32"),
            Core::DivMod64 => f.write_str("div_mod_64"),
            Core::DivMod8 => f.write_str("div_mod_8"),
            Core::Divide16 => f.write_str("divide_16"),
            Core::Divide32 => f.write_str("divide_32"),
            Core::Divide64 => f.write_str("divide_64"),
            Core::Divide8 => f.write_str("divide_8"),
            Core::Divides16 => f.write_str("divides_16"),
            Core::Divides32 => f.write_str("divides_32"),
            Core::Divides64 => f.write_str("divides_64"),
            Core::Divides8 => f.write_str("divides_8"),
            Core::Eq16 => f.write_str("eq_16"),
            Core::Eq256 => f.write_str("eq_256"),
            Core::Eq32 => f.write_str("eq_32"),
            Core::Eq64 => f.write_str("eq_64"),
            Core::Eq8 => f.write_str("eq_8"),
            Core::FeAdd => f.write_str("fe_add"),
            Core::FeInvert => f.write_str("fe_invert"),
            Core::FeIsOdd => f.write_str("fe_is_odd"),
            Core::FeIsZero => f.write_str("fe_is_zero"),
            Core::FeMultiply => f.write_str("fe_multiply"),
            Core::FeMultiplyBeta => f.write_str("fe_multiply_beta"),
            Core::FeNegate => f.write_str("fe_negate"),
            Core::FeNormalize => f.write_str("fe_normalize"),
            Core::FeSquare => f.write_str("fe_square"),
            Core::FeSquareRoot => f.write_str("fe_square_root"),
            Core::FullAdd16 => f.write_str("full_add_16"),
            Core::FullAdd32 => f.write_str("full_add_32"),
            Core::FullAdd64 => f.write_str("full_add_64"),
            Core::FullAdd8 => f.write_str("full_add_8"),
            Core::FullDecrement16 => f.write_str("full_decrement_16"),
            Core::FullDecrement32 => f.write_str("full_decrement_32"),
            Core::FullDecrement64 => f.write_str("full_decrement_64"),
            Core::FullDecrement8 => f.write_str("full_decrement_8"),
            Core::FullIncrement16 => f.write_str("full_increment_16"),
            Core::FullIncrement32 => f.write_str("full_increment_32"),
            Core::FullIncrement64 => f.write_str("full_increment_64"),
            Core::FullIncrement8 => f.write_str("full_increment_8"),
            Core::FullMultiply16 => f.write_str("full_multiply_16"),
            Core::FullMultiply32 => f.write_str("full_multiply_32"),
            Core::FullMultiply64 => f.write_str("full_multiply_64"),
            Core::FullMultiply8 => f.write_str("full_multiply_8"),
            Core::FullSubtract16 => f.write_str("full_subtract_16"),
            Core::FullSubtract32 => f.write_str("full_subtract_32"),
            Core::FullSubtract64 => f.write_str("full_subtract_64"),
            Core::FullSubtract8 => f.write_str("full_subtract_8"),
            Core::GeIsOnCurve => f.write_str("ge_is_on_curve"),
            Core::GeNegate => f.write_str("ge_negate"),
            Core::GejAdd => f.write_str("gej_add"),
            Core::GejDouble => f.write_str("gej_double"),
            Core::GejGeAdd => f.write_str("gej_ge_add"),
            Core::GejGeAddEx => f.write_str("gej_ge_add_ex"),
            Core::GejInfinity => f.write_str("gej_infinity"),
            Core::GejIsInfinity => f.write_str("gej_is_infinity"),
            Core::GejIsOnCurve => f.write_str("gej_is_on_curve"),
            Core::GejNegate => f.write_str("gej_negate"),
            Core::GejNormalize => f.write_str("gej_normalize"),
            Core::GejRescale => f.write_str("gej_rescale"),
            Core::GejXEquiv => f.write_str("gej_x_equiv"),
            Core::GejYIsOdd => f.write_str("gej_y_is_odd"),
            Core::Generate => f.write_str("generate"),
            Core::High16 => f.write_str("high_16"),
            Core::High32 => f.write_str("high_32"),
            Core::High64 => f.write_str("high_64"),
            Core::High8 => f.write_str("high_8"),
            Core::Increment16 => f.write_str("increment_16"),
            Core::Increment32 => f.write_str("increment_32"),
            Core::Increment64 => f.write_str("increment_64"),
            Core::Increment8 => f.write_str("increment_8"),
            Core::IsOne16 => f.write_str("is_one_16"),
            Core::IsOne32 => f.write_str("is_one_32"),
            Core::IsOne64 => f.write_str("is_one_64"),
            Core::IsOne8 => f.write_str("is_one_8"),
            Core::IsZero16 => f.write_str("is_zero_16"),
            Core::IsZero32 => f.write_str("is_zero_32"),
            Core::IsZero64 => f.write_str("is_zero_64"),
            Core::IsZero8 => f.write_str("is_zero_8"),
            Core::Le16 => f.write_str("le_16"),
            Core::Le32 => f.write_str("le_32"),
            Core::Le64 => f.write_str("le_64"),
            Core::Le8 => f.write_str("le_8"),
            Core::LinearCombination1 => f.write_str("linear_combination_1"),
            Core::LinearVerify1 => f.write_str("linear_verify_1"),
            Core::Low16 => f.write_str("low_16"),
            Core::Low32 => f.write_str("low_32"),
            Core::Low64 => f.write_str("low_64"),
            Core::Low8 => f.write_str("low_8"),
            Core::Lt16 => f.write_str("lt_16"),
            Core::Lt32 => f.write_str("lt_32"),
            Core::Lt64 => f.write_str("lt_64"),
            Core::Lt8 => f.write_str("lt_8"),
            Core::Maj16 => f.write_str("maj_16"),
            Core::Maj32 => f.write_str("maj_32"),
            Core::Maj64 => f.write_str("maj_64"),
            Core::Maj8 => f.write_str("maj_8"),
            Core::Max16 => f.write_str("max_16"),
            Core::Max32 => f.write_str("max_32"),
            Core::Max64 => f.write_str("max_64"),
            Core::Max8 => f.write_str("max_8"),
            Core::Median16 => f.write_str("median_16"),
            Core::Median32 => f.write_str("median_32"),
            Core::Median64 => f.write_str("median_64"),
            Core::Median8 => f.write_str("median_8"),
            Core::Min16 => f.write_str("min_16"),
            Core::Min32 => f.write_str("min_32"),
            Core::Min64 => f.write_str("min_64"),
            Core::Min8 => f.write_str("min_8"),
            Core::Modulo16 => f.write_str("modulo_16"),
            Core::Modulo32 => f.write_str("modulo_32"),
            Core::Modulo64 => f.write_str("modulo_64"),
            Core::Modulo8 => f.write_str("modulo_8"),
            Core::Multiply16 => f.write_str("multiply_16"),
            Core::Multiply32 => f.write_str("multiply_32"),
            Core::Multiply64 => f.write_str("multiply_64"),
            Core::Multiply8 => f.write_str("multiply_8"),
            Core::Negate16 => f.write_str("negate_16"),
            Core::Negate32 => f.write_str("negate_32"),
            Core::Negate64 => f.write_str("negate_64"),
            Core::Negate8 => f.write_str("negate_8"),
            Core::One16 => f.write_str("one_16"),
            Core::One32 => f.write_str("one_32"),
            Core::One64 => f.write_str("one_64"),
            Core::One8 => f.write_str("one_8"),
            Core::Or16 => f.write_str("or_16"),
            Core::Or32 => f.write_str("or_32"),
            Core::Or64 => f.write_str("or_64"),
            Core::Or8 => f.write_str("or_8"),
            Core::ParseLock => f.write_str("parse_lock"),
            Core::ParseSequence => f.write_str("parse_sequence"),
            Core::PointVerify1 => f.write_str("point_verify_1"),
            Core::ScalarAdd => f.write_str("scalar_add"),
            Core::ScalarInvert => f.write_str("scalar_invert"),
            Core::ScalarIsZero => f.write_str("scalar_is_zero"),
            Core::ScalarMultiply => f.write_str("scalar_multiply"),
            Core::ScalarMultiplyLambda => f.write_str("scalar_multiply_lambda"),
            Core::ScalarNegate => f.write_str("scalar_negate"),
            Core::ScalarNormalize => f.write_str("scalar_normalize"),
            Core::ScalarSquare => f.write_str("scalar_square"),
            Core::Scale => f.write_str("scale"),
            Core::Sha256Block => f.write_str("sha_256_block"),
            Core::Sha256Ctx8Add1 => f.write_str("sha_256_ctx_8_add_1"),
            Core::Sha256Ctx8Add128 => f.write_str("sha_256_ctx_8_add_128"),
            Core::Sha256Ctx8Add16 => f.write_str("sha_256_ctx_8_add_16"),
            Core::Sha256Ctx8Add2 => f.write_str("sha_256_ctx_8_add_2"),
            Core::Sha256Ctx8Add256 => f.write_str("sha_256_ctx_8_add_256"),
            Core::Sha256Ctx8Add32 => f.write_str("sha_256_ctx_8_add_32"),
            Core::Sha256Ctx8Add4 => f.write_str("sha_256_ctx_8_add_4"),
            Core::Sha256Ctx8Add512 => f.write_str("sha_256_ctx_8_add_512"),
            Core::Sha256Ctx8Add64 => f.write_str("sha_256_ctx_8_add_64"),
            Core::Sha256Ctx8Add8 => f.write_str("sha_256_ctx_8_add_8"),
            Core::Sha256Ctx8AddBuffer511 => f.write_str("sha_256_ctx_8_add_buffer_511"),
            Core::Sha256Ctx8Finalize => f.write_str("sha_256_ctx_8_finalize"),
            Core::Sha256Ctx8Init => f.write_str("sha_256_ctx_8_init"),
            Core::Sha256Iv => f.write_str("sha_256_iv"),
            Core::Some16 => f.write_str("some_16"),
            Core::Some32 => f.write_str("some_32"),
            Core::Some64 => f.write_str("some_64"),
            Core::Some8 => f.write_str("some_8"),
            Core::Subtract16 => f.write_str("subtract_16"),
            Core::Subtract32 => f.write_str("subtract_32"),
            Core::Subtract64 => f.write_str("subtract_64"),
            Core::Subtract8 => f.write_str("subtract_8"),
            Core::Verify => f.write_str("verify"),
            Core::Xor16 => f.write_str("xor_16"),
            Core::Xor32 => f.write_str("xor_32"),
            Core::Xor64 => f.write_str("xor_64"),
            Core::Xor8 => f.write_str("xor_8"),
            Core::XorXor16 => f.write_str("xor_xor_16"),
            Core::XorXor32 => f.write_str("xor_xor_32"),
            Core::XorXor64 => f.write_str("xor_xor_64"),
            Core::XorXor8 => f.write_str("xor_xor_8"),
        }
    }
}

impl str::FromStr for Core {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add_16" => Ok(Core::Add16),
            "add_32" => Ok(Core::Add32),
            "add_64" => Ok(Core::Add64),
            "add_8" => Ok(Core::Add8),
            "all_16" => Ok(Core::All16),
            "all_32" => Ok(Core::All32),
            "all_64" => Ok(Core::All64),
            "all_8" => Ok(Core::All8),
            "and_16" => Ok(Core::And16),
            "and_32" => Ok(Core::And32),
            "and_64" => Ok(Core::And64),
            "and_8" => Ok(Core::And8),
            "bip_0340_verify" => Ok(Core::Bip0340Verify),
            "ch_16" => Ok(Core::Ch16),
            "ch_32" => Ok(Core::Ch32),
            "ch_64" => Ok(Core::Ch64),
            "ch_8" => Ok(Core::Ch8),
            "check_sig_verify" => Ok(Core::CheckSigVerify),
            "complement_16" => Ok(Core::Complement16),
            "complement_32" => Ok(Core::Complement32),
            "complement_64" => Ok(Core::Complement64),
            "complement_8" => Ok(Core::Complement8),
            "decompress" => Ok(Core::Decompress),
            "decrement_16" => Ok(Core::Decrement16),
            "decrement_32" => Ok(Core::Decrement32),
            "decrement_64" => Ok(Core::Decrement64),
            "decrement_8" => Ok(Core::Decrement8),
            "div_mod_16" => Ok(Core::DivMod16),
            "div_mod_32" => Ok(Core::DivMod32),
            "div_mod_64" => Ok(Core::DivMod64),
            "div_mod_8" => Ok(Core::DivMod8),
            "divide_16" => Ok(Core::Divide16),
            "divide_32" => Ok(Core::Divide32),
            "divide_64" => Ok(Core::Divide64),
            "divide_8" => Ok(Core::Divide8),
            "divides_16" => Ok(Core::Divides16),
            "divides_32" => Ok(Core::Divides32),
            "divides_64" => Ok(Core::Divides64),
            "divides_8" => Ok(Core::Divides8),
            "eq_16" => Ok(Core::Eq16),
            "eq_256" => Ok(Core::Eq256),
            "eq_32" => Ok(Core::Eq32),
            "eq_64" => Ok(Core::Eq64),
            "eq_8" => Ok(Core::Eq8),
            "fe_add" => Ok(Core::FeAdd),
            "fe_invert" => Ok(Core::FeInvert),
            "fe_is_odd" => Ok(Core::FeIsOdd),
            "fe_is_zero" => Ok(Core::FeIsZero),
            "fe_multiply" => Ok(Core::FeMultiply),
            "fe_multiply_beta" => Ok(Core::FeMultiplyBeta),
            "fe_negate" => Ok(Core::FeNegate),
            "fe_normalize" => Ok(Core::FeNormalize),
            "fe_square" => Ok(Core::FeSquare),
            "fe_square_root" => Ok(Core::FeSquareRoot),
            "full_add_16" => Ok(Core::FullAdd16),
            "full_add_32" => Ok(Core::FullAdd32),
            "full_add_64" => Ok(Core::FullAdd64),
            "full_add_8" => Ok(Core::FullAdd8),
            "full_decrement_16" => Ok(Core::FullDecrement16),
            "full_decrement_32" => Ok(Core::FullDecrement32),
            "full_decrement_64" => Ok(Core::FullDecrement64),
            "full_decrement_8" => Ok(Core::FullDecrement8),
            "full_increment_16" => Ok(Core::FullIncrement16),
            "full_increment_32" => Ok(Core::FullIncrement32),
            "full_increment_64" => Ok(Core::FullIncrement64),
            "full_increment_8" => Ok(Core::FullIncrement8),
            "full_multiply_16" => Ok(Core::FullMultiply16),
            "full_multiply_32" => Ok(Core::FullMultiply32),
            "full_multiply_64" => Ok(Core::FullMultiply64),
            "full_multiply_8" => Ok(Core::FullMultiply8),
            "full_subtract_16" => Ok(Core::FullSubtract16),
            "full_subtract_32" => Ok(Core::FullSubtract32),
            "full_subtract_64" => Ok(Core::FullSubtract64),
            "full_subtract_8" => Ok(Core::FullSubtract8),
            "ge_is_on_curve" => Ok(Core::GeIsOnCurve),
            "ge_negate" => Ok(Core::GeNegate),
            "gej_add" => Ok(Core::GejAdd),
            "gej_double" => Ok(Core::GejDouble),
            "gej_ge_add" => Ok(Core::GejGeAdd),
            "gej_ge_add_ex" => Ok(Core::GejGeAddEx),
            "gej_infinity" => Ok(Core::GejInfinity),
            "gej_is_infinity" => Ok(Core::GejIsInfinity),
            "gej_is_on_curve" => Ok(Core::GejIsOnCurve),
            "gej_negate" => Ok(Core::GejNegate),
            "gej_normalize" => Ok(Core::GejNormalize),
            "gej_rescale" => Ok(Core::GejRescale),
            "gej_x_equiv" => Ok(Core::GejXEquiv),
            "gej_y_is_odd" => Ok(Core::GejYIsOdd),
            "generate" => Ok(Core::Generate),
            "high_16" => Ok(Core::High16),
            "high_32" => Ok(Core::High32),
            "high_64" => Ok(Core::High64),
            "high_8" => Ok(Core::High8),
            "increment_16" => Ok(Core::Increment16),
            "increment_32" => Ok(Core::Increment32),
            "increment_64" => Ok(Core::Increment64),
            "increment_8" => Ok(Core::Increment8),
            "is_one_16" => Ok(Core::IsOne16),
            "is_one_32" => Ok(Core::IsOne32),
            "is_one_64" => Ok(Core::IsOne64),
            "is_one_8" => Ok(Core::IsOne8),
            "is_zero_16" => Ok(Core::IsZero16),
            "is_zero_32" => Ok(Core::IsZero32),
            "is_zero_64" => Ok(Core::IsZero64),
            "is_zero_8" => Ok(Core::IsZero8),
            "le_16" => Ok(Core::Le16),
            "le_32" => Ok(Core::Le32),
            "le_64" => Ok(Core::Le64),
            "le_8" => Ok(Core::Le8),
            "linear_combination_1" => Ok(Core::LinearCombination1),
            "linear_verify_1" => Ok(Core::LinearVerify1),
            "low_16" => Ok(Core::Low16),
            "low_32" => Ok(Core::Low32),
            "low_64" => Ok(Core::Low64),
            "low_8" => Ok(Core::Low8),
            "lt_16" => Ok(Core::Lt16),
            "lt_32" => Ok(Core::Lt32),
            "lt_64" => Ok(Core::Lt64),
            "lt_8" => Ok(Core::Lt8),
            "maj_16" => Ok(Core::Maj16),
            "maj_32" => Ok(Core::Maj32),
            "maj_64" => Ok(Core::Maj64),
            "maj_8" => Ok(Core::Maj8),
            "max_16" => Ok(Core::Max16),
            "max_32" => Ok(Core::Max32),
            "max_64" => Ok(Core::Max64),
            "max_8" => Ok(Core::Max8),
            "median_16" => Ok(Core::Median16),
            "median_32" => Ok(Core::Median32),
            "median_64" => Ok(Core::Median64),
            "median_8" => Ok(Core::Median8),
            "min_16" => Ok(Core::Min16),
            "min_32" => Ok(Core::Min32),
            "min_64" => Ok(Core::Min64),
            "min_8" => Ok(Core::Min8),
            "modulo_16" => Ok(Core::Modulo16),
            "modulo_32" => Ok(Core::Modulo32),
            "modulo_64" => Ok(Core::Modulo64),
            "modulo_8" => Ok(Core::Modulo8),
            "multiply_16" => Ok(Core::Multiply16),
            "multiply_32" => Ok(Core::Multiply32),
            "multiply_64" => Ok(Core::Multiply64),
            "multiply_8" => Ok(Core::Multiply8),
            "negate_16" => Ok(Core::Negate16),
            "negate_32" => Ok(Core::Negate32),
            "negate_64" => Ok(Core::Negate64),
            "negate_8" => Ok(Core::Negate8),
            "one_16" => Ok(Core::One16),
            "one_32" => Ok(Core::One32),
            "one_64" => Ok(Core::One64),
            "one_8" => Ok(Core::One8),
            "or_16" => Ok(Core::Or16),
            "or_32" => Ok(Core::Or32),
            "or_64" => Ok(Core::Or64),
            "or_8" => Ok(Core::Or8),
            "parse_lock" => Ok(Core::ParseLock),
            "parse_sequence" => Ok(Core::ParseSequence),
            "point_verify_1" => Ok(Core::PointVerify1),
            "scalar_add" => Ok(Core::ScalarAdd),
            "scalar_invert" => Ok(Core::ScalarInvert),
            "scalar_is_zero" => Ok(Core::ScalarIsZero),
            "scalar_multiply" => Ok(Core::ScalarMultiply),
            "scalar_multiply_lambda" => Ok(Core::ScalarMultiplyLambda),
            "scalar_negate" => Ok(Core::ScalarNegate),
            "scalar_normalize" => Ok(Core::ScalarNormalize),
            "scalar_square" => Ok(Core::ScalarSquare),
            "scale" => Ok(Core::Scale),
            "sha_256_block" => Ok(Core::Sha256Block),
            "sha_256_ctx_8_add_1" => Ok(Core::Sha256Ctx8Add1),
            "sha_256_ctx_8_add_128" => Ok(Core::Sha256Ctx8Add128),
            "sha_256_ctx_8_add_16" => Ok(Core::Sha256Ctx8Add16),
            "sha_256_ctx_8_add_2" => Ok(Core::Sha256Ctx8Add2),
            "sha_256_ctx_8_add_256" => Ok(Core::Sha256Ctx8Add256),
            "sha_256_ctx_8_add_32" => Ok(Core::Sha256Ctx8Add32),
            "sha_256_ctx_8_add_4" => Ok(Core::Sha256Ctx8Add4),
            "sha_256_ctx_8_add_512" => Ok(Core::Sha256Ctx8Add512),
            "sha_256_ctx_8_add_64" => Ok(Core::Sha256Ctx8Add64),
            "sha_256_ctx_8_add_8" => Ok(Core::Sha256Ctx8Add8),
            "sha_256_ctx_8_add_buffer_511" => Ok(Core::Sha256Ctx8AddBuffer511),
            "sha_256_ctx_8_finalize" => Ok(Core::Sha256Ctx8Finalize),
            "sha_256_ctx_8_init" => Ok(Core::Sha256Ctx8Init),
            "sha_256_iv" => Ok(Core::Sha256Iv),
            "some_16" => Ok(Core::Some16),
            "some_32" => Ok(Core::Some32),
            "some_64" => Ok(Core::Some64),
            "some_8" => Ok(Core::Some8),
            "subtract_16" => Ok(Core::Subtract16),
            "subtract_32" => Ok(Core::Subtract32),
            "subtract_64" => Ok(Core::Subtract64),
            "subtract_8" => Ok(Core::Subtract8),
            "verify" => Ok(Core::Verify),
            "xor_16" => Ok(Core::Xor16),
            "xor_32" => Ok(Core::Xor32),
            "xor_64" => Ok(Core::Xor64),
            "xor_8" => Ok(Core::Xor8),
            "xor_xor_16" => Ok(Core::XorXor16),
            "xor_xor_32" => Ok(Core::XorXor32),
            "xor_xor_64" => Ok(Core::XorXor64),
            "xor_xor_8" => Ok(Core::XorXor8),
            x => Err(crate::Error::InvalidJetName(x.to_owned())),
        }
    }
}
