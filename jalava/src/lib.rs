mod elm;
mod elm_form;

pub use elm::Elm;
pub use elm_form::{ElmForm, ElmFormField};

#[macro_export]
/// Writes an Elm module named Bindings to the target.
macro_rules! export {
    ($target: expr, $($type: path),*) => {
        {
            fn _export(target: &mut impl std::io::Write) -> Result<(), std::io::Error> {
                use std::io::Write;
                use jalava::Elm;
                writeln!(target,
"-- GENERATED BY JALAVA


module Bindings exposing (..)

import Json.Decode
import Json.Encode
import Dict exposing (Dict)

")?;
                $(
                    if let Some(elm_definition) = <$type>::elm_definition() {
                        writeln!(target, "{}\n", elm_definition)?;
                    }
                    if let Some(encoder_definition) = <$type>::encoder_definition() {
                        writeln!(target, "{}\n", encoder_definition)?;
                    }
                    if let Some(decoder_definition) = <$type>::decoder_definition() {
                        writeln!(target, "{}\n", decoder_definition)?;
                    }

                )*
                Ok(())
            }
            _export($target)
        }
    };
}
