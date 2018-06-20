use avro_rs::Codec;

/// Replicates avro::Codec enum so we can use a C-compatible representation
#[repr(C)]
pub enum AvroCodec {
    Null,
    Deflate,
    #[cfg(feature = "snappy")]
    Snappy,
}

impl AvroCodec {
    pub fn to_codec(&self) -> Codec {
        match *self {
            AvroCodec::Null => Codec::Null,
            AvroCodec::Deflate => Codec::Deflate,
            #[cfg(feature = "snappy")]
            AvroCodec::Snappy => Codec::Snappy,
        }
    }
}
