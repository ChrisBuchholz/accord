#[derive(Debug, Clone, Serialize)]
pub struct Invalid {
    pub msg: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MultipleInvalid {
    pub tag: String,
    pub invalids: Vec<Invalid>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Error(pub Vec<Invalid>);

#[derive(Debug, Clone, Serialize)]
pub struct MultipleError(pub Vec<MultipleInvalid>);
