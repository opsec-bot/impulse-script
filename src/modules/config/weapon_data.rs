pub const DEFAULT_WEAPONS: &[(&str, i32, &str)] = &[
    // ARs
    ("416-C CARBINE", 740, "AR"),
    ("552 COMMANDO", 690, "AR"),
    ("556XI", 690, "AR"),
    ("AK-12", 850, "AR"),
    ("AK-74M", 650, "AR"),
    ("AR33", 749, "AR"),
    ("ARX200", 700, "AR"),
    ("AUG A2", 720, "AR"),
    ("C7E", 800, "AR"),
    ("C8-SFW", 837, "AR"),
    ("COMMANDO 9", 780, "AR"),
    ("F2", 980, "AR"),
    ("F90", 780, "AR"),
    ("G36C", 780, "AR"),
    ("L85A2", 670, "AR"),
    ("M4", 750, "AR"),
    ("M762", 730, "AR"),
    ("MK17 CQB", 585, "AR"),
    ("PARA-308", 650, "AR"),
    ("PCX-33", 745, "AR"),
    ("POF-9", 740, "AR"),
    ("R4-C", 860, "AR"),
    ("SC3000K", 800, "AR"),
    ("SPEAR .308", 700, "AR"),
    ("TYPE-89", 850, "AR"),
    ("V308", 700, "AR"),
    
    // SMGs
    ("9mm C1", 575, "SMG"),
    ("9x19VSN", 750, "SMG"),
    ("AUG A3", 700, "SMG"),
    ("FMG-9", 800, "SMG"),
    ("K1A", 720, "SMG"),
    ("M12", 550, "SMG"),
    ("MP5", 800, "SMG"),
    ("MP5K", 800, "SMG"),
    ("MP5SD", 800, "SMG"),
    ("MP7", 900, "SMG"),
    ("MPX", 830, "SMG"),
    ("Mx4 Storm", 950, "SMG"),
    ("P10 RONI", 980, "SMG"),
    ("P90", 970, "SMG"),
    ("PDW9", 800, "SMG"),
    ("SCORPION EVO 3 A1", 1080, "SMG"),
    ("T-5 SMG", 900, "SMG"),
    ("UMP45", 600, "SMG"),
    ("UZK50GI", 700, "SMG"),
    ("VECTOR .45 ACP", 1200, "SMG"),
    
    // LMGs
    ("6P41", 680, "LMG"),
    ("ALDA 5.56", 900, "LMG"),
    ("DP27", 550, "LMG"),
    ("G8A1", 850, "LMG"),
    ("LMG-E", 720, "LMG"),
    ("M249", 650, "LMG"),
    ("M249 SAW", 650, "LMG"),
    ("T-95 LSW", 650, "LMG"),
    
    // DMRs
    ("417", 430, "DMR"),
    ("AR-15.50", 430, "DMR"),
    ("CAMRS", 420, "DMR"),
    ("Mk 14 EBR", 440, "DMR"),
    ("OTs-03", 380, "DMR"),
    ("SR-25", 440, "DMR"),
    
    // Machine Pistols
    ("BEARING 9", 1100, "MP"),
    ("C75 Auto", 1000, "MP"),
    ("SMG-11", 1270, "MP"),
    ("SMG-12", 1270, "MP"),
    ("SPSMG9", 980, "MP"),
    
    // Shotguns
    ("FO-12", 400, "SG"),
    ("ITA12L", 80, "SG"),
    ("ITA12S", 80, "SG"),
    ("M1014", 215, "SG"),
    ("M590A1", 85, "SG"),
    ("M870", 100, "SG"),
    ("SASG-12", 340, "SG"),
    ("SG-CQB", 85, "SG"),
    ("SIX12", 220, "SG"),
    ("SIX12 SD", 220, "SG"),
    ("SKELETON KEY", 220, "SG"),
    ("SPAS-12", 220, "SG"),
    ("SPAS-15", 300, "SG"),
    ("SUPER 90", 220, "SG"),
    ("SUPER SHORTY", 100, "SG"),
    ("SUPERNOVA", 85, "SG"),
    
    // Slug Shotguns
    ("ACS12", 300, "Slug SG"),
    ("BOSG.12.2", 600, "Slug SG"),
    ("TCSG12", 490, "Slug SG"),
    
    // Handguns
    (".44 Mag Semi-Auto", 480, "Handgun"),
    ("1911 TACOPS", 480, "Handgun"),
    ("5.7 USG", 480, "Handgun"),
    ("D-50", 480, "Handgun"),
    ("GSH-18", 480, "Handgun"),
    ("LUISON", 440, "Handgun"),
    ("M45 MEUSOC", 480, "Handgun"),
    ("MK1 9mm", 480, "Handgun"),
    ("P-10C", 480, "Handgun"),
    ("P12", 480, "Handgun"),
    ("P226 MK 25", 480, "Handgun"),
    ("P229", 480, "Handgun"),
    ("P9", 480, "Handgun"),
    ("PMM", 480, "Handgun"),
    ("PRB92", 480, "Handgun"),
    ("Q-929", 480, "Handgun"),
    ("RG15", 480, "Handgun"),
    ("SDP 9mm", 480, "Handgun"),
    ("USP40", 480, "Handgun"),
    
    // Revolvers
    (".44 Vendetta", 480, "Revolver"),
    ("KERATOS .357", 480, "Revolver"),
    ("LFP586", 480, "Revolver"),
    
    // Special Weapons
    ("Bailiff 410", 500, "SG"),
];

