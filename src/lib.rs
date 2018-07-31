extern crate rand;

use rand::Rng;
use std::collections::HashMap;

/// Mapping of international phonetics to characters
struct Orthography {
    // String or Vec<Char>?
    orthography: HashMap<char, String>,
}

impl Orthography {
    fn new() -> Orthography {
        let mut orth = Orthography{orthography: HashMap::new()};
        orth.with_mapping(DEFAULT_MAP)
    }

    fn with_mapping(mut self, mapping: &[(char, &str)]) -> Orthography {
        for (ch, map) in mapping {
            self.orthography.insert(*ch, map.to_string());
        }
        self
    }

    // TODO: figure out how to avoid the cloning
    fn get_mapping(&self, ch: &char) -> String {
        self.orthography.get(ch).map(|s| s.to_string()).unwrap_or(ch.to_string())
    }
}

const DEFAULT_MAP: &[(char, &str)] = &[('ʃ', "sh"), ('ʒ', "zh"), ('ʧ', "ch"), ('ʤ', "j"),
                                       ('ŋ', "ng"), ('j', "y"), ('x', "kh"), ('ɣ', "gh"),
                                       ('ʔ', "‘"), ('A', "á"), ('E', "é"), ('I', "í"),
                                       ('O', "ó"), ('U', "ú")];
const NULL_MAP: &[(char, &str)] = &[];

// Consonant mappings
const SLAVIC_MAP: &[(char, &str)] = &[('ʃ', "š"), ('ʒ', "ž"), ('ʧ', "č"), ('ʤ', "ǧ"), ('j', "j")];
const GERMAN_MAP: &[(char, &str)] = &[('ʃ', "sch"), ('ʒ', "zh"), ('ʧ', "tsch"),
                                      ('ʤ', "dz"), ('j', "j"), ('x', "ch")];
const FRENCH_MAP: &[(char, &str)] = &[('ʃ', "ch"), ('ʒ', "j"), ('ʧ', "tch"),
                                      ('ʤ', "dj"), ('x', "kh")];
// Chinese (pinyin)
const PINYIN_MAP: &[(char, &str)] = &[('ʃ', "x"), ('ʧ', "q"), ('ʤ', "j")];

const CONSONANT_MAPPINGS: &[&[(char, &str)]] = &[NULL_MAP, SLAVIC_MAP, GERMAN_MAP, FRENCH_MAP,
                                                 PINYIN_MAP];

// Vowel Mappings
const UMLAUTS_MAP: &[(char, &str)] = &[('A', "ä"), ('E', "ë"), ('I', "ï"), ('O', "ö"), ('U', "ü")];
const WELSH_MAP: &[(char, &str)] = &[('A', "â"), ('E', "ê"), ('I', "y"), ('O', "ô"), ('U', "w")];
const DIPHTHONGS_MAP: &[(char, &str)] = &[('A', "au"), ('E', "ei"), ('I', "ie"),
                                          ('O', "ou"), ('U', "oo")];
const DOUBLES_MAP: &[(char, &str)] = &[('A', "qq"), ('E', "qq"), ('I', "ii"),
                                       ('O', "oo"), ('U', "uu")];

const VOWEL_MAPPINGS: &[&[(char, &str)]] = &[NULL_MAP, UMLAUTS_MAP, WELSH_MAP, DIPHTHONGS_MAP,
                                             DOUBLES_MAP];

const SYLLABLESTRUCT: &[&str] = &["CVC",
    "CVvC",
    "CVVc", "CVc", "CV", "VC", "CVF", "cVC", "CVf",
    "ClVC", "ClVF", "sCVC", "sCVF", "sCVc",
    "cVF", "cVc", "cVf", "clVC", "VC",
    "CVlc", "cVlC", "cVLc"];

// Consonant characters sets (international phonetic alphabet)
const MINIMAL: &str = "ptkmnls";
const ENGLISH_ISH: &str = "ptkbdgmnlrsʃzʒʧ";
const PIRAHA: &str = "ptkmnh"; // Very simple
const HAWAIIAN_ISH: &str = "hklmnpwʔ";
const GREENLANDIC_ISH: &str = "ptkqvsgrmnŋlj";
const ARABIC_ISH: &str = "tksʃdbqɣxmnlrwj";
const ARABIC_LITE: &str = "tkdgmnsʃ";
const ENGLISH_LITE: &str = "ptkbdgmnszʒʧhjw";

