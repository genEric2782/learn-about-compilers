use serde::Deserialize;

#[derive(serde::Serialize, Deserialize, Clone, Default, Debug)]

pub struct TACvar {
    #[serde(rename = "$type")]
    tac_type: String,
    #[serde(rename = "tacTempValue")]
    tac_temp_value: String,
    #[serde(default)]
    value: Option<String>,
    #[serde(default)]
    op: Option<String>, // ? used to mark as optional
    #[serde(default)]
    arg1: Option<String>,
    #[serde(default)]
    arg2: Option<String>,
}

#[derive(serde::Serialize, Deserialize, Clone, Default, Debug)]
pub struct TACInstruction {
    opcode: String, // Zig doesnt have strings it instead has slices of bytes
    tacvar: TACvar,
}