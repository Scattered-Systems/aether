/*
   Appellation: context
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

pub type ContextError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Context {
    pub configuration: crate::Configuration,
}

impl Context {
    pub fn constructor(configuration: crate::Configuration) -> Result<Self, ContextError> {
        Ok(Self { configuration })
    }

    pub fn new(configuration: crate::Configuration) -> Self {
        Self::constructor(configuration).ok().unwrap()
    }
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Context(configuration={})", self.configuration)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}
