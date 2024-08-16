pub use bytemuck_derive::{Pod, Zeroable};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Pod, Zeroable)]
#[repr(transparent)]
pub struct PodScalar(pub [u8; 32]);

#[cfg(not(target_os = "lumos"))]
mod target_arch {
    use {super::*, crate::errors::Curve25519Error, curve25519_dalek::scalar::Scalar};

    impl From<&Scalar> for PodScalar {
        fn from(scalar: &Scalar) -> Self {
            Self(scalar.to_bytes())
        }
    }

    impl TryFrom<&PodScalar> for Scalar {
        type Error = Curve25519Error;

        fn try_from(pod: &PodScalar) -> Result<Self, Self::Error> {
            //Scalar::from_canonical_bytes(pod.0).ok_or(Curve25519Error::PodConversion)
            //gaokanxu 2024.08.17 begin
            let ct_option = Scalar::from_canonical_bytes(pod.0);
            if ct_option.is_some().unwrap_u8() == 1 {
                Ok(ct_option.unwrap())
            } else {
                Err(Curve25519Error::PodConversion)
            }
            //gaokanxu 2024.08.17 end
        }
    }

    impl From<Scalar> for PodScalar {
        fn from(scalar: Scalar) -> Self {
            Self(scalar.to_bytes())
        }
    }

    impl TryFrom<PodScalar> for Scalar {
        type Error = Curve25519Error;

        fn try_from(pod: PodScalar) -> Result<Self, Self::Error> {
            //Scalar::from_canonical_bytes(pod.0).ok_or(Curve25519Error::PodConversion)
            //gaokanxu 2024.08.17 begin
            let ct_option = Scalar::from_canonical_bytes(pod.0);
            if ct_option.is_some().unwrap_u8() == 1 {
                Ok(ct_option.unwrap())
            } else {
                Err(Curve25519Error::PodConversion)
            }
            //gaokanxu 2024.08.17 end
        }
    }
}