const CONSONANT_SETS: &[&str] = &[MINIMAL, ENGLISH_ISH, PIRAHA, HAWAIIAN_ISH,
                                  GREENLANDIC_ISH, ARABIC_ISH, ARABIC_LITE, ENGLISH_LITE];

// Sibilants
const S: &str = "s";
const SSH: &str = "sʃ";
const SSHF: &str = "sʃf";

const SIBILANT_SETS: &[&str] = &[S, SSH, SSHF];

// Glides
const RL: &str = "rl";
const R: &str = "r";
const L: &str = "l";
const WJ: &str = "wj";
const RLWJ: &str = "rlwj";

const GLIDE_SETS: &[&str] = &[RL, R, L, WJ, RLWJ];

// Endings
const MN: &str = "mn";
const SK: &str = "sk";
const MNNG: &str = "mnŋ";
const SSHZZH: &str = "sʃzʒ";

const ENDING_SETS: &[&str] = &[MN, SK, MNNG, SSHZZH];

// Vowels
const FIVE: &str = "aeiou";
const THREE: &str = "aiu";
const FIVE_AEI: &str = "aeiouAEI";
const FIVE_U: &str = "aeiouU";
const THREE_AI: &str = "aiuAI";
const ALT_THREE: &str = "eou";
const FIVE_AOU: &str = "aeiouAOU";

const VOWEL_SETS: &[&str] = &[FIVE, THREE, FIVE_AEI, FIVE_U, THREE_AI, ALT_THREE, FIVE_AOU];

struct Phonemes {
    consonants: Vec<char>,
    vowels: Vec<char>,
    sibilants: Vec<char>,
    glides: Vec<char>,
    endings: Vec<char>,
}

impl Phonemes {
    fn from_rand<R: Rng>(rng: &mut R) -> Phonemes {
        // none of the unwraps in this method should ever fail
        let mut consonants: Vec<char> = rng.choose(CONSONANT_SETS).unwrap().chars().collect();
        rng.shuffle(&mut consonants);
        let mut vowels: Vec<char> = rng.choose(VOWEL_SETS).unwrap().chars().collect();
        rng.shuffle(&mut vowels);
        let mut sibilants: Vec<char> = rng.choose(SIBILANT_SETS).unwrap().chars().collect();
        rng.shuffle(&mut sibilants);
        let mut glides: Vec<char> = rng.choose(GLIDE_SETS).unwrap().chars().collect();
        rng.shuffle(&mut glides);
        let mut endings: Vec<char> = rng.choose(ENDING_SETS).unwrap().chars().collect();
        rng.shuffle(&mut endings);
        Phonemes{consonants, vowels, sibilants, glides, endings}
    }
}

struct Language {
    phonemes: Phonemes,
    orthography: Orthography,
    syllable: Vec<char>,
    min_syllable: u8,
    max_syllable: u8,
    genitive: Option<String>, // "of"
    definite: Option<String>, // "the"
}

impl Language {
    fn from_rand<R: Rng>(rng: &mut R) -> Language {
        // none of the unwraps in this method should ever fail
        let phonemes = Phonemes::from_rand(rng);
        let c_map = rng.choose(CONSONANT_MAPPINGS).unwrap();
        let v_map = rng.choose(VOWEL_MAPPINGS).unwrap();
        let orthography = Orthography::new().with_mapping(c_map).with_mapping(v_map);
        let syllable: Vec<char> = rng.choose(SYLLABLESTRUCT).unwrap().chars().collect();
        let min_syllable = if syllable.len() < 3 {
            rng.gen_range(1u8, 3u8) + 1
        } else {
            rng.gen_range(1u8, 3u8)
        };
        let max_syllable = rng.gen_range(min_syllable + 1, 7u8);
        Language{phonemes, orthography, syllable, min_syllable, max_syllable,
                 genitive: None, definite: None}
        // TODO: generate genitive and definite
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