pub const WEAPON_CLASSES: &[&str] = &[
    "AR",
    "SMG",
    "LMG",
    "DMR",
    "MP",
    "SG",
    "Slug SG",
    "Handgun",
    "Revolver",
];

/// Per-weapon recoil starting values (name, X, Y) mined from the VER2 Y10S1
/// (Attackers) Cronus GPC `GunAntiRecoil` table: averaged across the operators
/// that carry each gun, scaled by its 95% moveAdj, then normalized — vertical to
/// the Y `1..10` slider range and horizontal (clamped) to the X `-2..2` range.
///
/// These are RELATIVE defaults only: the source values are controller stick-%
/// units and are per-operator-loadout, so they're trustworthy mainly as an
/// ordering for sprayable classes (AR/SMG/LMG/DMR). Calibrate absolute strength
/// once with the global Recoil Scale; weapons absent here keep the flat 0/1
/// default. (Comments show the pre-normalization V/H for transparency.)
pub const VER2_RECOIL: &[(&str, f32, f32)] = &[
    ("F2", -1.0, 10.0),            // V=60.8 H=-2.8
    ("R4-C", -1.0, 9.2),           // V=56.0 H=-2.8
    ("LMG-E", 1.0, 8.4),           // V=51.3 H=3.3
    ("SPEAR .308", -1.0, 8.4),     // V=51.3 H=-2.8
    ("552 COMMANDO", -1.0, 7.7),   // V=46.5 H=-2.8
    ("AUG A2", -1.0, 7.7),         // V=46.5 H=-2.8
    ("M762", 2.0, 7.7),            // V=46.5 H=9.5
    ("6P41", -1.0, 7.5),           // V=45.6 H=-3.3
    ("AR-15.50", -1.0, 7.3),       // V=44.6 H=-2.8
    ("G36C", 0.0, 7.3),            // V=44.6 H=-1.9
    ("M4", -1.0, 7.3),             // V=44.6 H=-2.8
    ("C8-SFW", 0.0, 7.0),          // V=42.8 H=-0.9
    ("CAMRS", 0.0, 6.8),           // V=41.3 H=-1.9
    ("FMG-9", 1.0, 6.7),           // V=40.9 H=4.8
    ("G8A1", -1.0, 6.6),           // V=40.4 H=-3.3
    ("AK-12", -1.0, 6.6),          // V=39.9 H=-3.8
    ("PARA-308", -1.0, 6.6),       // V=39.9 H=-2.8
    ("V308", 1.0, 6.6),            // V=39.9 H=2.8
    ("417", 0.0, 6.4),             // V=39.2 H=0.5
    ("MK17 CQB", 0.0, 6.4),        // V=38.9 H=-0.9
    ("OTs-03", 0.0, 6.4),          // V=38.9 H=1.9
    ("TYPE-89", -1.0, 6.4),        // V=38.9 H=-3.8
    ("F90", 0.0, 6.2),             // V=38.0 H=1.9
    ("MP7", 0.0, 6.2),             // V=38.0 H=-1.9
    ("SC3000K", 0.0, 6.2),         // V=38.0 H=-1.9
    ("SR-25", 0.0, 6.2),           // V=38.0 H=-1.9
    ("M249", 0.0, 6.2),            // V=37.8 H=-1.2
    ("AK-74M", 0.0, 6.1),          // V=37.0 H=-2.4
    ("C7E", -1.0, 6.1),            // V=37.0 H=-2.8
    ("PDW9", 0.0, 6.1),            // V=37.0 H=-2.4
    ("556XI", 0.0, 6.0),           // V=36.6 H=1.4
    ("AR33", 0.0, 5.9),            // V=36.1 H=1.9
    ("L85A2", 1.0, 5.8),           // V=35.1 H=6.6
    ("T-95 LSW", 0.0, 5.8),        // V=35.1 H=0.9
    ("ARX200", 0.0, 5.7),          // V=34.7 H=-1.4
    ("Mk 14 EBR", -1.0, 4.8),      // V=29.4 H=-2.8
    ("POF-9", 1.0, 4.5),           // V=27.5 H=2.8
];

/// Look up a weapon's VER2 recoil default. Returns (X, Y) if present.
/// Case-insensitive: config section names are lowercased by configparser.
pub fn ver2_recoil(wep_name: &str) -> Option<(f32, f32)> {
    VER2_RECOIL.iter()
        .find(|(name, _, _)| name.eq_ignore_ascii_case(wep_name))
        .map(|(_, x, y)| (*x, *y))
}
