use super::{TreeItem, GIT_BLOB_DELIMITER, GIT_KIND_DELIMITER};
use bytes::{BufMut, Bytes, BytesMut};

pub struct Writer<'a> {
    pub items: &'a [TreeItem],
}

impl<'a> Writer<'a> {
    pub fn to_bytes(&self) -> Bytes {
        let mut buffer = BytesMut::new();
        for item in self.items {
            item.write(&mut buffer);
        }
        buffer.freeze()
    }
}

impl TreeItem {
    fn write(&self, buffer: &mut BytesMut) {
        buffer.put(self.mode.as_bytes());
        buffer.put_u8(GIT_KIND_DELIMITER);
        buffer.put(self.name.as_bytes());
        buffer.put_u8(GIT_BLOB_DELIMITER);
        buffer.put(self.sha.as_bytes());
    }
}
