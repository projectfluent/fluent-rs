// use crate::ast;
// use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
// use std::fmt;
// use std::marker::PhantomData;

// struct ContentVisitor<S>(PhantomData<S>);

// impl<'de, S> Visitor<'de> for ContentVisitor<S>
// where
//     S: de::Deserialize<'de>,
// {
//     type Value = Vec<String>;

//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         formatter.write_str("an integer between -2^31 and 2^31")
//     }

//     fn visit_seq<R>(self, visitor: R) -> Result<Self::Value, R::Error>
//     where
//         R: de::SeqAccess<'de>,
//     {
//         Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))
//     }
// }

// pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     deserializer.deserialize_any(ContentVisitor(PhantomData))
// }
