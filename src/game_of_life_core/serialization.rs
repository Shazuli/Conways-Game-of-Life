use super::{ChunkCellData, Chunk, Field as GOLField};
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
        s.serialize_field("data",&self.get_data_u64())?;
        s.end()
    }
}

impl Serialize for GOLField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
    {
        let mut s = serializer.serialize_struct("Field", 2)?;
        s.serialize_field("generation",&self.generation)?;
        s.serialize_field("current",&self.get_current())?;
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
                        formatter.write_str("`x`, `y` or `data`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: Error,
                    {
                        match value {
                            "x" => Ok(Field::X),
                            "y" => Ok(Field::Y),
                            "data" => Ok(Field::DATA),
                            _ => Err(Error::unknown_field(value, FIELDS)),
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
                let data = seq.next_element()?
                    .ok_or_else(|| Error::invalid_length(2, &self))?;
                Ok(Chunk { x, y, data: ChunkCellData { u64: data } })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Chunk, V::Error>
            where
                V: MapAccess<'de>
            {
                let mut x = None;
                let mut y = None;
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
                                return Err(Error::duplicate_field("data"));
                            }
                            data = Some(map.next_value()?);
                        }
                    }
                }
                let x = x.ok_or_else(|| Error::missing_field("x"))?;
                let y = y.ok_or_else(|| Error::missing_field("y"))?;
                let data = data.ok_or_else(|| Error::missing_field("data"))?;
                Ok(Chunk { x, y, data: ChunkCellData { u64: data } })
            }
        }

        const FIELDS: &[&str] = &["x", "y", "data"];
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
                        formatter.write_str("`generation` or `current`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: Error,
                    {
                        match value {
                            "generation" => Ok(Field::GENERATION),
                            "current" => Ok(Field::CURRENT),
                            _ => Err(Error::unknown_field(value, FIELDS)),
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

                let mut chunks: [Vec<Chunk>; 2] = [Vec::new(), Vec::new()];
                chunks[(generation as u8 & 1) as usize] = current.to_owned();

                Ok(GOLField { generation, chunks })
            }

            fn visit_map<V>(self, mut map: V) -> Result<GOLField, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut generation = None;
                let mut current: Option<Vec<Chunk>> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::GENERATION => {
                            if generation.is_some() {
                                return Err(Error::duplicate_field("generation"));
                            }
                            generation = Some(map.next_value()?);
                        },
                        Field::CURRENT => {
                            if current.is_some() {
                                return Err(Error::duplicate_field("current"));
                            }
                            current = Some(map.next_value()?);
                        }
                    }
                }
                let generation = generation.ok_or_else(|| Error::missing_field("generation"))?;
                let current = current.ok_or_else(|| Error::missing_field("current"))?;
                
                let mut chunks: [Vec<Chunk>; 2] = [Vec::new(), Vec::new()];
                chunks[(generation as u8 & 1) as usize] = current.to_owned();

                Ok(GOLField { generation, chunks })
            }
        }

        const FIELDS: &[&str] = &["generation", "current"];
        deserializer.deserialize_struct("Field", FIELDS, GOLFieldVisitor)
    }
}

    /*/// Writes Field struct to a file on the system.
    pub fn serialize(&self, path: OsString) -> Result<()>
    {
        let mut file = File::create(path)?;


        {
            // Change this when this method changes.
            let versioning: [u8; 1] = [0];

            file.write(&versioning)?;
        }

        
        file.write_all(&self.generation.to_be_bytes())?;

        {
            //let mut buffer: [u8; 16] = [0; 16];

            let current = self.get_current();

            // Write how long the list is.
            //file.write_all(&current.len().to_be_bytes())?;

            for c in current {

                file.write_all(&c.x.to_be_bytes())?;
                file.write_all(&c.y.to_be_bytes())?;
                unsafe { file.write_all(&c.data.u64.to_be_bytes())?; }
            }
        }

        Ok(())
    }

    /// Reads a file on the system to load a Field struct.
    pub fn deserialize(path: OsString) -> Result<Self>
    {
        let mut file = File::open(path)?;


        {
            // Check if it's the wrong version format.
            let mut buffer: [u8; 1] = [0; 1];

            file.read(&mut buffer)?;

            let versioning = buffer[0];

            if versioning & 0x1f != 0 {
                return Err(Error::other("Wrong format version"));
            }
        }

        let mut f = Field::new();

        {
            let mut buffer: [u8; 4] = [0; 4];

            file.read(&mut buffer)?;

            f.generation = u32::from_be_bytes(buffer);
        }

        {
            let mut buffer: Vec<u8> = Vec::new();

            // Read all bytes at once.
            let length = file.read_to_end(&mut buffer)? / 16;

            let buffer = buffer;

            let mut current: Vec<Chunk> = Vec::with_capacity(length);

            for i in 0..length {
                let chunk = Chunk {
                    x: i32::from_be_bytes(buffer[i*16]),
                    y: i32::from_be_bytes(buffer[i*16+4]),
                    data: ChunkCellData { u64: u64::from_be_bytes(buffer[i*16+8]) }
                };

                current[i] = chunk;
            }

            *f.get_mut_current() = current;

        }


        Ok(f)
    }*/