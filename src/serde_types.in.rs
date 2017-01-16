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

impl MultipleInvalid {
    pub fn invalid(tag: String, msg: String, args: Vec<String>) -> MultipleInvalid {
        MultipleInvalid {
            tag: tag,
            invalids: vec![Invalid {
                               msg: msg,
                               args: args,
                           }],
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Error(pub Vec<Invalid>);

#[derive(Debug, Clone, Serialize)]
pub struct MultipleError(pub Vec<MultipleInvalid>);
