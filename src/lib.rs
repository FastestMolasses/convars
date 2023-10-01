pub trait ConVarType: Sized {
    fn from_convar_str(value: &str) -> Result<Self, &'static str>;
    fn to_convar_str(&self) -> String;
}

impl ConVarType for i32 {
    fn from_convar_str(value: &str) -> Result<Self, &'static str> {
        value.parse::<i32>().map_err(|_| "failed to parse i32")
    }

    fn to_convar_str(&self) -> String {
        self.to_string()
    }
}

impl ConVarType for f32 {
    fn from_convar_str(value: &str) -> Result<Self, &'static str> {
        value.parse::<f32>().map_err(|_| "failed to parse f32")
    }

    fn to_convar_str(&self) -> String {
        self.to_string()
    }
}

macro_rules! convars {
    ($($name:ident: $type:ty = $value:expr),* $(,)? ) => {
        #[derive(Debug)]
        pub struct ConVars {
            $(pub $name: $type,)*
        }

        impl Default for ConVars {
            fn default() -> Self {
                Self {
                    $($name: $value,)*
                }
            }
        }

        impl ConVars {
            pub fn set_str(&mut self, name: &str, value: &str) -> Result<(), &'static str> {
                match name {
                    $(
                        stringify!($name) => {
                            self.$name = <$type as ConVarType>::from_convar_str(value)?
                        }
                    )*
                    _ => return Err("unknown convar"),
                }
                Ok(())
            }

            pub fn get_str(&self, name: &str) -> Result<String, &'static str> {
                match name {
                    $(
                        stringify!($name) => Ok(self.$name.to_convar_str()),
                    )*
                    _ => return Err("unknown convar"),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    pub struct PlayerConfig {
        pub level: i32,
        pub damage: f32,
    }

    impl ConVarType for PlayerConfig {
        fn from_convar_str(value: &str) -> Result<Self, &'static str> {
            let parts: Vec<&str> = value.split(',').collect();
            if parts.len() != 2 {
                return Err("invalid format for PlayerConfig");
            }
            let level = parts[0].parse::<i32>().map_err(|_| "failed to parse i32")?;
            let damage = parts[1].parse::<f32>().map_err(|_| "failed to parse f32")?;
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
        convars
            .set_str("player_config", "10,60.0")
            .unwrap();
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
        assert!(convars
            .set_str("player_config", "invalid")
            .is_err());
    }

    #[test]
    fn get_convars_str() {
        let convars = ConVars::default();

        // Get a new value for player_config
        match convars.get_str("player_config") {
            Ok(value) => assert_eq!(
                PlayerConfig::from_convar_str(&value),
                Ok(convars.player_config)
            ),
            Err(_) => panic!("failed to get convar"),
        }
    }
}
