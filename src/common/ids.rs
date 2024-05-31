use serde::{Deserialize, Serialize};

macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
        pub struct $name(pub u64);

        impl $name {
            pub fn new(raw: u64) -> Self {
                Self(raw)
            }
        }
    };
}

define_id!(MapId);
define_id!(MarkerId);
