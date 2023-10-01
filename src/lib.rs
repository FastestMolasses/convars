extern crate serde;
extern crate serde_json;

use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum ConVarError {
    UnknownConVar,
    ParseError(String),
}

impl Display for ConVarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConVarError::UnknownConVar => write!(f, "unknown convar"),
            ConVarError::ParseError(msg) => write!(f, "parse error: {}", msg),
        }
    }
}

pub trait ConVarType: Sized {
    fn from_convar_str(value: &str) -> Result<Self, ConVarError>;
    fn to_convar_str(&self) -> String;
}

impl ConVarType for i32 {
    fn from_convar_str(value: &str) -> Result<Self, ConVarError> {
        value
            .parse::<i32>()
            .map_err(|e| ConVarError::ParseError(e.to_string()))
    }

    fn to_convar_str(&self) -> String {
        self.to_string()
    }
}

impl ConVarType for f32 {
    fn from_convar_str(value: &str) -> Result<Self, ConVarError> {
        value
            .parse::<f32>()
            .map_err(|e| ConVarError::ParseError(e.to_string()))
    }

    fn to_convar_str(&self) -> String {
        self.to_string()
    }
}

impl ConVarType for bool {
    fn from_convar_str(value: &str) -> Result<Self, ConVarError> {
        match value.to_lowercase().as_str() {
            "true" | "t" | "1" | "y" | "yes" | "on" => Ok(true),
            "false" | "f" | "0" | "n" | "no" | "off" => Ok(false),
            _ => Err(ConVarError::ParseError(format!(
                "invalid boolean representation: {}",
                value
            ))),
        }
    }

    fn to_convar_str(&self) -> String {
        self.to_string()
    }
}

#[macro_export]
macro_rules! convars {
    ($($name:ident: $type:ty = $value:expr),* $(,)? ) => {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct ConVars {
            $(pub $name: $type,)*
        }

        impl ConVars {
            pub fn to_json(&self) -> Result<String, serde_json::Error> {
                serde_json::to_string(self)
            }

            pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
                serde_json::from_str(json_str)
            }
        }

        impl Default for ConVars {
            fn default() -> Self {
                Self {
                    $($name: $value,)*
                }
            }
        }

        impl ConVars {
            pub fn set_str(&mut self, name: &str, value: &str) -> Result<(), ConVarError> {
                match name {
                    $(
                        stringify!($name) => {
                            self.$name = <$type as ConVarType>::from_convar_str(value)
                                .map_err(|e| ConVarError::ParseError(e.to_string()))?
                        }
                    )*
                    _ => return Err(ConVarError::UnknownConVar),
                }
                Ok(())
            }

            pub fn get_str(&self, name: &str) -> Result<String, ConVarError> {
                match name {
                    $(
                        stringify!($name) => Ok(self.$name.to_convar_str()),
                    )*
                    _ => return Err(ConVarError::UnknownConVar),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct PlayerConfig {
        pub level: i32,
        pub damage: f32,
    }

    impl ConVarType for PlayerConfig {
        fn from_convar_str(value: &str) -> Result<Self, ConVarError> {
            let parts: Vec<&str> = value.split(',').collect();
            if parts.len() != 2 {
                return Err(ConVarError::ParseError(String::from(
                    "invalid format for PlayerConfig",
                )));
            }
            let level = parts[0]
                .parse::<i32>()
                .map_err(|_| ConVarError::ParseError("failed to parse i32".to_string()))?;
            let damage = parts[1]
                .parse::<f32>()
                .map_err(|_| ConVarError::ParseError("failed to parse f32".to_string()))?;
            Ok(PlayerConfig { level, damage })
        }

        fn to_convar_str(&self) -> String {
            format!("{},{}", self.level, self.damage)
        }
    }

    convars! {
        player_config: PlayerConfig = PlayerConfig { level: 5, damage: 30.0 },
        max_enemies: i32 = 10,
        view_distance: f32 = 100.0,
        debug_mode: bool = false,
    }

    #[test]
    fn test_convars() {
        let mut convars = ConVars::default();

        // Existing assertions
        assert_eq!(convars.max_enemies, 10);
        assert_eq!(convars.view_distance, 100.0);

        // New assertions for player_config
        assert_eq!(
            convars.player_config,
            PlayerConfig {
                level: 5,
                damage: 30.0
            }
        );

        // Set a new value for player_config
        convars.set_str("player_config", "10,60.0").unwrap();
        assert_eq!(
            convars.player_config,
            PlayerConfig {
                level: 10,
                damage: 60.0
            }
        );

        // Set a new value for view_distance
        convars.set_str("view_distance", "150.0").unwrap();
        assert_eq!(convars.view_distance, 150.0);

        // Test an invalid convar name
        assert!(convars.set_str("nonexistent_convar", "123").is_err());

        // Test an invalid format for player_config
        assert!(convars.set_str("player_config", "invalid").is_err());
    }

    #[test]
    fn get_convars_str() {
        let convars = ConVars::default();

        // Get a new value for player_config
        match convars.get_str("player_config") {
            Ok(value) => assert_eq!(
                PlayerConfig::from_convar_str(&value),
                Ok(convars.player_config.clone())
            ),
            Err(_) => panic!("failed to get convar"),
        }

        // Get a new value for view_distance
        match convars.get_str("view_distance") {
            Ok(value) => assert_eq!(f32::from_convar_str(&value), Ok(convars.view_distance)),
            Err(_) => panic!("failed to get convar"),
        }
    }

    #[test]
    fn test_bool_convar() {
        let mut convars = ConVars::default();

        // Test setting and getting boolean convar
        assert!(!convars.debug_mode);
        convars.set_str("debug_mode", "true").unwrap();
        assert!(convars.debug_mode);
        assert_eq!(convars.get_str("debug_mode").unwrap(), "true");

        // Test various representations of true and false
        convars.set_str("debug_mode", "true").unwrap();
        assert!(convars.debug_mode);
        convars.set_str("debug_mode", "false").unwrap();
        assert!(!convars.debug_mode);
        convars.set_str("debug_mode", "t").unwrap();
        assert!(convars.debug_mode);
        convars.set_str("debug_mode", "f").unwrap();
        assert!(!convars.debug_mode);
        convars.set_str("debug_mode", "1").unwrap();
        assert!(convars.debug_mode);
        convars.set_str("debug_mode", "0").unwrap();
        assert!(!convars.debug_mode);
        convars.set_str("debug_mode", "y").unwrap();
        assert!(convars.debug_mode);
        convars.set_str("debug_mode", "n").unwrap();
        assert!(!convars.debug_mode);
        convars.set_str("debug_mode", "yes").unwrap();
        assert!(convars.debug_mode);
        convars.set_str("debug_mode", "no").unwrap();
        assert!(!convars.debug_mode);
        convars.set_str("debug_mode", "on").unwrap();
        assert!(convars.debug_mode);
        convars.set_str("debug_mode", "off").unwrap();
        assert!(!convars.debug_mode);
    }
}
