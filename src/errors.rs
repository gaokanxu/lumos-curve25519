use thiserror::Error;

#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum Curve25519Error {
    #[error("pod conversion failed")]
    PodConversion,
    
    //gaokanxu 2024.08.16 begin
    //another errors.rs in zk-token-sdk/src/curve25519/
    #[error("signature verification failed")]
    SignatureError(String),
    
    #[error("conversion error")]
    ConversionError,
    

    //gaokanxu 2024.08.15 end
}
