use crate::web::AppData;
use actix_web::{post, web, HttpResponse, Responder};
use anyhow::{Context, Result};
use hashbrown::HashMap;
use log::error;
use maprando::settings::{
    parse_randomizer_settings, NotableSetting, RandomizerSettings, TechSetting,
};
use maprando_game::{NotableId, RoomId, TechId};

use super::VERSION;

fn assign_presets(settings: &mut serde_json::Value, app_data: &AppData) -> Result<()> {
    if let Some(preset) = settings["skill_assumption_settings"]["preset"].as_str() {
        let preset = preset.to_owned();
        for p in &app_data.preset_data.skill_presets {
            if p.preset.as_ref() == Some(&preset) {
                *settings.get_mut("skill_assumption_settings").unwrap() = serde_json::to_value(p)?;
            }
        }
    }
    if let Some(preset) = settings["item_progression_settings"]["preset"].as_str() {
        let preset = preset.to_owned();
        for p in &app_data.preset_data.item_progression_presets {
            if p.preset.as_ref() == Some(&preset) {
                *settings.get_mut("item_progression_settings").unwrap() = serde_json::to_value(p)?;
            }
        }
    }
    if let Some(preset) = settings["quality_of_life_settings"]["preset"].as_str() {
        let preset = preset.to_owned();
        for p in &app_data.preset_data.quality_of_life_presets {
            if p.preset.as_ref() == Some(&preset) {
                *settings.get_mut("quality_of_life_settings").unwrap() = serde_json::to_value(p)?;
            }
        }
    }
    if let Some(preset) = settings["name"].as_str() {
        let preset = preset.to_owned();
        for p in &app_data.preset_data.full_presets {
            if p.name.as_ref() == Some(&preset) {
                *settings = serde_json::to_value(p)?;
            }
        }
    }
    Ok(())
}

fn upgrade_tech_settings(settings: &mut serde_json::Value, app_data: &AppData) -> Result<()> {
    // This updates the names of tech, discards any obsolete tech settings, and disables
    // any new tech that are not referenced in the settings.
    let mut tech_map: HashMap<TechId, bool> = HashMap::new();
    for tech_setting in settings["skill_assumption_settings"]["tech_settings"]
        .as_array()
        .context("missing tech_settings")?
    {
        let tech_id = tech_setting["id"]
            .as_i64()
            .context("tech_setting missing id field")? as TechId;
        let enabled = tech_setting["enabled"]
            .as_bool()
            .context("tech_setting missing enabled field")?;
        tech_map.insert(tech_id, enabled);
    }

    let mut new_tech_settings: Vec<TechSetting> = vec![];
    for t in &app_data
        .preset_data
        .default_preset
        .skill_assumption_settings
        .tech_settings
    {
        new_tech_settings.push(TechSetting {
            id: t.id,
            name: t.name.clone(),
            enabled: tech_map.get(&t.id).map(|x| *x).unwrap_or(false),
        });
    }
    *settings
        .get_mut("skill_assumption_settings")
        .unwrap()
        .get_mut("tech_settings")
        .unwrap() = serde_json::to_value(new_tech_settings)?;

    Ok(())
}

fn upgrade_notable_settings(settings: &mut serde_json::Value, app_data: &AppData) -> Result<()> {
    // This updates the names of notables, discards any obsolete notables settings, and disables
    // any new notables that are not referenced in the settings.
    let mut notable_map: HashMap<(RoomId, NotableId), bool> = HashMap::new();
    for notable_setting in settings["skill_assumption_settings"]["notable_settings"]
        .as_array()
        .context("missing notable_settings")?
    {
        let room_id = notable_setting["room_id"]
            .as_i64()
            .context("notable_setting missing room_id field")? as RoomId;
        let notable_id = notable_setting["notable_id"]
            .as_i64()
            .context("notable_setting missing notable_id field")?
            as RoomId;
        let enabled = notable_setting["enabled"]
            .as_bool()
            .context("notable_setting missing enabled field")?;
        notable_map.insert((room_id, notable_id), enabled);
    }

    let mut new_notable_settings: Vec<NotableSetting> = vec![];
    for s in &app_data
        .preset_data
        .default_preset
        .skill_assumption_settings
        .notable_settings
    {
        new_notable_settings.push(NotableSetting {
            room_id: s.room_id,
            notable_id: s.notable_id,
            room_name: s.room_name.clone(),
            notable_name: s.notable_name.clone(),
            enabled: notable_map
                .get(&(s.room_id, s.notable_id))
                .map(|x| *x)
                .unwrap_or(false),
        });
    }
    *settings
        .get_mut("skill_assumption_settings")
        .unwrap()
        .get_mut("notable_settings")
        .unwrap() = serde_json::to_value(new_notable_settings)?;

    Ok(())
}

fn upgrade_item_progression_settings(settings: &mut serde_json::Value) -> Result<()> {
    let item_progression_settings = settings
        .get_mut("item_progression_settings")
        .context("missing item_progression_settings")?
        .as_object_mut()
        .context("item_progression_settings is not object")?;
    if !item_progression_settings.contains_key("ammo_collect_fraction") {
        item_progression_settings.insert("ammo_collect_fraction".to_string(), (0.7).into());
    }

    Ok(())
}

fn upgrade_map_setting(settings: &mut serde_json::Value) -> Result<()> {
    if settings["map_layout"].as_str() == Some("Tame") {
        *settings.get_mut("map_layout").unwrap() = "Standard".into();
    }
    Ok(())
}

fn upgrade_animals_setting(settings: &mut serde_json::Value) -> Result<()> {
    if settings["save_animals"].as_str() == Some("Maybe") {
        *settings.get_mut("save_animals").unwrap() = "Optional".into();
    }
    Ok(())
}

pub fn try_upgrade_settings(
    settings_str: String,
    app_data: &AppData,
) -> Result<(String, RandomizerSettings)> {
    let mut settings: serde_json::Value = serde_json::from_str(&settings_str)?;

    assign_presets(&mut settings, app_data)?;
    upgrade_tech_settings(&mut settings, app_data)?;
    upgrade_notable_settings(&mut settings, app_data)?;
    upgrade_item_progression_settings(&mut settings)?;
    upgrade_map_setting(&mut settings)?;
    upgrade_animals_setting(&mut settings)?;

    // Update version field to current version:
    *settings
        .get_mut("version")
        .context("missing version field")? = VERSION.into();

    // Validate that the upgraded settings will parse as a RandomizerSettings struct:
    let settings_str = settings.to_string();
    let settings_out = parse_randomizer_settings(&settings_str)?;
    let settings_out_str = serde_json::to_string(&settings_out)?;
    Ok((settings_out_str, settings_out))
}

#[post("/upgrade-settings")]
async fn upgrade_settings(settings_str: String, app_data: web::Data<AppData>) -> impl Responder {
    match try_upgrade_settings(settings_str, &app_data) {
        Ok((settings_str, _)) => HttpResponse::Ok()
            .content_type("application/json")
            .body(settings_str),
        Err(e) => {
            error!("Failed to upgrade settings: {}", e);
            HttpResponse::BadRequest().body(e.to_string())
        }
    }
}
