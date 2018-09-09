extern crate rand;

use rand::{Rng, SeedableRng, thread_rng};
use rand::prng::XorShiftRng;
use std::collections::HashMap;

/// Mapping of international phonetics to characters
#[derive(Debug, Clone)]
struct Orthography {
    // String or Vec<Char>?
    orthography: HashMap<char, String>,
}

impl Orthography {
    fn new() -> Orthography {
        let orth = Orthography{orthography: HashMap::new()};
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

    fn transform(&self, word: &str) -> String {
        word.chars().map(|ref ch| self.get_mapping(ch)).collect::<Vec<_>>().join("")
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
    "CVVc", "CVc", "CV", "VC", "CVE", "cVC", "CVe",
    "CgVC", "CgVE", "sCVC", "sCVE", "sCVc",
    "cVE", "cVc", "cVe", "cgVC", "VC",
    "CVgc", "cVgC", "cVGc"];

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

/// This probably breaks for some languages/alphabets, but for generating random
/// languages, ought to be fine. By Definition, really.
fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().to_string() + c.as_str(),
    }
}

fn weighted_char<R: Rng>(rng: &mut R, chars: &Vec<char>) -> char {
    // random select a character, weighted towards the beginning of the list
    // it should be impossible for the unwrap to panic
    *(chars.get((rng.gen::<f32>().powf(2.0) * (chars.len() as f32)).floor() as usize).unwrap())
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct BaseLanguage {
    phonemes: Phonemes,
    orthography: Orthography,
    syllable: Vec<char>,
    min_syllable: u8,
    max_syllable: u8,
}

impl BaseLanguage {
    fn make_syllable<R: Rng>(&self, rng: &mut R) -> String {
        self.syllable.iter().fold(String::new(), |mut acc, ch_type| {
            match ch_type {
                'C' => acc.push(weighted_char(rng, &self.phonemes.consonants)),
                'c' => if rng.gen::<bool>() {
                    acc.push(weighted_char(rng, &self.phonemes.consonants));
                },
                'V' => acc.push(weighted_char(rng, &self.phonemes.vowels)),
                'v' => if rng.gen::<bool>() {
                    acc.push(weighted_char(rng, &self.phonemes.vowels));
                },
                'E' => acc.push(weighted_char(rng, &self.phonemes.endings)),
                'e' => if rng.gen::<bool>() {
                    acc.push(weighted_char(rng, &self.phonemes.endings));
                },
                'G' => acc.push(weighted_char(rng, &self.phonemes.glides)),
                'g' => if rng.gen::<bool>() {
                    acc.push(weighted_char(rng, &self.phonemes.glides));
                },
                'S' => acc.push(weighted_char(rng, &self.phonemes.sibilants)),
                's' => if rng.gen::<bool>() {
                    acc.push(weighted_char(rng, &self.phonemes.sibilants));
                },
                _ => {
                    // this should never happen
                    panic!("Internal bug in namegen");
                }
            };
            acc
        })
    }

    fn make_word<R: Rng>(&self, rng: &mut R, morphemes: Option<&Vec<String>>) -> String {
        // Use a decaying distribution for syllable count. Short words
        // should be more common than long ones.
        let min = self.min_syllable as f32;
        let delta = (self.max_syllable + 1 - self.min_syllable) as f32; // we are using an inclusive range
        let count = (rng.gen::<f32>().powf(2.0)*delta + min).floor() as u8;
        let ipa_word = (0u8..count).
            map(|_| if let Some(ref morph) = morphemes {
                if rng.gen::<f32>() < 0.25 {
                    // weighted towards the beginning of the list
                    let index = (rng.gen::<f32>().powf(2.0) * (morph.len() as f32)).floor() as usize;
                    morph.get(index).unwrap().clone()
                } else {
                    self.make_syllable(rng)
                }
            } else {
                self.make_syllable(rng)
            }).
            collect::<Vec<_>>().join("");
        self.orthography.transform(&ipa_word)
    }
}

#[derive(Debug, Clone)]
pub struct Language {
    base: BaseLanguage,
    genitive: String, // "of"
    definite: String, // "the"
    // morphemes
    place: Vec<String>,
    region: Vec<String>,
    person: Vec<String>,
    titles: Vec<String>,
    surname_last: bool,
}

impl Language {
    /// Creates a random language from the provide random number generator.
    pub fn from_rng<R: Rng>(rng: &mut R) -> Language {
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
        let base = BaseLanguage{phonemes, orthography, syllable, min_syllable, max_syllable};
        // TODO: generate genitive and definite
        let genitive = base.orthography.transform(&base.make_syllable(rng));
        let definite = base.orthography.transform(&base.make_syllable(rng));
        let place = (0u8..rng.gen_range(5, 10)).map(|_| base.make_syllable(rng)).collect();
        let region = (0u8..rng.gen_range(5, 10)).map(|_| base.make_syllable(rng)).collect();
        let person = (0u8..rng.gen_range(7, 14)).map(|_| base.make_syllable(rng)).collect();
        fn make_title<R: Rng>(rng: &mut R, base: &BaseLanguage) -> String {
            let title = base.orthography.transform(&base.make_syllable(rng));
            if rng.gen::<f32>() < 0.9 {
                capitalize(&title)
            } else {
                title
            }
        }
        let mut titles = Vec::new();
        let title_count: usize = rng.gen_range::<u8>(4, 8) as usize;
        while titles.len() < title_count {
            let title = make_title(rng, &base);
            if !titles.contains(&title) {
                titles.push(title);
            }
        };
        let surname_last = rng.gen::<bool>();
        Language{base, genitive, definite, place, region, person, titles, surname_last}
    }

    /// Creates a random language using the provided string as a seed for a
    /// pseudo random number genrator. Only the first 16 bytes of the string are
    /// use.
    pub fn from_str(s: &str) -> Language {
        let mut seed = [1u8; 16]; // zeros would break on empty strings
        for (b, s) in s.bytes().zip(seed.iter_mut()) {
            *s = b;
        }
        let mut rng = XorShiftRng::from_seed(seed);
        Language::from_rng(&mut rng)
    }

    pub fn genitive(&self) -> &str {
        &self.genitive
    }

    pub fn definite(&self) -> &str {
        &self.definite
    }

    pub fn titles(&self) -> &Vec<String> {
        &self.titles
    }

    pub fn make_word_rng<R: Rng>(&self, rng: &mut R) -> String {
        self.base.make_word(rng, None)
    }

    fn make_name_rng<R: Rng>(&self, rng: &mut R, morphemes: Option<&Vec<String>>) -> (String, String) {
        let (long, short) = if rng.gen::<bool>() {
            // one word
            let w = capitalize(&self.base.make_word(rng, morphemes));
            (w.clone(), w)
        } else {
            let w1 = capitalize(&self.base.make_word(rng, morphemes));
            let w2 = capitalize(&self.base.make_word(rng, morphemes));
            let l = if rng.gen::<bool>() {
                w1.clone() + " " + &w2
            } else {
                w1.clone() + " " + &self.genitive + " " + &w2
            };
            (l, w1)
        };
        if rng.gen::<f32>() < 0.1 {
            (capitalize(&self.definite) + " " + &long, short)
        } else {
            (long, short)
        }
    }

    pub fn make_place_rng<R: Rng>(&self, rng: &mut R) -> (String, String) {
        self.make_name_rng(rng, Some(&self.place))
    }

    pub fn make_region_rng<R: Rng>(&self, rng: &mut R) -> (String, String) {
        self.make_name_rng(rng, Some(&self.region))
    }

    pub fn make_person_rng<R: Rng>(&self, rng: &mut R) -> (String, String) {
        // TODO: customize make_name_rng for person names
        //       1) Instead of ddefinite, create a list of titles to lead with
        //       2) surname_last: bool - to choose the short name
        let (long, short) = if rng.gen::<bool>() {
            // one word
            let w = capitalize(&self.base.make_word(rng, Some(&self.person)));
            (w.clone(), w)
        } else {
            let w1 = capitalize(&self.base.make_word(rng, Some(&self.person)));
            let w2 = capitalize(&self.base.make_word(rng, Some(&self.person)));
            let l = if rng.gen::<bool>() {
                w1.clone() + " " + &w2
            } else {
                w1.clone() + " " + &self.genitive + " " + &w2
            };
            if self.surname_last {
                (l, w2)
            } else {
                (l, w1)
            }
        };
        if rng.gen::<f32>() < 0.1 {
            // add a title
            let index = (rng.gen::<f32>().powf(2.0) * (self.titles.len() as f32)).floor() as usize;
            let title = self.titles.get(index).unwrap();
            (title.clone() + " " + &long, title.clone() + " " + &short)
        } else {
            (long, short)
        }
    }

    pub fn make_word(&self) -> String {
        self.base.make_word(&mut thread_rng(), None)
    }

    pub fn make_place(&self) -> (String, String) {
        self.make_place_rng(&mut thread_rng())
    }

    pub fn make_region(&self) -> (String, String) {
        self.make_region_rng(&mut thread_rng())
    }

    pub fn make_person(&self) -> (String, String) {
        self.make_person_rng(&mut thread_rng())
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand::prng::XorShiftRng;
    use super::Language;

    #[test]
    fn pseudo_rng() {
        let seed = [1u8; 16];
        let mut rng = XorShiftRng::from_seed(seed);
        let lang = Language::from_rng(&mut rng);
        println!("lang: {:?}", lang);

        let word = lang.base.make_word(&mut rng, None);
        println!("Word: {}", word);

        let place = lang.make_place_rng(&mut rng);
        let region = lang.make_region_rng(&mut rng);
        let person = lang.make_person_rng(&mut rng);
        println!("Place: {:?}, Region: {:?}, Person: {:?}", place, region, person);
        // These should aways be the same, so we can get the same language for
        // the same seed.
        assert_eq!(lang.genitive, "snuk");
        assert_eq!(lang.definite, "ta");
        assert_eq!(word, "muschman");
        assert_eq!(place.0, "Schtupmassan");
        assert_eq!(place.1, "Schtupmassan");
        assert_eq!(region.0, "Schmipmammum");
        assert_eq!(region.1, "Schmipmammum");
        assert_eq!(person.0, "Lamuk");
        assert_eq!(person.1, "Lamuk");
    }

    #[test]
    fn name_seeded() {
        let lang = Language::from_str("NotEnglish");
        println!("lang: {:?}", lang);

        let word = lang.make_word();
        println!("Word: {}", word);

        let place = lang.make_place();
        let region = lang.make_region();
        let person = lang.make_person();
        println!("Place: {:?}, Region: {:?}, Person: {:?}", place, region, person);
        // These should aways be the same, so we can get the same language for
        // the same seed.
        assert_eq!(lang.genitive, "doz");
        assert_eq!(lang.definite, "zulp");
    }
}
