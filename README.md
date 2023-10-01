# ConVars
ConVars is a Rust library to create console variables for games made. Designed with the [Bevy engine](https://bevyengine.org/) in mind but can work in any project. It allows developers to access and modify game configuration variables at runtime through a console or other user input mechanisms.

## Features
* Define configuration variables with default values
* Update configuration variables at runtime
* Structs supported
* Serialize and deserialize configuration variables

## Usage
Add this to your Cargo.toml:

```
cargo add convars
```

```toml
[dependencies]
convars = "0.1.0"
```

Here's a basic example:

```rust
use convars::convars;

convars! {
    max_enemies: i32 = 10,
    view_distance: f32 = 100.0,
}

fn main() {
    let mut convars = ConVars::default();
    
    // These values will come from a UI or console
    convars.set_str("max_enemies", "20").unwrap();
}
```

## User Defined Types
You can define your own configuration variable types by implementing the ConVarType trait:

```rust
use convars::ConVarType;

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
}
```
