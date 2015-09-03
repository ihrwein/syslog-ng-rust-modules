pub use self::message::MessageAction;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActionType {
    Message(self::message::MessageAction)
}

mod deser {
    use serde;
    use super::ActionType;

    impl serde::de::Deserialize for ActionType {
    fn deserialize<D>(deserializer: &mut D) -> Result<ActionType, D::Error>
                      where D: serde::de::Deserializer {
        enum Field {
            Message,
        }

        impl serde::de::Deserialize for Field {
            #[inline]
            fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error>
                where D: serde::de::Deserializer {
                    struct FieldVisitor;

                    impl serde::de::Visitor for FieldVisitor {
                        type Value = Field;

                        fn visit_str<E>(&mut self, value: &str) -> Result<Field, E> where E: serde::de::Error {
                            match value {
                                "message" => Ok(Field::Message),
                                _ => Err(serde::de::Error::unknown_field(value)),
                            }
                        }
                    }

                    deserializer.visit(FieldVisitor)
                }
            }

            struct Visitor;

            impl serde::de::EnumVisitor for Visitor {
                type Value = ActionType;

                fn visit<V>(&mut self, mut visitor: V) -> Result<ActionType, V::Error>
                    where V: serde::de::VariantVisitor
                {
                    match try!(visitor.visit_variant()) {
                        Field::Message => {
                            let value = try!(visitor.visit_newtype());
                            Ok(ActionType::Message(value))
                        }
                    }
                }
            }

            const VARIANTS: &'static [&'static str] = &["message"];

            deserializer.visit_enum("ActionType", VARIANTS, Visitor)
        }
    }

    #[cfg(test)]
    mod test {
        use serde_json::from_str;
        use config::action::ActionType;

        #[test]
        fn test_given_action_when_it_is_deserialized_then_we_get_the_right_result() {
            let text = r#"
                {
                    "message": null
                }
            "#;

            let result = from_str::<ActionType>(text);
            println!("{:?}", &result);
            let action = result.ok().expect("Failed to deserialize a valid ActionType");
            match action {
                ActionType::Message(_) => {}
            }
        }
    }
}

pub mod message {
    use super::ActionType;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct MessageAction;

    impl MessageAction {
        pub fn new() -> MessageAction {
            MessageAction
        }
    }

    impl From<MessageAction> for super::ActionType {
        fn from(action: MessageAction) -> super::ActionType {
            super::ActionType::Message(action)
        }
    }

    mod deser {

    use super::MessageAction;
    use serde;
    use serde::de::Deserialize;

    impl serde::Deserialize for MessageAction {
        fn deserialize<D>(deserializer: &mut D) -> Result<MessageAction, D::Error>
            where D: serde::de::Deserializer
        {
            deserializer.visit_unit_struct("MessageAction", MessageActionVisitor)
        }
    }

    struct MessageActionVisitor;

    impl serde::de::Visitor for MessageActionVisitor {
        type Value = MessageAction;
        fn visit_unit<E>(&mut self) -> Result<Self::Value, E>
            where E: serde::de::Error {
            Ok(MessageAction)
        }
    }
    }
}
