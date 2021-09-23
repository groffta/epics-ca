#[allow(non_camel_case_types)]
pub enum Command {
    /// UDP/TCP (0x00) Exchanges client and server protocol versions and desired circuit priority. MUST be the first message sent, by both client and server, when a new TCP (Virtual Circuit) connection is established. It is also sent as the first message in UDP search messages.
    CA_PROTO_VERSION,

    /// UDP/TCP (0x06) Searches for a given channel name.
    CA_PROTO_SEARCH,

    /// UDP (0x0E) Indicates that a channel with requested name does not exist. Sent in response to CA_PROTO_SEARCH, but only when its DO_REPLY flag was set.
    CA_PROTO_NOT_FOUND,

    /// TCP (0x17) Connection verify used by CA_V43.
    CA_PROTO_ECHO,

    /// UDP (0x0D) Beacon sent by a server when it becomes available. Beacons are also sent out periodically to announce the server is still alive. Another function of beacons is to allow detection of changes in network topology.
    CA_PROTO_RSRV_IS_UP,

    /// UDP (0x11) Confirms successful client registration with repeater.
    CA_REPEATER_CONFIRM,

    /// TCP (0x18) Requests registration with the repeater. Repeater will confirm successful registration using CA_REPEATER_CONFIRM.
    CA_REPEATER_REGISTER,

    /// TCP (0x01) Creates a subscription on a channel, allowing the client to be notified of changes in value. A request will produce at least one response.
    CA_PROTO_EVENT_ADD,

    /// TCP (0x02) Clears event subscription. This message will stop event updates for specified channel.
    CA_PROTO_EVENT_CANCEL,

    /// TCP (0x03) Read value of a channel. DEPRECATED since v3.13
    CA_PROTO_READ,

    /// TCP (0x04) Writes new channel value.
    CA_PROTO_WRITE,

    /// TCP (0x05) Obsolete.
    CA_PROTO_SNAPSHOT,

    /// TCP (0x07) Obsolete.
    CA_PROTO_BUILD,

    /// TCP (0x08) Disables a server from sending any subscription updates over this virtual circuit.
    CA_PROTO_EVENTS_OFF,

    /// TCP (0x09) Enables the server to resume sending subscription updates for this virtual circuit.
    CA_PROTO_EVENTS_ON,

    /// TCP (0x0A) DEPRECATED since v3.13
    CA_PROTO_READ_SYNC,

    /// TCP (0x0C) Clears a channel. This command will cause server to release the associated channel resources and no longer accept any requests for this SID/CID.
    CA_PROTO_CLEAR_CHANNEL,

    /// TCP (0x0F) Read value of a channel.
    CA_PROTO_READ_NOTIFY,

    /// TCP (0x10) Obsolete,
    CA_PROTO_READ_BUILD,

    /// TCP (0x12) Requests creation of channel. Server will allocate required resources and return initialized SID.
    CA_PROTO_CREATE_CHAN,

    /// TCP (0x13) Writes new channel value.
    CA_PROTO_WRITE_NOTIFY,

    /// TCP (0x14) Sends local username to virtual circuit peer. This name identifies the user and affects access rights.
    CA_PROTO_CLIENT_NAME,

    /// TCP (0x15) Sends local host name to virtual circuit peer. This name will affect access rights.
    CA_PROTO_HOST_NAME,

    /// TCP (0x16) Notifies of access rights for a channel. This value is determined based on host and client name and may change during runtime. Client cannot change access rights nor can it explicitly query its value, so last received value must be stored.
    CA_PROTO_ACCESS_RIGHTS,

    /// TCP (0x19) Obsolete.
    CA_PROTO_SIGNAL,

    /// TCP (0x1A) Reports that channel creation failed. This response is sent to when channel creation in CA_PROTO_CREATE_CHAN fails.
    CA_PROTO_CREATE_CH_FAIL,

    /// TCP (0x1B) Notifies the client that server has disconnected the channel. This may be since the channel has been destroyed on server.
    CA_PROTO_SERVER_DISCONN,
}

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
