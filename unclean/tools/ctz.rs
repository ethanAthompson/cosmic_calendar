use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum::EnumProperty;
use strum::*;
// https://terraforming.fandom.com/wiki/Time,_Day_And_Year_On_Other_Planets#Martian_moons
// https://terraforming.fandom.com/wiki/Time,_Day_And_Year_On_Other_Planets#Martian_moons
///
/// Celestial Time Zones
///
///
/// Contains all the Celestial Bodies used for this project
/// This format makes it easy to track what celestial body is being used.
/// This enum took inspiration from the chrono_tz's Tz enum
///
///
/// This is a Comphrehensive List of celestial bodies
///
///
#[allow(non_camel_case_types)]
#[derive(Clone, Display, Debug, Deserialize, Serialize, PartialEq, Eq, EnumString, EnumIter, EnumProperty)]
pub enum Ctz {
    // 1st Inner Planet: Mercury
    // https://en.wikipedia.org/wiki/List_of_geological_features_on_Mercury
    #[strum(props(Name = "Mercury_Mountains/Caloris_Montes"))]
    Mercury_Mountains__Caloris_Montes,
    #[strum(props(Name = "Mercury_Ridges/Antoniadi_Dorsum"))]
    Mercury_Ridges__Antoniadi_Dorsum,
    #[strum(props(Name = "Mercury_Ridges/Schiaparelli_Dorsum"))]
    Mercury_Ridges__Schiaparelli_Dorsum,
    #[strum(props(Name = "Mercury_Fossae/Borobudur_Fossae"))]
    Mercury_Fossae__Borobudur_Fossae,
    #[strum(props(Name = "Mercury_Fossae/Pantheon_Fossae"))]
    Mercury_Fossae__Pantheon_Fossae,
    #[strum(props(Name = "Mercury_Valley/Angkor_Vallis"))]
    Mercury_Valley__Angkor_Vallis,
    #[strum(props(Name = "Mercury_Valley/Cahokia_Vallis"))]
    Mercury_Valley__Cahokia_Vallis,
    #[strum(props(Name = "Mercury_Valley/Caral_Vallis"))]
    Mercury_Valley__Caral_Vallis,
    #[strum(props(Name = "Mercury_Valley/Paestum_Vallis"))]
    Mercury_Valley__Paestum_Vallis,
    #[strum(props(Name = "Mercury_Valley/Timgad_Vallis"))]
    Mercury_Valley__Timgad_Vallis,
    #[strum(props(Name = "Mercury_Valley/Apārangi_Planitia"))]
    Mercury_Valley__Apārangi_Planitia,
    #[strum(props(Name = "Mercury_Valley/Borealis_Planitia"))]
    Mercury_Valley__Borealis_Planitia,
    #[strum(props(Name = "Mercury_Valley/Budh_Planitia"))]
    Mercury_Valley__Budh_Planitia,
    #[strum(props(Name = "Mercury_Valley/Caloris_Planitia"))]
    Mercury_Valley__Caloris_Planitia,
    #[strum(props(Name = "Mercury_Valley/Catuilla_Planum"))]
    Mercury_Valley__Catuilla_Planum,
    #[strum(props(Name = "Mercury_Valley/Lugus_Planitia"))]
    Mercury_Valley__Lugus_Planitia,
    #[strum(props(Name = "Mercury_Valley/Mearcair_Planitia"))]
    Mercury_Valley__Mearcair_Planitia,
    #[strum(props(Name = "Mercury_Valley/Odin_Planitia"))]
    Mercury_Valley__Odin_Planitia,
    #[strum(props(Name = "Mercury_Valley/Otaared_Planitia"))]
    Mercury_Valley__Otaared_Planitia,
    #[strum(props(Name = "Mercury_Valley/Papsukkal_Planitia"))]
    Mercury_Valley__Papsukkal_Planitia,
    #[strum(props(Name = "Mercury_Valley/Sihtu_Planitia"))]
    Mercury_Valley__Sihtu_Planitia,
    #[strum(props(Name = "Mercury_Valley/Sobkou_Planitia"))]
    Mercury_Valley__Sobkou_Planitia,
    #[strum(props(Name = "Mercury_Valley/Stilbon_Planitia"))]
    Mercury_Valley__Stilbon_Planitia,
    #[strum(props(Name = "Mercury_Valley/Suisei_Planitia"))]
    Mercury_Valley__Suisei_Planitia,
    #[strum(props(Name = "Mercury_Valley/Tir_Planitia"))]
    Mercury_Valley__Tir_Planitia,
    #[strum(props(Name = "Mercury_Valley/Turms_Planitia"))]
    Mercury_Valley__Turms_Planitia,
    #[strum(props(Name = "Mercury_Valley/Utaridi_Planitia"))]
    Mercury_Valley__Utaridi_Planitia,
    #[strum(props(Name = "Mercury_Escarpment/Acadia_Rupes"))]
    Mercury_Escarpment__Acadia_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Adventure_Rupes"))]
    Mercury_Escarpment__Adventure_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Altair_Rupes"))]
    Mercury_Escarpment__Altair_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Alvin_Rupes"))]
    Mercury_Escarpment__Alvin_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Antares_Rupes"))]
    Mercury_Escarpment__Antares_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Astrolabe_Rupes"))]
    Mercury_Escarpment__Astrolabe_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Beagle_Rupes"))]
    Mercury_Escarpment__Beagle_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Belgica_Rupes"))]
    Mercury_Escarpment__Belgica_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Blossom_Rupes"))]
    Mercury_Escarpment__Blossom_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Calypso_Rupes"))]
    Mercury_Escarpment__Calypso_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Carnegie_Rupes"))]
    Mercury_Escarpment__Carnegie_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Challenger_Rupes"))]
    Mercury_Escarpment__Challenger_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Discovery_Rupes"))]
    Mercury_Escarpment__Discovery_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Duyfken_Rupes"))]
    Mercury_Escarpment__Duyfken_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Eltanin_Rupes"))]
    Mercury_Escarpment__Eltanin_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Endeavour_Rupes"))]
    Mercury_Escarpment__Endeavour_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Enterprise_Rupes"))]
    Mercury_Escarpment__Enterprise_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Fram_Rupes"))]
    Mercury_Escarpment__Fram_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Gjöa_Rupes"))]
    Mercury_Escarpment__Gjöa_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Heemskerck_Rupes"))]
    Mercury_Escarpment__Heemskerck_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Hero_Rupes"))]
    Mercury_Escarpment__Hero_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Kainan_Rupes"))]
    Mercury_Escarpment__Kainan_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/La_Duaphine_Rupes"))]
    Mercury_Escarpment__La_Duaphine_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Mirni_Rupes"))]
    Mercury_Escarpment__Mirni_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Nautilus_Rupes"))]
    Mercury_Escarpment__Nautilus_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Palmer_Rupes"))]
    Mercury_Escarpment__Palmer_Rupes,
    #[strum(props(Name = "Mercury_Escarpment/Paramour_Rupes"))]
    Mercury_Escarpment__Paramour_Rupes,
    #[strum(props(Name = "Mercury_Crater_Chain/Arecibo_Catena"))]
    Mercury_Crater_Chain__Arecibo_Catena,
    #[strum(props(Name = "Mercury_Crater_Chain/Goldstone_Catena"))]
    Mercury_Crater_Chain__Goldstone_Catena,
    #[strum(props(Name = "Mercury_Crater_Chain/Haystack_Catena"))]
    Mercury_Crater_Chain__Haystack_Catena,
    #[strum(props(Name = "Mercury_Facula/Abeeso_Facula"))]
    Mercury_Facula__Abeeso_Facula,
    #[strum(props(Name = "Mercury_Facula/Agwo_Facula"))]
    Mercury_Facula__Agwo_Facula,
    #[strum(props(Name = "Mercury_Facula/Amaru_Facula"))]
    Mercury_Facula__Amaru_Facula,
    #[strum(props(Name = "Mercury_Facula/Bibilava_Faculae"))]
    Mercury_Facula__Bibilava_Faculae,
    #[strum(props(Name = "Mercury_Facula/Bitin_Facula"))]
    Mercury_Facula__Bitin_Facula,
    #[strum(props(Name = "Mercury_Facula/Ejo_Faculae"))]
    Mercury_Facula__Ejo_Faculae,
    #[strum(props(Name = "Mercury_Facula/Gata_Facula"))]
    Mercury_Facula__Gata_Facula,
    #[strum(props(Name = "Mercury_Facula/Havu_Facula"))]
    Mercury_Facula__Havu_Facula,
    #[strum(props(Name = "Mercury_Facula/Ibab_Facula"))]
    Mercury_Facula__Ibab_Facula,
    #[strum(props(Name = "Mercury_Facula/Inyoka_Faculae"))]
    Mercury_Facula__Inyoka_Faculae,
    #[strum(props(Name = "Mercury_Facula/Maciji_Facula"))]
    Mercury_Facula__Maciji_Facula,
    #[strum(props(Name = "Mercury_Facula/Nākahi_Facula"))]
    Mercury_Facula__Nākahi_Facula,
    #[strum(props(Name = "Mercury_Facula/Nathair_Facula"))]
    Mercury_Facula__Nathair_Facula,
    #[strum(props(Name = "Mercury_Facula/Neidr_Facula"))]
    Mercury_Facula__Neidr_Facula,
    #[strum(props(Name = "Mercury_Facula/Nzoka_Facula"))]
    Mercury_Facula__Nzoka_Facula,
    #[strum(props(Name = "Mercury_Facula/Orm_Faculae"))]
    Mercury_Facula__Orm_Faculae,
    #[strum(props(Name = "Mercury_Facula/Pampu_Facula"))]
    Mercury_Facula__Pampu_Facula,
    #[strum(props(Name = "Mercury_Facula/Sarpa_Facula"))]
    Mercury_Facula__Sarpa_Facula,
    #[strum(props(Name = "Mercury_Facula/Slang_Faculae"))]
    Mercury_Facula__Slang_Faculae,
    #[strum(props(Name = "Mercury_Facula/Suge_Facula"))]
    Mercury_Facula__Suge_Facula,
    #[strum(props(Name = "Mercury_Facula/Thueban_Facula"))]
    Mercury_Facula__Thueban_Facula,
    #[strum(props(Name = "Mercury_Facula/Ular_Facula"))]
    Mercury_Facula__Ular_Facula,
    #[strum(props(Name = "Mercury_Facula/Yinshe_Facula"))]
    Mercury_Facula__Yinshe_Facula,
    #[strum(props(Name = "Mercury_Facula/Zmija_Facula"))]
    Mercury_Facula__Zmija_Facula,
    // 2nd Inner Planet: Venus
    // https://en.wikipedia.org/wiki/List_of_geological_features_on_Venus
    #[strum(props(Name = "Venus_Region/Alpha_Regio"))]
    Venus_Region__Alpha_Regio,
    #[strum(props(Name = "Venus_Region/Asteria_Regio"))]
    Venus_Region__Asteria_Regio,
    #[strum(props(Name = "Venus_Region/Atla_Regio"))]
    Venus_Region__Atla_Regio,
    #[strum(props(Name = "Venus_Region/Bell_Regio"))]
    Venus_Region__Bell_Regio,
    #[strum(props(Name = "Venus_Region/Beta_Regio"))]
    Venus_Region__Beta_Regio,
    #[strum(props(Name = "Venus_Region/Dione_Regio"))]
    Venus_Region__Dione_Regio,
    #[strum(props(Name = "Venus_Region/Dsonkwa_Regio"))]
    Venus_Region__Dsonkwa_Regio,
    #[strum(props(Name = "Venus_Region/Eistla_Regio"))]
    Venus_Region__Eistla_Regio,
    #[strum(props(Name = "Venus_Region/Hyndla_Regio"))]
    Venus_Region__Hyndla_Regio,
    #[strum(props(Name = "Venus_Region/Imdr_Regio"))]
    Venus_Region__Imdr_Regio,
    #[strum(props(Name = "Venus_Region/Ishkus_Regio"))]
    Venus_Region__Ishkus_Regio,
    #[strum(props(Name = "Venus_Region/Laufey_Regio"))]
    Venus_Region__Laufey_Regio,
    #[strum(props(Name = "Venus_Region/Neringa_Regio"))]
    Venus_Region__Neringa_Regio,
    #[strum(props(Name = "Venus_Region/Ovda_Regio"))]
    Venus_Region__Ovda_Regio,
    #[strum(props(Name = "Venus_Region/Phoebe_Regio"))]
    Venus_Region__Phoebe_Regio,
    #[strum(props(Name = "Venus_Region/Tethus_Regio"))]
    Venus_Region__Tethus_Regio,
    #[strum(props(Name = "Venus_Region/Themis_Regio"))]
    Venus_Region__Themis_Regio,
    #[strum(props(Name = "Venus_Region/Thetis_Regio"))]
    Venus_Region__Thetis_Regio,
    #[strum(props(Name = "Venus_Region/Ulfrun_Regio"))]
    Venus_Region__Ulfrun_Regio,
    #[strum(props(Name = "Venus_Rupes/Fornax_Rupes"))]
    Venus_Rupes__Fornax_Rupes,
    #[strum(props(Name = "Venus_Rupes/Gabie_Rupes"))]
    Venus_Rupes__Gabie_Rupes,
    #[strum(props(Name = "Venus_Rupes/Hestia_Rupes"))]
    Venus_Rupes__Hestia_Rupes,
    #[strum(props(Name = "Venus_Rupes/Uorsar_Rupes"))]
    Venus_Rupes__Uorsar_Rupes,
    #[strum(props(Name = "Venus_Rupes/Ut_Rupes"))]
    Venus_Rupes__Ut_Rupes,
    #[strum(props(Name = "Venus_Rupes/Vaidilute_Rupes"))]
    Venus_Rupes__Vaidilute_Rupes,
    #[strum(props(Name = "Venus_Rupes/Vesta_Rupes"))]
    Venus_Rupes__Vesta_Rupes,
    // 4th Inner Planet: Mars
    // https://en.wikipedia.org/wiki/List_of_terrae_on_Mars
    #[strum(props(Name = "Mars_Terra/Aonia_Terra"))]
    Mars_Terra__Aonia_Terra,
    #[strum(props(Name = "Mars_Terra/Arabia_Terra"))]
    Mars_Terra__Arabia_Terra,
    #[strum(props(Name = "Mars_Terra/Terra_Cimmeria"))]
    Mars_Terra__Terra_Cimmeria,
    #[strum(props(Name = "Mars_Terra/Margaritifer_Terra"))]
    Mars_Terra__Margaritifer_Terra,
    #[strum(props(Name = "Mars_Terra/Noachis_Terra"))]
    Mars_Terra__Noachis_Terra,
    #[strum(props(Name = "Mars_Terra/Promethei_Terra"))]
    Mars_Terra__Promethei_Terra,
    #[strum(props(Name = "Mars_Terra/Terra_Sabaea"))]
    Mars_Terra__Terra_Sabaea,
    #[strum(props(Name = "Mars_Terra/Terra_Sirenum"))]
    Mars_Terra__Terra_Sirenum,
    #[strum(props(Name = "Mars_Terra/Tempe_Terra"))]
    Mars_Terra__Tempe_Terra,
    #[strum(props(Name = "Mars_Terra/Tyrrhena_Terra"))]
    Mars_Terra__Tyrrhena_Terra,
    #[strum(props(Name = "Mars_Terra/Xanthe_Terra"))]
    Mars_Terra__Xanthe_Terra,
    // 1st Outer Planet: Jupiter
    // https://en.wikipedia.org/wiki/Jupiter
    #[strum(props(Name = "Jupiter_Region/South_Polar"))]
    Jupiter_Region__South_Polar,
    #[strum(props(Name = "Jupiter_Region/North_Polar"))]
    Jupiter_Region__North_Polar,
    #[strum(props(Name = "Jupiter_Zone/North_Temperate"))]
    Jupiter_Zone__North_Temperate,
    #[strum(props(Name = "Jupiter_Belt/North_Temperate"))]
    Jupiter_Belt__North_Temperate,
    #[strum(props(Name = "Jupiter_Zone/Tropical_North"))]
    Jupiter_Zone__Tropical_North,
    #[strum(props(Name = "Jupiter_Belt/Equatorial_North"))]
    Jupiter_Belt__Equatorial_North,
    #[strum(props(Name = "Jupiter_Zone/Equatorial_Zone"))]
    Jupiter_Zone__Equatorial_Zone,
    #[strum(props(Name = "Jupiter_Belt/Equatorial_South"))]
    Jupiter_Belt__Equatorial_South,
    #[strum(props(Name = "Jupiter_Spot/Great_Red"))]
    Jupiter_Spot_Great_Red,
    #[strum(props(Name = "Jupiter_Zone/Tropical_South"))]
    Jupiter_Zone_Tropical_South,
    #[strum(props(Name = "Jupiter_Belt/Temperate_South"))]
    Jupiter_Belt_Temperate_South,
    #[strum(props(Name = "Jupiter_Zone/Temperate_South"))]
    Jupiter_Zone_Temperate_South,
    // 2nd Outer Planet: Saturn
    // https://www.space.com/saturn-rings-younger-than-we-thought#:~:text=Saturn's%20rings%20are%20surprisingly%20youthful,around%20the%20gas%20giant%20planet.
    // https://en.wikipedia.org/wiki/Rings_of_Saturn
    // https://en.wikipedia.org/wiki/Saturn
    #[strum(props(Name = "Saturn_Ring/E"))]
    Saturn_Ring__E,
    #[strum(props(Name = "Saturn_Ring/D"))]
    Saturn_Ring__D,
    #[strum(props(Name = "Saturn_Ring/G"))]
    Saturn_Ring__G,
    #[strum(props(Name = "Saturn_Ring/C"))]
    Saturn_Ring__C,
    #[strum(props(Name = "Saturn_Ring/B"))]
    Saturn_Ring__B,
    #[strum(props(Name = "Saturn_Ring/A"))]
    Saturn_Ring__A,
    #[strum(props(Name = "Saturn_Ring/F"))]
    Saturn_Ring__F,
    // 3rd Outer Planet: Uranus
    // https://www.lpi.usra.edu/publications/slidesets/ss_tour/slide_33.html
    // https://en.wikipedia.org/wiki/Uranus#:~:text=Banded%20structure%2C%20winds%20and%20clouds,-Voyager%202's%20timelapse&text=In%201986%2C%20Voyager%202%20found,about%20%E2%88%9245%C2%B0%20of%20latitude.
    // https://science.nasa.gov/uranus/facts/#hds-sidebar-nav-10
    #[strum(props(Name = "Uranus_Ring/Zeta"))]
    Uranus_Ring__Zeta,
    #[strum(props(Name = "Uranus_Ring/6"))]
    Uranus_Ring__6,
    #[strum(props(Name = "Uranus_Ring/5"))]
    Uranus_Ring__5,
    #[strum(props(Name = "Uranus_Ring/4"))]
    Uranus_Ring__4,
    #[strum(props(Name = "Uranus_Ring/Alpha"))]
    Uranus_Ring__Alpha,
    #[strum(props(Name = "Uranus_Ring/Beta"))]
    Uranus_Ring__Beta,
    #[strum(props(Name = "Uranus_Ring/Eta"))]
    Uranus_Ring__Eta,
    #[strum(props(Name = "Uranus_Ring/Gamma"))]
    Uranus_Ring__Gamma,
    #[strum(props(Name = "Uranus_Ring/Delta"))]
    Uranus_Ring__Delta,
    #[strum(props(Name = "Uranus_Ring/Epsilon"))]
    Uranus_Ring__Epsilon,
    #[strum(props(Name = "Uranus_Ring/Nu"))]
    Uranus_Ring__Nu,
    #[strum(props(Name = "Uranus_Ring/Mu"))]
    Uranus_Ring__Mu,
    #[strum(props(Name = "Uranus_Cap/Bright_Polar"))]
    Uranus_Cap__Bright_Polar,
    #[strum(props(Name = "Uranus_Band/Dark_Equatorial"))]
    Uranus_Band__Dark_Equatorial,
    // 4th Outer Planet: Neptune
    #[strum(props(Name = "Neptune/South_Polar"))]
    Neptune__South_Polar,
    #[strum(props(Name = "Neptune/Dark_Spot2"))]
    Neptune__Dark_Spot2,
    // Jupiter Moons
    #[strum(props(Name = "Jupiter_Moon/Io"))]
    Jupiter_Moon__Io,
    #[strum(props(Name = "Jupiter_Moon/Europa"))]
    Jupiter_Moon__Europa,
    #[strum(props(Name = "Jupiter_Moon/Ganymede"))]
    Jupiter_Moon__Ganymede,
    #[strum(props(Name = "Jupiter_Moon/Callisto"))]
    Jupiter_Moon__Callisto,
    // Saturn Moons
    #[strum(props(Name = "Saturn_Moon/Titan"))]
    Saturn_Moon_Titan,
    #[strum(props(Name = "Saturn_Moon/Mimas"))]
    Saturn_Moon_Mimas,
    #[strum(props(Name = "Saturn_Moon/Enceladus"))]
    Saturn_Moon_Enceladus,
    #[strum(props(Name = "Saturn_Moon/Tethys"))]
    Saturn_Moon_Tethys,
    #[strum(props(Name = "Saturn_Moon/Dione"))]
    Saturn_Moon_Dione,
    #[strum(props(Name = "Saturn_Moon/Rhea"))]
    Saturn_Moon_Rhea,
    // Uranus Moons
    #[strum(props(Name = "Uranus_Moon/Miranda"))]
    Uranus_Moon_Miranda,
    #[strum(props(Name = "Uranus_Moon/Ariel"))]
    Uranus_Moon_Ariel,
    #[strum(props(Name = "Uranus_Moon/Umbriel"))]
    Uranus_Moon_Umbriel,
    #[strum(props(Name = "Uranus_Moon/Titania"))]
    Uranus_Moon_Titania,
    #[strum(props(Name = "Uranus_Moon/Oberon"))]
    Uranus_Moon_Oberon,
    // Neptune Moons
    #[strum(props(Name = "Neptune_Moon/Triton"))]
    Neptune_Moon__Triton,
    #[strum(props(Name = "Neptune_Moon/Galatea"))]
    Neptune_Moon_Galatea,
    #[strum(props(Name = "Neptune_Moon/Naiad"))]
    Neptune_Moon_Naiad,
    #[strum(props(Name = "Neptune_Moon/Thalassa"))]
    Neptune_Moon_Thalassa,
    #[strum(props(Name = "Neptune_Moon/Despina"))]
    Neptune_Moon_Despina,
    #[strum(props(Name = "Neptune_Moon/Larissa"))]
    Neptune_Moon_Larissa,
    #[strum(props(Name = "Neptune_Moon/Proteus"))]
    Neptune_Moon_Proteus,
    // 1st Largest Asteroid: Ceres
    // https://en.wikipedia.org/wiki/List_of_geological_features_on_Ceres
    #[strum(props(Name = "Ceres_Catenae/Baltay_Catena"))]
    Ceres_Catenae_Baltay_Catena,
    #[strum(props(Name = "Ceres_Catenae/Gerber_Catena"))]
    Ceres_Catenae_Gerber_Catena,
    #[strum(props(Name = "Ceres_Catenae/Junina_Catenae"))]
    Ceres_Catenae_Junina_Catenae,
    #[strum(props(Name = "Ceres_Catenae/Pongal_Catena"))]
    Ceres_Catenae_Pongal_Catena,
    #[strum(props(Name = "Ceres_Catenae/Samhain_Catenae"))]
    Ceres_Catenae_Samhain_Catenae,
    #[strum(props(Name = "Ceres_Catenae/Uhola_Catenae"))]
    Ceres_Catenae_Uhola_Catenae,
    #[strum(props(Name = "Ceres_Fossae/Nabanna_Fossa"))]
    Ceres_Fossae_Nabanna_Fossa,
    #[strum(props(Name = "Ceres_Labes/Dankdag_Labes"))]
    Ceres_Labes_Dankdag_Labes,
    #[strum(props(Name = "Ceres_Labes/Onam_Labes"))]
    Ceres_Labes_Onam_Labes,
    #[strum(props(Name = "Ceres_Labes/Sukkot_Labes"))]
    Ceres_Labes_Sukkot_Labes,
    #[strum(props(Name = "Ceres_Labyrinthus/Makahiki_Labyrinthus"))]
    Ceres_Labyrinthus_Makahiki_Labyrinthus,
    #[strum(props(Name = "Ceres_Montes/Ahuna_Mons"))]
    Ceres_Montes_Ahuna_Mons,
    #[strum(props(Name = "Ceres_Montes/Liberalia_Mons"))]
    Ceres_Montes_Liberalia_Mons,
    #[strum(props(Name = "Ceres_Montes/Yamor_Mons"))]
    Ceres_Montes_Yamor_Mons,
    #[strum(props(Name = "Ceres_Plana/Hanami_Planum"))]
    Ceres_Plana_Hanami_Planum,
    #[strum(props(Name = "Ceres_Planitiae/Vendimia_Planitia"))]
    Ceres_Planitiae_Vendimia_Planitia,
    #[strum(props(Name = "Ceres_Regiones/Homowo_Regio"))]
    Ceres_Regiones_Homowo_Regio,
    #[strum(props(Name = "Ceres_Rupes/Niman_Rupes"))]
    Ceres_Rupes_Niman_Rupes,
    #[strum(props(Name = "Ceres_Sulci/Nar_Sulcus"))]
    Ceres_Sulci_Nar_Sulcus,
    #[strum(props(Name = "Ceres_Tholi/Aymuray_Tholi"))]
    Ceres_Tholi_Aymuray_Tholi,
    #[strum(props(Name = "Ceres_Tholi/Bagach_Tholus"))]
    Ceres_Tholi_Bagach_Tholus,
    #[strum(props(Name = "Ceres_Tholi/Cerealia_Tholus"))]
    Ceres_Tholi_Cerealia_Tholus,
    #[strum(props(Name = "Ceres_Tholi/Cosecha_Tholus"))]
    Ceres_Tholi_Cosecha_Tholus,
    #[strum(props(Name = "Ceres_Tholi/Dalien_Tholus"))]
    Ceres_Tholi_Dalien_Tholus,
    #[strum(props(Name = "Ceres_Tholi/Hosil_Tholus"))]
    Ceres_Tholi_Hosil_Tholus,
    #[strum(props(Name = "Ceres_Tholi/Kekri_Tholus"))]
    Ceres_Tholi_Kekri_Tholus,
    #[strum(props(Name = "Ceres_Tholi/Kwanzaa_Tholus"))]
    Ceres_Tholi_Kwanzaa_Tholus,
    #[strum(props(Name = "Ceres_Tholi/Lohri_Tholus"))]
    Ceres_Tholi_Lohri_Tholus,
    #[strum(props(Name = "Ceres_Tholi/Lughnasa_Tholus"))]
    Ceres_Tholi_Lughnasa_Tholus,
    #[strum(props(Name = "Ceres_Tholi/Mikeli_Tholus"))]
    Ceres_Tholi_Mikeli_Tholus,
    #[strum(props(Name = "Ceres_Tholi/Wangala_Tholus"))]
    Ceres_Tholi_Wangala_Tholus,
    #[strum(props(Name = "Ceres_Faculae/Haulani_Facula"))]
    Ceres_Faculae_Haulani_Facula,
    #[strum(props(Name = "Ceres_Faculae/Dantu_Faculae"))]
    Ceres_Faculae_Dantu_Faculae,
    #[strum(props(Name = "Ceres_Faculae/Kupalo_Facula"))]
    Ceres_Faculae_Kupalo_Facula,
    #[strum(props(Name = "Ceres_Craters/Abellio"))]
    Ceres_Craters_Abellio,
    #[strum(props(Name = "Ceres_Craters/Achita"))]
    Ceres_Craters_Achita,
    #[strum(props(Name = "Ceres_Craters/Annona"))]
    Ceres_Craters_Annona,
    #[strum(props(Name = "Ceres_Craters/Anura"))]
    Ceres_Craters_Anura,
    #[strum(props(Name = "Ceres_Craters/Aristaeus"))]
    Ceres_Craters_Aristaeus,
    #[strum(props(Name = "Ceres_Craters/Asari"))]
    Ceres_Craters_Asari,
    #[strum(props(Name = "Ceres_Craters/Attis"))]
    Ceres_Craters_Attis,
    #[strum(props(Name = "Ceres_Craters/Axomama"))]
    Ceres_Craters_Axomama,
    #[strum(props(Name = "Ceres_Craters/Azacca"))]
    Ceres_Craters_Azacca,
    #[strum(props(Name = "Ceres_Craters/Begbalel"))]
    Ceres_Craters_Begbalel,
    #[strum(props(Name = "Ceres_Craters/Belun"))]
    Ceres_Craters_Belun,
    #[strum(props(Name = "Ceres_Craters/Besua"))]
    Ceres_Craters_Besua,
    #[strum(props(Name = "Ceres_Craters/Bilwis"))]
    Ceres_Craters_Bilwis,
    #[strum(props(Name = "Ceres_Craters/Binayo"))]
    Ceres_Craters_Binayo,
    #[strum(props(Name = "Ceres_Craters/Bonsu"))]
    Ceres_Craters_Bonsu,
    #[strum(props(Name = "Ceres_Craters/Braciaca"))]
    Ceres_Craters_Braciaca,
    #[strum(props(Name = "Ceres_Craters/Cacaguat"))]
    Ceres_Craters_Cacaguat,
    #[strum(props(Name = "Ceres_Craters/Centeotl"))]
    Ceres_Craters_Centeotl,
    #[strum(props(Name = "Ceres_Craters/Chaminuka"))]
    Ceres_Craters_Chaminuka,
    #[strum(props(Name = "Ceres_Craters/Coniraya"))]
    Ceres_Craters_Coniraya,
    #[strum(props(Name = "Ceres_Craters/Consus"))]
    Ceres_Craters_Consus,
    #[strum(props(Name = "Ceres_Craters/Cozobi"))]
    Ceres_Craters_Cozobi,
    #[strum(props(Name = "Ceres_Craters/Dada"))]
    Ceres_Craters_Dada,
    #[strum(props(Name = "Ceres_Craters/Damia"))]
    Ceres_Craters_Damia,
    #[strum(props(Name = "Ceres_Craters/Dantu"))]
    Ceres_Craters_Dantu,
    #[strum(props(Name = "Ceres_Craters/Darzamat"))]
    Ceres_Craters_Darzamat,
    #[strum(props(Name = "Ceres_Craters/Datan"))]
    Ceres_Craters_Datan,
    #[strum(props(Name = "Ceres_Craters_Dikhan"))]
    Ceres_Craters_Dikhan,
    #[strum(props(Name = "Ceres_Craters/Doliku"))]
    Ceres_Craters_Doliku,
    #[strum(props(Name = "Ceres_Craters/Duginavi"))]
    Ceres_Craters_Duginavi,
    #[strum(props(Name = "Ceres_Craters/Emesh"))]
    Ceres_Craters_Emesh,
    #[strum(props(Name = "Ceres_Craters/Enkimdu"))]
    Ceres_Craters_Enkimdu,
    #[strum(props(Name = "Ceres_Craters/Ernutet"))]
    Ceres_Craters_Ernutet,
    #[strum(props(Name = "Ceres_Craters/Ezinu"))]
    Ceres_Craters_Ezinu,
    #[strum(props(Name = "Ceres_Craters/Fejokoo"))]
    Ceres_Craters_Fejokoo,
    #[strum(props(Name = "Ceres_Craters/Fluusa"))]
    Ceres_Craters_Fluusa,
    #[strum(props(Name = "Ceres_Craters/Gaue"))]
    Ceres_Craters_Gaue,
    #[strum(props(Name = "Ceres_Craters/Geshtin"))]
    Ceres_Craters_Geshtin,
    #[strum(props(Name = "Ceres_Craters/Ghanan"))]
    Ceres_Craters_Ghanan,
    #[strum(props(Name = "Ceres_Craters/Hakumyi"))]
    Ceres_Craters_Hakumyi,
    #[strum(props(Name = "Ceres_Craters/Hamori"))]
    Ceres_Craters_Hamori,
    #[strum(props(Name = "Ceres_Craters/Hatipowa"))]
    Ceres_Craters_Hatipowa,
    #[strum(props(Name = "Ceres_Craters/Haulani"))]
    Ceres_Craters_Haulani,
    #[strum(props(Name = "Ceres_Craters/Heneb"))]
    Ceres_Craters_Heneb,
    #[strum(props(Name = "Ceres_Craters/Homshuk"))]
    Ceres_Craters_Homshuk,
    #[strum(props(Name = "Ceres_Craters/Ialonus"))]
    Ceres_Craters_Ialonus,
    #[strum(props(Name = "Ceres_Craters/Ikapati"))]
    Ceres_Craters_Ikapati,
    #[strum(props(Name = "Ceres_Craters/Inamahari"))]
    Ceres_Craters_Inamahari,
    #[strum(props(Name = "Ceres_Craters/Inkosazana"))]
    Ceres_Craters_Inkosazana,
    #[strum(props(Name = "Ceres_Craters/Insitor"))]
    Ceres_Craters_Insitor,
    #[strum(props(Name = "Ceres_Craters/Jacheongbi"))]
    Ceres_Craters_Jacheongbi,
    #[strum(props(Name = "Ceres_Craters/Jaja"))]
    Ceres_Craters_Jaja,
    #[strum(props(Name = "Ceres_Craters/Jarimba"))]
    Ceres_Craters_Jarimba,
    #[strum(props(Name = "Ceres_Craters/Jarovit"))]
    Ceres_Craters_Jarovit,
    #[strum(props(Name = "Ceres_Craters/Juling"))]
    Ceres_Craters_Juling,
    #[strum(props(Name = "Ceres_Craters/Jumis"))]
    Ceres_Craters_Jumis,
    #[strum(props(Name = "Ceres_Craters/Kahukura"))]
    Ceres_Craters_Kahukura,
    #[strum(props(Name = "Ceres_Craters/Kaikara"))]
    Ceres_Craters_Kaikara,
    #[strum(props(Name = "Ceres_Craters/Kait"))]
    Ceres_Craters_Kait,
    #[strum(props(Name = "Ceres_Craters/Kaneki"))]
    Ceres_Craters_Kaneki,
    #[strum(props(Name = "Ceres_Craters/Kerwan"))]
    Ceres_Craters_Kerwan,
    #[strum(props(Name = "Ceres_Craters/Kiriamma"))]
    Ceres_Craters_Kiriamma,
    #[strum(props(Name = "Ceres_Craters/Kirnis"))]
    Ceres_Craters_Kirnis,
    #[strum(props(Name = "Ceres_Craters/Kokopelli"))]
    Ceres_Craters_Kokopelli,
    #[strum(props(Name = "Ceres_Craters/Kondos"))]
    Ceres_Craters_Kondos,
    #[strum(props(Name = "Ceres_Craters/Kumitoga"))]
    Ceres_Craters_Kumitoga,
    #[strum(props(Name = "Ceres_Craters/Kupalo"))]
    Ceres_Craters_Kupalo,
    #[strum(props(Name = "Ceres_Craters/Laukumate"))]
    Ceres_Craters_Laukumate,
    #[strum(props(Name = "Ceres_Craters/Liber"))]
    Ceres_Craters_Liber,
    #[strum(props(Name = "Ceres_Craters/Lociyo"))]
    Ceres_Craters_Lociyo,
    #[strum(props(Name = "Ceres_Craters/Lono"))]
    Ceres_Craters_Lono,
    #[strum(props(Name = "Ceres_Craters/Meanderi"))]
    Ceres_Craters_Meanderi,
    #[strum(props(Name = "Ceres_Craters/Megwomets"))]
    Ceres_Craters_Megwomets,
    #[strum(props(Name = "Ceres_Craters/Messor"))]
    Ceres_Craters_Messor,
    #[strum(props(Name = "Ceres_Craters/Mlezi"))]
    Ceres_Craters_Mlezi,
    #[strum(props(Name = "Ceres_Craters/Mondamin"))]
    Ceres_Craters_Mondamin,
    #[strum(props(Name = "Ceres_Craters/Nawish"))]
    Ceres_Craters_Nawish,
    #[strum(props(Name = "Ceres_Craters/Nepen"))]
    Ceres_Craters_Nepen,
    #[strum(props(Name = "Ceres_Craters/Ninsar"))]
    Ceres_Craters_Ninsar,
    #[strum(props(Name = "Ceres_Craters/Nunghui"))]
    Ceres_Craters_Nunghui,
    #[strum(props(Name = "Ceres_Craters/Occator"))]
    Ceres_Craters_Occator,
    #[strum(props(Name = "Ceres_Craters/Oltagon"))]
    Ceres_Craters_Oltagon,
    #[strum(props(Name = "Ceres_Craters/Omonga"))]
    Ceres_Craters_Omonga,
    #[strum(props(Name = "Ceres_Craters/Oxo"))]
    Ceres_Craters_Oxo,
    #[strum(props(Name = "Ceres_Craters/Peko"))]
    Ceres_Craters_Peko,
    #[strum(props(Name = "Ceres_Craters/Piuku"))]
    Ceres_Craters_Piuku,
    #[strum(props(Name = "Ceres_Craters/Rao"))]
    Ceres_Craters_Rao,
    #[strum(props(Name = "Ceres_Craters/Ratumaibulu"))]
    Ceres_Craters_Ratumaibulu,
    #[strum(props(Name = "Ceres_Craters/Razeka"))]
    Ceres_Craters_Razeka,
    #[strum(props(Name = "Ceres_Craters/Rongo"))]
    Ceres_Craters_Rongo,
    #[strum(props(Name = "Ceres_Craters/Roskva"))]
    Ceres_Craters_Roskva,
    #[strum(props(Name = "Ceres_Craters/Sekhet"))]
    Ceres_Craters_Sekhet,
    #[strum(props(Name = "Ceres_Craters/Shakaema"))]
    Ceres_Craters_Shakaema,
    #[strum(props(Name = "Ceres_Craters/Shennong"))]
    Ceres_Craters_Shennong,
    #[strum(props(Name = "Ceres_Craters/Sintana"))]
    Ceres_Craters_Sintana,
    #[strum(props(Name = "Ceres_Craters/Tafakula"))]
    Ceres_Craters_Tafakula,
    #[strum(props(Name = "Ceres_Craters/Tahu"))]
    Ceres_Craters_Tahu,
    #[strum(props(Name = "Ceres_Craters/Takel"))]
    Ceres_Craters_Takel,
    #[strum(props(Name = "Ceres_Craters/Tawals"))]
    Ceres_Craters_Tawals,
    #[strum(props(Name = "Ceres_Craters/Telepinu"))]
    Ceres_Craters_Telepinu,
    #[strum(props(Name = "Ceres_Craters/Thrud"))]
    Ceres_Craters_Thrud,
    #[strum(props(Name = "Ceres_Craters/Tibong"))]
    Ceres_Craters_Tibong,
    #[strum(props(Name = "Ceres_Craters/Toharu"))]
    Ceres_Craters_Toharu,
    #[strum(props(Name = "Ceres_Craters/Tupo"))]
    Ceres_Craters_Tupo,
    #[strum(props(Name = "Ceres_Craters/Urvara"))]
    Ceres_Craters_Urvara,
    // Ceres Craters
    #[strum(props(Name = "Ceres_Craters/Victa"))]
    Ceres_Craters_Victa,
    #[strum(props(Name = "Ceres_Craters/Vinotonus"))]
    Ceres_Craters_Vinotonus,
    #[strum(props(Name = "Ceres_Craters/Xevioso"))]
    Ceres_Craters_Xevioso,
    #[strum(props(Name = "Ceres_Craters/Xochipilli"))]
    Ceres_Craters_Xochipilli,
    #[strum(props(Name = "Ceres_Craters/Yalode"))]
    Ceres_Craters_Yalode,
    #[strum(props(Name = "Ceres_Craters/Zadeni"))]
    Ceres_Craters_Zadeni,
    #[strum(props(Name = "Ceres_Craters/Zatik"))]
    Ceres_Craters_Zatik,

