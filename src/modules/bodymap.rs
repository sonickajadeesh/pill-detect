use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodyParts {
    Common,
    Skin,
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
    Back,
    Buttocks,
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
            Self::Common => "Common",
            Self::Skin => "Skin",
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
            Self::Back => "Back",
            Self::Buttocks => "Buttocks",
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
            Self::Common => &[
                "Absence seizure",
                "Anxiety",
                "Arm or leg is weak",
                "Arm or leg itching",
                "Arm or leg muscle spasm on one side",
                "Arm or leg shaking",
                "Arthritis in many joints that moves around",
                "Arthritis of joints of hands and feet",
                "Autism",
                "Bad body odor",
                "Binge eating",
                "Bleeding beneath nail",
                "Bleeding into the muscle",
                "Blister(s) between fingers",
                "Blood vessel inflammation",
                "Blue or white extremities due to lack of blood flow",
                "Body is swollen all over",
                "Body piercing",
                "Body temperature goes up and down",
                "Bone pain",
                "Bone tenderness",
                "Burning feeling in arm or leg",
                "Can't feel hot or cold in fingers or toes",
                "Can't feel pain",
                "Can't feel things as well",
                "Can't move arm or leg",
                "Can't move arm or leg for long period",
                "Can't stand the heat",
                "Can't sweat",
                "Chills",
                "Chlorpropamide ingestion",
                "Chronic pain",
                "Cold sensation",
                "Cold sensitivity",
                "Cold sore",
                "Coma",
                "Constant fever with varying temperatures",
                "Dactylitis (finger or toe inflammation)",
                "Dazed",
                "Dehydration",
                "Dizziness",
                "Easily distracted",
                "Easily irritated",
                "Easy bleeding",
                "Enlarged vein",
                "Exposure to heat",
                "Extremity areflexia",
                "Extremity asterixis",
                "Extremity cold",
                "Extremity deformity",
                "Extremity hyporeflexia",
                "Extremity swelling",
                "Fatigue",
                "Feeling flat or dull",
                "Feeling hot",
                "Feels like bugs crawling on skin",
                "Female getting male characteristics",
                "Fever",
                "Fever 103f to 104f",
                "Fever below 100.4f",
                "Fever extreme above 106f",
                "Flaky bump(s) limited to elbows or knees",
                "Fluid in the joint",
                "Generalized lymphadenopathy",
                "Generalized muscle weakness",
                "Half of body feels numb",
                "Hard lump beneath skin",
                "High blood sugar with insulin resistance",
                "Hot flashes",
                "Hungry a lot more",
                "Hurts to move",
                "Impulsive behavior",
                "Inflamed blood vessels with rheumatoid arthritis",
                "Inflammation of a bursa",
                "Insect bite",
                "Insomnia",
                "Interphalangeal synostosis",
                "Involuntary gross movement",
                "Irregular heartbeat",
                "Itchy red eczema-like rash often allergic",
                "Jerky involuntary movements",
                "Jittery",
                "Joint bends too much",
                "Joint cracking",
                "Joint inflammation",
                "Joint inflammation in 1-4 joints",
                "Joint inflammation in 5 or more joints",
                "Joint inflammation in one joint",
                "Joint warmth",
                "Knuckles of hands or toes hurts",
                "Large single lump",
                "Laying on the ground exhausted",
                "Lethargy",
                "Local or widespread shingles",
                "Loss of appetite",
                "Loss of balance",
                "Loss of blood supply to finger or toe",
                "Low body temperature",
                "Lump under skin",
                "Lymph node firmness",
                "Lymphangitis",
                "Lymphatic nodule",
                "Lymphedema",
                "Movements are slow",
                "Muscle cramp",
                "Muscle cramps at night",
                "Muscle increase in size",
                "Muscle inflammation",
                "Muscle pain all over",
                "Muscle spasm",
                "Muscle spasm in arm or leg",
                "Muscle spasm in hands or feet",
                "Muscle spasm with repeating contractions",
                "Muscle weakness",
                "Muscle-building steroid use",
                "Muscles getting weaker over time",
                "Muscles shrinking",
                "Myoclonus",
                "Nausea",
                "Nervousness",
                "Night sweats",
                "No muscle tone in arms and legs",
                "Not able to tell what's causing the fever",
                "Not keeping up with self-care",
                "Numbness",
                "Obese",
                "Orange skin on palms or feet",
                "Osler node",
                "Osteoarthritis",
                "Pain in arm or leg",
                "Pain in last knuckle of fingers or toes",
                "Pain in middle knuckle on fingers or toes",
                "Painful lump beneath skin",
                "Painless lymphadenopathy",
                "Paralysis",
                "Pass out",
                "Polyarticular",
                "Poor nutrition",
                "Premature aging of skin",
                "Psoriasis",
                "Really sleepy during day",
                "Recurring fever",
                "Recurring seizures",
                "Red or purple raised spots",
                "Red skin on arms or legs",
                "Regional lymph node enlargement",
                "Sadness",
                "Scratching",
                "Seizure",
                "Self injury",
                "Sensitive to pain",
                "Several painful glands in a region",
                "Severe weight loss",
                "Severely underweight",
                "Shakiness, slowness, stiffness and unsteadiness",
                "Shaking",
                "Single flaky raised skin patch on elbows or knees",
                "Skin is really sensitive",
                "Skinny arms and legs",
                "Slight weight loss",
                "Small flat red or purple spots",
                "Stiff arm or leg",
                "Stiff knuckles in hands or toes",
                "Stiff muscles",
                "Stressed out",
                "Subcutaneous fat preservation",
                "Suddenly falling to the ground",
                "Swollen joint",
                "Swollen knuckles in hands or toes",
                "Synkinesia",
                "Temperature sensation distortion",
                "Tender glands",
                "Tender joints",
                "Tender lump beneath skin",
                "Tender muscle",
                "Tendon inflammation",
                "Tic",
                "Tingling or pricking of arm or leg",
                "Tingling or prickling feeling",
                "Too much hair on body",
                "Tough or thick skinned pea-sized lump(s) on joint",
                "Tullio phenomenon",
                "Uncoordinated",
                "Underweight",
                "Unsteady with eyes closed",
                "Wake up feeling stiff",
                "Weakness on one side of body",
                "Weight gain",
                "Weight loss",
                "White spots on nail",
                "Worse in the morning",
                "Worse with activity",
                "Yellow material under nail",
            ],
            Self::Skin => &[
                "Abnormal skin",
                "Acne-like rash in one area",
                "Air under the skin",
                "Area of red skin",
                "Beau's lines",
                "Black colored skin",
                "Black open sore(s)",
                "Bleeding open sore(s)",
                "Bleeding skin sore(s)",
                "Blotchy skin",
                "Blotchy skin on arm or leg",
                "Blue skin comes and goes",
                "Bluish-red flat skin patch(es) in one area",
                "Brittle hair",
                "Bronze colored skin",
                "Brown flat skin patch(es) all over",
                "Brown line on nail",
                "Brown or black flat skin patch(es) in one area",
                "Brown or black, tough or thick skin all over",
                "Brown or black, tough or thick skin in one area",
                "Brown skin discoloration",
                "Bruise",
                "Bruise easily",
                "Bump(s) in ring shape in one area",
                "Bump(s) limited to scalp",
                "Bump(s) merging together in one area",
                "Bump(s) with clear borders all over",
                "Bump(s) with clear borders in one area",
                "Bump(s) with indented center in one area",
                "Burn on skin",
                "Burning pain in wound",
                "Burning sensation on skin",
                "Calcium deposits under skin",
                "Changes in color of skin",
                "Circular dry flat skin patch(es) in one area",
                "Clearly outlined tough or thick skin all over",
                "Coarse hair",
                "Complete loss of skin color all over",
                "Cracking or popping under skin",
                "Cracks in skin",
                "Creeping, spreading bump(s) in one area",
                "Creeping, spreading rash in one area",
                "Crusty bump(s) in one area",
                "Crusty bump(s) limited to scalp",
                "Crusty pus-filled bump(s) in one area",
                "Cyst-like acne in one area",
                "Darier's sign",
                "Darkened skin",
                "Darkened skin all over",
                "Darkened skin in folds",
                "Darkened skin in one area",
                "Darkened skin limited to lining of mouth",
                "Deep pus-filled bump(s) in hair follicle(s)",
                "Different colored streak on nail",
                "Dry cracked skin between fingers",
                "Dry scalp",
                "Dry scaly skin sore(s)",
                "Dry scaly thick skin",
                "Dry skin",
                "Dry skin in one area",
                "Dry skin limited to scalp",
                "Early grey hair",
                "Eunuchoidism",
                "Extra stretchy skin",
                "Flaking of skin all over",
                "Flaking of skin in one area",
                "Flaky bump(s) all over",
                "Flaky bump(s) in one area",
                "Flaky bump(s) limited to elbows or knees",
                "Flaky flat skin patch(es) in one area",
                "Flaky on edges, raised skin patch(es) all over",
                "Flaky on edges, raised skin patch(es) in one area",
                "Flaky raised skin patch(es) all over",
                "Flaky raised skin patch(es) in one area",
                "Flaky raised skin patch(es) limited to elbows or knees",
                "Flaky raised skin patch(es) limited to scalp",
                "Flaky skin sore(s)",
                "Flat skin patch(es) with clear borders all over",
                "Flat skin patch(es) with clear borders in one area",
                "Flat, light brown pigmented birthmarks",
                "Fragile or thin skin",
                "Fragile or thin skin all over",
                "Fragile or thin skin in one area",
                "Fragile skin",
                "Generalized hyperkeratosis",
                "Generalized plaques, poikiloderma",
                "Generalized scaling of skin, sparing face and flexures",
                "Generalized shiny wickham's striae papules",
                "Generalized skin depigmentation, follicular",
                "Grooves running across nail",
                "Grooves running lengthwise down nail",
                "Hair abnormality",
                "Hair dryness",
                "Hair finding",
                "Hair sparse",
                "Hard lumpy red rash doesn't go away",
                "Hard skin sore(s)",
                "Hive-like rash all over",
                "Hive-like rash in one area",
                "Hives",
                "Increased amount of sweating",
                "Irritated skin",
                "Itching",
                "Itching after bath",
                "Itchy blister(s) all over",
                "Itchy bump(s) all over",
                "Itchy bump(s) in one area",
                "Itchy scalp",
                "Jaundice",
                "Jaundice keeps coming back",
                "Knoblike skin growth",
                "Koebner's phenomenon",
                "Koebnerization",
                "Large bleeding blister(s) in one area",
                "Large blister(s) all over",
                "Large blister(s) in one area",
                "Large blister(s) limited to skin creases",
                "Large crusting blister(s) all over",
                "Large crusting blister(s) in one area",
                "Large darkened flat skin patch(es) all over",
                "Large darkened flat skin patch(es) in one area",
                "Large darker flaking of skin all over",
                "Large darker flaking of skin in one area",
                "Large deep pea-sized lump(s) in one area",
                "Large elevated skin patch(es) all over",
                "Large elevated skin patch(es) in one area",
                "Large flaky blister(s) all over",
                "Large flaky blister(s) in one area",
                "Large flat skin patch(es) all over",
                "Large flat skin patch(es) in one area",
                "Large itchy blister(s) all over",
                "Large itchy blister(s) in one area",
                "Large raised flaking rash all over with clear borders",
                "Large raised pea-sized lump(s) all over",
                "Large raised pea-sized lump(s) in one area",
                "Large red blister(s) in one area",
                "Large red thick raised skin patch(es) under hair",
                "Lengthwise skin sore(s)",
                "Lichenification",
                "Lightened skin",
                "Localized hyperkeratosis",
                "Localized papules, scaling, auspitz phenomenon",
                "Localized plaques, auspitz phenomenon",
                "Localized plaques, christmas tree distribution",
                "Localized scaling of skin, sparing face / flexures",
                "Localized shiny wickham's striae papules",
                "Localized skin depigmentation, follicular",
                "Loose hanging skin",
                "Losing hair",
                "Loss of hair color",
                "Loss of skin color",
                "Loss of skin color all over",
                "Loss of skin color in one area",
                "Lump on nailbed",
                "Moist skin",
                "Mottled skin",
                "Muehrcke's nails",
                "Multiple bruises of different ages",
                "Multiple target-like or bullseye skin sore(s)",
                "Nail color changing",
                "Nail destruction",
                "Nail doesn't look right",
                "Nail getting thick",
                "Nail is splitting lengthwise",
                "Nail looks deformed",
                "Nail looks dull",
                "Nail looks milky",
                "Nail loss",
                "Nail not growing the way it should",
                "Nail pulling away from cuticle",
                "Nail ridge increase",
                "Nail with jagged edge",
                "Nail, blue lunulae",
                "Nail, hutchinson's sign",
                "Nails fragile or easily breakable",
                "Nails look cloudy and yellow",
                "Nails not growing very well",
                "Nails turn black",
                "Nails turning yellow",
                "No fingernail on pinky or baby toe",
                "Oily skin",
                "Oozing skin sore(s)",
                "Open sore pea-sized lump(s) in one area",
                "Open sore(s)",
                "Open sore(s) in one area",
                "Open sore(s) limited to area over heels or lower back",
                "Open sore(s) limited to tail bone",
                "Open sore(s) on skin",
                "Open sore(s) with irregular borders",
                "Open sores in hair follicle(s)",
                "Orange skin discoloration",
                "Pale skin",
                "Partial loss of skin color all over",
                "Pea-sized lump under skin on arm or leg",
                "Pea-sized lump(s) with raised borders in one area",
                "Peeling skin",
                "Pinkish-red flat skin patch(es) in one area",
                "Problem at roots of hair",
                "Purple skin discoloration",
                "Purplish bump(s) all over",
                "Pus-filled bump(s) all over",
                "Pus-filled bump(s) in hair follicle(s)",
                "Pus-filled bump(s) in one area",
                "Pus-filled infected lump or sore",
                "Raised skin patch(es) with clear borders all over",
                "Raised skin patch(es) with clear borders in one area",
                "Rash after sun exposure in one area",
                "Rash all over",
                "Rash in one area",
                "Rash limited to elbow or knees",
                "Rash limited to legs or arms",
                "Rash looks like bullseye that gets larger over time",
                "Rash oozing fluid or pus in one area",
                "Rash spreads from arms or legs to abdomen",
                "Rash with open sores in one area",
                "Red bump(s) in one area",
                "Red bumps around hair follicles",
                "Red clearly outlined flaking of skin all over",
                "Red clearly outlined flaking of skin in one area",
                "Red flaky bump(s) limited to scalp",
                "Red flaky rash all over",
                "Red flaky rash in one area",
                "Red flat skin patch(es) all over",
                "Red inflamed pus-filled bump(s) all over",
                "Red or brown streak on nail",
                "Red or purple flat spot(s) in one area",
                "Red or purple flat spots all over",
                "Red or purplish rash running across nose and cheeks",
                "Red raised skin patch(es) all over",
                "Red raised skin patch(es) in one area",
                "Red rash all over",
                "Red rash in one area",
                "Red skin",
                "Red sores on face caused by strep or staph bacteria",
                "Reddish inflamed skin all over",
                "Reddish-brown bump(s) all over",
                "Reddish-brown bump(s) in one area",
                "Reddish-brown flat spots all over",
                "Reddish-brown flat spots in one area",
                "Ring or arc shaped redness of skin in one area",
                "Rough skin",
                "Scaling, peeling, and flaking of skin",
                "Scratch mark",
                "Scratch marks all over",
                "Scratch marks in one area",
                "See through skin",
                "Shallow open sore(s)",
                "Shiny skin",
                "Short lasting rash",
                "Shrinking or thinning of nail",
                "Single blister",
                "Single cyst",
                "Single flaky raised skin patch on elbows or knees",
                "Single flat discolored spot with clear borders",
                "Single flat discolored spot with irregular borders",
                "Single flat skin patch with clear borders",
                "Single giant flat skin patch present at birth",
                "Single glossy flat skin patch with spider veins",
                "Single hairy flat skin patch present at birth",
                "Single itchy flat skin patch",
                "Single midline port-wine skin patch",
                "Single multi-colored flat discolored spot",
                "Single multi-colored pea-sized lump",
                "Single pea-sized lump",
                "Single pea-sized lump with irregular borders",
                "Single port-wine skin patch",
                "Single purple raised skin patch on arm or leg",
                "Single raised pus-filled bump",
                "Single red flat discolored spot",
                "Single red raised skin patch",
                "Single scar tissue skin growth",
                "Single spider-like veins port-wine skin patch",
                "Skin deposit",
                "Skin destruction",
                "Skin discoloration",
                "Skin feels cold in one area",
                "Skin feels too cold",
                "Skin feels warm",
                "Skin hurts",
                "Skin infection",
                "Skin infiltration",
                "Skin lesion margin undulation",
                "Skin lesion, fragility",
                "Skin lesion, polygonal",
                "Skin lesion, special",
                "Skin scarring",
                "Skin sore(s) developed at same time",
                "Skin sore(s) developing at different times",
                "Skin sore(s) oozing fluid or pus",
                "Skin sore(s) varying in color",
                "Skin tumor",
                "Skin turgor decrease",
                "Skin ulceration calcified nodule",
                "Skin ulceration, demarcated",
                "Skin ulceration, necrotic",
                "Skin with smooth, soft appearance",
                "Skin wound tender to touch",
                "Small blistery rash all over",
                "Small blistery rash in one area",
                "Small flat discolored spot(s) all over",
                "Small flat discolored spot(s) in one area",
                "Small flat red or purple spots all over",
                "Small glossy pea-sized lump(s) with spider veins in one area",
                "Small grouped together blister(s) in one area",
                "Small herpes-like blister(s) all over",
                "Small herpes-like blister(s) in one area",
                "Small itchy blister(s) in one area",
                "Small pits on nail surface",
                "Small raised bumpy rash all over",
                "Small raised bumpy rash in one area",
                "Small red bump(s) all over",
                "Small red bump(s) that blend together all over",
                "Small red bump(s) that blend together in one area",
                "Small red colored spot under nail",
                "Small red flaky bump(s) all over",
                "Small red flat discolored spot(s) all over",
                "Small red flat discolored spot(s) in one area",
                "Small red inflamed pus-filled bump(s) all over",
                "Smooth skin",
                "Soft downy hair",
                "Soft skin",
                "Sore on skin hurts",
                "Sore skin",
                "Sore(s) with skin breaking down",
                "Spider bite",
                "Spider vein(s) around nails",
                "Spider vein(s) in one area",
                "Spider vein(s) limited to face",
                "Spider vein(s) limited to inside eye",
                "Spider vein(s) limited to inside mouth",
                "Spider vein(s) limited to roof of mouth",
                "Spider veins all over",
                "Spotted darkened skin all over",
                "Spotted darkened skin in one area",
                "Star-shaped skin sore(s)",
                "Stiff joints",
                "Subcutaneous tophus",
                "Sudden yellow pea-sized lump(s) all over",
                "Sudden yellow pea-sized lump(s) in one area",
                "Superficial skin movable",
                "Sweating more on one side of body",
                "Swollen raised skin patch(es)",
                "Target-like or bullseye skin sore",
                "Thick scar tissue",
                "Thick yellow nail",
                "Thin nails",
                "Tick bite",
                "Tightening of skin",
                "Tingling and prickling around wound",
                "Tiny red or purple spots in one area",
                "Tiny red or purple spots limited to lining of mouth",
                "Tough or thick skin all over",
                "Tough or thick skin in one area",
                "Tough or thick skin limited to between shoulder blades",
                "Tough or thick skin limited to body folds",
                "Tough or thick skin limited to hair follicle(s)",
                "Tough or thick skin limited to head and face",
                "Tough or thick skin limited to palms or soles of foot",
                "Tough or thick skin sore",
                "Tough or thick skin sores",
                "Tuberous xanthoma",
                "Turning blue",
                "Vein-like darkened skin all over",
                "Vein-like darkened skin in one area",
                "Very sensitive to pain",
                "Very thin hair",
                "Wart-like skin sore(s)",
                "White flat skin patch(es) limited to inside mouth",
                "White lines go across nail",
                "White or lighter patch(es) of skin",
                "White-centered bump(s) all over",
                "Whitehead(s) and blackhead(s) in one area",
                "Whitish-flesh colored bump(s) all over",
                "Whitish-flesh colored bump(s) in one area",
                "Whole nail turned white",
                "Widespread flaky flat skin patch(es)",
                "Wound pain",
                "Wrinkled skin",
                "Yellow pea-sized lump(s) all over",
                "Yellow pea-sized lump(s) all over on elbows or knees",
                "Yellow pea-sized lump(s) in one area",
                "Yellow pea-sized lump(s) limited to elbows or knees",
                "Yellow raised skin rash",
                "Yellow skin discoloration",
                "Yellow-ringed, flat skin patch(es) in one area",
                "Yellowish-white bump(s) in one area",
            ],
            Self::Scalp => &[
                "Baby's soft spot is bulging",
                "Baby's soft spot is still open",
                "Baby's soft spot is sunken",
                "Baby's soft spot is tight",
                "Bald spots (hair)",
                "Blond hair",
                "Bulging out of back of skull",
                "Clear ringing sound note when tapping the skull",
                "Clogged pores in bald spots",
                "Complete loss of hair over entire body",
                "Completely bald",
                "Cranial osteoma",
                "Craniosynostosis",
                "Cut on scalp",
                "Dandruff",
                "Deformed forehead",
                "Dry scalp",
                "Dull sound when tapping the skull",
                "Early grey hair",
                "Enlarged vein on scalp",
                "Flaky or greasy skin on scalp",
                "Follicular scarring",
                "Forehead bones breaking down",
                "Forehead sticks out",
                "Hair dryness",
                "Hair getting straighter",
                "Hair loss with crusty rash",
                "Hair loss with scarring",
                "Hair sparse",
                "Head lice",
                "Headache",
                "Increased pressure in skull",
                "Intracranial bruit",
                "Itchy scalp",
                "Long hair",
                "Long nails",
                "Losing hair",
                "Losing hair in patch(es)",
                "Low hairline",
                "Lump on scalp",
                "Male pattern baldness",
                "Matted hair",
                "Open pores in bald spots",
                "Open sore(s) on scalp",
                "Pointed head",
                "Pulling out hair",
                "Pus-filled bump(s) in bald spot",
                "Pus-filled bump(s) in scalp hair follicle(s)",
                "Random hairs in bald patch(es)",
                "Random white hairs in bald spots",
                "Rash limited to scalp",
                "Red bumps around hair follicles on scalp",
                "Red hair",
                "Red skin in bald areas",
                "Redness and dry scaly skin with hair loss",
                "Scalp feels overly sensitive",
                "Scalp feels warm",
                "Scalp hurts",
                "Scalp tender to touch",
                "Scalp vessel pulse increase",
                "Seams of skull separate",
                "Shiny bald head",
                "Shiny scalp",
                "Skin on scalp feels thinner",
                "Soft skull",
                "Swollen scalp",
                "Tough or thick skin on scalp",
                "White hair",
                "Widespread loss of hair",
            ],
            Self::Forehead => &[
                "Can't pay attention",
                "Confused thinking and reduced awareness of your environment",
                "Enlarged vein on forehead",
                "Fatigue",
                "Fever",
                "Forehead is tender",
                "Forehead sticks out",
                "Hairy forehead",
                "Hallucination",
                "Headache",
                "Headache in front of head",
                "High forehead",
                "Inappropriate behavior",
                "Lightheadedness",
                "Paranoia",
                "Wide forehead",
                "Wrinkled forehead",
            ],
            Self::Eyes => &[
                "Argyll-robertson pupil",
                "Astigmatism",
                "Black eye",
                "Black spots floating in my eye",
                "Bleeding around the eye",
                "Bleeding eyelid",
                "Bleeding in eye",
                "Bleeding in front part of eye",
                "Blind spot",
                "Blind spot that appears blank",
                "Blind spot that appears dark",
                "Blood vessels in colored part of eye",
                "Blurry vision",
                "Blurry vision in one eye",
                "Brownish-yellowish ring around the color of eye",
                "Bruising around eyes",
                "Bugs in my eye",
                "Bulging eyes",
                "Burn to part of eye",
                "Bushy eyebrows",
                "Can't close eye all the way",
                "Can't focus eyes",
                "Can't look up",
                "Can't move eyes to the side",
                "Can't recognize things i see",
                "Can't turn my eyes",
                "Can't see far away",
                "Can't see up close",
                "Cataract",
                "Central scotoma",
                "Central vision loss",
                "Chorioretinitis",
                "Choroid coloboma",
                "Choroiditis",
                "Cloudy area on cornea",
                "Color blindness",
                "Conjunctival contracture",
                "Conjunctival fold necrosis, yellow white",
                "Conjunctival papillary flatness",
                "Conjunctival papillary hardening",
                "Conjunctival pseudomembrane",
                "Conjunctival smoothness",
                "Conjunctivitis, follicular",
                "Constant red dry eyes",
                "Corneal areflexia",
                "Corneal contracture",
                "Corneal dystrophy",
                "Corneal protrusion",
                "Cross eyed",
                "Decreased tears",
                "Decreased vision",
                "Dilated pupil",
                "Distorted vision",
                "Double vision",
                "Double vision in one eye",
                "Downslanting palpebral fissues",
                "Drooping eyelid",
                "Drusen",
                "Dry eyes",
                "Enlarged brow ridge",
                "Enlarged vein on clear part of eye",
                "Extraocular muscle imbalance",
                "Extraocular muscle weakness",
                "Eye bleeds",
                "Eye blinking",
                "Eye color is changing to blue",
                "Eye contact impairment",
                "Eye discharge",
                "Eye hurts",
                "Eye hurts when i move it",
                "Eye is burning",
                "Eye is red and irritated",
                "Eye not in normal position in socket",
                "Eye opening is narrow",
                "Eye pain",
                "Eye socket hurts",
                "Eye socket is sinking inward",
                "Eye strain",
                "Eye too large",
                "Eye twitching",
                "Eyeball is swollen",
                "Eyebrow hair loss",
                "Eyebrow lice",
                "Eyebrow loss of color",
                "Eyelid closes too slowly",
                "Eyelid crusting and sticking together",
                "Eyelid feels like it is burning",
                "Eyelid feels scratchy",
                "Eyelid feels thick",
                "Eyelid flipped up",
                "Eyelid folds inward",
                "Eyelid granulation",
                "Eyelid hurts",
                "Eyelid is red and irritated",
                "Eyelid pink gray",
                "Eyelid pus",
                "Eyelid tender to touch",
                "Eyelid twitching",
                "Eyelids feel hard",
                "Eyelids feel heavy",
                "Eyes are irritated",
                "Eyes bulge out",
                "Eyes don't move together",
                "Eyes fixed on a single location",
                "Eyes rolling back",
                "Eyes tearing more",
                "Eyes wide open",
                "Eyesight getting worse",
                "Eyesight worse in one eye",
                "Feeling pressure below the eye",
                "Flashing lights in vision",
                "Flickering uncolored zig-zag lines in vision",
                "Frequent squinting",
                "Front part of eye is swollen",
                "Front part of eye looks cloudy",
                "Glaucoma",
                "Grayish/brown spots on the outside of colored part of eye",
                "Growth that looks like a yellow spot or bump on the eyeball",
                "Hemianopia",
                "Hole in colored part of eye",
                "Hypopyon",
                "Infected lump or sore on eyelid",
                "Inner corner of eye is swollen",
                "Intraocular pressure decrease",
                "Intraocular pressure increase",
                "Iridocyclitis",
                "Iritis",
                "Irritated eye",
                "Itchy eye",
                "Itchy eyelid",
                "Keratitis",
                "Lacrimal gland lobulation",
                "Lacrimal sac inflammation",
                "Large blister on the eye",
                "Lateral vision loss",
                "Lazy eye",
                "Lens dislocation",
                "Light hurts eyes",
                "Little eyes",
                "Losing eye color",
                "Losing eyelashes",
                "Loss of an area of vision",
                "Loss of an area of vision in both eyes",
                "Loss of an area of vision in one eye",
                "Loss of color vision in one spot",
                "Loss of eyelashes",
                "Loss of vision in both eyes",
                "Loss of vision in one eye",
                "Lump in eye socket",
                "Maculae ceruleae",
                "Macular degeneration",
                "Mucus coming from the eye",
                "No color in eye",
                "No peripheral vision",
                "Not able to make tears",
                "Nystagmus",
                "Nystagmus latency",
                "Nystagmus reversal",
                "Nystagmus, fatigue",
                "Nystagmus, rotary",
                "Ocular cherry red spot",
                "One eye bulges",
                "One eye not turning",
                "One eye sees better than the other",
                "One eyelid swollen",
                "One or both eyes look downward",
                "One or both eyes look to the side",
                "Open sore(s) on colored part of eye",
                "Open sore(s) on eye",
                "Open sore(s) on eyelid",
                "Opening snap",
                "Optic nerve atrophy",
                "Optic neuritis",
                "Pain around the eye",
                "Pain behind the eye",
                "Painful and weak eye movement",
                "Papilledema",
                "Part of outer layer of eye sticking to another part",
                "Peripheral vision loss",
                "Pink eye",
                "Pinpoint pupils",
                "Poor night vision",
                "Prominent brow ridge",
                "Pupil fixed",
                "Pupil irregularity",
                "Pupillary deformity",
                "Pupillary inequality",
                "Pupillary whiteness",
                "Pus coming from the eye",
                "Rash limited to eyelid",
                "Red color blindness",
                "Red eye",
                "Red eyelid",
                "Retinal angioid streaks",
                "Retinal bleeding",
                "Retinal coloboma",
                "Retinal detachment",
                "Retinal exudate",
                "Retinal granuloma",
                "Retinal opacity",
                "Retinal pallor",
                "Retinal pigmentation",
                "Retinitis",
                "Roth spots",
                "Scar on clear part eye",
                "Scar on the eye",
                "Seeing halos of light around things",
                "Severe eye pain",
                "Single red eye",
                "Skin and eyes more sensitive to sunlight",
                "Skin folded over upper eyelid",
                "Slower blinking",
                "Small blister on eye",
                "Small dot of light or zigzag shape in your vision",
                "Small flat spots of loss of skin color",
                "Something stuck inside the eye",
                "Sore eye",
                "Specks or spots in colored part of eye",
                "Spider vein(s) in eye",
                "Stye",
                "Sunken eyes",
                "Swelling around the eyes",
                "Swollen eyelid",
                "Swollen tear duct",
                "Tear duct hurts",
                "Tear duct is red",
                "Tearing in one eye",
                "Temporary vision loss",
                "Things appear smaller than they are",
                "Things in vision appear yellowish",
                "Thinning eyebrows",
                "Thinning eyelashes",
                "Third nerve paralysis",
                "Tiny red or purple spots limited to inside eye",
                "Torn eyelid",
                "Trouble looking up",
                "Trouble moving eyes",
                "Trouble opening eye",
                "Tumor on one eye",
                "Twitching of colored part of eye",
                "Unable to see clearly",
                "Uveitis",
                "Uveitis, bilateral",
                "Violet color to eyelid",
                "Vision loss",
                "Visual aura",
                "Visual hallucination",
                "Vitreous hemorrhage",
                "Watery eyes",
                "White part of eye is black",
                "White part of eye is blue",
                "White part of eye is white",
                "White patch(es) around eye",
                "White, grey or blue ring seen around color part of eye",
                "Whitish material on eyelid",
                "Wideset eyes",
                "Wrinkle between eyebrows",
                "Yellow eyes",
                "Yellow open sore(s) on eye",
                "Yellow pea-sized lump(s) on eyelid",
            ],
            Self::Nose => &[
                "Along smile or laugh lines are red",
                "Blockage in nose",
                "Bloody nose",
                "Boil on nose",
                "Bridge of nose looks flat",
                "Clear runny nose",
                "Cleft nose",
                "Dented nostril",
                "Deviated septum",
                "Dry nasal passages",
                "Growth in nose",
                "Hay fever",
                "Head congestion",
                "High nose bridge",
                "Hooked nose",
                "Inside of nose is black",
                "Inside of nose is red",
                "Inside of nose is swollen",
                "Irritated nose",
                "Itchy nose",
                "Low nose bridge",
                "Nasal sinus draining",
                "Nasal sinus feels full",
                "Nasal sinus is blocked",
                "Nasal sinus pain",
                "Nasal sinus sore",
                "Nose and throat are inflamed",
                "Nose destruction",
                "Nose discharge, foul smelling, unilateral",
                "Nose discharge, purulent, unilateral",
                "Nose feels like it is burning",
                "Nose flares open",
                "Nose getting bigger",
                "Nose hair burned",
                "Nose hurts",
                "Nose is turned upward",
                "Nose misshapen",
                "Nose mucous membrane atrophy",
                "Nose oral communication",
                "Nose septum destruction",
                "Nose septum necrosis",
                "Nose septum perforation",
                "Nose septum ulceration",
                "Nose skin infected",
                "Nose tender to touch",
                "Nosebleed",
                "Nostrils are very small",
                "Nostrils tilt down",
                "Open sore(s) on nose",
                "Open sore(s) on the nostril",
                "Postnasal drip",
                "Pus coming out of nose",
                "Red nose",
                "Rounded nose",
                "Runny nose",
                "Shingles on tip of nose",
                "Sinus pain or infection bridge of nose",
                "Sinus ulceration",
                "Sinusitis",
                "Small nose",
                "Smelling things that aren't there",
                "Smelly, runny nose",
                "Sneezing",
                "Snotty, runny nose",
                "Snout reflex",
                "Stuffy nose",
                "Swollen nose",
                "Thin nose",
                "Tingling or pricking of nose",
                "Trouble smelling",
                "Using decongestant nose drops",
                "Wide nose",
            ],
            Self::Ears => &[
                "Big ears",
                "Blocked ear",
                "Bony area behind ear is infected with pus",
                "Bony area behind ear is swollen",
                "Bony area behind ear is tender",
                "Bony growths in the ear",
                "Bruising on skull behind the ear",
                "Can't hear on one side",
                "Conductive hearing loss",
                "Constant ear ringing",
                "Diagonal crease in ear lobe",
                "Dry skin in ear",
                "Ear bleeding",
                "Ear cartilage is blue or black",
                "Ear doesn't look right",
                "Ear infection",
                "Ear infection middle ear",
                "Ear is red",
                "Ear lesion, mucoid",
                "Ear tender to touch",
                "Ear wax blocking ear",
                "Earache",
                "Earlobe crease",
                "Ears feel full",
                "Ears set low",
                "Fluid leaking from my ear",
                "Hard lumps around joints",
                "Headache behind ears",
                "Hear crackling noises in my ears",
                "Hearing is getting worse",
                "Hearing things that aren't there",
                "Hole in eardrum",
                "Inner ear infection",
                "Itchy ear",
                "Large blister on eardrum",
                "Large earlobes",
                "Lump in front of ear",
                "Lump on ear",
                "Mastoid bruit",
                "Mastoiditis",
                "Middle ear infection",
                "Otitis externa",
                "Otitis interna",
                "Outside of ear hurts",
                "Pain in bony area behind ear",
                "Pus coming from my ear",
                "Rash limited to ear",
                "Red and irritated swollen ear",
                "Redness of skin on skull behind ear",
                "Scaly or greasy skin on or behind ear",
                "Single skin growth on ear lobe",
                "Small ear",
                "Small ear canal",
                "Something is stuck in my ear",
                "Swelling in front of ears",
                "Swollen ear cartilage",
                "Tough or thick skin around joints",
                "Trouble hearing",
                "Tympanic membrane bulging",
                "Tympanic membrane hypomobile",
                "Tympanic membrane inflammation",
                "Tympanic membrane opaque",
                "Tympanic membrane retraction",
                "Tympanic membrane scarring",
                "Very sensitive to noise",
                "Very sensitive to sounds",
                "Vestibular impairment",
                "Whole ear swollen ear",
            ],
            Self::Face => &[
                "Can't feel temperature on face",
                "Can't move my face",
                "Can't move my face well",
                "Can't move one side of my face",
                "Cheek bone pain",
                "Cheek pain",
                "Enlarged vein on face",
                "Expressionless face",
                "Face extremely thin and bony",
                "Face feels full",
                "Face feels numb",
                "Face feels weak",
                "Face hair turning white",
                "Face hurts",
                "Face is blotchy",
                "Face is swollen",
                "Face is turning blue",
                "Face is yellow",
                "Face misshapen",
                "Face round",
                "Face spasms when stimulated",
                "Face sweats a lot",
                "Face tender to touch",
                "Face turns red when eating, drinking or exercising",
                "Face turns reddish color",
                "Face twitches",
                "Face twitching",
                "Facies coarse",
                "Facies grotesque",
                "Facies mongoloid",
                "Facies triangular",
                "Feels like air is under my face",
                "Fragile or thin skin on face",
                "Hair on face",
                "Half of face is flushed",
                "Horner syndrome",
                "Huge cheek bones",
                "Infected lump or sore on face",
                "Itchy face",
                "Limp muscle in face",
                "Long face",
                "Losing fat in face",
                "Loss of facial hair",
                "Lump on face",
                "Nasal sinus pain",
                "No facial sweating",
                "Numbness of face",
                "One side of face feels weak",
                "One side of face not the same as the other",
                "One side of my face hurts",
                "Open sore(s) on face",
                "Pale face",
                "Pea-sized lump under skin on face",
                "Pinched expression",
                "Rash limited to face",
                "Red face",
                "Red flaky rash limited to smile or laugh lines",
                "Red, swollen, runny nose",
                "Rough hair on face",
                "Sagging skin on face",
                "Skin on face feels hard",
                "Sore facial hair",
                "Spider vein(s) on face",
                "Stiff muscle in face",
                "Thinning facial hair",
                "Tingling or pricking face skin",
                "Tingling or pricking on one side of face",
                "Tingling or pricking skin of face",
                "Trigeminal neuralgia",
                "Trigeminal paralysis",
                "Unibrow",
                "Veins on face dilated",
                "Weak muscles in face",
                "Winking caused by jaw movement",
            ],
            Self::Mouth => &[
                "Area of mucus on tongue",
                "Area under tongue is swollen",
                "Back of mouth is red",
                "Bad breath",
                "Black stuff coating tongue",
                "Bleeding gums",
                "Blisters on tongue",
                "Breath has a fruity smell",
                "Breath has a sweet and tarry smell",
                "Breath smells like almonds",
                "Breath smells like garlic",
                "Breath smells like urine",
                "Breath smells metallic",
                "Broken speech pattern",
                "Brown flat discolored spot(s) limited to lips",
                "Buccal patch(es) mucus",
                "Bulimia",
                "Can't pucker lips",
                "Can't speak",
                "Canker sore",
                "Chin recession",
                "Cleft lip",
                "Cleft palate",
                "Cold sore",
                "Corner of mouth hurts",
                "Corner of mouth is sagging",
                "Cough",
                "Crack at the corner of mouth",
                "Crack on tongue",
                "Cracked lips",
                "Crave salt",
                "Damaged teeth enamel",
                "Dehydration",
                "Dental alveolar suppuration",
                "Dental arch narrowness",
                "Dental caries",
                "Dentition delay",
                "Denture pain",
                "Diminished gag reflex",
                "Drooling",
                "Dry lips",
                "Dry mouth",
                "Dry tongue",
                "Edentulous",
                "Food doesn't taste good",
                "Furry green coating on tongue",
                "Geographic tongue",
                "Gingival erythema",
                "Gingival fistula",
                "Gingival lead line, purple",
                "Gingival leukoplakia",
                "Gingival tenderness",
                "Gingival ulceration",
                "Gingival vesicle",
                "Gingivitis",
                "Gums hurt",
                "Hives inside of mouth",
                "Hives on lips",
                "Hot food or liquids hurt tooth",
                "Infected lump or sore on lip",
                "Inflamed tongue",
                "Inside of mouth is black",
                "Inside of mouth is brown",
                "Inside of mouth is red",
                "Inside of mouth is white",
                "Inside of mouth is yellow",
                "Inside of mouth swollen",
                "Interdental papillary ulceration",
                "Involuntary jerky or fitful movement of tongue",
                "Koplik spot",
                "Large blister(s) in mouth",
                "Large tongue",
                "Lip chewing",
                "Lip hurts",
                "Lip is tingling or prickling",
                "Lip pulled back",
                "Lip tender to touch",
                "Lip trembling",
                "Lipoatrophy",
                "Lips are thicker",
                "Lips turning blue",
                "Long groove between nose and lip",
                "Lower lip droops",
                "Lump on tongue",
                "Mallampati grade i",
                "Mallampati grade iii-iv",
                "Malocclusion",
                "Metal taste in mouth",
                "Microdontia",
                "Micrognathia",
                "Molar loosening, deciduous",
                "More thirsty than usual",
                "Mouth bleeding",
                "Mouth breathing",
                "Mouth burn",
                "Mouth hurts",
                "Mouth is sore",
                "Mouth is swollen",
                "Mouth itches",
                "Mouth looks crooked",
                "Mouth mucous membrane bleeding",
                "Mouth mucous membrane ulceration",
                "Mouth opened",
                "Mouth tender to touch",
                "Mouth wideness",
                "Mucous membrane petechia",
                "Mucous membrane scarring",
                "Mute",
                "Open sore(s) in mouth",
                "Open sore(s) inside of cheek",
                "Open sore(s) on back of mouth",
                "Open sore(s) on inside of cheek",
                "Open sore(s) on lip",
                "Open sore(s) on roof of mouth",
                "Open sore(s) on tongue",
                "Orange tonsils",
                "Pain in tooth socket",
                "Palatal muscle weakness",
                "Palatal paralysis",
                "Palatal tremor",
                "Pale around mouth",
                "Pea-sized lump on tongue",
                "Producing too much saliva",
                "Pseudomembrane",
                "Puckered lip",
                "Raised skin patch(es) on tongue",
                "Red bump(s) inside of cheek",
                "Red irritated throat",
                "Red lips",
                "Red or purple flat spots on inside of cheeks",
                "Red tonsil",
                "Roof of mouth has high arch",
                "Roof of mouth is inflamed",
                "Roof of mouth is misshapen",
                "Roof of mouth is numb",
                "Roof of mouth is red",
                "Roof of mouth narrow",
                "Roof of mouth red",
                "Roof of mouth swollen",
                "Round ball in back of throat is out of place",
                "Round ball in back of throat is swollen",
                "Self induced vomiting",
                "Severely bad breath",
                "Short groove between nose and lip",
                "Shrinking tongue",
                "Skin sore(s) inside of mouth",
                "Skin sore(s) on tonsil",
                "Small blister on roof of mouth",
                "Small bump on inside of cheek",
                "Small bump(s) on back of mouth",
                "Small flat red or purple spots on back of mouth",
                "Small flat red or purple spots on round ball in back of throat",
                "Small flat red or purple spots on tonsil",
                "Small red spots on roof of mouth",
                "Small white bump(s) on inside of cheek",
                "Smooth groove between nose and lip",
                "Snoring",
                "Soft palate atrophy",
                "Soft palate numbness",
                "Soft palate paralysis",
                "Soft palate swelling",
                "Sores in or on side of mouth",
                "Speech is slow",
                "Spider vein(s) on roof of mouth",
                "Stuff coats top of tongue",
                "Stuttering",
                "Swelling around the mouth",
                "Swollen gums",
                "Swollen lips",
                "Swollen throat",
                "Swollen tongue",
                "Swollen tonsil on one side",
                "Swollen tonsils",
                "Tasting things that aren't there",
                "Teeth do not fit well",
                "Teeth grinding",
                "Thin lips",
                "Throat is dry",
                "Thrush",
                "Tingling or numbness around mouth",
                "Tingling or pricking inside mouth",
                "Tingling or pricking tongue",
                "Tiny mouth",
                "Tongue biting",
                "Tongue blanching",
                "Tongue feels like it is burning",
                "Tongue glazing",
                "Tongue has no grooves",
                "Tongue hurts",
                "Tongue infection",
                "Tongue is more red than usual",
                "Tongue is out of place",
                "Tongue is weak",
                "Tongue not normal size and shape",
                "Tongue pushed out too far",
                "Tongue quivers",
                "Tongue trembling",
                "Tonsil inflammation",
                "Tonsil is out of place",
                "Tonsillar leukoplakia",
                "Tooth cold sensitivity",
                "Tooth deformity",
                "Tooth discoloration",
                "Tooth enamel hypoplasia",
                "Tooth enamel pitting",
                "Tooth erosion",
                "Tooth extraction",
                "Tooth impaction",
                "Tooth loose",
                "Tooth loss",
                "Tooth pegged",
                "Tooth root defect",
                "Tooth spacing irregularity",
                "Toothache",
                "Top lip hangs over",
                "Trouble chewing",
                "Trouble communicating",
                "Trouble producing saliva",
                "Trouble speaking",
                "Trouble tasting",
                "Tumor in mouth",
                "Upper lip is swollen",
                "Voice doesn't sound right",
                "Vomiting blood",
                "White coating on tongue",
                "White rash on inside of mouth",
                "White rash on roof of mouth",
                "White skin sore(s) on back of mouth",
                "White skin sore(s) on round ball in back of throat",
                "Whitish coating on tonsil",
                "Yawning",
                "Yellow skin sore(s) on back of mouth",
                "Yellow skin sore(s) on uvula",
            ],
            Self::Neck => &[
                "Blister(s) on back of throat",
                "Brown mucous in throat",
                "Burn back of throat",
                "Can't bend head forward",
                "Can't turn head",
                "Carotid artery bruit",
                "Carotid artery distention",
                "Carotid artery mass",
                "Carotid pulse absence",
                "Carotid pulse increase",
                "Carotodynia",
                "Cervical erosion",
                "Cervical lymph node bleeding",
                "Cervical stenosis",
                "Choking",
                "Choking sensation",
                "Clearly outlined pea-sized lump on neck",
                "Cough",
                "Cracking sound in neck",
                "Cricothyroid paralysis",
                "Enlarged jugular vein",
                "Epiglottic enlargement",
                "Epiglottic erythema",
                "Epiglottis swelling",
                "Epiglottitis",
                "Episodes of not breathing during sleep",
                "Feel pressure on neck",
                "Feels like something is stuck in my throat",
                "Food comes back up",
                "Food or liquid goes down wrong pipe",
                "Globus major nodule",
                "Hair on neck feels tender",
                "Hair roots on neck are red",
                "Hashimoto disease",
                "Head turned to one side",
                "Hepatojugular reflux",
                "High pitched breathing",
                "Infected lump or sore on neck",
                "Itchy neck",
                "Itchy throat",
                "Jugular vein a wave increased",
                "Jugular venous distention with inspiration",
                "Laryngeal anesthesia",
                "Laryngeal crepitation",
                "Laryngeal dryness",
                "Laryngeal edema",
                "Laryngeal erythema",
                "Laryngeal hematoma",
                "Laryngeal mass",
                "Laryngeal mobility increase",
                "Laryngeal obstruction",
                "Laryngeal pain",
                "Laryngeal papilloma",
                "Laryngeal pressure sensation",
                "Laryngeal stenosis",
                "Laryngeal tenderness",
                "Laryngeal ulceration",
                "Laryngitis",
                "Lump on neck",
                "Lump on one side of neck",
                "Lump on one side of throat",
                "Lump on the front of neck",
                "Nasopharyngeal induration",
                "Neck bones fused together",
                "Neck bones sticking out",
                "Neck does not sweat",
                "Neck fascia thickening",
                "Neck has changed colors",
                "Neck hurts",
                "Neck is blue",
                "Neck is red",
                "Neck is swollen",
                "Neck lymph node too big",
                "Neck mass, anterior cervical",
                "Neck mass, posterior cervical",
                "Neck muscles are weak",
                "Neck subcutaneous emphysema",
                "Neck tender to touch",
                "Neck vasodilatation",
                "Neck vessel bruit",
                "No fat in neck",
                "Open sore(s) on back of throat",
                "Orange mucous in throat",
                "Pain on one side of throat",
                "Pain when i swallow",
                "Painful swollen gland in front part of neck",
                "Pea-sized lump(s) in neck",
                "Pharyngeal mucous membrane edema",
                "Pharyngeal paralysis",
                "Prickling or tingling in neck",
                "Pus-filled bump(s) in neck hair follicle(s)",
                "Rash limited to neck",
                "Red bumps around hair follicles on neck",
                "Red open sore(s) on neck",
                "Red pea-sized lump(s) in lining of throat",
                "Removal of thyroid gland",
                "Short neck",
                "Small red or purple spots on back of throat",
                "Sore throat",
                "Spider vein(s) on neck",
                "Sternocleidomastoid muscle paralysis",
                "Stiff neck",
                "Swelling at back of throat",
                "Tender neck lymph node",
                "Throat bleeding",
                "Throat burning sensation",
                "Throat clearing",
                "Throat dryness",
                "Throat feels numb",
                "Throat feels tender",
                "Throat feels weak",
                "Throat is red",
                "Throat spasm",
                "Thyroid bruit",
                "Thyroid enlargement",
                "Thyroid nodule",
                "Tightness in throat",
                "Tingling and prickling in throat",
                "Tracheal compression",
                "Trouble swallowing",
                "Tumor on back of throat",
                "Voice deepening",
                "Voice is hoarse",
                "Webbing on side of neck",
                "Whispered pectoriloquy",
                "White mucous in throat",
                "White stuff on throat",
                "Windpipe is shifted",
            ],
            Self::Shoulders => &[
                "Large shoulder vein",
                "Lump in shoulder",
                "Shoulder girdle fascia thickening",
                "Shoulder girdle muscle weakness",
                "Shoulder granule",
                "Shoulder muscle pain",
                "Shoulder muscle twitching",
                "Shoulder shrug sign",
                "Shoulder tender to touch",
                "Subacromial bursal tenderness",
                "Subdeltoid bursal tenderness",
                "Swollen shoulder",
            ],
            Self::UpperArms => &[
                "Bicep shaking",
                "Biceps and triceps hyperreflexia",
                "Biceps hyporeflexia",
                "Humeral swelling, lower",
                "Triceps hyporeflexia",
                "Upper arm pain",
            ],
            Self::Elbows => &[
                "Darkened skin on elbow",
                "Elbow bones out of place",
                "Elbow pain",
                "Flaky bump(s) limited to elbows or knees",
                "Forearm is angled away from the body",
                "Red bump(s) on elbow",
                "Single flaky raised skin patch on elbows or knees",
                "Stiff elbow",
                "Tenderness lateral epicondyle",
            ],
            Self::Forearms => &[
                "Forearm feels more sensitive",
                "Forearm feels weak",
                "Forearm hurts",
                "Forearm itches",
                "Forearm turning up",
                "Forearm turns in",
                "Lump on forearm",
                "Tingling or prickling in forearm",
            ],
            Self::Wrists => &[
                "Able to bend wrist backwards",
                "Crackling sound when moving wrist",
                "Phalen's maneuver positive",
                "Tough or thick skin on base of wrist",
                "Wrist flexor muscle atrophy",
                "Wrist hurts when moved",
                "Wrist is red",
                "Wrist is swollen on thumb side",
                "Wrist muscle weakness",
                "Wrist pain",
                "Wrist stiffness",
                "Wrist swelling",
                "Wrist tenderness, radial",
                "Wristdrop",
            ],
            Self::Hands => &[
                "Abnormal creases in palm",
                "Arm/hand pain due to median nerve problem",
                "Asterixis",
                "Brachymesophalangia, fifth finger",
                "Brown nails",
                "Burning feeling in hand",
                "Camptodactyly",
                "Can't write",
                "Cold hand",
                "Compressed nerve in wrist/hand",
                "Cramp in my palm",
                "Curved fingers",
                "Darkened skin on knuckle(s)",
                "Double jointed hand",
                "Durkan's compression test positive",
                "Fatty yellowish skin rash on palms",
                "Fist clenching",
                "Flaky tough or thick skin on palms of hand",
                "Hand asterixis",
                "Hand changing colors",
                "Hand cramping at night",
                "Hand hurts",
                "Hand hurts when moving",
                "Hand is numb",
                "Hand is red",
                "Hand is turned towards little finger",
                "Hand muscle weakness",
                "Hand or arm shaking when performing task",
                "Hand shaking",
                "Hand shortness",
                "Hand smallness",
                "Hand swelling",
                "Hand wringing",
                "Hands move too slowly",
                "Hives on hand",
                "Infected lump or sore on hand",
                "Itchy palms",
                "Knuckle deformity",
                "Knuckle joint on hand hurts",
                "Large hands",
                "Milkmaid's grip",
                "Muscles on outside of palm are shrinking or thinning",
                "Open sore(s) on hand",
                "Open sore(s) on palm",
                "Pain in palm of hand",
                "Palm area under thumb is flat",
                "Palm is swollen",
                "Palms sweating more",
                "Pea-sized lump(s) on palm of hand",
                "Peeling hands",
                "Rash limited to hand",
                "Rash limited to palm",
                "Rash on hand",
                "Red flaky rash limited to palms or soles",
                "Red palms",
                "Shrinking or thinning muscles in hand",
                "Single line that runs across the palm of hand",
                "Skin on palm peeling off",
                "Smooth, soft palms",
                "Stiff hands",
                "Stiff knuckles in hands or toes",
                "Swollen knuckles",
                "Tingling or prickling in hand",
                "Tough or thick skin on palms of hand",
                "Trouble moving hands",
                "Trouble speaking or talking",
                "Weak hand grip",
                "Wide flat hand",
                "Yellow palms",
            ],
            Self::Fingers => &[
                "Blue nails",
                "Can't recognize fingers",
                "Can't straighten bent finger(s)",
                "Clinodactyly",
                "Discolored fingertip",
                "Enlarged fingertips",
                "Finger blood vessel obstruction",
                "Finger chewing",
                "Finger deformity",
                "Finger shaking",
                "Finger(s) are swollen",
                "Finger(s) feel stiff",
                "Finger(s) feel tender",
                "Finger(s) feel tight",
                "Finger(s) hurts",
                "Finger(s) locks in place",
                "Finger(s) point to little finger side of hand",
                "Finger(s) really sensitive to touch",
                "Finger(s) too cold",
                "Finger(s) turn red",
                "Finger(s) turns blue",
                "Fingernail(s) hurt",
                "Fingertip tender to touch",
                "Half of nail is white and other half is deeper pink",
                "Hand spiderlike",
                "Hard bumps on finger(s)",
                "Hoffman sign positive",
                "Nail loss",
                "Nail not growing the way it should",
                "Nail pulling away from cuticle",
                "Nail(s) are blue",
                "Open sore(s) on finger(s)",
                "Open sore(s) on fingertip(s)",
                "Pale finger(s)",
                "Pea-sized lump under skin on finger(s)",
                "Skin on finger(s) is thick",
                "Spider veins in fingernails",
                "Syndactyly",
                "Thumb absence",
                "Thumb hurts",
                "Thumb microdactyly",
                "Thumb spatulate",
                "Thumb synostosis",
                "Thumb, distal phalynx, shortness",
                "Thumb, triphalangeal",
                "Tingling and prickling in finger(s)",
                "Unusually short fingers",
                "Weak finger(s)",
                "Weak thumb muscle",
            ],
            Self::UpperChest => &[
                "Fatty area above collar bone",
                "Left supraclavicular lymph node enlargement",
                "Supraclavicular fossa bruit",
                "Supraclavicular lymph node enlargement",
                "Supraclavicular pulsation",
            ],
            Self::Sternum => &[
                "Aortic dilation, ascending",
                "Aortic dissection",
                "Aortic infection",
                "Behind the breastbone hurts",
                "Breast bone hurts",
                "Breastbone is abnormal",
                "Breastbone tender to touch",
                "Breath sound decrease, basilar, unilateral",
                "Cardiomegaly",
                "Chest bones cave in",
                "Chest bones stick out",
                "Chest pain that spreads to arm, shoulder, neck or jaw",
                "Congestive heart failure",
                "Ejection fraction reduced",
                "Feeling of pressure in food pipe",
                "Food gets stuck",
                "Gibson's murmur",
                "Hard for food to go down",
                "Heart beats faster when exercising",
                "Heart displacement",
                "Heart displacement, left",
                "Heart displacement, right",
                "Heart murmur",
                "Heart murmur increased with inspiration",
                "Heart murmur, changing",
                "Heart murmur, diastolic",
                "Heart murmur, diastolic, pulmonic",
                "Heart murmur, holosystolic",
                "Heart murmur, machinery",
                "Heart murmur, presystolic",
                "Heart murmur, systolic",
                "Heart murmur, systolic, apical",
                "Heart murmur, systolic, crescendo-decrescendo",
                "Heart murmur, systolic, pulmonic",
                "Heart size decrease",
                "Heart sound absence, second",
                "Heart sound decrease, first",
                "Heart sound decrease, second",
                "Heart sound increase",
                "Heart sound increase, first",
                "Heart sound increase, second",
                "Heart sound increase, second, pulmonic",
                "Heart sound irregularity",
                "Heart sound split, first",
                "Heart sound split, second",
                "Heart sound variation, first",
                "Heart sound, fourth",
                "Heart sound, third",
                "Heart sounds muffled",
                "Heart thrill",
                "Heart thrill, apical",
                "Heart thrill, diastolic",
                "Heart thrill, pulmonic",
                "Heartburn",
                "Hiccups",
                "Inflammation of esophagus",
                "Mitral valve prolapse",
                "Nipple hurts",
                "Palpitations",
                "Pericardial friction rub",
                "Pressure on heart due to fluid buildup",
                "Pulmonary ejection click",
                "Pulmonic sound absence",
                "Pulmonic sound decrease",
                "Severe chest pain/pressure",
                "Sternal lift",
                "Sternal pulsation visible",
                "Sternoclavicular joint pulsation",
                "Systolic heart murmur, increased with valsalva",
                "Systolic thrill",
                "Tightening of esophagus",
            ],
            Self::Breasts => &[
                "Abnormal growth of male breasts",
                "Bloody nipple discharge",
                "Breast cancer",
                "Breast feels harder",
                "Breast feels heavy",
                "Breast getting bigger",
                "Breast getting smaller",
                "Breast hurts",
                "Breast mass roundness",
                "Breast mass smoothness",
                "Breast mass, unilateral",
                "Breast redness",
                "Breast skin feels like an orange peel",
                "Breastfeeding mom",
                "Breasts not developing",
                "Darkened skin on nipple",
                "Enlarged vein on breast",
                "Fluid leaking from nipple",
                "Growth on nipple",
                "Hard lump in breast",
                "Infected lump or sore on breast",
                "Loss of skin color on nipple",
                "Lump in breast",
                "Lump in breast that can be moved",
                "Lump in breast that doesn't move",
                "Nipple doesn't move",
                "Nipple pulling to one side",
                "Nipple redness",
                "Nipple stays hard all the time",
                "Nipple tender to touch",
                "Painful tube like lump in breast",
                "Part of breast skin appears pulled inward",
                "Rash limited to under the breast",
                "Red, irritated nipple",
                "Squishy lump in breast",
                "Swollen breast",
                "Swollen nipples",
                "Wide set nipples",
            ],
            Self::Chest => &[
                "Abnormal growth of male breasts",
                "Absent breath sounds, unilateral",
                "Aortic dilation, ascending",
                "Aortic dissection",
                "Aortic infection",
                "Asthma",
                "Austin flint murmur",
                "Barky cough",
                "Behind the breastbone hurts",
                "Between right lower ribs hurts",
                "Blood clot traveled to lung",
                "Bloody nipple discharge",
                "Breast bone hurts",
                "Breast cancer",
                "Breast feels harder",
                "Breast feels heavy",
                "Breast getting bigger",
                "Breast getting smaller",
                "Breast hurts",
                "Breast mass roundness",
                "Breast mass smoothness",
                "Breast mass, unilateral",
                "Breast redness",
                "Breast skin feels like an orange peel",
                "Breastbone is abnormal",
                "Breastbone tender to touch",
                "Breastfeeding mom",
                "Breasts not developing",
                "Breath sound decrease, basilar, unilateral",
                "Breathing too fast",
                "Breathing too slowly",
                "Buildup of fluid in lungs",
                "Burning sensation in chest",
                "Can't cough up mucus or phlegm",
                "Can't feel hot or cold on upper body",
                "Cardiomegaly",
                "Chest bones cave in",
                "Chest bones stick out",
                "Chest bruit",
                "Chest crepitation",
                "Chest decreased resonance",
                "Chest deformity",
                "Chest deformity on left side",
                "Chest feels tender to the touch",
                "Chest hair loss",
                "Chest hyperresonance",
                "Chest hyperresonance, unilateral",
                "Chest infection",
                "Chest is rigid",
                "Chest muscle spasm",
                "Chest overly expanded",
                "Chest pain",
                "Chest pain after vomiting",
                "Chest pain made worse by breathing",
                "Chest pain made worse by exertion/exercise",
                "Chest pain that spreads to arm, shoulder, neck or jaw",
                "Chest peristaltic sound",
                "Chest redness",
                "Chest subcutaneous emphysema",
                "Chest tightness",
                "Chest wall fistula",
                "Chest wall suppuration",
                "Chronic cough (more than 8 weeks) with normal chest xray",
                "Clavicular hypoplasia",
                "Congestive heart failure",
                "Continued inflammation of bronchial tubes",
                "Cough",
                "Cough out mucus",
                "Cough up black phlegm",
                "Cough up frothy or bubbly gunk",
                "Cough up thick gunk",
                "Cough up yellow gunk",
                "Cough with mucus long time",
                "Cough with swallowing",
                "Coughing at night",
                "Coughing attacks",
                "Coughing up bad smelling mucus",
                "Coughing up blood",
                "Coughing up white mucus",
                "Crushing chest pain",
                "Darkened skin on nipple",
                "Decreased breath sounds",
                "Decreased breath sounds, unilateral",
                "Difficulty breathing with normal chest x-ray",
                "Dry cough",
                "Egophony",
                "Ejection fraction reduced",
                "Emphysema",
                "Enlarged chest vein",
                "Enlarged vein on breast",
                "Fat chest",
                "Fatty area above collar bone",
                "Feeling of pressure in food pipe",
                "Fluid leaking from nipple",
                "Food gets stuck",
                "Forceful cough",
                "Gibson's murmur",
                "Growth on nipple",
                "Hacking cough",
                "Hamman sign positive",
                "Hard for food to go down",
                "Hard lump in breast",
                "Heart beats faster when exercising",
                "Heart displacement",
                "Heart displacement, left",
                "Heart displacement, right",
                "Heart murmur",
                "Heart murmur increased with inspiration",
                "Heart murmur, changing",
                "Heart murmur, diastolic",
                "Heart murmur, diastolic, aortic",
                "Heart murmur, diastolic, apical",
                "Heart murmur, diastolic, pulmonic",
                "Heart murmur, holosystolic",
                "Heart murmur, machinery",
                "Heart murmur, presystolic",
                "Heart murmur, systolic",
                "Heart murmur, systolic, aortic",
                "Heart murmur, systolic, apical",
                "Heart murmur, systolic, crescendo-decrescendo",
                "Heart murmur, systolic, pulmonic",
                "Heart size decrease",
                "Heart sound absence, second",
                "Heart sound decrease, first",
                "Heart sound decrease, second",
                "Heart sound increase",
                "Heart sound increase, first",
                "Heart sound increase, second",
                "Heart sound increase, second, pulmonic",
                "Heart sound irregularity",
                "Heart sound split, first",
                "Heart sound split, second",
                "Heart sound variation, first",
                "Heart sound, fourth",
                "Heart sound, third",
                "Heart sounds muffled",
                "Heart thrill",
                "Heart thrill, apical",
                "Heart thrill, diastolic",
                "Heart thrill, pulmonic",
                "Heartburn",
                "Hiccups",
                "Infected lump or sore on breast",
                "Infected lump or sore on upper body",
                "Inflammation of bronchial tubes",
                "Inflammation of esophagus",
                "Intercostal space retraction",
                "Irregular breathing pattern",
                "Kussmaul respiration",
                "Left supraclavicular lymph node enlargement",
                "Loss of fat in chest area",
                "Loss of skin color on nipple",
                "Lower rib tender to touch",
                "Lower ribs moving abnormally",
                "Lump in breast",
                "Lump in breast that can be moved",
                "Lump in breast that doesn't move",
                "Lung cancer",
                "Lung disease",
                "Lung infection",
                "Making a whooping noise when inhaling",
                "Mitral valve prolapse",
                "Morning cough",
                "Muscle cramp on trunk",
                "Muscles between ribs are weak",
                "Nipple doesn't move",
                "Nipple hurts",
                "Nipple pulling to one side",
                "Nipple redness",
                "Nipple stays hard all the time",
                "Nipple tender to touch",
                "Not breathing",
                "Numbness tingling in chest",
                "Obese upper body",
                "Pain in chest not related to breathing",
                "Pain in upper body",
                "Painful tube like lump in breast",
                "Palpitations",
                "Part of breast skin appears pulled inward",
                "Pea-sized lump in skin of torso",
                "Pericardial friction rub",
                "Pneumonia",
                "Posterior trunk fascia thickening",
                "Pressure on heart due to fluid buildup",
                "Pulmonary ejection click",
                "Pulmonary percussion dullness",
                "Pulmonic sound absence",
                "Pulmonic sound decrease",
                "Pus-filled bump(s) in stomach or back hair follicle(s)",
                "Rales, bilateral subcrepitant",
                "Rales, right basilar",
                "Rales, subcrepitant",
                "Rapid breathing",
                "Rash limited to chest",
                "Rash limited to under the breast",
                "Rate of breathing slows",
                "Red bumps around hair follicles on stomach or back",
                "Red upper body",
                "Red, irritated nipple",
                "Rib angle widening",
                "Ribs pulled inward",
                "Right supraclavicular lymph node enlargement",
                "Sensitive skin on upper body",
                "Severe chest pain/pressure",
                "Shallow breathing",
                "Sharp chest pain",
                "Sharp distinct sound to each cough",
                "Short of breath better when lay down",
                "Shortness breath leaning forward",
                "Shortness of breath",
                "Shortness of breath when lying flat",
                "Shortness of breath with activity",
                "Squishy lump in breast",
                "Sternal lift",
                "Sternal pulsation visible",
                "Sternoclavicular joint pulsation",
                "Stiff muscles in trunk",
                "Sudden shortness of breath at night",
                "Supraclavicular fossa bruit",
                "Supraclavicular lymph node enlargement",
                "Supraclavicular pulsation",
                "Swollen breast",
                "Swollen nipples",
                "Systolic heart murmur, increased with valsalva",
                "Systolic thrill",
                "Tender between right lower ribs",
                "Tightening of esophagus",
                "Tingling or prickling in upper body",
                "Trunk asterixis",
                "Trunk longness",
                "Trunk shortness",
                "Trunk tremor, bobbing",
                "Upper body feels warm",
                "Upper body itchy",
                "Upper body muscles shrinking",
                "Upper body spasm",
                "Upper body tilted",
                "Upper body trembling",
                "Upper respiratory infection",
                "Vomiting after cough",
                "Weak upper body",
                "Wet cough",
                "Wheezing",
                "Wide set nipples",
                "Worsening shortness of breath",
            ],
            Self::Abdomen => &[
                "Abdominal mass, movable, upper",
                "Abdominal mass, right upper quadrant",
                "Abdominal mass, upper",
                "Abdominal tenderness, left lower quadrant",
                "Abdominal tenderness, lower",
                "Abdominal tenderness, left upper quadrant",
                "Bladder distention",
                "Bladder feels full",
                "Burping",
                "C-section",
                "Can't digest fatty foods",
                "Change in bowel habits",
                "Courvoisier sign",
                "Diarrhea",
                "Diarrhea after meals",
                "Epigastric abdominal tenderness",
                "Fatty liver",
                "Feels like need to pee all the time",
                "Frequent bowel movements",
                "Gall bladder distention",
                "Gallbladder inflammation",
                "Gallstones",
                "Gassy",
                "Heartburn",
                "Hepatic friction rub",
                "Hepatosplenomegaly",
                "Hernia in belly button",
                "Hurts when ovulating",
                "Indigestion",
                "Indirect tenderness right lower quadrant",
                "Inflammation of colon",
                "Inflammation of stomach and intestines",
                "Liver border irregularity",
                "Liver bruit",
                "Liver disease",
                "Liver displacement",
                "Liver enlargement",
                "Liver hard",
                "Liver mass",
                "Liver pulsation",
                "Liver tenderness",
                "Lower belly bloating",
                "Lower stomach pain",
                "Murphy sign positive",
                "Nausea",
                "Open sore in stomach or esophagus",
                "Ovarian mass",
                "Ovarian mass, irregular",
                "Ovarian swelling",
                "Ovary palpable",
                "Pain around belly button",
                "Pain in diaphragm",
                "Pain in middle of belly",
                "Pain near belly button spreading to lower right side of stomach",
                "Pancreas inflammation",
                "Past appendix removal",
                "Past gallbladder removal",
                "Reflux",
                "Scarring of the liver",
                "Spleen enlargement",
                "Spleen friction rub",
                "Spleen palpable",
                "Spleen tenderness",
                "Stomach inflammation",
                "Stomach pain lower left side",
                "Stomach pain lower right side",
                "Stomach pain upper left side",
                "Stomach pain upper right side",
                "Stomach pushes through diaphragm",
                "Ulcer in muscle connecting stomach to duodenum",
                "Upper abdominal wound",
                "Upper belly bloating",
                "Upper stomach pain",
                "Urine leaking from belly button",
                "Vomiting blood",
            ],
            Self::Pelvis => &[
                "Acetabular dysplasia",
                "Bend at hip",
                "Coxa valga",
                "Coxa vara",
                "Darkened skin on groin",
                "Difficulty getting up from a chair",
                "Duroziez sign",
                "Feeling of heaviness in groin",
                "Femoral bruit",
                "Femoral lymph node enlargement",
                "Femoral pulse absence",
                "Femoral pulse decrease",
                "Greater tuberosity tenderness",
                "Groin pain",
                "Groin tenderness",
                "Hernia, femoral",
                "Hip deformity",
                "Hip feels like it pops out of socket",
                "Hip feels stiff",
                "Hip hurts",
                "Hip is swollen",
                "Hip muscle is weak",
                "Hip tenderness",
                "Hurts to walk",
                "Inguinal hernia",
                "Inguinal lymph node abscess",
                "Inguinal lymph node enlargement",
                "Inguinal lymph node firmness",
                "Inguinal lymph node matting",
                "Inguinal lymph node tenderness",
                "Ischial tuberosity tenderness",
                "Lump comes and goes on groin",
                "Lump in groin",
                "Painful gland in groin",
                "Pea-sized lump(s) on groin",
                "Pelvic muscles are tight",
                "Pelvic muscles feel weak",
                "Pelvic smallness",
                "Pelvis tilted",
                "Pelvis wide",
                "Rash limited to groin",
                "Redness of groin",
            ],
            Self::Genitals => &[
                "A lot of blood in urine",
                "Able to feel vein in scrotum",
                "Abnormal swelling of penis",
                "Balls turning blue",
                "Bladder distention",
                "Bladder edema",
                "Bladder erythema",
                "Bladder feels full",
                "Bladder infection",
                "Bladder mass",
                "Blood in urine",
                "Bloody pee",
                "Bloody sperm",
                "Bubbles in my urine",
                "Bulging veins in scrotum",
                "Can't have orgasm",
                "Can't pee",
                "Can't tell when bladder is full",
                "Change in bladder habits",
                "Chlamydial infection",
                "Cloudy pee",
                "Cremasteric reflex absent, unilateral",
                "Cyst on genitals",
                "Cyst on testicle",
                "Dark pee",
                "Decreased sex drive",
                "Deformed scrotum",
                "Delayed or late period",
                "Difficult to pee",
                "Discharge from penis",
                "Double ureter",
                "Enlarged prostate",
                "Epididymal mass",
                "Epididymal tenderness",
                "Epididymitis",
                "Erection that won't go down or soften",
                "Feels like need to pee all the time",
                "Firm pus filled rash around head of penis",
                "Foreskin stuck over head of penis",
                "Foreskin stuck to penis",
                "Genital abnormality",
                "Genital necrosis",
                "Genital numbness",
                "Genital pain",
                "Genital underdevelopment",
                "Genitalia, ambiguous",
                "Genitals getting larger",
                "Genitals itching",
                "Genitals swollen",
                "Glans penis calculus",
                "Glans penis scar tissue formation",
                "Gonad disorder",
                "Gonadal hypoplasia",
                "Gray skin peeling off penis",
                "Green urine",
                "Hard bump(s) around head of penis",
                "Hard bump(s) on head of penis",
                "Hard pus-filled rash on head of penis",
                "Head of penis curves downward",
                "Head of penis hurts",
                "Head of penis is irritated",
                "Head of penis is red and swollen",
                "Head of penis is swollen",
                "Hematuria, microscopic",
                "Hemoglobinuria",
                "Hives on penis",
                "Hurts to ejaculate or cum",
                "Immediate urge to pee",
                "Impotence",
                "Incontinence",
                "Infected testicles",
                "Infertility",
                "Inflamed scrotum",
                "Inflammation of urinary tract",
                "Irritation between butt and genitals",
                "Itching on urethra",
                "Iud in place",
                "Large blister(s) on penis",
                "Large non-emptying bladder",
                "Large penis",
                "Light colored pee",
                "Lump between butt and genitals",
                "Lump in genital area",
                "Lump in urinary tract",
                "Lump on penis",
                "Lump on scrotum",
                "Lump on testicle",
                "Lymphogranuloma venereum",
                "Man ejaculates sooner during sexual intercourse than he or his partner would like",
                "Massive scrotal swelling",
                "Muscle twitching in genital area",
                "Need to pee often",
                "Open sore(s) around head of penis",
                "Open sore(s) between butt and genitals",
                "Open sore(s) on genitals",
                "Open sore(s) on head of penis",
                "Open sore(s) on penis",
                "Open sore(s) on urethra",
                "Opening of urinary tract is blocked",
                "Orange pee",
                "Pain at the opening of urinary tract",
                "Pain between butt and genitals",
                "Pain in cord running vertically behind testicle",
                "Pain in testicle",
                "Pain in testicle or ovary",
                "Pain in tube behind testicle",
                "Pain while peeing",
                "Painful erection",
                "Painless ulcer on the genitals",
                "Passing small kidney stones",
                "Pea-sized lump(s) on prostate",
                "Pee comes out of top of penis",
                "Pee hole is on bottom side of penis",
                "Pee more than usual",
                "Pee too much at night",
                "Pelvic calculus palpable, rectum",
                "Pelvic mass",
                "Penis hurts",
                "Penis is red",
                "Penis is red and irritated",
                "Penis pulled in",
                "Penis tenderness",
                "Prostate fluctuance",
                "Prostate hardening",
                "Prostate infection",
                "Prostate pain",
                "Prostate tenderness",
                "Prostatitis",
                "Rash limited to genitals",
                "Rectovaginal fistula",
                "Red bump(s) around head of penis",
                "Red bump(s) on head of penis",
                "Redness around urinary tract",
                "Redness of private parts",
                "Redness of testicle sac",
                "Scrotal mass",
                "Scrotal mass, firm",
                "Scrotal pulling sensation",
                "Scrotal ulceration",
                "Scrotum cyst",
                "Scrotum hurts",
                "Scrotum pain goes away when lift testicle",
                "Seminal vesicular induration",
                "Seminal vesicular swelling",
                "Sexual desire increased",
                "Shrunken testicles",
                "Small penis",
                "Spermatic cord cyst",
                "Spermatic cord enlargement",
                "Spermatic cord hydrocele",
                "Spermatic cord inflammation",
                "Spermatic cord mass",
                "Spermatic cord tenderness",
                "Spermatic cord torsion",
                "Std transmission",
                "Sterile pyuria",
                "Stopping the flow of urine",
                "Swelling at opening of urinary tract",
                "Swelling between butt and genitals",
                "Swollen scrotum",
                "Swollen testicle",
                "Tenderness of private parts",
                "Testicle feels squishy",
                "Testicle riding too high",
                "Testicles feel tight",
                "Testicles hurt to touch",
                "Testicles never fully developed",
                "Tight scrotum",
                "Trouble starting to pee",
                "Unable to pee",
                "Uncircumcised penis",
                "Undescended testicles",
                "Ureteral mass",
                "Urethral fistula",
                "Urethral meatus protrusion",
                "Urethral obstruction",
                "Urethral pain",
                "Urinary incontinence",
                "Urinary tract abnormality",
                "Urinary tract infection",
                "Urinary tract obstruction",
                "Urinating less",
                "Urinating stool",
                "Vas deferens swelling",
                "Vas deferens tenderness",
                "Weak pee stream",
                "Wet dream",
            ],
            Self::Thighs => &[
                "Back of upper leg is weak",
                "Burning feeling on thigh",
                "Can't feel hot or cold on thigh",
                "Cramp in thigh muscle",
                "Dahl's sign positive",
                "Fat thigh",
                "Itching thigh",
                "Large thigh muscle",
                "Movement of upper leg outward",
                "Numb thigh muscle",
                "Pain in thigh",
                "Popping sound when turn thigh outward",
                "Red thigh",
                "Thigh muscle feels firm",
                "Thigh muscle mass",
                "Thigh twitching",
                "Weak thigh muscle",
            ],
            Self::Back => &[
                "Back muscle spasm",
                "Back pain",
                "Back pain standing",
                "Back tenderness",
                "Can't bend backwards",
                "Compression of spinal nerves",
                "Extreme curve in low back",
                "Fat hump top of my back",
                "Flank bruit",
                "Flank tenderness",
                "Hunched or stooped posture",
                "Kidney abscess",
                "Kidney disease",
                "Kidney infection",
                "Kidney mass tenderness",
                "Kidney palpable",
                "Kidney problem",
                "Kidney stone",
                "Kidney, polycystic",
                "Kyphoscoliosis",
                "Lordosis, loss of lumbar",
                "Low back pain",
                "Low back tenderness",
                "Lower back muscle spasm",
                "Lump on back",
                "Lump on lower back",
                "Muscle spasms along the spine",
                "Muscles are tender along the spine",
                "Nephrocalcinosis",
                "Nephrosclerosis",
                "Nephrotic syndrome",
                "One side of low back hurts",
                "One side of low back is red",
                "Open sore(s) on back",
                "Opisthotonos",
                "Pain from kidney stone",
                "Pain in tailbone",
                "Pain under left shoulder blade",
                "Pain under right shoulder blade",
                "Pain under shoulder blade",
                "Pain when sitting",
                "Rash limited to back",
                "Red or purple flat spots on side of body",
                "Renal failure",
                "Sacroiliac pain",
                "Scapula, winged",
                "Severe back pain",
                "Shoulder blade is higher on one side",
                "Solitary patch, rough, raised, lumbosacral",
                "Spinal cord inflammation",
                "Spinal cord lesion",
                "Spinal defect",
                "Spinal movement impairment",
                "Spinal muscle atrophy",
                "Spine curvature in side-to-side direction",
                "Spine is swollen",
                "Stiff back",
                "Tingling feeling in back",
                "Trapezius muscle paralysis",
                "Upper back pain",
                "Vertebral deformity",
            ],
            Self::Buttocks => &[
                "Anal bleeding",
                "Anal bulla, hemorrhagic",
                "Anal canal induration",
                "Anal fistula",
                "Anal inflammation",
                "Anal mass tenderness",
                "Anal nodule",
                "Anal sphincter spasm",
                "Anal ulceration",
                "Anus, imperforate",
                "Blood in poop",
                "Bright red blood in poop",
                "Bulky poop",
                "Butt feels numb",
                "Butt hurts",
                "Butt pain",
                "Can't poop completely",
                "Clay colored poop",
                "Clotted swollen veins near anus",
                "Constipation",
                "Diarrhea",
                "Diarrhea after meals",
                "Diarrhea at night",
                "Discharge from rectum smells bad",
                "Feels like need to poop all the time",
                "Fluid leaking from butt",
                "Gassy",
                "Gluteal crease low, unilateral",
                "Green colored poop",
                "Hard stool",
                "Hemorrhoid",
                "Infected lump or sore on tailbone",
                "Inflamed tailbone",
                "Infrequent bowel movements",
                "Itchy anus",
                "Itchy buttocks",
                "Large crack between or under butt cheeks",
                "Large stool",
                "Levator ani spasm",
                "Lump on butt",
                "Melanosis coli",
                "Mucous leaking from butt",
                "Mucus on poop",
                "Oily greasy looking poop",
                "Pain during pooping",
                "Poop is thin",
                "Poop leaking",
                "Poop smells bad",
                "Pus coming from butt",
                "Pus-filled bump(s) in buttocks hair follicle(s)",
                "Rash limited to anus",
                "Rash limited to buttocks",
                "Rectal fissure",
                "Rectal fistula",
                "Rectal mass",
                "Rectal mucous membrane discoloration, orange",
                "Rectal pain",
                "Rectal tenderness",
                "Rectal ulceration",
                "Rectal urine",
                "Rectoperineal fistula",
                "Rectum paralyzed",
                "Rectum protrudes out anus",
                "Red bumps around hair follicles on buttocks",
                "Relaxed anal sphincter",
                "Sacroileitis",
                "Sacroiliac tenderness",
                "Sciatic nerve pain",
                "Sciatic nerve tenderness",
                "Severe constipation",
                "Stool backed up or blocked",
                "Straining during bowel movement",
                "Swollen vein protrudes from anus",
                "Tarry stool",
                "Tingling and numbness on butt",
                "Tissue death of anus",
                "Wart on butt",
                "Yellow colored poop",
            ],
            Self::Knees => &[
                "Back of knee hurts",
                "Back of knee is swollen",
                "Can feel small lump in knee",
                "Clutton joints",
                "Darkened skin on knee",
                "Dislocated knee",
                "Flaky bump(s) limited to elbows or knees",
                "Front of knee hurts",
                "Front of knee is swollen",
                "Genu valgum",
                "Genu varum",
                "Hurts to kneel",
                "Hurts to walk",
                "Inflamed fluid sac in knee",
                "Inside edge of knee is swollen",
                "Joint fluid swelling of back off knee joint",
                "Knee cracking when moving",
                "Knee feels like it is slipping",
                "Knee gets stuck when moving",
                "Knee hurts",
                "Knee instability",
                "Knee is able to bend",
                "Knee joint inflammation",
                "Knee joint makes popping sounds",
                "Knee tender to touch",
                "Lachman test positive",
                "Lump on knee",
                "Mcmurray test positive",
                "Outer side of knee hurts",
                "Pain on inside edge of knee",
                "Patellar tendon reflex absent",
                "Patellar tendon reflex decreased",
                "Patellar tendon reflex increased",
                "Pulsating lump around knee",
                "Single flaky raised skin patch on elbows or knees",
                "Stiff knee",
                "Swollen knee",
                "Tibial tuberosity tenderness",
                "Trouble moving knee",
                "Weak knee muscle",
            ],
            Self::LowerLegs => &[
                "Calf muscle cramp",
                "Calf muscle feels hard",
                "Calf muscle is larger than normal",
                "Calf pain",
                "Calf swelling",
                "Hurts to walk",
                "Peroneal sign positive",
                "Ridges on shin bone",
                "Sharp forward bowing of shin",
                "Tender calf muscle",
                "Tibial bone mass",
                "Tibial deformity",
                "Tibial pulse absence",
                "Weakness in lower legs",
            ],
            Self::Ankles => &[
                "Achilles areflexia",
                "Ankle is overly flexible",
                "Ankle pain",
                "Ankle redness",
                "Ankle reflex decreased",
                "Ankle swollen",
                "Arthritis in ankle",
                "Bruise on ankle",
                "Lump on ankle",
            ],
            Self::Feet => &[
                "Arthritis in the arch of foot",
                "Ball of foot joint hurts when move",
                "Bottom of foot pain",
                "Bottom of foot peeling",
                "Bottom of foot red",
                "Bottom of foot sweats more",
                "Bottom of foot swelling",
                "Bottom of foot yellow",
                "Can't hold foot up",
                "Charcot joint",
                "Clubfoot",
                "Extreme arch in foot",
                "Flat feet",
                "Foot changing color",
                "Foot deformity",
                "Foot feels cold",
                "Foot feels hot or warm",
                "Foot feels stiff",
                "Foot feels weak",
                "Foot hurts",
                "Foot is numb",
                "Foot is turned out",
                "Foot is turned up",
                "Foot muscle is thinning",
                "Foot peeling",
                "Foot pulse absence",
                "Foot smallness",
                "Foot turning blue",
                "Foot turning red",
                "Heel hurts",
                "Heel is swollen",
                "Heel is turning in",
                "Heel is turning out",
                "Heel spur",
                "Heel tenderness",
                "Hives on foot",
                "Infected lump or sore on foot",
                "Itchy foot",
                "Large feet",
                "Matles test positive",
                "Metacarpal shortness",
                "Open sore(s) on foot",
                "Open sore(s) on soles of feet",
                "Pain in the arch of foot",
                "Pes cavus",
                "Plantar reflex, absent",
                "Rash limited to feet",
                "Rash limited to soles of feet",
                "Rocker bottom feet",
                "Swelling in the arch of foot",
                "Swollen foot",
                "Tingling or prickling in foot",
                "Tripping",
                "Trouble moving foot",
            ],
            Self::Toes => &[
                "Arthritis in big toe joint",
                "Big toe bends too far up",
                "Big toe hurts",
                "Big toe hurts when moving",
                "Big toe is under the second toe",
                "Big toe joint is swollen",
                "Big toe joint is stiff",
                "Big toe joint is swollen",
                "Big toe joint is tender to touch",
                "Enlarged rounded toe",
                "Feels like toe is burning",
                "Great toe metatarsophalangeal prominence",
                "Great toe microdactyly",
                "Great toe synostosis",
                "Nail loss",
                "Nail not growing the way it should",
                "Nail pulling away from cuticle",
                "Pale toe",
                "Pigeon toed",
                "Rash limited to between toes",
                "Stiff big toe",
                "Tingling and prickling in toe",
                "Toe angle cleft",
                "Toe deformity",
                "Toe pain",
                "Toe pulse absence",
                "Toe pulse weakness",
                "Toe shortness",
                "Up-going toe",
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
            "cursor-pointer fill-blue-600 stroke-white stroke-2 hover:opacity-80"
        } else {
            "cursor-pointer fill-blue-100 stroke-white stroke-2 hover:opacity-80"
        }
    };

    rsx! {
        svg {
            class: "block h-auto w-full max-w-[280px]",
            view_box: "45 100 310 735",
            role: "img",

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
                onclick: move |_| props.on_select.call(BodyParts::Scalp),
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
                onclick: move |_| props.on_select.call(BodyParts::Forehead),
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
                onclick: move |_| props.on_select.call(BodyParts::Ears),
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
                onclick: move |_| props.on_select.call(BodyParts::Ears),
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
                onclick: move |_| props.on_select.call(BodyParts::Neck),
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
                onclick: move |_| props.on_select.call(BodyParts::Face),
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
                onclick: move |_| props.on_select.call(BodyParts::Eyes),
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
                onclick: move |_| props.on_select.call(BodyParts::Eyes),
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
                onclick: move |_| props.on_select.call(BodyParts::Nose),
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
                onclick: move |_| props.on_select.call(BodyParts::Mouth),
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
                onclick: move |_| props.on_select.call(BodyParts::UpperChest),
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
                onclick: move |_| props.on_select.call(BodyParts::UpperArms),
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
                onclick: move |_| props.on_select.call(BodyParts::UpperArms),
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
                onclick: move |_| props.on_select.call(BodyParts::Elbows),
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
                onclick: move |_| props.on_select.call(BodyParts::Elbows),
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
                onclick: move |_| props.on_select.call(BodyParts::Forearms),
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
                onclick: move |_| props.on_select.call(BodyParts::Wrists),
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
                onclick: move |_| props.on_select.call(BodyParts::Hands),
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
                onclick: move |_| props.on_select.call(BodyParts::Fingers),
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
                onclick: move |_| props.on_select.call(BodyParts::Fingers),
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
                onclick: move |_| props.on_select.call(BodyParts::Forearms),
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
                onclick: move |_| props.on_select.call(BodyParts::Wrists),
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
                onclick: move |_| props.on_select.call(BodyParts::Hands),
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
                onclick: move |_| props.on_select.call(BodyParts::Fingers),
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
                onclick: move |_| props.on_select.call(BodyParts::Fingers),
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
                onclick: move |_| props.on_select.call(BodyParts::Fingers),
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
                onclick: move |_| props.on_select.call(BodyParts::Fingers),
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
                onclick: move |_| props.on_select.call(BodyParts::Breasts),
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
                onclick: move |_| props.on_select.call(BodyParts::Breasts),
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
                onclick: move |_| props.on_select.call(BodyParts::Shoulders),
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
                onclick: move |_| props.on_select.call(BodyParts::Shoulders),
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
                onclick: move |_| props.on_select.call(BodyParts::Sternum),
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
                onclick: move |_| props.on_select.call(BodyParts::Chest),
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
                onclick: move |_| props.on_select.call(BodyParts::Pelvis),
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
                onclick: move |_| props.on_select.call(BodyParts::Abdomen),
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
                onclick: move |_| props.on_select.call(BodyParts::Genitals),
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
                onclick: move |_| props.on_select.call(BodyParts::Thighs),
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
                onclick: move |_| props.on_select.call(BodyParts::Thighs),
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
                onclick: move |_| props.on_select.call(BodyParts::Knees),
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
                onclick: move |_| props.on_select.call(BodyParts::Knees),
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
                onclick: move |_| props.on_select.call(BodyParts::LowerLegs),
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
                onclick: move |_| props.on_select.call(BodyParts::LowerLegs),
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
                onclick: move |_| props.on_select.call(BodyParts::Ankles),
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
                onclick: move |_| props.on_select.call(BodyParts::Ankles),
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
                onclick: move |_| props.on_select.call(BodyParts::Feet),
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
                onclick: move |_| props.on_select.call(BodyParts::Feet),
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
                onclick: move |_| props.on_select.call(BodyParts::Toes),
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
                onclick: move |_| props.on_select.call(BodyParts::Toes),
            }
        }
    }
}
