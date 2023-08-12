use super::{SubcriberName, SubscriberEmail};

#[derive(Debug)]
pub struct NewSubcriber {
    pub email: SubscriberEmail,
    pub name: SubcriberName,
}
