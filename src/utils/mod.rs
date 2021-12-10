pub fn random_u8() -> Vec<u8> {
    rbatis::plugin::object_id::ObjectId::new().bytes().to_vec()
}

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl IsEmpty for Option<String> {
    fn is_empty(&self) -> bool {
        return match self {
            Some(s) => {
                s.is_empty()
            }
            _ => {
                true
            }
        }
    }
}