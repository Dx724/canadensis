use crate::uavcan::node::port::service_id_list::ServiceIdList;
use crate::uavcan::node::port::subject_id_list::SubjectIdList;
use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, Message, ReadCursor, Serialize, WriteCursor,
};

/// uavcan.node.port.List version 0.1
#[derive(Debug, Clone, Default)]
pub struct List {
    pub publishers: SubjectIdList,
    pub subscribers: SubjectIdList,
    pub clients: ServiceIdList,
    pub servers: ServiceIdList,
}

impl List {
    pub const SUBJECT: canadensis_core::SubjectId =
        canadensis_core::SubjectId::from_truncating(7510);
    pub const MAX_PUBLICATION_PERIOD: u8 = 10;
}

impl DataType for List {
    // Sealed type
    const EXTENT_BYTES: Option<u32> = None;
}

impl Message for List {}

impl Serialize for List {
    fn size_bits(&self) -> usize {
        // Add 32 bits for the delimiter header because each field is delimited
        32 + self.publishers.size_bits()
            + 32
            + self.subscribers.size_bits()
            + 32
            + self.clients.size_bits()
            + 32
            + self.servers.size_bits()
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_composite(&self.publishers);
        cursor.write_composite(&self.subscribers);
        cursor.write_composite(&self.clients);
        cursor.write_composite(&self.servers);
    }
}

impl Deserialize for List {
    fn in_bit_length_set(bit_length: usize) -> bool {
        // TODO: This may be too permissive
        bit_length % 8 == 0 && {
            let bytes = bit_length / 8;
            (16..=8466).contains(&bytes)
        }
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        self.publishers = cursor.read_composite()?;
        self.subscribers = cursor.read_composite()?;
        self.clients = cursor.read_composite()?;
        self.servers = cursor.read_composite()?;
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut value = List::default();
        value.deserialize_in_place(cursor)?;
        Ok(value)
    }
}
