use serde::{Serialize, Deserialize};

use de::Deserializer;
use error::Error;
use ser::Serializer;
use token::Token;

use std::fmt::Debug;

/// Runs both `assert_ser_tokens` and `assert_de_tokens`.
pub fn assert_tokens<T>(value: &T, tokens: &[Token<'static>])
    where T: Serialize + Deserialize + PartialEq + Debug
{
    assert_ser_tokens(value, tokens);
    assert_de_tokens(value, tokens);
}

/// Asserts that `value` serializes to the given `tokens`.
pub fn assert_ser_tokens<T>(value: &T, tokens: &[Token])
    where T: Serialize
{
    let mut ser = Serializer::new(tokens.iter());
    assert_eq!(Serialize::serialize(value, &mut ser), Ok(()));
    assert_eq!(ser.next_token(), None);
}

/// Asserts that `value` serializes to the given `tokens`, and then yields `error`.
pub fn assert_ser_tokens_error<T>(value: &T, tokens: &[Token], error: Error)
    where T: Serialize + PartialEq + Debug
{
    let mut ser = Serializer::new(tokens.iter());
    let v: Result<(), Error> = Serialize::serialize(value, &mut ser);
    assert_eq!(v.as_ref(), Err(&error));
    assert_eq!(ser.next_token(), None);
}

/// Asserts that the given `tokens` deserialize into `value`.
pub fn assert_de_tokens<T>(value: &T, tokens: &[Token<'static>])
    where T: Deserialize + PartialEq + Debug
{
    let mut de = Deserializer::new(tokens.to_vec().into_iter());
    let v: Result<T, Error> = Deserialize::deserialize(&mut de);
    assert_eq!(v.as_ref(), Ok(value));
    assert_eq!(de.next_token(), None);
}

/// Asserts that the given `tokens` yield `error` when deserializing.
pub fn assert_de_tokens_error<T>(tokens: &[Token<'static>], error: Error)
    where T: Deserialize + PartialEq + Debug
{
    let mut de = Deserializer::new(tokens.to_vec().into_iter());
    let v: Result<T, Error> = Deserialize::deserialize(&mut de);
    assert_eq!(v, Err(error));
    // There may be one token left if a peek caused the error
    de.next_token();
    assert_eq!(de.next_token(), None);
}