    // Vesta Catenae
    #[strum(props(Name = "Vesta_Catenae/Albalonga"))]
    Vesta_Catenae_Albalonga,
    #[strum(props(Name = "Vesta_Catenae/Robigalia"))]
    Vesta_Catenae_Robigalia,

    // Vesta Dorsa
    #[strum(props(Name = "Vesta_Dorsa/Lavinium"))]
    Vesta_Dorsa_Lavinium,
    #[strum(props(Name = "Vesta_Dorsa/Neptunalia"))]
    Vesta_Dorsa_Neptunalia,
    #[strum(props(Name = "Vesta_Dorsa/Parilia"))]
    Vesta_Dorsa_Parilia,

    // Vesta Craters
    #[strum(props(Name = "Vesta_Craters/Aconia"))]
    Vesta_Craters_Aconia,
    #[strum(props(Name = "Vesta_Craters/Aelia"))]
    Vesta_Craters_Aelia,
    #[strum(props(Name = "Vesta_Craters/Africana"))]
    Vesta_Craters_Africana,
    #[strum(props(Name = "Vesta_Craters/Albana"))]
    Vesta_Craters_Albana,
    #[strum(props(Name = "Vesta_Craters/Albia"))]
    Vesta_Craters_Albia,
    #[strum(props(Name = "Vesta_Craters/Alypia"))]
    Vesta_Craters_Alypia,
    #[strum(props(Name = "Vesta_Craters/Angioletta"))]
    Vesta_Craters_Angioletta,
    #[strum(props(Name = "Vesta_Craters/Antonia"))]
    Vesta_Craters_Antonia,
    #[strum(props(Name = "Vesta_Craters/Aquilia"))]
    Vesta_Craters_Aquilia,
    #[strum(props(Name = "Vesta_Craters/Arruntia"))]
    Vesta_Craters_Arruntia,
    #[strum(props(Name = "Vesta_Craters/Bellicia"))]
    Vesta_Craters_Bellicia,
    #[strum(props(Name = "Vesta_Craters/Bruttia"))]
    Vesta_Craters_Bruttia,
    #[strum(props(Name = "Vesta_Craters/Caesonia"))]
    Vesta_Craters_Caesonia,
    #[strum(props(Name = "Vesta_Craters/Calpurnia"))]
    Vesta_Craters_Calpurnia,
    #[strum(props(Name = "Vesta_Craters/Cannutia"))]
    Vesta_Craters_Cannutia,
    #[strum(props(Name = "Vesta_Craters/Canuleia"))]
    Vesta_Craters_Canuleia,
    #[strum(props(Name = "Vesta_Craters/Caparronia"))]
    Vesta_Craters_Caparronia,
    #[strum(props(Name = "Vesta_Craters/Charito"))]
    Vesta_Craters_Charito,
    #[strum(props(Name = "Vesta_Craters/Claudia"))]
    Vesta_Craters_Claudia,
    #[strum(props(Name = "Vesta_Craters/Coelia"))]
    Vesta_Craters_Coelia,
    #[strum(props(Name = "Vesta_Craters/Cornelia"))]
    Vesta_Craters_Cornelia,
    #[strum(props(Name = "Vesta_Craters/Cossinia"))]
    Vesta_Craters_Cossinia,
    #[strum(props(Name = "Vesta_Craters/Domitia"))]
    Vesta_Craters_Domitia,
    #[strum(props(Name = "Vesta_Craters/Domna"))]
    Vesta_Craters_Domna,
    #[strum(props(Name = "Vesta_Craters/Drusilla"))]
    Vesta_Craters_Drusilla,
    #[strum(props(Name = "Vesta_Craters/Eumachia"))]
    Vesta_Craters_Eumachia,
    #[strum(props(Name = "Vesta_Craters/Eusebia"))]
    Vesta_Craters_Eusebia,
    #[strum(props(Name = "Vesta_Craters/Eutropia"))]
    Vesta_Craters_Eutropia,
    #[strum(props(Name = "Vesta_Craters/Fabia"))]
    Vesta_Craters_Fabia,
    #[strum(props(Name = "Vesta_Craters/Fausta"))]
    Vesta_Craters_Fausta,
    #[strum(props(Name = "Vesta_Craters/Flavola"))]
    Vesta_Craters_Flavola,
    #[strum(props(Name = "Vesta_Craters/Floronia"))]
    Vesta_Craters_Floronia,
    #[strum(props(Name = "Vesta_Craters/Fonteia"))]
    Vesta_Craters_Fonteia,
    #[strum(props(Name = "Vesta_Craters/Fulvia"))]
    Vesta_Craters_Fulvia,
    #[strum(props(Name = "Vesta_Craters/Fundania"))]
    Vesta_Craters_Fundania,
    #[strum(props(Name = "Vesta_Craters/Galeria"))]
    Vesta_Craters_Galeria,
    #[strum(props(Name = "Vesta_Craters/Gegania"))]
    Vesta_Craters_Gegania,
    #[strum(props(Name = "Vesta_Craters/Graecina"))]
    Vesta_Craters_Graecina,
    #[strum(props(Name = "Vesta_Craters/Helena"))]
    Vesta_Craters_Helena,
    #[strum(props(Name = "Vesta_Craters/Herennia"))]
    Vesta_Craters_Herennia,
    #[strum(props(Name = "Vesta_Craters/Hortensia"))]
    Vesta_Craters_Hortensia,
    #[strum(props(Name = "Vesta_Craters/Iunia"))]
    Vesta_Craters_Iunia,
    #[strum(props(Name = "Vesta_Craters/Justina"))]
    Vesta_Craters_Justina,
    #[strum(props(Name = "Vesta_Craters/Laelia"))]
    Vesta_Craters_Laelia,
    #[strum(props(Name = "Vesta_Craters/Laeta"))]
    Vesta_Craters_Laeta,
    #[strum(props(Name = "Vesta_Craters/Laurentia"))]
    Vesta_Craters_Laurentia,
    #[strum(props(Name = "Vesta_Craters/Lepida"))]
    Vesta_Craters_Lepida,
    #[strum(props(Name = "Vesta_Craters/Licinia"))]
    Vesta_Craters_Licinia,
    #[strum(props(Name = "Vesta_Craters/Lollia"))]
    Vesta_Craters_Lollia,
    #[strum(props(Name = "Vesta_Craters/Longina"))]
    Vesta_Craters_Longina,
    #[strum(props(Name = "Vesta_Craters/Lucilla"))]
    Vesta_Craters_Lucilla,
    #[strum(props(Name = "Vesta_Craters/Mamilia"))]
    Vesta_Craters_Mamilia,
    #[strum(props(Name = "Vesta_Craters/Marcia"))]
    Vesta_Craters_Marcia,
    #[strum(props(Name = "Vesta_Craters/Mariamne"))]
    Vesta_Craters_Mariamne,
    #[strum(props(Name = "Vesta_Craters/Metrodora"))]
    Vesta_Craters_Metrodora,
    #[strum(props(Name = "Vesta_Craters/Minervina"))]
    Vesta_Craters_Minervina,
    #[strum(props(Name = "Vesta_Craters/Minucia"))]
    Vesta_Craters_Minucia,
    #[strum(props(Name = "Vesta_Craters/Myia"))]
    Vesta_Craters_Myia,
    #[strum(props(Name = "Vesta_Craters/Numisia"))]
    Vesta_Craters_Numisia,
    #[strum(props(Name = "Vesta_Craters/Occia"))]
    Vesta_Craters_Occia,
    #[strum(props(Name = "Vesta_Craters/Octavia"))]
    Vesta_Craters_Octavia,
    #[strum(props(Name = "Vesta_Craters/Oppia"))]
    Vesta_Craters_Oppia,
    #[strum(props(Name = "Vesta_Craters/Paculla"))]
    Vesta_Craters_Paculla,
    #[strum(props(Name = "Vesta_Craters/Paulina"))]
    Vesta_Craters_Paulina,
    #[strum(props(Name = "Vesta_Craters/Perpennia"))]
    Vesta_Craters_Perpennia,
    #[strum(props(Name = "Vesta_Craters/Pinaria"))]
    Vesta_Craters_Pinaria,
    #[strum(props(Name = "Vesta_Craters/Placidia"))]
    Vesta_Craters_Placidia,
    #[strum(props(Name = "Vesta_Craters/Plancia"))]
    Vesta_Craters_Plancia,
    #[strum(props(Name = "Vesta_Craters/Pomponia"))]
    Vesta_Craters_Pomponia,
    #[strum(props(Name = "Vesta_Craters/Portia"))]
    Vesta_Craters_Portia,
    #[strum(props(Name = "Vesta_Craters/Postumia"))]
    Vesta_Craters_Postumia,
    #[strum(props(Name = "Vesta_Craters/Publicia"))]
    Vesta_Craters_Publicia,
    #[strum(props(Name = "Vesta_Craters/Rheasilvia"))]
    Vesta_Craters_Rheasilvia,
    #[strum(props(Name = "Vesta_Craters/Rubria"))]
    Vesta_Craters_Rubria,
    #[strum(props(Name = "Vesta_Craters/Rufillia"))]
    Vesta_Craters_Rufillia,
    #[strum(props(Name = "Vesta_Craters/Scantia"))]
    Vesta_Craters_Scantia,
    #[strum(props(Name = "Vesta_Craters/Sentia"))]
    Vesta_Craters_Sentia,
    #[strum(props(Name = "Vesta_Craters/Serena"))]
    Vesta_Craters_Serena,
    #[strum(props(Name = "Vesta_Craters/Severina"))]
    Vesta_Craters_Severina,
    #[strum(props(Name = "Vesta_Craters/Sextilia"))]
    Vesta_Craters_Sextilia,
    #[strum(props(Name = "Vesta_Craters/Sossia"))]
    Vesta_Craters_Sossia,
    #[strum(props(Name = "Vesta_Craters/Tarpeia"))]
    Vesta_Craters_Tarpeia,
    #[strum(props(Name = "Vesta_Craters/Teia"))]
    Vesta_Craters_Teia,
    #[strum(props(Name = "Vesta_Craters/Torquata"))]
    Vesta_Craters_Torquata,
    #[strum(props(Name = "Vesta_Craters/Tuccia"))]
    Vesta_Craters_Tuccia,
    #[strum(props(Name = "Vesta_Craters/Urbinia"))]
    Vesta_Craters_Urbinia,
    #[strum(props(Name = "Vesta_Craters/Varronilla"))]
    Vesta_Craters_Varronilla,
    #[strum(props(Name = "Vesta_Craters/Veneneia"))]
    Vesta_Craters_Veneneia,
    #[strum(props(Name = "Vesta_Craters/Vettenia"))]
    Vesta_Craters_Vettenia,
    #[strum(props(Name = "Vesta_Craters/Vibidia"))]
    Vesta_Craters_Vibidia,
    // Vesta Fossae
    #[strum(props(Name = "Vesta_Fossae/Divalia"))]
    Vesta_Fossae_Divalia,
    // Vesta Fossae
    #[strum(props(Name = "Vesta_Fossae/Lupercalia"))]
    Vesta_Fossae_Lupercalia,
    #[strum(props(Name = "Vesta_Fossae/Saturnalia"))]
    Vesta_Fossae_Saturnalia,

