//! Contains methods to serialize and deserialize a struct Field and struct Chunk using Serde.

use super::{ChunkCellData8x8, Chunk, Field as GOLField, get_current_gen_index};
use alloc::{borrow::ToOwned, vec::Vec};
use serde::{de::{Deserialize, Deserializer, Error, MapAccess, SeqAccess, Visitor}, ser::SerializeStruct, Serialize, Serializer};
use core::fmt;


impl Serialize for Chunk {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
    {
        let mut s = serializer.serialize_struct("Chunk", 3)?;
        s.serialize_field("x",&self.x)?;
        s.serialize_field("y",&self.y)?;
        s.serialize_field("d",&{ let data: u64 = self.data.into(); data })?;
        s.end()
    }
}

impl Serialize for GOLField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
    {
        let mut s = serializer.serialize_struct("Field", 2)?;
        s.serialize_field("gen",&self.generation)?;
        s.serialize_field("crt",&self.get_current())?;
        s.end()        
    }
}


impl<'de> Deserialize<'de> for Chunk {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        enum Field { X, Y, DATA }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`x`, `y` or `d`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: Error,
                    {
                        match value {
                            "x" => Ok(Field::X),
                            "y" => Ok(Field::Y),
                            "d" => Ok(Field::DATA),
                            _ => Err(Error::unknown_field(value, FIELDS))
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ChunkVisitor;

        impl<'de> Visitor<'de> for ChunkVisitor {
            type Value = Chunk;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
            {
                formatter.write_str("struct Chunk")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Chunk, V::Error>
            where
                V: SeqAccess<'de>
            {
                let x = seq.next_element()?
                    .ok_or_else(|| Error::invalid_length(0, &self))?;
                let y = seq.next_element()?
                    .ok_or_else(|| Error::invalid_length(1, &self))?;
                let data: u64 = seq.next_element()?
                    .ok_or_else(|| Error::invalid_length(2, &self))?;
                Ok(Chunk { x, y, data: ChunkCellData8x8::from(data) })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Chunk, V::Error>
            where
                V: MapAccess<'de>
            {
                let mut x: Option<i32> = None;
                let mut y: Option<i32> = None;
                let mut data: Option<u64> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::X => {
                            if x.is_some() {
                                return Err(Error::duplicate_field("x"));
                            }
                            x = Some(map.next_value()?);
                        },
                        Field::Y => {
                            if y.is_some() {
                                return Err(Error::duplicate_field("y"));
                            }
                            y = Some(map.next_value()?);
                        },
                        Field::DATA => {
                            if data.is_some() {
                                return Err(Error::duplicate_field("d"));
                            }
                            data = Some(map.next_value()?);
                        }
                    }
                }
                let x = x.ok_or_else(|| Error::missing_field("x"))?;
                let y = y.ok_or_else(|| Error::missing_field("y"))?;
                let data = data.ok_or_else(|| Error::missing_field("d"))?;
                Ok(Chunk { x, y, data: ChunkCellData8x8::from(data) })
            }
        }

        const FIELDS: &[&str] = &["x", "y", "d"];
        deserializer.deserialize_struct("Chunk", FIELDS, ChunkVisitor)
    }
}


impl<'de> Deserialize<'de> for GOLField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        enum Field { GENERATION, CURRENT }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
                    {
                        formatter.write_str("`gen` or `crt`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: Error,
                    {
                        match value {
                            "gen" => Ok(Field::GENERATION),
                            "crt" => Ok(Field::CURRENT),
                            _ => Err(Error::unknown_field(value, FIELDS))
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct GOLFieldVisitor;

        impl<'de> Visitor<'de> for GOLFieldVisitor {
            type Value = GOLField;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
            {
                formatter.write_str("struct Field")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<GOLField, V::Error>
            where
                V: SeqAccess<'de>
            {
                let generation = seq.next_element()?
                    .ok_or_else(|| Error::invalid_length(0, &self))?;
                let current: Vec<Chunk> = seq.next_element()?
                    .ok_or_else(|| Error::invalid_length(1, &self))?;

                let chunks = {
                    let mut chunks: [Vec<Chunk>; 2] = [Vec::with_capacity(current.len()), Vec::with_capacity(current.len())];
                    chunks[get_current_gen_index(generation)] = current.to_owned();
                    chunks
                };

                Ok(GOLField { generation, chunks })
            }

            fn visit_map<V>(self, mut map: V) -> Result<GOLField, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut generation: Option<u64> = None;
                let mut current: Option<Vec<Chunk>> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::GENERATION => {
                            if generation.is_some() {
                                return Err(Error::duplicate_field("gen"));
                            }
                            generation = Some(map.next_value()?);
                        },
                        Field::CURRENT => {
                            if current.is_some() {
                                return Err(Error::duplicate_field("crt"));
                            }
                            current = Some(map.next_value()?);
                        }
                    }
                }
                let generation = generation.ok_or_else(|| Error::missing_field("gen"))?;
                let current = current.ok_or_else(|| Error::missing_field("crt"))?;
                
                let chunks = {
                    let mut chunks: [Vec<Chunk>; 2] = [Vec::with_capacity(current.len()), Vec::with_capacity(current.len())];
                    chunks[get_current_gen_index(generation)] = current.to_owned();
                    chunks
                };

                Ok(GOLField { generation, chunks })
            }
        }

        const FIELDS: &[&str] = &["gen", "crt"];
        deserializer.deserialize_struct("Field", FIELDS, GOLFieldVisitor)
    }
}