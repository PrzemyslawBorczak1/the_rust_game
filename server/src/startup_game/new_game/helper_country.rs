use anyhow::Result;
use bevy::math::vec4;
use shared::resources::Country;

// always 4 countries
pub fn generate_country(path: &str) -> Result<()> {
    let countries: Vec<Country> = vec![
        Country {
            color: vec4(0.85, 0.18, 0.20, 1.0),
            flag_path: "flags/empire.png".to_string(),
            army: 0,
            gold: 0,
        },
        Country {
            color: vec4(0.16, 0.65, 0.30, 1.0),
            flag_path: "flags/republic.png".to_string(),
            army: 0,
            gold: 0,
        },
        Country {
            color: vec4(0.20, 0.35, 0.82, 1.0),
            flag_path: "flags/kingdom.png".to_string(),
            army: 0,
            gold: 0,
        },
        Country {
            color: vec4(0.92, 0.72, 0.18, 1.0),
            flag_path: "flags/duchy.png".to_string(),
            army: 0,
            gold: 0,
        },
    ];

    let json = serde_json::to_string_pretty(&countries)?;
    std::fs::write("assets\\".to_string() + path, json)?;

    Ok(())
}