    // Vesta Planitiae
    #[strum(props(Name = "Vesta_Planitiae/Feralia"))]
    Vesta_Planitiae_Feralia,
    // Vesta Rupes
    #[strum(props(Name = "Vesta_Rupes/Agonium"))]
    Vesta_Rupes_Agonium,
    #[strum(props(Name = "Vesta_Rupes/Matronalia"))]
    Vesta_Rupes_Matronalia,
    #[strum(props(Name = "Vesta_Rupes/Parentatio"))]
    Vesta_Rupes_Parentatio,
    // Vesta Terrae
    #[strum(props(Name = "Vesta_Terrae/Vestalia"))]
    Vesta_Terrae_Vestalia,
    // Vesta Tholi
    #[strum(props(Name = "Vesta_Tholi/Aricia"))]
    Vesta_Tholi_Aricia,
    #[strum(props(Name = "Vesta_Tholi/Brumalia"))]
    Vesta_Tholi_Brumalia,
    #[strum(props(Name = "Vesta_Tholi/Lucaria"))]
    Vesta_Tholi_Lucaria,
    // Pallas Craters (Southern Hemisphere)
    #[strum(props(Name = "Pallas_Crater/Akontia"))]
    Pallas_Crater_Akontia,
    #[strum(props(Name = "Pallas_Crater/Doru"))]
    Pallas_Crater_Doru,
    #[strum(props(Name = "Pallas_Crater/Hoplon"))]
    Pallas_Crater_Hoplon,
    #[strum(props(Name = "Pallas_Crater/Kopis"))]
    Pallas_Crater_Kopis,
    #[strum(props(Name = "Pallas_Crater/Sarissa"))]
    Pallas_Crater_Sarissa,
    #[strum(props(Name = "Pallas_Crater/Sfendonai"))]
    Pallas_Crater_Sfendonai,
    #[strum(props(Name = "Pallas_Crater/Toxa"))]
    Pallas_Crater_Toxa,
    #[strum(props(Name = "Pallas_Crater/Xiphos"))]
    Pallas_Crater_Xiphos,
    #[strum(props(Name = "Pallas_Crater/Xyston"))]
    Pallas_Crater_Xyston,
    // Pallas Craters (Northern Hemisphere)
    #[strum(props(Name = "Pallas_Crater/Aklys"))]
    Pallas_Crater_Aklys,
    #[strum(props(Name = "Pallas_Crater/Falcata"))]
    Pallas_Crater_Falcata,
    #[strum(props(Name = "Pallas_Crater/Makhaira"))]
    Pallas_Crater_Makhaira,
    #[strum(props(Name = "Pallas_Crater/Pilum"))]
    Pallas_Crater_Pilum,
    #[strum(props(Name = "Pallas_Crater/Scutum"))]
    Pallas_Crater_Scutum,
    #[strum(props(Name = "Pallas_Crater/Sica"))]
    Pallas_Crater_Sica,
    #[strum(props(Name = "Pallas_Crater/Spatha"))]
    Pallas_Crater_Spatha,
    // 4th Largest Asteroid: Hygiea
    // https://en.wikipedia.org/wiki/10_Hygiea
    // https://www.syfy.com/syfy-wire/new-observations-show-the-asteroid-hygiea-is-round
    #[strum(props(Name = "Hygiea/Calix"))]
    Hygiea_Calix,
    #[strum(props(Name = "Hygiea/Serpens"))]
    Hygiea_Serpens,
}
