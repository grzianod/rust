struct ValueStruct {
    _type: i32,
    val: f32,
    timestamp: i64,
}

struct MValueStruct {
    _type: i32,
    val: [ f32; 10 ],
    timestamp: i64,
}

struct MessageStruct {
    _type: i32,
    message: [u8; 21],
}

union ValueUnion {
    val: ValueStruct,
    mvals: MValueStruct,
    messages: MessageStruct,
}

struct ExportData {
    type_: i32,
    value_union: ValueUnion,
}


fn main() {

}
