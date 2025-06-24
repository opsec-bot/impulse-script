use crate::modules::handlers::settings::Settings;
use std::collections::HashMap;

pub struct SettingsConverter {
    settings: Settings,
}

impl SettingsConverter {
    pub fn new() -> Self {
        let mut settings = Settings::new("./config.ini");
        settings.read();
        let sections = settings.sections();

        println!("{:?}", sections);

        for section in &sections {
            if section == "HELPY" {
                continue;
            }

            if
                let (Some(x), Some(y), Some(t)) = (
                    settings.get(section, "X"),
                    settings.get(section, "Y"),
                    settings.get(section, "Timing"),
                )
            {
                let parsed = vec![x.parse::<i32>(), y.parse::<i32>(), t.parse::<i32>()];
                if parsed.iter().all(|v| v.is_ok()) {
                    let joined = Settings::comma_join(
                        &[parsed[0].unwrap(), parsed[1].unwrap(), parsed[2].unwrap()]
                    );
                    settings.update(section, "combined", joined);
                }
            }
        }

        if !settings.check_section_exist("HELPY") {
            settings.create_section("HELPY");
            settings.create_section("HELPY_HOTKEY");

            let helpy_default = SettingsConverter::default_helpy_map();
            for (key, val) in helpy_default {
                settings.update("HELPY", &key, val);
            }
        }

        settings.write();

        Self { settings }
    }

    fn default_helpy_map() -> HashMap<String, String> {
        HashMap::from([
            ("ingame_default".into(), "90,7,58,146".into()),
            (
                "ar_timings".into(),
                "{'416-C': 8, '552 COMMANDO': 9, '556XI': 9, 'AK-12': 7, 'AK-74M': 9, 'AR33': 8, 'ARX200': 9, 'AUG A2': 8, 'C7E': 8, 'C8-SFW': 7, 'F2': 6, 'G36C': 8, 'L85A2': 9, 'M4': 8, 'M762': 8, 'R4-C': 7, 'TYPE-89': 7, 'Test': 6}".into(),
            ),
            (
                "smg_timings".into(),
                "{'9mm C1': 10,'9x19VSN': 8,'AUG A3': 9,'FMG-9': 8,'K1A': 8,'M12': 11,'MP5': 8,'MP5K': 8,'MP5SD': 8,'MP7': 7,'MPX': 7,'Mx4 Storm': 6,'P10 RONI': 6,'P90': 6,'PDW9': 8,'SCORPION EVO 3 A1': 6,'T-5 SMG': 7,'UMP45': 10,'UZK50GI': 9,'VECTOR .45 ACP': 5}".into(),
            ),
            (
                "lmg_timings".into(),
                "{'6P41': 9,'ALDA 5.56': 7,'DP27': 11,'G8A1': 7,'LMG-E': 8,'M249 SAW': 9,'M249': 9,'T-95 LSW': 9}".into(),
            ),
            (
                "mp_timings".into(),
                "{'BEARING 9': 5,'C75 Auto': 6,'SMG-11': 5,'SMG-12': 5,'SPSMG9': 6,'REAPER MK2':6}".into(),
            ),
            ("bind_panic".into(), "End".into()),
            ("bind_toggle_menu".into(), "Ins".into()),
            ("window_width".into(), "500".into()),
            ("window_height".into(), "800".into()),
            ("converted".into(), "True".into()),
        ])
    }
}
