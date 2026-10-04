use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodyParts {
    Scalp,
    Forehead,
    Eyes,
    Nose,
    Ears,
    Neck,
    Face,
    Mouth,
    Shoulders,
    UpperArms,
    Elbows,
    Forearms,
    Wrists,
    Hands,
    Fingers,
    UpperChest,
    Sternum,
    Breasts,
    Chest,
    Abdomen,
    Pelvis,
    Genitals,
    Thighs,
    Knees,
    LowerLegs,
    Ankles,
    Feet,
    Toes,
}

impl BodyParts {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Scalp => "Scalp",
            Self::Forehead => "Forehead",
            Self::Eyes => "Eyes",
            Self::Nose => "Nose",
            Self::Ears => "Ears",
            Self::Neck => "Neck",
            Self::Face => "Face",
            Self::Mouth => "Mouth",
            Self::Shoulders => "Shoulders",
            Self::UpperArms => "Upper Arms",
            Self::Elbows => "Elbow",
            Self::Forearms => "Forearms",
            Self::Wrists => "Wrists",
            Self::Hands => "Hands",
            Self::Fingers => "Fingers",
            Self::UpperChest => "Upper Chest",
            Self::Sternum => "Sternum",
            Self::Breasts => "Breasts",
            Self::Chest => "Chest",
            Self::Abdomen => "Abdomen",
            Self::Pelvis => "Pelvis",
            Self::Genitals => "Genitals",
            Self::Thighs => "Thighs",
            Self::Knees => "Knees",
            Self::LowerLegs => "Lower Legs",
            Self::Ankles => "Ankles",
            Self::Feet => "Feet",
            Self::Toes => "Toes",
        }
    }

    pub fn symptoms(&self) -> &'static [&'static str] {
        match self {
            Self::Scalp => &[
                "baby's soft spot is bulging",
                "baby's soft spot is still open",
                "baby's soft spot is sunken",
                "baby's soft spot is tight",
                "bald spots (hair)",
                "blond hair",
                "bulging out of back of skull",
                "clear ringing sound note when tapping the skull",
                "clogged pores in bald spots",
                "complete loss of hair over entire body",
                "completely bald",
                "cranial osteoma",
                "craniosynostosis",
                "cut on scalp",
                "dandruff",
                "deformed forehead",
                "dry scalp",
                "dull sound when tapping the skull",
                "early grey hair",
                "enlarged vein on scalp",
                "flaky or greasy skin on scalp",
                "follicular scarring",
                "forehead bones breaking down",
                "forehead sticks out",
                "hair dryness",
                "hair getting straighter",
                "hair loss with crusty rash",
                "hair loss with scarring",
                "hair sparse",
                "head lice",
                "headache",
                "increased pressure in skull",
                "intracranial bruit",
                "itchy scalp",
                "long hair",
                "long nails",
                "losing hair",
                "losing hair in patch(es)",
                "low hairline",
                "lump on scalp",
                "male pattern baldness",
                "matted hair",
                "open pores in bald spots",
                "open sore(s) on scalp",
                "pointed head",
                "pulling out hair",
                "pus-filled bump(s) in bald spot",
                "pus-filled bump(s) in scalp hair follicle(s)",
                "random hairs in bald patch(es)",
                "random white hairs in bald spots",
                "rash limited to scalp",
                "red bumps around hair follicles on scalp",
                "red hair",
                "red skin in bald areas",
                "redness and dry scaly skin with hair loss",
                "scalp feels overly sensitive",
                "scalp feels warm",
                "scalp hurts",
                "scalp tender to touch",
                "scalp vessel pulse increase",
                "seams of skull separate",
                "shiny bald head",
                "shiny scalp",
                "skin on scalp feels thinner",
                "soft skull",
                "swollen scalp",
                "tough or thick skin on scalp",
                "white hair",
                "widespread loss of hair",
            ],
            Self::Forehead => &[
                "can't pay attention",
                "confused thinking and reduced awareness of your environment",
                "enlarged vein on forehead",
                "fatigue",
                "fever",
                "forehead is tender",
                "forehead sticks out",
                "hairy forehead",
                "hallucination",
                "headache",
                "headache in front of head",
                "high forehead",
                "inappropriate behavior",
                "lightheadedness",
                "paranoia",
                "wide forehead",
                "wrinkled forehead",
            ],
            Self::Eyes => &[
                "argyll-robertson pupil",
                "astigmatism",
                "black eye",
                "black spots floating in my eye",
                "bleeding around the eye",
                "bleeding eyelid",
                "bleeding in eye",
                "bleeding in front part of eye",
                "blind spot",
                "blind spot that appears blank",
                "blind spot that appears dark",
                "blood vessels in colored part of eye",
                "blurry vision",
                "blurry vision in one eye",
                "brownish-yellowish ring around the color of eye",
                "bruising around eyes",
                "bugs in my eye",
                "bulging eyes",
                "burn to part of eye",
                "bushy eyebrows",
                "can't close eye all the way",
                "can't focus eyes",
                "can't look up",
                "can't move eyes to the side",
                "can't recognize things i see",
                "can't turn my eyes",
                "can? see far away",
                "can? see up close",
                "cataract",
                "central scotoma",
                "central vision loss",
                "chorioretinitis",
                "choroid coloboma",
                "choroiditis",
                "cloudy area on cornea",
                "color blindness",
                "conjunctival contracture",
                "conjunctival fold necrosis, yellow white",
                "conjunctival papillary flatness",
                "conjunctival papillary hardening",
                "conjunctival pseudomembrane",
                "conjunctival smoothness",
                "conjunctivitis, follicular",
                "constant red dry eyes",
                "corneal areflexia",
                "corneal contracture",
                "corneal dystrophy",
                "corneal protrusion",
                "cross eyed",
                "decreased tears",
                "decreased vision",
                "dilated pupil",
                "distorted vision",
                "double vision",
                "double vision in one eye",
                "downslanting palpebral fissues",
                "drooping eyelid",
                "drusen",
                "dry eyes",
                "enlarged brow ridge",
                "enlarged vein on clear part of eye",
                "extraocular muscle imbalance",
                "extraocular muscle weakness",
                "eye bleeds",
                "eye blinking",
                "eye color is changing to blue",
                "eye contact impairment",
                "eye discharge",
                "eye hurts",
                "eye hurts when i move it",
                "eye is burning",
                "eye is red and irritated",
                "eye not in normal position in socket",
                "eye opening is narrow",
                "eye pain",
                "eye socket hurts",
                "eye socket is sinking inward",
                "eye strain",
                "eye too large",
                "eye twitching",
                "eyeball is swollen",
                "eyebrow hair loss",
                "eyebrow lice",
                "eyebrow loss of color",
                "eyelid closes too slowly",
                "eyelid crusting and sticking together",
                "eyelid feels like it is burning",
                "eyelid feels scratchy",
                "eyelid feels thick",
                "eyelid flipped up",
                "eyelid folds inward",
                "eyelid granulation",
                "eyelid hurts",
                "eyelid is red and irritated",
                "eyelid pink gray",
                "eyelid pus",
                "eyelid tender to touch",
                "eyelid twitching",
                "eyelids feel hard",
                "eyelids feel heavy",
                "eyes are irritated",
                "eyes bulge out",
                "eyes don't move together",
                "eyes fixed on a single location",
                "eyes rolling back",
                "eyes tearing more",
                "eyes wide open",
                "eyesight getting worse",
                "eyesight worse in one eye",
                "feeling pressure below the eye",
                "flashing lights in vision",
                "flickering uncolored zig-zag lines in vision",
                "frequent squinting",
                "front part of eye is swollen",
                "front part of eye looks cloudy",
                "glaucoma",
                "grayish/brown spots on the outside of colored part of eye",
                "growth that looks like a yellow spot or bump on the eyeball",
                "hemianopia",
                "hole in colored part of eye",
                "hypopyon",
                "infected lump or sore on eyelid",
                "inner corner of eye is swollen",
                "intraocular pressure decrease",
                "intraocular pressure increase",
                "iridocyclitis",
                "iritis",
                "irritated eye",
                "itchy eye",
                "itchy eyelid",
                "keratitis",
                "lacrimal gland lobulation",
                "lacrimal sac inflammation",
                "large blister on the eye",
                "lateral vision loss",
                "lazy eye",
                "lens dislocation",
                "light hurts eyes",
                "little eyes",
                "losing eye color",
                "losing eyelashes",
                "loss of an area of vision",
                "loss of an area of vision in both eyes",
                "loss of an area of vision in one eye",
                "loss of color vision in one spot",
                "loss of eyelashes",
                "loss of vision in both eyes",
                "loss of vision in one eye",
                "lump in eye socket",
                "maculae ceruleae",
                "macular degeneration",
                "mucus coming from the eye",
                "no color in eye",
                "no peripheral vision",
                "not able to make tears",
                "nystagmus",
                "nystagmus latency",
                "nystagmus reversal",
                "nystagmus, fatigue",
                "nystagmus, rotary",
                "ocular cherry red spot",
                "one eye bulges",
                "one eye not turning",
                "one eye sees better than the other",
                "one eyelid swollen",
                "one or both eyes look downward",
                "one or both eyes look to the side",
                "open sore(s) on colored part of eye",
                "open sore(s) on eye",
                "open sore(s) on eyelid",
                "opening snap",
                "optic nerve atrophy",
                "optic neuritis",
                "pain around the eye",
                "pain behind the eye",
                "painful and weak eye movement",
                "papilledema",
                "part of outer layer of eye sticking to another part",
                "peripheral vision loss",
                "pink eye",
                "pinpoint pupils",
                "poor night vision",
                "prominent brow ridge",
                "pupil fixed",
                "pupil irregularity",
                "pupillary deformity",
                "pupillary inequality",
                "pupillary whiteness",
                "pus coming from the eye",
                "rash limited to eyelid",
                "red color blindness",
                "red eye",
                "red eyelid",
                "retinal angioid streaks",
                "retinal bleeding",
                "retinal coloboma",
                "retinal detachment",
                "retinal exudate",
                "retinal granuloma",
                "retinal opacity",
                "retinal pallor",
                "retinal pigmentation",
                "retinitis",
                "roth spots",
                "scar on clear part eye",
                "scar on the eye",
                "seeing halos of light around things",
                "severe eye pain",
                "single red eye",
                "skin and eyes more sensitive to sunlight",
                "skin folded over upper eyelid",
                "slower blinking",
                "small blister on eye",
                "small dot of light or zigzag shape in your vision",
                "small flat spots of loss of skin color",
                "something stuck inside the eye",
                "sore eye",
                "specks or spots in colored part of eye",
                "spider vein(s) in eye",
                "stye",
                "sunken eyes",
                "swelling around the eyes",
                "swollen eyelid",
                "swollen tear duct",
                "tear duct hurts",
                "tear duct is red",
                "tearing in one eye",
                "temporary vision loss",
                "things appear smaller than they are",
                "things in vision appear yellowish",
                "thinning eyebrows",
                "thinning eyelashes",
                "third nerve paralysis",
                "tiny red or purple spots limited to inside eye",
                "torn eyelid",
                "trouble looking up",
                "trouble moving eyes",
                "trouble opening eye",
                "tumor on one eye",
                "twitching of colored part of eye",
                "unable to see clearly",
                "uveitis",
                "uveitis, bilateral",
                "violet color to eyelid",
                "vision loss",
                "visual aura",
                "visual hallucination",
                "vitreous hemorrhage",
                "watery eyes",
                "white part of eye is black",
                "white part of eye is blue",
                "white part of eye is white",
                "white patch(es) around eye",
                "white, grey or blue ring seen around color part of eye",
                "whitish material on eyelid",
                "wideset eyes",
                "wrinkle between eyebrows",
                "yellow eyes",
                "yellow open sore(s) on eye",
                "yellow pea-sized lump(s) on eyelid",
            ],
            Self::Nose => &[
                "along smile or laugh lines are red",
                "blockage in nose",
                "bloody nose",
                "boil on nose",
                "bridge of nose looks flat",
                "clear runny nose",
                "cleft nose",
                "dented nostril",
                "deviated septum",
                "dry nasal passages",
                "growth in nose",
                "hay fever",
                "head congestion",
                "high nose bridge",
                "hooked nose",
                "inside of nose is black",
                "inside of nose is red",
                "inside of nose is swollen",
                "irritated nose",
                "itchy nose",
                "low nose bridge",
                "nasal sinus draining",
                "nasal sinus feels full",
                "nasal sinus is blocked",
                "nasal sinus pain",
                "nasal sinus sore",
                "nose and throat are inflamed",
                "nose destruction",
                "nose discharge, foul smelling, unilateral",
                "nose discharge, purulent, unilateral",
                "nose feels like it is burning",
                "nose flares open",
                "nose getting bigger",
                "nose hair burned",
                "nose hurts",
                "nose is turned upward",
                "nose misshapen",
                "nose mucous membrane atrophy",
                "nose oral communication",
                "nose septum destruction",
                "nose septum necrosis",
                "nose septum perforation",
                "nose septum ulceration",
                "nose skin infected",
                "nose tender to touch",
                "nosebleed",
                "nostrils are very small",
                "nostrils tilt down",
                "open sore(s) on nose",
                "open sore(s) on the nostril",
                "postnasal drip",
                "pus coming out of nose",
                "red nose",
                "rounded nose",
                "runny nose",
                "shingles on tip of nose",
                "sinus pain or infection bridge of nose",
                "sinus ulceration",
                "sinusitis",
                "small nose",
                "smelling things that aren't there",
                "smelly, runny nose",
                "sneezing",
                "snotty, runny nose",
                "snout reflex",
                "stuffy nose",
                "swollen nose",
                "thin nose",
                "tingling or pricking of nose",
                "trouble smelling",
                "using decongestant nose drops",
                "wide nose",
            ],
            Self::Ears => &[
                "big ears",
                "blocked ear",
                "bony area behind ear is infected with pus",
                "bony area behind ear is swollen",
                "bony area behind ear is tender",
                "bony growths in the ear",
                "bruising on skull behind the ear",
                "can't hear on one side",
                "conductive hearing loss",
                "constant ear ringing",
                "diagonal crease in ear lobe",
                "dry skin in ear",
                "ear bleeding",
                "ear cartilage is blue or black",
                "ear doesn't look right",
                "ear infection",
                "ear infection middle ear",
                "ear is red",
                "ear lesion, mucoid",
                "ear tender to touch",
                "ear wax blocking ear",
                "earache",
                "earlobe crease",
                "ears feel full",
                "ears set low",
                "fluid leaking from my ear",
                "hard lumps around joints",
                "headache behind ears",
                "hear crackling noises in my ears",
                "hearing is getting worse",
                "hearing things that aren't there",
                "hole in eardrum",
                "inner ear infection",
                "itchy ear",
                "large blister on eardrum",
                "large earlobes",
                "lump in front of ear",
                "lump on ear",
                "mastoid bruit",
                "mastoiditis",
                "middle ear infection",
                "otitis externa",
                "otitis interna",
                "outside of ear hurts",
                "pain in bony area behind ear",
                "pus coming from my ear",
                "rash limited to ear",
                "red and irritated swollen ear",
                "redness of skin on skull behind ear",
                "scaly or greasy skin on or behind ear",
                "single skin growth on ear lobe",
                "small ear",
                "small ear canal",
                "something is stuck in my ear",
                "swelling in front of ears",
                "swollen ear cartilage",
                "tough or thick skin around joints",
                "trouble hearing",
                "tympanic membrane bulging",
                "tympanic membrane hypomobile",
                "tympanic membrane inflammation",
                "tympanic membrane opaque",
                "tympanic membrane retraction",
                "tympanic membrane scarring",
                "very sensitive to noise",
                "very sensitive to sounds",
                "vestibular impairment",
                "whole ear swollen ear",
            ],
            Self::Face => &[
                "can't feel temperature on face",
                "can't move my face",
                "can't move my face well",
                "can't move one side of my face",
                "cheek bone pain",
                "cheek pain",
                "enlarged vein on face",
                "expressionless face",
                "face extremely thin and bony",
                "face feels full",
                "face feels numb",
                "face feels weak",
                "face hair turning white",
                "face hurts",
                "face is blotchy",
                "face is swollen",
                "face is turning blue",
                "face is yellow",
                "face misshapen",
                "face round",
                "face spasms when stimulated",
                "face sweats a lot",
                "face tender to touch",
                "face turns red when eating, drinking or exercising",
                "face turns reddish color",
                "face twitches",
                "face twitching",
                "facies coarse",
                "facies grotesque",
                "facies mongoloid",
                "facies triangular",
                "feels like air is under my face",
                "fragile or thin skin on face",
                "hair on face",
                "half of face is flushed",
                "horner syndrome",
                "huge cheek bones",
                "infected lump or sore on face",
                "itchy face",
                "limp muscle in face",
                "long face",
                "losing fat in face",
                "loss of facial hair",
                "lump on face",
                "nasal sinus pain",
                "no facial sweating",
                "numbness of face",
                "one side of face feels weak",
                "one side of face not the same as the other",
                "one side of my face hurts",
                "open sore(s) on face",
                "pale face",
                "pea-sized lump under skin on face",
                "pinched expression",
                "rash limited to face",
                "red face",
                "red flaky rash limited to smile or laugh lines",
                "red, swollen, runny nose",
                "rough hair on face",
                "sagging skin on face",
                "skin on face feels hard",
                "sore facial hair",
                "spider vein(s) on face",
                "stiff muscle in face",
                "thinning facial hair",
                "tingling or pricking face skin",
                "tingling or pricking on one side of face",
                "tingling or pricking skin of face",
                "trigeminal neuralgia",
                "trigeminal paralysis",
                "unibrow",
                "veins on face dilated",
                "weak muscles in face",
                "winking caused by jaw movement",
            ],
            Self::Mouth => &[
                "area of mucus on tongue",
                "area under tongue is swollen",
                "back of mouth is red",
                "bad breath",
                "black stuff coating tongue",
                "bleeding gums",
                "blisters on tongue",
                "breath has a fruity smell",
                "breath has a sweet and tarry smell",
                "breath smells like almonds",
                "breath smells like garlic",
                "breath smells like urine",
                "breath smells metallic",
                "broken speech pattern",
                "brown flat discolored spot(s) limited to lips",
                "buccal patch(es) mucus",
                "bulimia",
                "can't pucker lips",
                "can't speak",
                "canker sore",
                "chin recession",
                "cleft lip",
                "cleft palate",
                "cold sore",
                "corner of mouth hurts",
                "corner of mouth is sagging",
                "cough",
                "crack at the corner of mouth",
                "crack on tongue",
                "cracked lips",
                "crave salt",
                "damaged teeth enamel",
                "dehydration",
                "dental alveolar suppuration",
                "dental arch narrowness",
                "dental caries",
                "dentition delay",
                "denture pain",
                "diminished gag reflex",
                "drooling",
                "dry lips",
                "dry mouth",
                "dry tongue",
                "edentulous",
                "food doesn't taste good",
                "furry green coating on tongue",
                "geographic tongue",
                "gingival erythema",
                "gingival fistula",
                "gingival lead line, purple",
                "gingival leukoplakia",
                "gingival tenderness",
                "gingival ulceration",
                "gingival vesicle",
                "gingivitis",
                "gums hurt",
                "hives inside of mouth",
                "hives on lips",
                "hot food or liquids hurt tooth",
                "infected lump or sore on lip",
                "inflamed tongue",
                "inside of mouth is black",
                "inside of mouth is brown",
                "inside of mouth is red",
                "inside of mouth is white",
                "inside of mouth is yellow",
                "inside of mouth swollen",
                "interdental papillary ulceration",
                "involuntary jerky or fitful movement of tongue",
                "koplik spot",
                "large blister(s) in mouth",
                "large tongue",
                "lip chewing",
                "lip hurts",
                "lip is tingling or prickling",
                "lip pulled back",
                "lip tender to touch",
                "lip trembling",
                "lipoatrophy",
                "lips are thicker",
                "lips turning blue",
                "long groove between nose and lip",
                "lower lip droops",
                "lump on tongue",
                "mallampati grade i",
                "mallampati grade iii-iv",
                "malocclusion",
                "metal taste in mouth",
                "microdontia",
                "micrognathia",
                "molar loosening, deciduous",
                "more thirsty than usual",
                "mouth bleeding",
                "mouth breathing",
                "mouth burn",
                "mouth hurts",
                "mouth is sore",
                "mouth is swollen",
                "mouth itches",
                "mouth looks crooked",
                "mouth mucous membrane bleeding",
                "mouth mucous membrane ulceration",
                "mouth opened",
                "mouth tender to touch",
                "mouth wideness",
                "mucous membrane petechia",
                "mucous membrane scarring",
                "mute",
                "open sore(s) in mouth",
                "open sore(s) inside of cheek",
                "open sore(s) on back of mouth",
                "open sore(s) on inside of cheek",
                "open sore(s) on lip",
                "open sore(s) on roof of mouth",
                "open sore(s) on tongue",
                "orange tonsils",
                "pain in tooth socket",
                "palatal muscle weakness",
                "palatal paralysis",
                "palatal tremor",
                "pale around mouth",
                "pea-sized lump on tongue",
                "producing too much saliva",
                "pseudomembrane",
                "puckered lip",
                "raised skin patch(es) on tongue",
                "red bump(s) inside of cheek",
                "red irritated throat",
                "red lips",
                "red or purple flat spots on inside of cheeks",
                "red tonsil",
                "roof of mouth has high arch",
                "roof of mouth is inflamed",
                "roof of mouth is misshapen",
                "roof of mouth is numb",
                "roof of mouth is red",
                "roof of mouth narrow",
                "roof of mouth red",
                "roof of mouth swollen",
                "round ball in back of throat is out of place",
                "round ball in back of throat is swollen",
                "self induced vomiting",
                "severely bad breath",
                "short groove between nose and lip",
                "shrinking tongue",
                "skin sore(s) inside of mouth",
                "skin sore(s) on tonsil",
                "small blister on roof of mouth",
                "small bump on inside of cheek",
                "small bump(s) on back of mouth",
                "small flat red or purple spots on back of mouth",
                "small flat red or purple spots on round ball in back of throat",
                "small flat red or purple spots on tonsil",
                "small red spots on roof of mouth",
                "small white bump(s) on inside of cheek",
                "smooth groove between nose and lip",
                "snoring",
                "soft palate atrophy",
                "soft palate numbness",
                "soft palate paralysis",
                "soft palate swelling",
                "sores in or on side of mouth",
                "speech is slow",
                "spider vein(s) on roof of mouth",
                "stuff coats top of tongue",
                "stuttering",
                "swelling around the mouth",
                "swollen gums",
                "swollen lips",
                "swollen throat",
                "swollen tongue",
                "swollen tonsil on one side",
                "swollen tonsils",
                "tasting things that aren't there",
                "teeth do not fit well",
                "teeth grinding",
                "thin lips",
                "throat is dry",
                "thrush",
                "tingling or numbness around mouth",
                "tingling or pricking inside mouth",
                "tingling or pricking tongue",
                "tiny mouth",
                "tongue biting",
                "tongue blanching",
                "tongue feels like it is burning",
                "tongue glazing",
                "tongue has no grooves",
                "tongue hurts",
                "tongue infection",
                "tongue is more red than usual",
                "tongue is out of place",
                "tongue is weak",
                "tongue not normal size and shape",
                "tongue pushed out too far",
                "tongue quivers",
                "tongue trembling",
                "tonsil inflammation",
                "tonsil is out of place",
                "tonsillar leukoplakia",
                "tooth cold sensitivity",
                "tooth deformity",
                "tooth discoloration",
                "tooth enamel hypoplasia",
                "tooth enamel pitting",
                "tooth erosion",
                "tooth extraction",
                "tooth impaction",
                "tooth loose",
                "tooth loss",
                "tooth pegged",
                "tooth root defect",
                "tooth spacing irregularity",
                "toothache",
                "top lip hangs over",
                "trouble chewing",
                "trouble communicating",
                "trouble producing saliva",
                "trouble speaking",
                "trouble tasting",
                "tumor in mouth",
                "upper lip is swollen",
                "voice doesn't sound right",
                "vomiting blood",
                "white coating on tongue",
                "white rash on inside of mouth",
                "white rash on roof of mouth",
                "white skin sore(s) on back of mouth",
                "white skin sore(s) on round ball in back of throat",
                "whitish coating on tonsil",
                "yawning",
                "yellow skin sore(s) on back of mouth",
                "yellow skin sore(s) on uvula",
            ],
            Self::Neck => &[
                "blister(s) on back of throat",
                "brown mucous in throat",
                "burn back of throat",
                "can't bend head forward",
                "can't turn head",
                "carotid artery bruit",
                "carotid artery distention",
                "carotid artery mass",
                "carotid pulse absence",
                "carotid pulse increase",
                "carotodynia",
                "cervical erosion",
                "cervical lymph node bleeding",
                "cervical stenosis",
                "choking",
                "choking sensation",
                "clearly outlined pea-sized lump on neck",
                "cough",
                "cracking sound in neck",
                "cricothyroid paralysis",
                "enlarged jugular vein",
                "epiglottic enlargement",
                "epiglottic erythema",
                "epiglottis swelling",
                "epiglottitis",
                "episodes of not breathing during sleep",
                "feel pressure on neck",
                "feels like something is stuck in my throat",
                "food comes back up",
                "food or liquid goes down wrong pipe",
                "globus major nodule",
                "hair on neck feels tender",
                "hair roots on neck are red",
                "hashimoto disease",
                "head turned to one side",
                "hepatojugular reflux",
                "high pitched breathing",
                "infected lump or sore on neck",
                "itchy neck",
                "itchy throat",
                "jugular vein a wave increased",
                "jugular venous distention with inspiration",
                "laryngeal anesthesia",
                "laryngeal crepitation",
                "laryngeal dryness",
                "laryngeal edema",
                "laryngeal erythema",
                "laryngeal hematoma",
                "laryngeal mass",
                "laryngeal mobility increase",
                "laryngeal obstruction",
                "laryngeal pain",
                "laryngeal papilloma",
                "laryngeal pressure sensation",
                "laryngeal stenosis",
                "laryngeal tenderness",
                "laryngeal ulceration",
                "laryngitis",
                "lump on neck",
                "lump on one side of neck",
                "lump on one side of throat",
                "lump on the front of neck",
                "nasopharyngeal induration",
                "neck bones fused together",
                "neck bones sticking out",
                "neck does not sweat",
                "neck fascia thickening",
                "neck has changed colors",
                "neck hurts",
                "neck is blue",
                "neck is red",
                "neck is swollen",
                "neck lymph node too big",
                "neck mass, anterior cervical",
                "neck mass, posterior cervical",
                "neck muscles are weak",
                "neck subcutaneous emphysema",
                "neck tender to touch",
                "neck vasodilatation",
                "neck vessel bruit",
                "no fat in neck",
                "open sore(s) on back of throat",
                "orange mucous in throat",
                "pain on one side of throat",
                "pain when i swallow",
                "painful swollen gland in front part of neck",
                "pea-sized lump(s) in neck",
                "pharyngeal mucous membrane edema",
                "pharyngeal paralysis",
                "prickling or tingling in neck",
                "pus-filled bump(s) in neck hair follicle(s)",
                "rash limited to neck",
                "red bumps around hair follicles on neck",
                "red open sore(s) on neck",
                "red pea-sized lump(s) in lining of throat",
                "removal of thyroid gland",
                "short neck",
                "small red or purple spots on back of throat",
                "sore throat",
                "spider vein(s) on neck",
                "sternocleidomastoid muscle paralysis",
                "stiff neck",
                "swelling at back of throat",
                "tender neck lymph node",
                "throat bleeding",
                "throat burning sensation",
                "throat clearing",
                "throat dryness",
                "throat feels numb",
                "throat feels tender",
                "throat feels weak",
                "throat is red",
                "throat spasm",
                "thyroid bruit",
                "thyroid enlargement",
                "thyroid nodule",
                "tightness in throat",
                "tingling and prickling in throat",
                "tracheal compression",
                "trouble swallowing",
                "tumor on back of throat",
                "voice deepening",
                "voice is hoarse",
                "webbing on side of neck",
                "whispered pectoriloquy",
                "white mucous in throat",
                "white stuff on throat",
                "windpipe is shifted",
            ],
            Self::Shoulders => &[
                "large shoulder vein",
                "lump in shoulder",
                "shoulder girdle fascia thickening",
                "shoulder girdle muscle weakness",
                "shoulder granule",
                "shoulder muscle pain",
                "shoulder muscle twitching",
                "shoulder shrug sign",
                "shoulder tender to touch",
                "subacromial bursal tenderness",
                "subdeltoid bursal tenderness",
                "swollen shoulder",
            ],
            Self::UpperArms => &[
                "bicep shaking",
                "biceps and triceps hyperreflexia",
                "biceps hyporeflexia",
                "humeral swelling, lower",
                "triceps hyporeflexia",
                "upper arm pain",
            ],
            Self::Elbows => &[
                "darkened skin on elbow",
                "elbow bones out of place",
                "elbow pain",
                "flaky bump(s) limited to elbows or knees",
                "forearm is angled away from the body",
                "red bump(s) on elbow",
                "single flaky raised skin patch on elbows or knees",
                "stiff elbow",
                "tenderness lateral epicondyle",
            ],
            Self::Forearms => &[
                "forearm feels more sensitive",
                "forearm feels weak",
                "forearm hurts",
                "forearm itches",
                "forearm turning up",
                "forearm turns in",
                "lump on forearm",
                "tingling or prickling in forearm",
            ],
            Self::Wrists => &[
                "able to bend wrist backwards",
                "crackling sound when moving wrist",
                "phalen's maneuver positive",
                "tough or thick skin on base of wrist",
                "wrist flexor muscle atrophy",
                "wrist hurts when moved",
                "wrist is red",
                "wrist is swollen on thumb side",
                "wrist muscle weakness",
                "wrist pain",
                "wrist stiffness",
                "wrist swelling",
                "wrist tenderness, radial",
                "wristdrop",
            ],
            Self::Hands => &[
                "abnormal creases in palm",
                "arm/hand pain due to median nerve problem",
                "asterixis",
                "brachymesophalangia, fifth finger",
                "brown nails",
                "burning feeling in hand",
                "camptodactyly",
                "can't write",
                "cold hand",
                "compressed nerve in wrist/hand",
                "cramp in my palm",
                "curved fingers",
                "darkened skin on knuckle(s)",
                "double jointed hand",
                "durkan's compression test positive",
                "fatty yellowish skin rash on palms",
                "fist clenching",
                "flaky tough or thick skin on palms of hand",
                "hand asterixis",
                "hand changing colors",
                "hand cramping at night",
                "hand hurts",
                "hand hurts when moving",
                "hand is numb",
                "hand is red",
                "hand is turned towards little finger",
                "hand muscle weakness",
                "hand or arm shaking when performing task",
                "hand shaking",
                "hand shortness",
                "hand smallness",
                "hand swelling",
                "hand wringing",
                "hands move too slowly",
                "hives on hand",
                "infected lump or sore on hand",
                "itchy palms",
                "knuckle deformity",
                "knuckle joint on hand hurts",
                "large hands",
                "milkmaid's grip",
                "muscles on outside of palm are shrinking or thinning",
                "open sore(s) on hand",
                "open sore(s) on palm",
                "pain in palm of hand",
                "palm area under thumb is flat",
                "palm is swollen",
                "palms sweating more",
                "pea-sized lump(s) on palm of hand",
                "peeling hands",
                "rash limited to hand",
                "rash limited to palm",
                "rash on hand",
                "red flaky rash limited to palms or soles",
                "red palms",
                "shrinking or thinning muscles in hand",
                "single line that runs across the palm of hand",
                "skin on palm peeling off",
                "smooth, soft palms",
                "stiff hands",
                "stiff knuckles in hands or toes",
                "swollen knuckles",
                "tingling or prickling in hand",
                "tough or thick skin on palms of hand",
                "trouble moving hands",
                "trouble speaking or talking",
                "weak hand grip",
                "wide flat hand",
                "yellow palms",
            ],
            Self::Fingers => &[
                "blue nails",
                "can't recognize fingers",
                "can't straighten bent finger(s)",
                "clinodactyly",
                "discolored fingertip",
                "enlarged fingertips",
                "finger blood vessel obstruction",
                "finger chewing",
                "finger deformity",
                "finger shaking",
                "finger(s) are swollen",
                "finger(s) feel stiff",
                "finger(s) feel tender",
                "finger(s) feel tight",
                "finger(s) hurts",
                "finger(s) locks in place",
                "finger(s) point to little finger side of hand",
                "finger(s) really sensitive to touch",
                "finger(s) too cold",
                "finger(s) turn red",
                "finger(s) turns blue",
                "fingernail(s) hurt",
                "fingertip tender to touch",
                "half of nail is white and other half is deeper pink",
                "hand spiderlike",
                "hard bumps on finger(s)",
                "hoffman sign positive",
                "nail loss",
                "nail not growing the way it should",
                "nail pulling away from cuticle",
                "nail(s) are blue",
                "open sore(s) on finger(s)",
                "open sore(s) on fingertip(s)",
                "pale finger(s)",
                "pea-sized lump under skin on finger(s)",
                "skin on finger(s) is thick",
                "spider veins in fingernails",
                "syndactyly",
                "thumb absence",
                "thumb hurts",
                "thumb microdactyly",
                "thumb spatulate",
                "thumb synostosis",
                "thumb, distal phalynx, shortness",
                "thumb, triphalangeal",
                "tingling and prickling in finger(s)",
                "unusually short fingers",
                "weak finger(s)",
                "weak thumb muscle",
            ],
            Self::UpperChest => &[
                "fatty area above collar bone",
                "left supraclavicular lymph node enlargement",
                "supraclavicular fossa bruit",
                "supraclavicular lymph node enlargement",
                "supraclavicular pulsation",
            ],
            Self::Sternum => &[
                "aortic dilation, ascending",
                "aortic dissection",
                "aortic infection",
                "behind the breastbone hurts",
                "breast bone hurts",
                "breastbone is abnormal",
                "breastbone tender to touch",
                "breath sound decrease, basilar, unilateral",
                "cardiomegaly",
                "chest bones cave in",
                "chest bones stick out",
                "chest pain that spreads to arm, shoulder, neck or jaw",
                "congestive heart failure",
                "ejection fraction reduced",
                "feeling of pressure in food pipe",
                "food gets stuck",
                "gibson's murmur",
                "hard for food to go down",
                "heart beats faster when exercising",
                "heart displacement",
                "heart displacement, left",
                "heart displacement, right",
                "heart murmur",
                "heart murmur increased with inspiration",
                "heart murmur, changing",
                "heart murmur, diastolic",
                "heart murmur, diastolic, pulmonic",
                "heart murmur, holosystolic",
                "heart murmur, machinery",
                "heart murmur, presystolic",
                "heart murmur, systolic",
                "heart murmur, systolic, apical",
                "heart murmur, systolic, crescendo-decrescendo",
                "heart murmur, systolic, pulmonic",
                "heart size decrease",
                "heart sound absence, second",
                "heart sound decrease, first",
                "heart sound decrease, second",
                "heart sound increase",
                "heart sound increase, first",
                "heart sound increase, second",
                "heart sound increase, second, pulmonic",
                "heart sound irregularity",
                "heart sound split, first",
                "heart sound split, second",
                "heart sound variation, first",
                "heart sound, fourth",
                "heart sound, third",
                "heart sounds muffled",
                "heart thrill",
                "heart thrill, apical",
                "heart thrill, diastolic",
                "heart thrill, pulmonic",
                "heartburn",
                "hiccups",
                "inflammation of esophagus",
                "mitral valve prolapse",
                "nipple hurts",
                "palpitations",
                "pericardial friction rub",
                "pressure on heart due to fluid buildup",
                "pulmonary ejection click",
                "pulmonic sound absence",
                "pulmonic sound decrease",
                "severe chest pain/pressure",
                "sternal lift",
                "sternal pulsation visible",
                "sternoclavicular joint pulsation",
                "systolic heart murmur, increased with valsalva",
                "systolic thrill",
                "tightening of esophagus",
            ],
            Self::Breasts => &[
                "abnormal growth of male breasts",
                "bloody nipple discharge",
                "breast cancer",
                "breast feels harder",
                "breast feels heavy",
                "breast getting bigger",
                "breast getting smaller",
                "breast hurts",
                "breast mass roundness",
                "breast mass smoothness",
                "breast mass, unilateral",
                "breast redness",
                "breast skin feels like an orange peel",
                "breastfeeding mom",
                "breasts not developing",
                "darkened skin on nipple",
                "enlarged vein on breast",
                "fluid leaking from nipple",
                "growth on nipple",
                "hard lump in breast",
                "infected lump or sore on breast",
                "loss of skin color on nipple",
                "lump in breast",
                "lump in breast that can be moved",
                "lump in breast that doesn't move",
                "nipple doesn't move",
                "nipple pulling to one side",
                "nipple redness",
                "nipple stays hard all the time",
                "nipple tender to touch",
                "painful tube like lump in breast",
                "part of breast skin appears pulled inward",
                "rash limited to under the breast",
                "red, irritated nipple",
                "squishy lump in breast",
                "swollen breast",
                "swollen nipples",
                "wide set nipples",
            ],
            Self::Chest => &[
                "abnormal growth of male breasts",
                "absent breath sounds, unilateral",
                "aortic dilation, ascending",
                "aortic dissection",
                "aortic infection",
                "asthma",
                "austin flint murmur",
                "barky cough",
                "behind the breastbone hurts",
                "between right lower ribs hurts",
                "blood clot traveled to lung",
                "bloody nipple discharge",
                "breast bone hurts",
                "breast cancer",
                "breast feels harder",
                "breast feels heavy",
                "breast getting bigger",
                "breast getting smaller",
                "breast hurts",
                "breast mass roundness",
                "breast mass smoothness",
                "breast mass, unilateral",
                "breast redness",
                "breast skin feels like an orange peel",
                "breastbone is abnormal",
                "breastbone tender to touch",
                "breastfeeding mom",
                "breasts not developing",
                "breath sound decrease, basilar, unilateral",
                "breathing too fast",
                "breathing too slowly",
                "buildup of fluid in lungs",
                "burning sensation in chest",
                "can't cough up mucus or phlegm",
                "can't feel hot or cold on upper body",
                "cardiomegaly",
                "chest bones cave in",
                "chest bones stick out",
                "chest bruit",
                "chest crepitation",
                "chest decreased resonance",
                "chest deformity",
                "chest deformity on left side",
                "chest feels tender to the touch",
                "chest hair loss",
                "chest hyperresonance",
                "chest hyperresonance, unilateral",
                "chest infection",
                "chest is rigid",
                "chest muscle spasm",
                "chest overly expanded",
                "chest pain",
                "chest pain after vomiting",
                "chest pain made worse by breathing",
                "chest pain made worse by exertion/exercise",
                "chest pain that spreads to arm, shoulder, neck or jaw",
                "chest peristaltic sound",
                "chest redness",
                "chest subcutaneous emphysema",
                "chest tightness",
                "chest wall fistula",
                "chest wall suppuration",
                "chronic cough (more than 8 weeks) with normal chest xray",
                "clavicular hypoplasia",
                "congestive heart failure",
                "continued inflammation of bronchial tubes",
                "cough",
                "cough out mucus",
                "cough up black phlegm",
                "cough up frothy or bubbly gunk",
                "cough up thick gunk",
                "cough up yellow gunk",
                "cough with mucus long time",
                "cough with swallowing",
                "coughing at night",
                "coughing attacks",
                "coughing up bad smelling mucus",
                "coughing up blood",
                "coughing up white mucus",
                "crushing chest pain",
                "darkened skin on nipple",
                "decreased breath sounds",
                "decreased breath sounds, unilateral",
                "difficulty breathing with normal chest x-ray",
                "dry cough",
                "egophony",
                "ejection fraction reduced",
                "emphysema",
                "enlarged chest vein",
                "enlarged vein on breast",
                "fat chest",
                "fatty area above collar bone",
                "feeling of pressure in food pipe",
                "fluid leaking from nipple",
                "food gets stuck",
                "forceful cough",
                "gibson's murmur",
                "growth on nipple",
                "hacking cough",
                "hamman sign positive",
                "hard for food to go down",
                "hard lump in breast",
                "heart beats faster when exercising",
                "heart displacement",
                "heart displacement, left",
                "heart displacement, right",
                "heart murmur",
                "heart murmur increased with inspiration",
                "heart murmur, changing",
                "heart murmur, diastolic",
                "heart murmur, diastolic, aortic",
                "heart murmur, diastolic, apical",
                "heart murmur, diastolic, pulmonic",
                "heart murmur, holosystolic",
                "heart murmur, machinery",
                "heart murmur, presystolic",
                "heart murmur, systolic",
                "heart murmur, systolic, aortic",
                "heart murmur, systolic, apical",
                "heart murmur, systolic, crescendo-decrescendo",
                "heart murmur, systolic, pulmonic",
                "heart size decrease",
                "heart sound absence, second",
                "heart sound decrease, first",
                "heart sound decrease, second",
                "heart sound increase",
                "heart sound increase, first",
                "heart sound increase, second",
                "heart sound increase, second, pulmonic",
                "heart sound irregularity",
                "heart sound split, first",
                "heart sound split, second",
                "heart sound variation, first",
                "heart sound, fourth",
                "heart sound, third",
                "heart sounds muffled",
                "heart thrill",
                "heart thrill, apical",
                "heart thrill, diastolic",
                "heart thrill, pulmonic",
                "heartburn",
                "hiccups",
                "infected lump or sore on breast",
                "infected lump or sore on upper body",
                "inflammation of bronchial tubes",
                "inflammation of esophagus",
                "intercostal space retraction",
                "irregular breathing pattern",
                "kussmaul respiration",
                "left supraclavicular lymph node enlargement",
                "loss of fat in chest area",
                "loss of skin color on nipple",
                "lower rib tender to touch",
                "lower ribs moving abnormally",
                "lump in breast",
                "lump in breast that can be moved",
                "lump in breast that doesn't move",
                "lung cancer",
                "lung disease",
                "lung infection",
                "making a whooping noise when inhaling",
                "mitral valve prolapse",
                "morning cough",
                "muscle cramp on trunk",
                "muscles between ribs are weak",
                "nipple doesn't move",
                "nipple hurts",
                "nipple pulling to one side",
                "nipple redness",
                "nipple stays hard all the time",
                "nipple tender to touch",
                "not breathing",
                "numbness tingling in chest",
                "obese upper body",
                "pain in chest not related to breathing",
                "pain in upper body",
                "painful tube like lump in breast",
                "palpitations",
                "part of breast skin appears pulled inward",
                "pea-sized lump in skin of torso",
                "pericardial friction rub",
                "pneumonia",
                "posterior trunk fascia thickening",
                "pressure on heart due to fluid buildup",
                "pulmonary ejection click",
                "pulmonary percussion dullness",
                "pulmonic sound absence",
                "pulmonic sound decrease",
                "pus-filled bump(s) in stomach or back hair follicle(s)",
                "rales, bilateral subcrepitant",
                "rales, right basilar",
                "rales, subcrepitant",
                "rapid breathing",
                "rash limited to chest",
                "rash limited to under the breast",
                "rate of breathing slows",
                "red bumps around hair follicles on stomach or back",
                "red upper body",
                "red, irritated nipple",
                "rib angle widening",
                "ribs pulled inward",
                "right supraclavicular lymph node enlargement",
                "sensitive skin on upper body",
                "severe chest pain/pressure",
                "shallow breathing",
                "sharp chest pain",
                "sharp distinct sound to each cough",
                "short of breath better when lay down",
                "shortness breath leaning forward",
                "shortness of breath",
                "shortness of breath when lying flat",
                "shortness of breath with activity",
                "squishy lump in breast",
                "sternal lift",
                "sternal pulsation visible",
                "sternoclavicular joint pulsation",
                "stiff muscles in trunk",
                "sudden shortness of breath at night",
                "supraclavicular fossa bruit",
                "supraclavicular lymph node enlargement",
                "supraclavicular pulsation",
                "swollen breast",
                "swollen nipples",
                "systolic heart murmur, increased with valsalva",
                "systolic thrill",
                "tender between right lower ribs",
                "tightening of esophagus",
                "tingling or prickling in upper body",
                "trunk asterixis",
                "trunk longness",
                "trunk shortness",
                "trunk tremor, bobbing",
                "upper body feels warm",
                "upper body itchy",
                "upper body muscles shrinking",
                "upper body spasm",
                "upper body tilted",
                "upper body trembling",
                "upper respiratory infection",
                "vomiting after cough",
                "weak upper body",
                "wet cough",
                "wheezing",
                "wide set nipples",
                "worsening shortness of breath",
            ],
            Self::Abdomen => &[
                "abdominal mass, movable, upper",
                "abdominal mass, right upper quadrant",
                "abdominal mass, upper",
                "abdominal tenderness, left lower quadrant",
                "abdominal tenderness, lower",
                "abdominal tenderness, left upper quadrant",
                "bladder distention",
                "bladder feels full",
                "burping",
                "c-section",
                "can't digest fatty foods",
                "change in bowel habits",
                "courvoisier sign",
                "diarrhea",
                "diarrhea after meals",
                "epigastric abdominal tenderness",
                "fatty liver",
                "feels like need to pee all the time",
                "frequent bowel movements",
                "gall bladder distention",
                "gallbladder inflammation",
                "gallstones",
                "gassy",
                "heartburn",
                "hepatic friction rub",
                "hepatosplenomegaly",
                "hernia in belly button",
                "hurts when ovulating",
                "indigestion",
                "indirect tenderness right lower quadrant",
                "inflammation of colon",
                "inflammation of stomach and intestines",
                "liver border irregularity",
                "liver bruit",
                "liver disease",
                "liver displacement",
                "liver enlargement",
                "liver hard",
                "liver mass",
                "liver pulsation",
                "liver tenderness",
                "lower belly bloating",
                "lower stomach pain",
                "murphy sign positive",
                "nausea",
                "open sore in stomach or esophagus",
                "ovarian mass",
                "ovarian mass, irregular",
                "ovarian swelling",
                "ovary palpable",
                "pain around belly button",
                "pain in diaphragm",
                "pain in middle of belly",
                "pain near belly button spreading to lower right side of stomach",
                "pancreas inflammation",
                "past appendix removal",
                "past gallbladder removal",
                "reflux",
                "scarring of the liver",
                "spleen enlargement",
                "spleen friction rub",
                "spleen palpable",
                "spleen tenderness",
                "stomach inflammation",
                "stomach pain lower left side",
                "stomach pain lower right side",
                "stomach pain upper left side",
                "stomach pain upper right side",
                "stomach pushes through diaphragm",
                "ulcer in muscle connecting stomach to duodenum",
                "upper abdominal wound",
                "upper belly bloating",
                "upper stomach pain",
                "urine leaking from belly button",
                "vomiting blood",
            ],
            Self::Pelvis => &[
                "acetabular dysplasia",
                "bend at hip",
                "coxa valga",
                "coxa vara",
                "darkened skin on groin",
                "difficulty getting up from a chair",
                "duroziez sign",
                "feeling of heaviness in groin",
                "femoral bruit",
                "femoral lymph node enlargement",
                "femoral pulse absence",
                "femoral pulse decrease",
                "greater tuberosity tenderness",
                "groin pain",
                "groin tenderness",
                "hernia, femoral",
                "hip deformity",
                "hip feels like it pops out of socket",
                "hip feels stiff",
                "hip hurts",
                "hip is swollen",
                "hip muscle is weak",
                "hip tenderness",
                "hurts to walk",
                "inguinal hernia",
                "inguinal lymph node abscess",
                "inguinal lymph node enlargement",
                "inguinal lymph node firmness",
                "inguinal lymph node matting",
                "inguinal lymph node tenderness",
                "ischial tuberosity tenderness",
                "lump comes and goes on groin",
                "lump in groin",
                "painful gland in groin",
                "pea-sized lump(s) on groin",
                "pelvic muscles are tight",
                "pelvic muscles feel weak",
                "pelvic smallness",
                "pelvis tilted",
                "pelvis wide",
                "rash limited to groin",
                "redness of groin",
            ],
            Self::Genitals => &[
                "a lot of blood in urine",
                "able to feel vein in scrotum",
                "abnormal swelling of penis",
                "balls turning blue",
                "bladder distention",
                "bladder edema",
                "bladder erythema",
                "bladder feels full",
                "bladder infection",
                "bladder mass",
                "blood in urine",
                "bloody pee",
                "bloody sperm",
                "bubbles in my urine",
                "bulging veins in scrotum",
                "can't have orgasm",
                "can't pee",
                "can't tell when bladder is full",
                "change in bladder habits",
                "chlamydial infection",
                "cloudy pee",
                "cremasteric reflex absent, unilateral",
                "cyst on genitals",
                "cyst on testicle",
                "dark pee",
                "decreased sex drive",
                "deformed scrotum",
                "delayed or late period",
                "difficult to pee",
                "discharge from penis",
                "double ureter",
                "enlarged prostate",
                "epididymal mass",
                "epididymal tenderness",
                "epididymitis",
                "erection that won't go down or soften",
                "feels like need to pee all the time",
                "firm pus filled rash around head of penis",
                "foreskin stuck over head of penis",
                "foreskin stuck to penis",
                "genital abnormality",
                "genital necrosis",
                "genital numbness",
                "genital pain",
                "genital underdevelopment",
                "genitalia, ambiguous",
                "genitals getting larger",
                "genitals itching",
                "genitals swollen",
                "glans penis calculus",
                "glans penis scar tissue formation",
                "gonad disorder",
                "gonadal hypoplasia",
                "gray skin peeling off penis",
                "green urine",
                "hard bump(s) around head of penis",
                "hard bump(s) on head of penis",
                "hard pus-filled rash on head of penis",
                "head of penis curves downward",
                "head of penis hurts",
                "head of penis is irritated",
                "head of penis is red and swollen",
                "head of penis is swollen",
                "hematuria, microscopic",
                "hemoglobinuria",
                "hives on penis",
                "hurts to ejaculate or cum",
                "immediate urge to pee",
                "impotence",
                "incontinence",
                "infected testicles",
                "infertility",
                "inflamed scrotum",
                "inflammation of urinary tract",
                "irritation between butt and genitals",
                "itching on urethra",
                "iud in place",
                "large blister(s) on penis",
                "large non-emptying bladder",
                "large penis",
                "light colored pee",
                "lump between butt and genitals",
                "lump in genital area",
                "lump in urinary tract",
                "lump on penis",
                "lump on scrotum",
                "lump on testicle",
                "lymphogranuloma venereum",
                "man ejaculates sooner during sexual intercourse than he or his partner would like",
                "massive scrotal swelling",
                "muscle twitching in genital area",
                "need to pee often",
                "open sore(s) around head of penis",
                "open sore(s) between butt and genitals",
                "open sore(s) on genitals",
                "open sore(s) on head of penis",
                "open sore(s) on penis",
                "open sore(s) on urethra",
                "opening of urinary tract is blocked",
                "orange pee",
                "pain at the opening of urinary tract",
                "pain between butt and genitals",
                "pain in cord running vertically behind testicle",
                "pain in testicle",
                "pain in testicle or ovary",
                "pain in tube behind testicle",
                "pain while peeing",
                "painful erection",
                "painless ulcer on the genitals",
                "passing small kidney stones",
                "pea-sized lump(s) on prostate",
                "pee comes out of top of penis",
                "pee hole is on bottom side of penis",
                "pee more than usual",
                "pee too much at night",
                "pelvic calculus palpable, rectum",
                "pelvic mass",
                "penis hurts",
                "penis is red",
                "penis is red and irritated",
                "penis pulled in",
                "penis tenderness",
                "prostate fluctuance",
                "prostate hardening",
                "prostate infection",
                "prostate pain",
                "prostate tenderness",
                "prostatitis",
                "rash limited to genitals",
                "rectovaginal fistula",
                "red bump(s) around head of penis",
                "red bump(s) on head of penis",
                "redness around urinary tract",
                "redness of private parts",
                "redness of testicle sac",
                "scrotal mass",
                "scrotal mass, firm",
                "scrotal pulling sensation",
                "scrotal ulceration",
                "scrotum cyst",
                "scrotum hurts",
                "scrotum pain goes away when lift testicle",
                "seminal vesicular induration",
                "seminal vesicular swelling",
                "sexual desire increased",
                "shrunken testicles",
                "small penis",
                "spermatic cord cyst",
                "spermatic cord enlargement",
                "spermatic cord hydrocele",
                "spermatic cord inflammation",
                "spermatic cord mass",
                "spermatic cord tenderness",
                "spermatic cord torsion",
                "std transmission",
                "sterile pyuria",
                "stopping the flow of urine",
                "swelling at opening of urinary tract",
                "swelling between butt and genitals",
                "swollen scrotum",
                "swollen testicle",
                "tenderness of private parts",
                "testicle feels squishy",
                "testicle riding too high",
                "testicles feel tight",
                "testicles hurt to touch",
                "testicles never fully developed",
                "tight scrotum",
                "trouble starting to pee",
                "unable to pee",
                "uncircumcised penis",
                "undescended testicles",
                "ureteral mass",
                "urethral fistula",
                "urethral meatus protrusion",
                "urethral obstruction",
                "urethral pain",
                "urinary incontinence",
                "urinary tract abnormality",
                "urinary tract infection",
                "urinary tract obstruction",
                "urinating less",
                "urinating stool",
                "vas deferens swelling",
                "vas deferens tenderness",
                "weak pee stream",
                "wet dream",
            ],
            Self::Thighs => &[
                "back of upper leg is weak",
                "burning feeling on thigh",
                "can't feel hot or cold on thigh",
                "cramp in thigh muscle",
                "dahl's sign positive",
                "fat thigh",
                "itching thigh",
                "large thigh muscle",
                "movement of upper leg outward",
                "numb thigh muscle",
                "pain in thigh",
                "popping sound when turn thigh outward",
                "red thigh",
                "thigh muscle feels firm",
                "thigh muscle mass",
                "thigh twitching",
                "weak thigh muscle",
            ],
            Self::Knees => &[
                "back of knee hurts",
                "back of knee is swollen",
                "can feel small lump in knee",
                "clutton joints",
                "darkened skin on knee",
                "dislocated knee",
                "flaky bump(s) limited to elbows or knees",
                "front of knee hurts",
                "front of knee is swollen",
                "genu valgum",
                "genu varum",
                "hurts to kneel",
                "hurts to walk",
                "inflamed fluid sac in knee",
                "inside edge of knee is swollen",
                "joint fluid swelling of back off knee joint",
                "knee cracking when moving",
                "knee feels like it is slipping",
                "knee gets stuck when moving",
                "knee hurts",
                "knee instability",
                "knee is able to bend",
                "knee joint inflammation",
                "knee joint makes popping sounds",
                "knee tender to touch",
                "lachman test positive",
                "lump on knee",
                "mcmurray test positive",
                "outer side of knee hurts",
                "pain on inside edge of knee",
                "patellar tendon reflex absent",
                "patellar tendon reflex decreased",
                "patellar tendon reflex increased",
                "pulsating lump around knee",
                "single flaky raised skin patch on elbows or knees",
                "stiff knee",
                "swollen knee",
                "tibial tuberosity tenderness",
                "trouble moving knee",
                "weak knee muscle",
            ],
            Self::LowerLegs => &[
                "calf muscle cramp",
                "calf muscle feels hard",
                "calf muscle is larger than normal",
                "calf pain",
                "calf swelling",
                "hurts to walk",
                "peroneal sign positive",
                "ridges on shin bone",
                "sharp forward bowing of shin",
                "tender calf muscle",
                "tibial bone mass",
                "tibial deformity",
                "tibial pulse absence",
                "weakness in lower legs",
            ],
            Self::Ankles => &[
                "achilles areflexia",
                "ankle is overly flexible",
                "ankle pain",
                "ankle redness",
                "ankle reflex decreased",
                "ankle swollen",
                "arthritis in ankle",
                "bruise on ankle",
                "lump on ankle",
            ],
            Self::Feet => &[
                "arthritis in the arch of foot",
                "ball of foot joint hurts when move",
                "bottom of foot pain",
                "bottom of foot peeling",
                "bottom of foot red",
                "bottom of foot sweats more",
                "bottom of foot swelling",
                "bottom of foot yellow",
                "can't hold foot up",
                "charcot joint",
                "clubfoot",
                "extreme arch in foot",
                "flat feet",
                "foot changing color",
                "foot deformity",
                "foot feels cold",
                "foot feels hot or warm",
                "foot feels stiff",
                "foot feels weak",
                "foot hurts",
                "foot is numb",
                "foot is turned out",
                "foot is turned up",
                "foot muscle is thinning",
                "foot peeling",
                "foot pulse absence",
                "foot smallness",
                "foot turning blue",
                "foot turning red",
                "heel hurts",
                "heel is swollen",
                "heel is turning in",
                "heel is turning out",
                "heel spur",
                "heel tenderness",
                "hives on foot",
                "infected lump or sore on foot",
                "itchy foot",
                "large feet",
                "matles test positive",
                "metacarpal shortness",
                "open sore(s) on foot",
                "open sore(s) on soles of feet",
                "pain in the arch of foot",
                "pes cavus",
                "plantar reflex, absent",
                "rash limited to feet",
                "rash limited to soles of feet",
                "rocker bottom feet",
                "swelling in the arch of foot",
                "swollen foot",
                "tingling or prickling in foot",
                "tripping",
                "trouble moving foot",
            ],
            Self::Toes => &[
                "arthritis in big toe joint",
                "big toe bends too far up",
                "big toe hurts",
                "big toe hurts when moving",
                "big toe is under the second toe",
                "big toe joint is swollen",
                "big toe joint is stiff",
                "big toe joint is swollen",
                "big toe joint is tender to touch",
                "enlarged rounded toe",
                "feels like toe is burning",
                "great toe metatarsophalangeal prominence",
                "great toe microdactyly",
                "great toe synostosis",
                "nail loss",
                "nail not growing the way it should",
                "nail pulling away from cuticle",
                "pale toe",
                "pigeon toed",
                "rash limited to between toes",
                "stiff big toe",
                "tingling and prickling in toe",
                "toe angle cleft",
                "toe deformity",
                "toe pain",
                "toe pulse absence",
                "toe pulse weakness",
                "toe shortness",
                "up-going toe",
            ],
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct BodyMapProps {
    pub selected: Option<BodyParts>,
    pub on_select: EventHandler<BodyParts>,
}

#[component]
pub fn BodyMap(props: BodyMapProps) -> Element {
    let selected = props.selected;

    let class_for = |part: BodyParts| {
        if selected == Some(part) {
            "body-part selected"
        } else {
            "body-part"
        }
    };

    rsx! {
        svg {
            class: "body-map",
            view_box:  "0 0 400 840",
            role: "img",
            "aria-label": "Interactive body map selector",

            // SCALP
            path {
                class: "{class_for(BodyParts::Scalp)}",
                d: "
                        M150,129
                        C150,122 155,116 163,112
                        C174,106 186,103 200,103
                        C214,103 226,106 237,112
                        C245,116 250,122 250,129
                        C237,134 220,137 200,137
                        C180,137 163,134 150,129
                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Scalp)
            }

            // FOREHEAD
            path {
                class: "{class_for(BodyParts::Forehead)}",
                d: "
                        M150,129
                        C165,134 182,137 200,137
                        C218,137 235,134 250,129
                        L250,153
                        C235,160 218,164 200,164
                        C182,164 165,160 150,153
                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Forehead)
            }

            // LEFT EAR
            path {
                class: "{class_for(BodyParts::Ears)}",
                d: "
                    M151,150
                    C144,148 138,154 138,163
                    C138,174 142,186 148,192
                    C153,196 158,192 159,186
                    C160,180 156,175 155,170
                    C154,166 157,162 158,158
                    C159,154 155,151 151,150
                    Z
                ",
                onclick: move |_| props.on_select.call(BodyParts::Ears)
            }

            // RIGHT EAR
            path {
                class: "{class_for(BodyParts::Ears)}",
                d: "
                    M249,150
                    C256,148 262,154 262,163
                    C262,174 258,186 252,192
                    C247,196 242,192 241,186
                    C240,180 244,175 245,170
                    C246,166 243,162 242,158
                    C241,154 245,151 249,150
                    Z
                ",
                onclick: move |_| props.on_select.call(BodyParts::Ears)
            }

            // NECK
            path {
                class: "{class_for(BodyParts::Neck)}",
                d: "
                        M174,225

                        L174,238
                        L168,253
                        L142,286

                        L258,286
                        L232,253
                        L226,238
                        L226,225

                        C218,234 209,240 200,242
                        C191,240 182,234 174,225

                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Neck)
            }

            // FACE
            path {
                class: "{class_for(BodyParts::Face)}",
                d: "
                    M154,153
                    C151,165 152,181 156,195
                    C158,210 164,222 174,234
                    C181,243 189,248 200,250
                    C211,248 219,243 226,234
                    C236,222 242,210 244,195
                    C248,181 249,165 246,153

                    C233,160 217,164 200,164
                    C183,164 167,160 154,153
                    Z
                ",
                onclick: move |_| props.on_select.call(BodyParts::Face)
            }

            // LEFT EYE
            path {
                class: "{class_for(BodyParts::Eyes)}",
                d: "
                        M160,171
                        C169,165.5 181,165.5 191,171
                        C181,176.5 169,176.5 160,171
                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Eyes)
            }

            // RIGHT EYE
            path {
                class: "{class_for(BodyParts::Eyes)}",
                d: "
                        M240,171
                        C231,165.5 219,165.5 209,171
                        C219,176.5 231,176.5 240,171
                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Eyes)
            }

            // NOSE
            path {
                class: "{class_for(BodyParts::Nose)}",
                d: "
                        M200,166

                        C196,166 193,171 193,178
                        C193,184 195,190 195,195

                        C195,201 192,206 188,211

                        C186,214 190,217 195,217
                        L205,217

                        C210,217 214,214 212,211

                        C208,206 205,201 205,195
                        C205,190 207,184 207,178

                        C207,171 204,166 200,166

                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Nose)
            }

            // MOUTH
            path {
                class: "{class_for(BodyParts::Mouth)}",
                d: "
                        M181,224

                        C187,221 194,221 200,223
                        C206,221 213,221 219,224

                        C215,231 208,235 200,235
                        C192,235 185,231 181,224

                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Mouth)
            }

            // UPPER CHEST
            path {
                class: "{class_for(BodyParts::UpperChest)}",
                d: "
                        M130,286

                        C143,284 158,283 174,283
                        C185,284 193,286 200,289
                        C207,286 215,284 226,283
                        C242,283 257,284 270,286

                        L274,301

                        C259,304 243,305 226,305
                        C215,304 207,302 200,299
                        C193,302 185,304 174,305
                        C157,305 141,304 126,301

                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::UpperChest)
            }

            // LEFT UPPER ARM
            path {
                class: "{class_for(BodyParts::UpperArms)}",
                d: "
                    M112,295
                    C104,297 98,302 92,309
                    C85,318 78,329 72,341
                    C66,353 60,365 55,376
                    C51,384 51,391 55,397
                    C59,403 66,405 72,402
                    C81,398 89,392 97,384
                    C105,376 113,368 120,359
                    C124,352 126,344 125,335
                    C124,320 119,306 112,295
                    Z
                ",
                onclick: move |_| props.on_select.call(BodyParts::UpperArms)
            }

            // RIGHT UPPER ARM
            path {
                class: "{class_for(BodyParts::UpperArms)}",
                d: "
                    M288,295
                    C296,297 302,302 308,309
                    C315,318 322,329 328,341
                    C334,353 340,365 345,376
                    C349,384 349,391 345,397
                    C341,403 334,405 328,402
                    C319,398 311,392 303,384
                    C295,376 287,368 280,359
                    C276,352 274,344 275,335
                    C276,320 281,306 288,295
                    Z
                ",
                onclick: move |_| props.on_select.call(BodyParts::UpperArms)
            }

            // LEFT ELBOW
            path {
                class: "{class_for(BodyParts::Elbows)}",
                d: "
                        M64,390
                        C57,391 52,395 50,401
                        C48,407 51,413 57,416
                        C63,419 70,417 75,413
                        C80,409 82,403 81,397
                        C76,393 70,391 64,390
                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Elbows)
            }

            // RIGHT ELBOW
            path {
                class: "{class_for(BodyParts::Elbows)}",
                d: "
                        M336,390
                        C343,391 348,395 350,401
                        C352,407 349,413 343,416
                        C337,419 330,417 325,413
                        C320,409 318,403 319,397
                        C324,393 330,391 336,390
                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Elbows)
            }

            // LEFT FOREARM
            path {
                class: "{class_for(BodyParts::Forearms)}",
                d: "
                        M52,416
                        C49,426 48,438 49,449
                        C50,461 52,473 54,485
                        C55,492 59,497 65,499
                        C71,500 77,497 80,492
                        C82,482 83,470 83,458
                        C83,446 82,433 76,419
                        C70,418 58,419 52,416
                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Forearms)
            }

            // LEFT WRIST
            path {
                class: "{class_for(BodyParts::Wrists)}",
                d: "
                        M80,485
                        C80,492 77,498 72,501
                        C67,503 61,501 57,497
                        C55,494 54,492 54,489
                        C61,492 73,492 80,485
                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Wrists)
            }

            // LEFT HAND
            path {
                class: "{class_for(BodyParts::Hands)}",
                d: "
                        M72,501
                        C78,505 83,511 86,518
                        C89,525 89,533 85,539
                        C81,545 74,547 68,544
                        C62,541 57,536 54,529
                        C51,522 51,513 57,497
                        C61,501 67,503 72,501
                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Hands)
            }

            // LEFT THUMB
            path {
                class: "{class_for(BodyParts::Fingers)}",
                d: "
                        M72,502
                        C77,499 82,501 85,505
                        C88,509 87,514 83,517
                        C79,519 74,517 71,514
                        C68,511 68,506 72,502
                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Fingers)
            }

            // LEFT FINGERS
            path {
                class: "{class_for(BodyParts::Fingers)}",
                d: "
                        M84,517
                        C89,515 93,516 95,519
                        C97,522 95,525 91,526
                        L83,527
                        C87,528 90,531 90,534
                        C90,537 87,539 84,539
                        L77,536
                        C80,539 80,542 78,544
                        C76,546 73,546 70,543
                        L66,538
                        C67,542 65,544 62,544
                        C59,543 57,540 56,536
                        L54,529
                        C58,532 62,533 66,532
                        C72,531 78,526 84,517
                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Fingers)
            }

            // RIGHT FOREARM
            path {
                class: "{class_for(BodyParts::Forearms)}",
                d: "
                        M348,416
                        C351,426 352,438 351,449
                        C350,461 348,473 346,485
                        C345,492 341,497 335,499
                        C329,500 323,497 320,492
                        C318,482 317,470 317,458
                        C317,446 318,433 324,419
                        C330,418 342,419 348,416
                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Forearms)
            }

            // RIGHT WRIST
            path {
                class: "{class_for(BodyParts::Wrists)}",
                d: "
                        M320,485
                        C320,492 323,498 328,501
                        C333,503 339,501 343,497
                        C345,494 346,492 346,489
                        C339,492 327,492 320,485
                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Wrists)
            }

            // RIGHT HAND
            path {
                class: "{class_for(BodyParts::Hands)}",
                d: "
                        M328,501
                        C322,505 317,511 314,518
                        C311,525 311,533 315,539
                        C319,545 326,547 332,544
                        C338,541 343,536 346,529
                        C349,522 349,513 343,497
                        C339,501 333,503 328,501
                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Hands)
            }

            // RIGHT THUMB
            path {
                class: "{class_for(BodyParts::Fingers)}",
                d: "
                        M328,502
                        C323,499 318,501 315,505
                        C312,509 313,514 317,517
                        C321,519 326,517 329,514
                        C332,511 332,506 328,502
                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Fingers)
            }

            // RIGHT FINGERS
            path {
                class: "{class_for(BodyParts::Fingers)}",
                d: "
                        M316,517
                        C311,515 307,516 305,519
                        C303,522 305,525 309,526
                        L317,527
                        C313,528 310,531 310,534
                        C310,537 313,539 316,539
                        L323,536
                        C320,539 320,542 322,544
                        C324,546 327,546 330,543
                        L334,538
                        C333,542 335,544 338,544
                        C341,543 343,540 344,536
                        L346,529
                        C342,532 338,533 334,532
                        C328,531 322,526 316,517
                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Fingers)
            }
            // RIGHT THUMB
            path {
                class: "{class_for(BodyParts::Fingers)}",
                d: "
                        M328,502
                        C323,499 318,501 315,505
                        C312,509 313,514 317,517
                        C321,519 326,517 329,514
                        C332,511 332,506 328,502
                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Fingers)
            }

            // RIGHT FINGERS
            path {
                class: "{class_for(BodyParts::Fingers)}",
                d: "
                        M316,517
                        C311,515 307,516 305,519
                        C303,522 305,525 309,526
                        L317,527
                        C313,528 310,531 310,534
                        C310,537 313,539 316,539
                        L323,536
                        C320,539 320,542 322,544
                        C324,546 327,546 330,543
                        L334,538
                        C333,542 335,544 338,544
                        C341,543 343,540 344,536
                        L346,529
                        C342,532 338,533 334,532
                        C328,531 322,526 316,517
                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Fingers)
            }

            // LEFT PECTORAL
            path {
                class: "{class_for(BodyParts::Breasts)}",
                d: "
                        M125,312
                        C143,305 165,304 181,308
                        C188,310 193,313 197,317
                        L197,351
                        C188,357 176,360 163,359
                        C149,358 137,353 131,345
                        C127,336 125,324 125,312
                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Breasts)
            }

            // RIGHT PECTORAL
            path {
                class: "{class_for(BodyParts::Breasts)}",
                d: "
                        M275,312
                        C257,305 235,304 219,308
                        C212,310 207,313 203,317
                        L203,351
                        C212,357 224,360 237,359
                        C251,358 263,353 269,345
                        C273,336 275,324 275,312
                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Breasts)
            }

            // LEFT SHOULDER
            path {
                class: "{class_for(BodyParts::Shoulders)}",
                d: "
                    M130,286
                    C120,286 110,290 103,296
                    C93,302 85,310 81,319
                    C86,323 94,324 102,322
                    C113,320 123,315 134,307
                    L140,299
                    Z
                ",
                onclick: move |_| props.on_select.call(BodyParts::Shoulders)
            }

            // RIGHT SHOULDER
            path {
                class: "{class_for(BodyParts::Shoulders)}",
                d: "
                    M270,286
                    C280,286 290,290 297,296
                    C307,302 315,310 319,319
                    C314,323 306,324 298,322
                    C287,320 277,315 266,307
                    L260,299
                    Z
                ",
                onclick: move |_| props.on_select.call(BodyParts::Shoulders)
            }

            // STERNUM
            path {
                class: "{class_for(BodyParts::Sternum)}",
                d: "
                        M194,300
                        C197,301 203,301 206,300

                        L204,321
                        C203,329 203,336 204,343
                        C205,349 204,354 200,358
                        C196,354 195,349 196,343
                        C197,336 197,329 196,321

                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Sternum)
            }

            // CHEST
            path {
                class: "{class_for(BodyParts::Chest)}",
                d: "
                        M128,354

                        C143,358 159,359 174,359
                        C184,359 193,357 200,354
                        C207,357 216,359 226,359
                        C241,359 257,358 272,354

                        L270,375

                        C255,378 241,379 226,378
                        C215,377 207,375 200,372
                        C193,375 185,377 174,378
                        C159,379 145,378 130,375

                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Chest)
            }

            // PELVIS
            path {
                class: "{class_for(BodyParts::Pelvis)}",
                d: "
                    M133,500
                    L267,500

                    C265,488 261,476 256,466
                    C248,458 233,452 221,448
                    C211,446 205,445 200,445
                    C195,445 189,446 179,448
                    C167,452 152,458 144,466
                    C139,476 135,488 133,500

                    Z
                ",
                onclick: move |_| props.on_select.call(BodyParts::Pelvis)
            }

            // ABDOMEN
            path {
                class: "{class_for(BodyParts::Abdomen)}",
                d: "
                    M130,375

                    C145,378 160,379 175,379
                    C185,379 193,377 200,374
                    C207,377 215,379 225,379
                    C240,379 255,378 270,375

                    C269,390 267,405 265,420
                    C263,434 260,447 256,458

                    C244,463 229,468 214,471
                    C208,473 204,475 200,476
                    C196,475 192,473 186,471
                    C171,468 156,463 144,458

                    C140,447 137,434 135,420
                    C133,405 131,390 130,375

                    Z
                ",
                onclick: move |_| props.on_select.call(BodyParts::Abdomen)
            }

            // GENITALS
            path {
                class: "{class_for(BodyParts::Genitals)}",
                d: "
                    M177,500

                    C184,502 192,503 200,503
                    C208,503 216,502 223,500

                    C220,509 216,516 210,522
                    C206,525 203,527 200,529
                    C197,527 194,525 190,522
                    C184,516 180,509 177,500

                    Z
                ",
                onclick: move |_| props.on_select.call(BodyParts::Genitals)
            }

            // LEFT THIGH
            path {
                class: "{class_for(BodyParts::Thighs)}",
                d: "
                        M157,486

                        C146,489 135,495 128,504
                        C121,516 119,533 120,551
                        L124,610

                        C133,615 145,617 157,616
                        C169,615 179,611 187,603
                        C191,589 192,570 191,550
                        C190,530 185,511 178,498
                        C171,491 164,487 157,486

                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Thighs)
            }

            // RIGHT THIGH
            path {
                class: "{class_for(BodyParts::Thighs)}",
                d: "
                        M243,486

                        C254,489 265,495 272,504
                        C279,516 281,533 280,551
                        L276,610

                        C267,615 255,617 243,616
                        C231,615 221,611 213,603
                        C209,589 208,570 209,550
                        C210,530 215,511 222,498
                        C229,491 236,487 243,486

                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Thighs)
            }

            // LEFT KNEE
            path {
                class: "{class_for(BodyParts::Knees)}",
                d: "
                        M124,610

                        C132,607 144,607 156,610
                        C166,612 174,617 180,624
                        C182,633 181,643 177,651
                        C171,658 162,662 151,662
                        C140,662 130,658 124,651
                        C120,642 120,632 124,610

                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Knees)
            }

            // RIGHT KNEE
            path {
                class: "{class_for(BodyParts::Knees)}",
                d: "
                        M276,610

                        C268,607 256,607 244,610
                        C234,612 226,617 220,624
                        C218,633 219,643 223,651
                        C229,658 238,662 249,662
                        C260,662 270,658 276,651
                        C280,642 280,632 276,610

                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Knees)
            }

            // LEFT SHIN
            path {
                class: "{class_for(BodyParts::LowerLegs)}",
                d: "
                        M124,651

                        C132,656 141,658 151,658
                        C161,658 170,656 177,651

                        C174,670 171,690 169,710
                        C168,729 168,748 170,766

                        C162,770 151,771 142,768
                        C137,750 134,730 132,710
                        C130,689 128,669 124,651

                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::LowerLegs)
            }

            // RIGHT SHIN
            path {
                class: "{class_for(BodyParts::LowerLegs)}",
                d: "
                        M276,651

                        C268,656 259,658 249,658
                        C239,658 230,656 223,651

                        C226,670 229,690 231,710
                        C232,729 232,748 230,766

                        C238,770 249,771 258,768
                        C263,750 266,730 268,710
                        C270,689 272,669 276,651

                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::LowerLegs)
            }

            // LEFT ANKLE
            path {
                class: "{class_for(BodyParts::Ankles)}",
                d: "
                        M142,766
                        C149,769 160,769 170,766
                        L169,779
                        C162,783 151,784 143,780
                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Ankles)
            }

            // RIGHT ANKLE
            path {
                class: "{class_for(BodyParts::Ankles)}",
                d: "
                        M230,766
                        C240,769 251,769 258,766
                        L257,780
                        C249,784 238,783 231,779
                        Z
                    ",
                onclick: move |_| props.on_select.call(BodyParts::Ankles)
            }

            // LEFT FOOT
            path {
                class: "{class_for(BodyParts::Feet)}",
                d: "
                    M143,780
                    C151,782 161,782 169,779

                    C169,790 168,799 165,806
                    C162,812 157,817 151,820

                    C143,823 132,823 125,820
                    C121,818 120,814 122,810
                    C125,800 133,789 143,780

                    Z
                ",
                onclick: move |_| props.on_select.call(BodyParts::Feet)
            }

            // RIGHT FOOT
            path {
                class: "{class_for(BodyParts::Feet)}",
                d: "
                    M257,780
                    C249,782 239,782 231,779

                    C231,790 232,799 235,806
                    C238,812 243,817 249,820

                    C257,823 268,823 275,820
                    C279,818 280,814 278,810
                    C275,800 267,789 257,780

                    Z
                ",
                onclick: move |_| props.on_select.call(BodyParts::Feet)
            }

            // LEFT TOES
            path {
                class: "{class_for(BodyParts::Toes)}",
                d: "
                    M238,818
                    C241,817 245,818 248,820
                    C250,822 250,825 248,827
                    C246,829 242,829 239,827
                    C237,825 236,821 238,818
                    Z

                    M249,820
                    C252,819 255,820 257,822
                    C259,824 258,827 256,829
                    C254,831 251,830 249,828
                    C247,826 247,822 249,820
                    Z

                    M258,821
                    C261,820 264,821 265,823
                    C267,825 266,828 264,830
                    C262,831 259,830 258,828
                    C256,826 256,823 258,821
                    Z

                    M267,822
                    C270,821 272,822 274,824
                    C275,826 274,829 272,830
                    C270,832 267,830 266,828
                    C265,826 265,824 267,822
                    Z

                    M276,823
                    C278,822 281,823 282,825
                    C283,827 282,829 280,830
                    C278,831 276,830 275,828
                    C274,826 274,824 276,823
                    Z
                ",
                onclick: move |_| props.on_select.call(BodyParts::Toes)
            }
            // RIGHT TOES
            path {
                class: "{class_for(BodyParts::Toes)}",
                d: "
                    M162,818
                    C159,817 155,818 152,820
                    C150,822 150,825 152,827
                    C154,829 158,829 161,827
                    C163,825 164,821 162,818
                    Z

                    M151,820
                    C148,819 145,820 143,822
                    C141,824 142,827 144,829
                    C146,831 149,830 151,828
                    C153,826 153,822 151,820
                    Z

                    M142,821
                    C139,820 136,821 135,823
                    C133,825 134,828 136,830
                    C138,831 141,830 142,828
                    C144,826 144,823 142,821
                    Z

                    M133,822
                    C130,821 128,822 126,824
                    C125,826 126,829 128,830
                    C130,832 133,830 134,828
                    C135,826 135,824 133,822
                    Z

                    M124,823
                    C122,822 119,823 118,825
                    C117,827 118,829 120,830
                    C122,831 124,830 125,828
                    C126,826 126,824 124,823
                    Z
                ",
                onclick: move |_| props.on_select.call(BodyParts::Toes)
            }
        }
    }
}
