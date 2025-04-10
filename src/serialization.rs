use std::io::{self};
use std::num::ParseFloatError;
use std::{
    fs::File,
    io::{BufWriter, Write},
};

use bevy::prelude::*;

#[derive(Debug)]
pub struct WhereWasIParseError {
    pub message: String,
}

impl WhereWasIParseError {
    pub fn expected_line() -> Self {
        Self {
            message: "Expected line to be there, but it wasn't there".into(),
        }
    }
}

impl From<io::Error> for WhereWasIParseError {
    fn from(value: io::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}
impl From<ParseFloatError> for WhereWasIParseError {
    fn from(value: ParseFloatError) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

/// Serializes a [`Transform`] and writes it to the BufWriter
///
/// Note: we could use serde using the `serialization` feature of Bevy. However, that requires
/// external depedencies which we can avoid by doing the (de)serialization manually.
pub fn serialize_transform(
    writer: &mut BufWriter<File>,
    transform: &Transform,
) -> Result<(), io::Error> {
    writer.write_all(b"v0\n\n")?;

    writer.write_all(b"translation:\n")?;
    writer.write_all(transform.translation.x.to_string().as_bytes())?;
    writer.write_all(b"\n")?;
    writer.write_all(transform.translation.y.to_string().as_bytes())?;
    writer.write_all(b"\n")?;
    writer.write_all(transform.translation.z.to_string().as_bytes())?;
    writer.write_all(b"\n\n")?;

    writer.write_all(b"rotation:\n")?;
    writer.write_all(transform.rotation.x.to_string().as_bytes())?;
    writer.write_all(b"\n")?;
    writer.write_all(transform.rotation.y.to_string().as_bytes())?;
    writer.write_all(b"\n")?;
    writer.write_all(transform.rotation.z.to_string().as_bytes())?;
    writer.write_all(b"\n")?;
    writer.write_all(transform.rotation.w.to_string().as_bytes())?;
    writer.write_all(b"\n\n")?;

    writer.write_all(b"scale:\n")?;
    writer.write_all(transform.scale.x.to_string().as_bytes())?;
    writer.write_all(b"\n")?;
    writer.write_all(transform.scale.y.to_string().as_bytes())?;
    writer.write_all(b"\n")?;
    writer.write_all(transform.scale.z.to_string().as_bytes())?;
    writer.write_all(b"\n")?;

    Ok(())
}

fn next_float(lines: &mut io::Lines<io::BufReader<File>>) -> Result<f32, WhereWasIParseError> {
    Ok(lines
        .next()
        .ok_or(WhereWasIParseError::expected_line())??
        .parse::<f32>()?)
}

/// Deserializes lines into a [`Transform`]
///
/// Note: we could use serde using the `serialization` feature of Bevy. However, that requires
/// external depedencies which we can avoid by doing the (de)serialization manually.
pub fn deserialize_transform(
    mut lines: io::Lines<io::BufReader<File>>,
) -> Result<Transform, WhereWasIParseError> {
    let version = lines.next().ok_or(WhereWasIParseError::expected_line())??;
    if version != "v0" {
        return Err(WhereWasIParseError {
            message: format!("Wrong version: {version}"),
        });
    }

    lines.next().ok_or(WhereWasIParseError::expected_line())??;
    lines.next().ok_or(WhereWasIParseError::expected_line())??;

    let translation = Vec3::new(
        next_float(&mut lines)?,
        next_float(&mut lines)?,
        next_float(&mut lines)?,
    );

    lines.next().ok_or(WhereWasIParseError::expected_line())??;
    lines.next().ok_or(WhereWasIParseError::expected_line())??;

    let rotation = Vec4::new(
        next_float(&mut lines)?,
        next_float(&mut lines)?,
        next_float(&mut lines)?,
        next_float(&mut lines)?,
    );

    lines.next().ok_or(WhereWasIParseError::expected_line())??;
    lines.next().ok_or(WhereWasIParseError::expected_line())??;

    let scale = Vec3::new(
        next_float(&mut lines)?,
        next_float(&mut lines)?,
        next_float(&mut lines)?,
    );

    Ok(Transform {
        translation,
        rotation: Quat::from_vec4(rotation),
        scale,
    })
}
