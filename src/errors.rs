use thiserror::Error;

//gaokanxu 2024.08.17
use std::array::TryFromSliceError;

//#[derive(Error, Clone, Debug, Eq, PartialEq)]
//gaokanxu 2024.08.17
#[derive(Error, Clone, Debug)]
pub enum Curve25519Error {
    #[error("pod conversion failed")]
    PodConversion,
    
    //gaokanxu 2024.08.16 begin
    //another errors.rs in zk-token-sdk/src/curve25519/
    #[error("signature verification failed")]
    SignatureError(String),
    
    #[error("conversion error")]
    ConversionError,
    
    #[error("slice error")]
    SliceError(TryFromSliceError),

    #[error("decompression failed")]
    DecompressionFailed,
    //gaokanxu 2024.08.15 end
}

//gaokanxu 2024.08.17 为 Curve25519Error 实现 From<TryFromSliceError>
impl From<TryFromSliceError> for Curve25519Error {
    fn from(err: TryFromSliceError) -> Curve25519Error {
        Curve25519Error::SliceError(err)
    }
}
//gaokanxu 2024.08.17 end
