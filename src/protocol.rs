#[allow(non_camel_case_types)]
pub enum DataType {
    /// (0x07) DBR_STS structure for string type.
    DBR_STS_STRING,

    /// (0x08) DBR_STS structure for UINT16 type. May be referred to as DBR_STS_INT.
    DBR_STS_SHORT,

    /// (0x09) DBR_STS structure for FLOAT type.
    DBR_STS_FLOAT,

    /// (0x0A) DBR_STS structure for ENUM type.
    DBR_STS_ENUM,

    /// (0x0B) DBR_STS structure for CHAR type.
    DBR_STS_CHAR,

    /// (0x0C) DBR_STS structure for LONG type.
    DBR_STS_LONG,

    /// (0x0D) DBR_STS structure for DOUBLE type.
    DBR_STS_DOUBLE,

    /// (0x0E) DBR_TIME structure for string type.
    DBR_TIME_STRING,

    /// (0x0F) DBR_TIME structure for UINT16 type. May be referred to as DBR_TIME_INT.
    DBR_TIME_SHORT,

    /// (0x10) DBR_TIME structure for FLOAT type.
    DBR_TIME_FLOAT,
    
    /// (0x11) DBR_TIME structure for ENUM type.
    DBR_TIME_ENUM,

    /// (0x12) DBR_TIME structure for CHAR type.
    DBR_TIME_CHAR,

    /// (0x13) DBR_TIME structure for LONG type.
    DBR_TIME_LONG,

    /// (0x14) DBR_TIME structure for DOUBLE type.
    DBR_TIME_DOUBLE,

    /// (0x15) DBR_GR structure for string type.
    DBR_GR_STRING,

    /// (0x16) DBR_GR structure for short type.
    DBR_GR_SHORT,

    /// (0x16) DBR_GR structure for int type.
    DBR_GR_INT,

    /// (0x17) DBR_GR structure for float type.
    DBR_GR_FLOAT,

    /// (0x18) DBR_GR structure for ENUM type.
    DBR_GR_ENUM,

    /// (0x19) DBR_GR structure for char type (UINT8 representation).
    DBR_GR_CHAR,

    /// (0x1A) DBR_GR structure for long type (INT32 representation).
    DBR_GR_LONG,

    /// (0x1B) DBR_GR structure for double type.
    DBR_GR_DOUBLE,

    /// (0x1C) DBR_CTRL structure for string type.
    DBR_CTRL_STRING,

    /// (0x1D) DBR_CTRL structure for short type.
    DBR_CTRL_SHORT,

    /// (0x1D) DBR_CTRL structure for INT16 type.
    DBR_CTRL_INT,

    /// (0x1E) DBR_CTRL structure for float type.
    DBR_CTRL_FLOAT,

    /// (0x1F) DBR_CTRL structure for ENUM type.
    DBR_CTRL_ENUM,

    /// (0x20) DBR_CTRL structure for char type (UINT8 representation).
    DBR_CTRL_CHAR,

    /// (0x21) DBR_CTRL structure for INT32 type.
    DBR_CTRL_LONG,

    /// (0x22) DBR_CTRL structure for DOUBLE type.
    DBR_CTRL_DOUBLE,


}

pub struct MessageHeader {
    /// Identifier of the command this message requests. The meaning of other header fields and the payload depends on the command.
    command: u16,

    /// Size of the payload (in bytes). Must not exceed 0x4000. Value of 0xFFFF indicates extended message.
    payload_size: u16,

    /// Identifier of the data type carried in the payload.
    data_type: u16,

    /// Number of elements in the payload.
    data_count: u16,

    /// Command-dependent parameter
    parameter_1: u32,
    /// Command-dependent parameter
    parameter_2: u32,
}

pub struct ExtendedMessageHeader {
    /// Identifier of the command this message requests. The meaning of other header fields and the payload depends on the command.
    command: u16,

    // Extended Message Marker: 0xFFFF,
    
    /// Identifier of the data type carried in the payload.
    data_type: u16,
    
    // Extended Message Marker: 0x0000,
    
    /// Command-dependent parameter
    parameter_1: u32,

    /// Command-dependent parameter
    parameter_2: u32,

    /// Size of the payload (in bytes). Must not exceed 0x4000. Value of 0xFFFF indicates extended message.
    payload_size: u32,

    /// Number of elements in the payload.
    data_count: u32,
}
