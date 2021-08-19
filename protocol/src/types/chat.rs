use protocol_derive::Json;

#[derive(Json, Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum Chat {
    Primitive(Prim),
    Array(Vec<Chat>),
    Obj(ChatObj),
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Prim {
    Float(f64),
    Str(String),
    Bool(bool),
}

#[serde_with::skip_serializing_none]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatObj {
    pub text: Option<String>,
    pub translate: Option<String>,
    pub score: Option<Score>,
    pub keybind: Option<String>,
    pub selector: Option<String>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
    pub color: Option<String>,
    pub insertion: Option<String>,
    pub click_event: Option<ActionPair>,
    pub hover_event: Option<ActionPair>,
    pub extra: Option<Box<Chat>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActionPair {
    pub action: String,
    pub value: Prim,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Score {
    pub name: String,
    pub objective: String,
    pub value: Option<String>,
}
