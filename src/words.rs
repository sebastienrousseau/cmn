// Copyright © 2023 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// The `serde` crate provides the `Serialize` and `Deserialize` traits
/// that are used to serialize and deserialize the data.
extern crate serde;
use serde::{Deserialize, Serialize};

/// Contains several words for use in generating passphrases.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Words;

impl Words {
    /// Creates a new instance of `Words`.
    pub fn new() -> Self {
        Words
    }
    /// Returns a list of words for use in generating passphrases.
    pub fn words_list(&self) -> &'static [&'static str] {
        WORD_LIST
    }
}

impl Default for Words {
    fn default() -> Self {
        Self::new()
    }
}

/// The list of words.
pub const WORD_LIST: &[&str] = &[
    "aboard", "abode", "abort", "abound", "about", "above", "abroad",
    "abrupt", "absent", "absorb", "absurd", "abuse", "accent",
    "accept", "access", "accord", "accuse", "ace", "ache", "aching",
    "acid", "acidic", "acorn", "acre", "across", "act", "action",
    "active", "actor", "actual", "acute", "adapt", "add", "added",
    "addict", "adept", "adhere", "adjust", "admire", "admit", "adam",
    "afghan", "alaska", "alice", "allah", "amazon", "andrew", "anglo",
    "angola", "antony", "adobe", "adopt", "adrift", "adult", "adverb",
    "advert", "aerial", "afar", "affair", "affect", "afford", "afield",
    "afloat", "afraid", "afresh", "after", "again", "age", "agency",
    "agenda", "agent", "aghast", "agile", "ago", "agony", "agree",
    "agreed", "ahead", "aid", "aide", "aim", "air", "airman", "airy",
    "akin", "alarm", "albeit", "album", "alert", "alibi", "alien",
    "alight", "align", "alike", "alive", "alkali", "all", "alley",
    "allied", "allow", "alloy", "ally", "almond", "almost", "aloft",
    "alone", "along", "aloof", "aloud", "alpha", "alpine", "also",
    "altar", "alter", "always", "amaze", "amber", "ambush", "amen",
    "amend", "amid", "amidst", "amiss", "among", "amount", "ample",
    "amuse", "anchor", "and", "anew", "angel", "anger", "angle",
    "angry", "animal", "ankle", "annoy", "annual", "answer", "anthem",
    "anti", "any", "anyhow", "anyway", "apart", "apathy", "apex",
    "apiece", "appeal", "appear", "apple", "apply", "apron", "arcade",
    "arcane", "arch", "ardent", "are", "area", "argue", "arid",
    "april", "arab", "arctic", "athens", "austin", "bach", "baltic",
    "basque", "berlin", "bible", "arise", "arm", "armful", "armpit",
    "army", "aroma", "around", "arouse", "array", "arrest", "arrive",
    "arrow", "arson", "art", "artery", "artful", "artist", "ascent",
    "ashen", "ashore", "aside", "ask", "asleep", "aspect", "assay",
    "assent", "assert", "assess", "asset", "assign", "assist",
    "assume", "assure", "asthma", "astute", "asylum", "ate", "atlas",
    "atom", "atomic", "attach", "attack", "attain", "attend", "attic",
    "auburn", "audio", "audit", "august", "aunt", "auntie", "aura",
    "author", "auto", "autumn", "avail", "avenge", "avenue", "avert",
    "avid", "avoid", "await", "awake", "awaken", "award", "aware",
    "awash", "away", "awful", "awhile", "axes", "axiom", "axis",
    "axle", "aye", "babe", "baby", "back", "backup", "bacon", "bad",
    "badge", "badly", "bag", "baggy", "bail", "bait", "bake", "baker",
    "bakery", "bald", "ball", "ballad", "ballet", "ballot", "bamboo",
    "ban", "banal", "banana", "band", "bang", "bank", "bar", "barber",
    "bare", "barely", "barge", "bark", "barley", "barn", "baron",
    "barrel", "barren", "basalt", "base", "basic", "basil", "basin",
    "basis", "basket", "bass", "bat", "batch", "bath", "baton",
    "battle", "bay", "beach", "beacon", "beak", "beam", "bean", "bear",
    "beard", "beast", "beat", "beauty", "become", "bed", "beech",
    "beef", "beefy", "beep", "beer", "beet", "beetle", "before",
    "beggar", "begin", "behalf", "behave", "behind", "beige", "being",
    "belief", "bell", "belly", "belong", "below", "belt", "bench",
    "bend", "benign", "bent", "berry", "berth", "beset", "beside",
    "best", "bestow", "bet", "beta", "betray", "better", "beware",
    "beyond", "bias", "biceps", "bicker", "bid", "big", "bigger",
    "bike", "bile", "bill", "binary", "bind", "biopsy", "birch",
    "bird", "birdie", "birth", "bishop", "bit", "bitch", "bite",
    "bitter", "black", "blade", "blame", "bland", "blast", "blaze",
    "bleak", "blend", "bless", "blew", "blind", "blink", "blip",
    "bliss", "blitz", "block", "blond", "blood", "bloody", "bloom",
    "blot", "blouse", "blow", "blue", "bluff", "blunt", "blur",
    "blush", "boar", "board", "boast", "boat", "bodily", "body",
    "bogus", "boil", "bold", "bolt", "bomb", "bond", "bombay", "bonn",
    "boston", "brazil", "briton", "buddha", "burma", "caesar", "cairo",
    "canada", "bone", "bonnet", "bonus", "bony", "book", "boom",
    "boost", "boot", "booth", "booze", "border", "bore", "borrow",
    "bosom", "boss", "both", "bother", "bottle", "bottom", "bought",
    "bounce", "bound", "bounty", "bout", "bovine", "bow", "bowel",
    "bowl", "box", "boy", "boyish", "brace", "brain", "brainy",
    "brake", "bran", "branch", "brand", "brandy", "brass", "brave",
    "bravo", "breach", "bread", "break", "breast", "breath", "bred",
    "breed", "breeze", "brew", "brick", "bride", "bridge", "brief",
    "bright", "brim", "brine", "bring", "brink", "brisk", "broad",
    "broke", "broken", "bronze", "brook", "broom", "brown", "bruise",
    "brush", "brutal", "brute", "bubble", "buck", "bucket", "buckle",
    "budget", "buffet", "buggy", "build", "bulb", "bulge", "bulk",
    "bulky", "bull", "bullet", "bully", "bump", "bumpy", "bunch",
    "bundle", "bunk", "bunny", "burden", "bureau", "burial", "buried",
    "burly", "burn", "burnt", "burrow", "burst", "bury", "bus", "bush",
    "bust", "bustle", "busy", "but", "butler", "butt", "butter",
    "button", "buy", "buyer", "buzz", "bye", "byte", "cab", "cabin",
    "cable", "cache", "cactus", "cage", "cake", "calf", "call",
    "caller", "calm", "calmly", "came", "camel", "camera", "camp",
    "campus", "can", "canal", "canary", "cancel", "cancer", "candid",
    "candle", "candy", "cane", "canine", "canoe", "canopy", "canvas",
    "canyon", "cap", "cape", "car", "carbon", "card", "care", "career",
    "caress", "cargo", "carnal", "carp", "carpet", "carrot", "carry",
    "cart", "carl", "carol", "celtic", "chile", "china", "christ",
    "congo", "cuba", "cyprus", "czech", "cartel", "case", "cash",
    "cask", "cast", "castle", "casual", "cat", "catch", "cater",
    "cattle", "caught", "causal", "cause", "cave", "cease", "celery",
    "cell", "cellar", "cement", "censor", "census", "cereal", "cervix",
    "chain", "chair", "chalk", "chalky", "champ", "chance", "change",
    "chant", "chaos", "chap", "chapel", "charge", "charm", "chart",
    "chase", "chat", "cheap", "cheat", "check", "cheek", "cheeky",
    "cheer", "cheery", "cheese", "chef", "cherry", "chess", "chest",
    "chew", "chic", "chick", "chief", "child", "chill", "chilly",
    "chin", "chip", "choice", "choir", "choose", "chop", "choppy",
    "chord", "chorus", "chose", "chosen", "chrome", "chunk", "chunky",
    "church", "cider", "cigar", "cinema", "circa", "circle", "circus",
    "cite", "city", "civic", "civil", "clad", "claim", "clammy",
    "clan", "clap", "clash", "clasp", "class", "clause", "claw",
    "clay", "clean", "clear", "clergy", "clerk", "clever", "click",
    "client", "cliff", "climax", "climb", "clinch", "cling", "clinic",
    "clip", "cloak", "clock", "clone", "close", "closer", "closet",
    "cloth", "cloud", "cloudy", "clout", "clown", "club", "clue",
    "clumsy", "clung", "clutch", "coach", "coal", "coarse", "coast",
    "coat", "coax", "cobalt", "cobra", "coca", "cock", "cocoa", "code",
    "coffee", "coffin", "cohort", "coil", "coin", "coke", "cold",
    "collar", "colon", "colony", "colt", "column", "comb", "combat",
    "come", "comedy", "comic", "commit", "common", "compel", "comply",
    "concur", "cone", "confer", "consul", "convex", "convey", "convoy",
    "cook", "cool", "cope", "copper", "copy", "coral", "cord", "core",
    "cork", "corn", "corner", "corps", "corpse", "corpus", "cortex",
    "cosmic", "cosmos", "cost", "costly", "cosy", "cotton", "couch",
    "cough", "could", "count", "county", "coup", "couple", "coupon",
    "course", "court", "cousin", "cove", "cover", "covert", "cow",
    "coward", "cowboy", "crab", "crack", "cradle", "craft", "crafty",
    "crag", "crane", "crap", "crash", "crate", "crater", "crawl",
    "crazy", "creak", "cream", "creamy", "create", "credit", "creed",
    "creek", "creep", "creepy", "crept", "crest", "crew", "cried",
    "crime", "crisis", "crisp", "critic", "croft", "crook", "crop",
    "cross", "crow", "crowd", "crown", "crude", "cruel", "cruise",
    "crunch", "crush", "crust", "crux", "cry", "crypt", "cube",
    "cubic", "cuckoo", "cuff", "cult", "cup", "curb", "cure", "curfew",
    "curl", "curry", "curse", "cursor", "curve", "custom", "cut",
    "cute", "cycle", "cyclic", "cynic", "dad", "daddy", "dagger",
    "daily", "dairy", "daisy", "dale", "damage", "damn", "damp",
    "dampen", "dance", "danger", "dare", "dallas", "danish", "darwin",
    "david", "delhi", "derby", "diana", "dublin", "dutch", "east",
    "dark", "darken", "dash", "data", "date", "dawn", "day", "dead",
    "deadly", "deaf", "deal", "dealer", "dean", "dear", "death",
    "debate", "debit", "debris", "debt", "debtor", "decade", "decay",
    "decent", "decide", "deck", "decor", "decree", "deduce", "deed",
    "deep", "deeply", "deer", "defeat", "defect", "defend", "defer",
    "define", "defy", "degree", "deity", "delay", "delete", "delta",
    "demand", "demise", "demo", "demon", "demure", "denial", "denote",
    "dense", "dental", "deny", "depart", "depend", "depict", "deploy",
    "depot", "depth", "deputy", "derive", "desert", "design", "desire",
    "desist", "desk", "detail", "detect", "deter", "detest", "detour",
    "device", "devil", "devise", "devoid", "devote", "devour", "dial",
    "diary", "dice", "dictum", "did", "die", "diesel", "diet",
    "differ", "digest", "digit", "dine", "dinghy", "dinner", "diode",
    "dire", "direct", "dirt", "dirty", "disc", "disco", "dish", "disk",
    "dismal", "dispel", "ditch", "dive", "divert", "divide", "divine",
    "dizzy", "docile", "dock", "doctor", "dog", "dogma", "dole",
    "doll", "dollar", "dolly", "domain", "dome", "domino", "donate",
    "done", "donkey", "donor", "doom", "door", "dorsal", "dose",
    "double", "doubt", "dough", "dour", "dove", "down", "dozen",
    "draft", "drag", "dragon", "drain", "drama", "drank", "draw",
    "drawer", "dread", "dream", "dreary", "dress", "drew", "dried",
    "drift", "drill", "drink", "drip", "drive", "driver", "drop",
    "drove", "drown", "drug", "drum", "drunk", "dry", "dual", "duck",
    "duct", "due", "duel", "duet", "duke", "dull", "duly", "dumb",
    "dummy", "dump", "dune", "dung", "duress", "during", "dusk",
    "dust", "dusty", "duty", "dwarf", "dwell", "dyer", "dying",
    "dynamo", "each", "eager", "eagle", "ear", "earl", "early", "earn",
    "earth", "ease", "easel", "easily", "easter", "easy", "eat",
    "eaten", "eater", "echo", "eddy", "edge", "edible", "eden",
    "edward", "eric", "essex", "europe", "eve", "exodus", "france",
    "french", "friday", "edict", "edit", "editor", "eerie", "eerily",
    "effect", "effort", "egg", "ego", "eight", "eighth", "eighty",
    "either", "elbow", "elder", "eldest", "elect", "eleven", "elicit",
    "elite", "else", "elude", "elves", "embark", "emblem", "embryo",
    "emerge", "emit", "empire", "employ", "empty", "enable", "enamel",
    "end", "endure", "enemy", "energy", "engage", "engine", "enjoy",
    "enlist", "enough", "ensure", "entail", "enter", "entire", "entry",
    "envoy", "envy", "enzyme", "epic", "epoch", "equal", "equate",
    "equip", "equity", "era", "erase", "erect", "erode", "erotic",
    "errant", "error", "escape", "escort", "essay", "estate", "esteem",
    "ethic", "ethnic", "evade", "even", "event", "ever", "every",
    "evict", "evil", "evoke", "evolve", "exact", "exam", "exceed",
    "excel", "except", "excess", "excise", "excite", "excuse",
    "exempt", "exert", "exile", "exist", "exit", "exotic", "expand",
    "expect", "expert", "expire", "export", "expose", "extend",
    "extra", "eye", "eyed", "fabric", "face", "facial", "fact",
    "factor", "fade", "fail", "faint", "fair", "fairly", "fairy",
    "faith", "fake", "falcon", "fall", "false", "falter", "fame",
    "family", "famine", "famous", "fan", "fancy", "far", "farce",
    "fare", "farm", "farmer", "fast", "fasten", "faster", "fat",
    "fatal", "fate", "father", "fatty", "fault", "faulty", "fauna",
    "fear", "feast", "feat", "fed", "fee", "feeble", "feed", "feel",
    "feet", "fell", "fellow", "felt", "female", "fence", "fend",
    "ferry", "fetal", "fetch", "feudal", "fever", "few", "fewer",
    "fiance", "fiasco", "fiddle", "field", "fiend", "fierce", "fiery",
    "fifth", "fifty", "fig", "fight", "figure", "file", "fill",
    "filled", "filler", "film", "filter", "filth", "filthy", "final",
    "finale", "find", "fine", "finger", "finish", "finite", "fire",
    "firm", "firmly", "first", "fiscal", "fish", "fisher", "fist",
    "fit", "fitful", "five", "fix", "flag", "flair", "flak", "flame",
    "flank", "flap", "flare", "flash", "flask", "flat", "flaw", "fled",
    "flee", "fleece", "fleet", "flesh", "fleshy", "flew", "flick",
    "flight", "flimsy", "flint", "flirt", "float", "flock", "flood",
    "floor", "floppy", "flora", "floral", "flour", "flow", "flower",
    "fluent", "fluffy", "fluid", "flung", "flurry", "flush", "flute",
    "flux", "fly", "flyer", "foal", "foam", "focal", "focus", "fog",
    "foil", "fold", "folk", "follow", "folly", "fond", "fondly",
    "font", "food", "fool", "foot", "for", "forbid", "force", "ford",
    "forest", "forge", "forget", "fork", "form", "formal", "format",
    "former", "fort", "forth", "forty", "forum", "fossil", "foster",
    "foul", "found", "four", "fourth", "fox", "foyer", "frail",
    "frame", "franc", "frank", "fraud", "free", "freed", "freely",
    "freer", "freeze", "frenzy", "fresh", "friar", "fridge", "fried",
    "friend", "fright", "fringe", "frock", "frog", "from", "front",
    "frost", "frosty", "frown", "frozen", "frugal", "fruit", "fudge",
    "fuel", "fulfil", "full", "fully", "fun", "fund", "funny", "fur",
    "furry", "fury", "fuse", "fusion", "fuss", "fussy", "futile",
    "future", "fuzzy", "gadget", "gag", "gain", "gala", "galaxy",
    "gale", "gall", "galley", "gallon", "gallop", "gamble", "game",
    "gamma", "gang", "gandhi", "gaul", "gemini", "geneva", "george",
    "german", "gloria", "god", "gothic", "greece", "gap", "garage",
    "garden", "garlic", "gas", "gasp", "gate", "gather", "gauge",
    "gaunt", "gave", "gay", "gaze", "gear", "geese", "gender", "gene",
    "genial", "genius", "genre", "gentle", "gently", "gentry", "genus",
    "get", "ghetto", "ghost", "giant", "gift", "giggle", "gill",
    "gilt", "ginger", "girl", "give", "given", "glad", "glade",
    "glance", "gland", "glare", "glass", "glassy", "gleam", "glee",
    "glide", "global", "globe", "gloom", "gloomy", "glory", "gloss",
    "glossy", "glove", "glow", "glue", "goal", "goat", "gold",
    "golden", "golf", "gone", "gong", "good", "goose", "gorge", "gory",
    "gosh", "gospel", "gossip", "got", "govern", "gown", "grab",
    "grace", "grade", "grain", "grand", "grant", "grape", "graph",
    "grasp", "grass", "grassy", "grate", "grave", "gravel", "gravy",
    "gray", "grease", "greasy", "great", "greed", "greedy", "green",
    "greet", "grew", "grey", "grid", "grief", "grill", "grim", "grin",
    "grind", "greek", "hague", "haiti", "hanoi", "harry", "havana",
    "hawaii", "hebrew", "henry", "hermes", "grip", "grit", "gritty",
    "groan", "groin", "groom", "groove", "gross", "ground", "group",
    "grove", "grow", "grown", "growth", "grudge", "grunt", "guard",
    "guess", "guest", "guide", "guild", "guilt", "guilty", "guise",
    "guitar", "gulf", "gully", "gun", "gunman", "guru", "gut", "guy",
    "gypsy", "habit", "hack", "had", "hail", "hair", "hairy", "hale",
    "half", "hall", "halt", "hamlet", "hammer", "hand", "handle",
    "handy", "hang", "hangar", "happen", "happy", "harass", "hard",
    "harder", "hardly", "hare", "harem", "harm", "harp", "harsh",
    "has", "hash", "hassle", "haste", "hasten", "hasty", "hat",
    "hatch", "hate", "haul", "haunt", "have", "haven", "havoc", "hawk",
    "hazard", "haze", "hazel", "hazy", "head", "heal", "health",
    "heap", "hear", "heard", "heart", "hearth", "hearty", "heat",
    "heater", "heaven", "heavy", "heck", "hectic", "hedge", "heel",
    "hefty", "height", "heir", "held", "helium", "helix", "hell",
    "hello", "helm", "helmet", "help", "hemp", "hence", "her",
    "herald", "herb", "herd", "here", "hereby", "hernia", "hero",
    "heroic", "heroin", "hey", "heyday", "hick", "hidden", "hide",
    "high", "higher", "highly", "hill", "him", "hind", "hint", "hippy",
    "hire", "his", "hiss", "hit", "hive", "hindu", "hitler", "idaho",
    "inca", "india", "indian", "iowa", "iran", "iraq", "irish",
    "hoard", "hoarse", "hobby", "hockey", "hold", "holder", "hole",
    "hollow", "holly", "holy", "home", "honest", "honey", "hood",
    "hook", "hope", "horn", "horny", "horrid", "horror", "horse",
    "hose", "host", "hot", "hotel", "hound", "hour", "house", "hover",
    "how", "huge", "hull", "human", "humane", "humble", "humid",
    "hung", "hunger", "hungry", "hunt", "hurdle", "hurl", "hurry",
    "hurt", "hush", "hut", "hybrid", "hymn", "hyphen", "ice", "icing",
    "icon", "idea", "ideal", "idiom", "idiot", "idle", "idly", "idol",
    "ignite", "ignore", "ill", "image", "immune", "impact", "imply",
    "import", "impose", "incest", "inch", "income", "incur", "indeed",
    "index", "indoor", "induce", "inept", "inert", "infant", "infect",
    "infer", "influx", "inform", "inject", "injure", "injury",
    "inlaid", "inland", "inlet", "inmate", "inn", "innate", "inner",
    "input", "insane", "insect", "insert", "inset", "inside", "insist",
    "insult", "insure", "intact", "intake", "intend", "inter", "into",
    "invade", "invent", "invest", "invite", "invoke", "inward", "iron",
    "ironic", "irony", "island", "isle", "issue", "itch", "item",
    "isaac", "isabel", "islam", "israel", "italy", "ivan", "jack",
    "jacob", "james", "japan", "itself", "ivory", "jacket", "jade",
    "jaguar", "jail", "jargon", "jaw", "jazz", "jeep", "java",
    "jersey", "jesus", "jewish", "jim", "john", "jordan", "joseph",
    "judas", "judy", "jelly", "jerky", "jest", "jet", "jewel", "job",
    "jock", "jockey", "join", "joint", "joke", "jolly", "jolt", "joy",
    "joyful", "joyous", "judge", "juice", "juicy", "jumble", "jumbo",
    "july", "june", "kansas", "karl", "kenya", "koran", "korea",
    "kuwait", "laos", "latin", "leo", "jump", "jungle", "junior",
    "junk", "junta", "jury", "just", "karate", "keel", "keen", "keep",
    "keeper", "kept", "kernel", "kettle", "key", "khaki", "kick",
    "kid", "kidnap", "kidney", "kill", "killer", "kin", "kind",
    "kindly", "king", "kiss", "kite", "kitten", "knack", "knee",
    "knew", "knife", "knight", "knit", "knob", "knock", "knot", "know",
    "known", "label", "lace", "lack", "lad", "ladder", "laden", "lady",
    "lagoon", "laity", "lake", "lamb", "lame", "lamp", "lance", "land",
    "lane", "lap", "lapse", "large", "larval", "laser", "last",
    "latch", "late", "lately", "latent", "later", "latest", "latter",
    "laugh", "launch", "lava", "lavish", "law", "lawful", "lawn",
    "lawyer", "lay", "layer", "layman", "lazy", "lead", "leader",
    "leaf", "leafy", "league", "leak", "leaky", "lean", "leap",
    "learn", "lease", "leash", "least", "leave", "led", "ledge",
    "left", "leg", "legacy", "legal", "legend", "legion", "lemon",
    "lend", "length", "lens", "lent", "leper", "lesion", "less",
    "lessen", "lesser", "lesson", "lest", "let", "lethal", "letter",
    "level", "lever", "levy", "lewis", "liable", "liar", "libel",
    "libya", "lima", "lisbon", "liz", "london", "louvre", "lucy",
    "luther", "madame", "madrid", "lice", "lick", "lid", "lie", "lied",
    "life", "lift", "light", "like", "likely", "limb", "lime", "limit",
    "limp", "line", "linear", "linen", "linger", "link", "lion", "lip",
    "liquid", "liquor", "list", "listen", "lit", "live", "lively",
    "liver", "lizard", "load", "loaf", "loan", "lobby", "lobe",
    "local", "locate", "lock", "locus", "lodge", "loft", "lofty",
    "log", "logic", "logo", "lone", "lonely", "long", "longer", "look",
    "loop", "loose", "loosen", "loot", "lord", "lorry", "lose", "loss",
    "lost", "lot", "lotion", "lotus", "loud", "loudly", "lounge",
    "lousy", "love", "lovely", "lover", "low", "lower", "lowest",
    "loyal", "lucid", "luck", "lucky", "lull", "lump", "lumpy",
    "lunacy", "lunar", "lunch", "lung", "lure", "lurid", "lush",
    "lust", "lute", "luxury", "lying", "lymph", "lynch", "lyric",
    "macho", "macro", "mad", "madam", "made", "mafia", "magic",
    "magma", "magnet", "magnum", "maid", "maiden", "mail", "main",
    "mainly", "major", "make", "maker", "male", "malice", "mall",
    "malt", "mammal", "manage", "mane", "malta", "maria", "mars",
    "mary", "maya", "mecca", "mexico", "miami", "mickey", "milan",
    "mania", "manic", "manner", "manor", "mantle", "manual", "manure",
    "many", "map", "maple", "marble", "march", "mare", "margin",
    "marina", "mark", "market", "marry", "marsh", "martin", "martyr",
    "mask", "mason", "mass", "mast", "master", "match", "mate",
    "matrix", "matter", "mature", "maxim", "may", "maybe", "mayor",
    "maze", "mead", "meadow", "meal", "mean", "meant", "meat", "medal",
    "media", "median", "medic", "medium", "meet", "mellow", "melody",
    "melon", "melt", "member", "memo", "memory", "menace", "mend",
    "mental", "mentor", "menu", "mercy", "mere", "merely", "merge",
    "merger", "merit", "merry", "mesh", "mess", "messy", "met",
    "metal", "meter", "method", "methyl", "metric", "metro", "mid",
    "midday", "middle", "midst", "midway", "might", "mighty", "mild",
    "mildew", "mile", "milk", "milky", "mill", "mimic", "mince",
    "mind", "mine", "mini", "mink", "minor", "mint", "minus", "minute",
    "mirror", "mirth", "misery", "miss", "mist", "misty", "mite",
    "mix", "moan", "moat", "mobile", "mock", "mode", "model", "modem",
    "modern", "modest", "modify", "module", "moist", "molar", "mole",
    "molten", "moment", "monaco", "monday", "moscow", "moses",
    "moslem", "mrs", "munich", "muslim", "naples", "nazi", "money",
    "monies", "monk", "monkey", "month", "mood", "moody", "moon",
    "moor", "moral", "morale", "morbid", "more", "morgue", "mortal",
    "mortar", "mosaic", "mosque", "moss", "most", "mostly", "moth",
    "mother", "motion", "motive", "motor", "mould", "mount", "mourn",
    "mouse", "mouth", "move", "movie", "much", "muck", "mucus", "mud",
    "muddle", "muddy", "mule", "mummy", "murder", "murky", "murmur",
    "muscle", "museum", "music", "mussel", "must", "mutant", "mute",
    "mutiny", "mutter", "mutton", "mutual", "muzzle", "myopic",
    "myriad", "myself", "mystic", "myth", "nadir", "nail", "naked",
    "name", "namely", "nape", "napkin", "narrow", "nasal", "nasty",
    "nation", "native", "nature", "nausea", "naval", "nave", "navy",
    "near", "nearer", "nearly", "neat", "neatly", "neck", "need",
    "needle", "needy", "negate", "neon", "nephew", "nepal", "newark",
    "nile", "nobel", "north", "norway", "ohio", "oscar", "oslo",
    "oxford", "nerve", "nest", "neural", "never", "newly", "next",
    "nice", "nicely", "niche", "nickel", "niece", "night", "nimble",
    "nine", "ninety", "ninth", "noble", "nobody", "node", "noise",
    "noisy", "non", "none", "noon", "nor", "norm", "normal", "nose",
    "nosy", "not", "note", "notice", "notify", "notion", "nought",
    "noun", "novel", "novice", "now", "nozzle", "nude", "null", "numb",
    "number", "nurse", "nylon", "nymph", "oak", "oasis", "oath",
    "obese", "obey", "object", "oblige", "oboe", "obtain", "occult",
    "occupy", "occur", "ocean", "octave", "odd", "off", "offend",
    "offer", "office", "offset", "often", "oil", "oily", "okay", "old",
    "older", "oldest", "olive", "omega", "omen", "omit", "once", "one",
    "onion", "only", "onset", "onto", "onus", "onward", "opaque",
    "open", "openly", "opera", "opium", "oppose", "optic", "option",
    "oracle", "oral", "orange", "orbit", "orchid", "ordeal", "order",
    "organ", "orgasm", "orient", "origin", "ornate", "orphan", "other",
    "otter", "ought", "ounce", "our", "out", "outer", "output",
    "outset", "oval", "oven", "over", "overt", "owe", "owing", "owl",
    "own", "owner", "oxide", "oxygen", "oyster", "ozone", "pace",
    "pack", "packet", "pact", "paddle", "paddy", "pagan", "page",
    "paid", "pain", "paint", "pair", "palace", "pale", "palm", "panel",
    "panic", "panama", "paris", "pascal", "paul", "peking", "peru",
    "peter", "philip", "poland", "polish", "papa", "papal", "paper",
    "parade", "parcel", "pardon", "parent", "parish", "park", "parody",
    "parrot", "part", "partly", "party", "pass", "past", "paste",
    "pastel", "pastor", "pastry", "pat", "patch", "patent", "path",
    "patio", "patrol", "patron", "pause", "pave", "pawn", "pay",
    "peace", "peach", "peak", "pear", "pearl", "pedal", "peel", "peer",
    "pelvic", "pelvis", "pen", "penal", "pence", "pencil", "penis",
    "penny", "people", "pepper", "per", "perch", "peril", "period",
    "perish", "permit", "person", "pest", "petite", "petrol", "petty",
    "phase", "phone", "photo", "phrase", "piano", "pick", "picket",
    "picnic", "pie", "piece", "pier", "pierce", "piety", "pig",
    "pigeon", "piggy", "pike", "pile", "pill", "pillar", "pillow",
    "pilot", "pin", "pinch", "pine", "pink", "pint", "pious", "pipe",
    "pirate", "piss", "pistol", "piston", "pit", "pitch", "pity",
    "pivot", "pixel", "pizza", "place", "placid", "plague", "plain",
    "plan", "plane", "planet", "plank", "plant", "plasma", "plate",
    "play", "player", "plea", "plead", "please", "pledge", "plenty",
    "plenum", "plight", "plot", "ploy", "plug", "plum", "plump",
    "plunge", "plural", "plus", "plush", "pocket", "poem", "poet",
    "poetic", "poetry", "point", "poison", "polar", "pole", "police",
    "policy", "polite", "poll", "pollen", "polo", "pond", "ponder",
    "pony", "pool", "poor", "poorly", "pop", "pope", "poppy", "pore",
    "pork", "port", "portal", "pose", "posh", "post", "postal", "pot",
    "potato", "potent", "pouch", "pound", "pour", "powder", "power",
    "praise", "pray", "prayer", "preach", "prefer", "prefix", "press",
    "prague", "quebec", "rex", "rhine", "ritz", "robert", "roman",
    "rome", "rosa", "russia", "pretty", "price", "pride", "priest",
    "primal", "prime", "prince", "print", "prior", "prism", "prison",
    "privy", "prize", "probe", "profit", "prompt", "prone", "proof",
    "propel", "proper", "prose", "proton", "proud", "prove", "proven",
    "proxy", "prune", "psalm", "pseudo", "psyche", "pub", "public",
    "puff", "pull", "pulp", "pulpit", "pulsar", "pulse", "pump",
    "punch", "punish", "punk", "pupil", "puppet", "puppy", "pure",
    "purely", "purge", "purify", "purple", "purse", "pursue", "push",
    "pushy", "pussy", "put", "putt", "puzzle", "quaint", "quake",
    "quarry", "quartz", "quay", "queen", "queer", "query", "quest",
    "queue", "quick", "quid", "quiet", "quilt", "quirk", "quit",
    "quite", "quiver", "quiz", "quota", "quote", "rabbit", "race",
    "racial", "racism", "rack", "racket", "radar", "radio", "radish",
    "radius", "raffle", "raft", "rage", "raid", "rail", "rain",
    "rainy", "raise", "rally", "ramp", "random", "range", "rank",
    "ransom", "rape", "rapid", "rare", "rarely", "rarity", "rash",
    "rat", "rate", "rather", "ratify", "ratio", "rattle", "rave",
    "raven", "raw", "ray", "razor", "reach", "react", "read", "reader",
    "ready", "real", "really", "realm", "reap", "rear", "reason",
    "rebel", "recall", "recent", "recess", "recipe", "reckon",
    "record", "recoup", "rector", "red", "redeem", "reduce", "reed",
    "reef", "refer", "reform", "refuge", "refuse", "regal", "regard",
    "regent", "regime", "region", "regret", "reign", "reject",
    "relate", "relax", "relay", "relic", "relief", "relish", "rely",
    "remain", "remark", "remedy", "remind", "remit", "remote",
    "remove", "renal", "render", "rent", "rental", "repair", "repeal",
    "repeat", "repent", "reply", "report", "rescue", "resent",
    "reside", "resign", "resin", "resist", "resort", "rest", "result",
    "resume", "retail", "retain", "retina", "retire", "return",
    "reveal", "review", "revise", "revive", "revolt", "reward",
    "rhino", "rhyme", "rhythm", "ribbon", "rice", "rich", "rick",
    "rid", "ride", "rider", "ridge", "rife", "rifle", "rift", "right",
    "rigid", "ring", "rinse", "riot", "ripe", "ripen", "ripple",
    "rise", "risk", "risky", "rite", "ritual", "rival", "river",
    "road", "roar", "roast", "rob", "robe", "robin", "robot", "robust",
    "rock", "rocket", "rocky", "rod", "rode", "rodent", "rogue",
    "role", "roll", "roof", "room", "root", "rope", "rose", "rosy",
    "rotate", "rotor", "rotten", "rouge", "rough", "round", "route",
    "rover", "row", "royal", "rubble", "ruby", "rudder", "rude",
    "rugby", "ruin", "rule", "ruler", "rumble", "rump", "run", "rune",
    "rung", "runway", "rural", "rush", "rust", "rustic", "rusty",
    "sack", "sacred", "sad", "saddle", "sadism", "sadly", "safari",
    "safe", "safely", "safer", "safety", "saga", "sage", "said",
    "sail", "sailor", "saint", "sake", "salad", "salary", "sale",
    "saline", "sahara", "sam", "saturn", "saudi", "saxon", "scot",
    "seoul", "somali", "sony", "soviet", "saliva", "salmon", "saloon",
    "salt", "salty", "salute", "same", "sample", "sand", "sandy",
    "sane", "sash", "satan", "satin", "satire", "sauce", "sauna",
    "savage", "save", "say", "scale", "scalp", "scan", "scant", "scar",
    "scarce", "scare", "scarf", "scary", "scene", "scenic", "scent",
    "school", "scope", "score", "scorn", "scotch", "scout", "scrap",
    "scream", "screen", "screw", "script", "scroll", "scrub", "scum",
    "sea", "seal", "seam", "seaman", "search", "season", "seat",
    "second", "secret", "sect", "sector", "secure", "see", "seed",
    "seeing", "seek", "seem", "seize", "seldom", "select", "self",
    "sell", "seller", "semi", "senate", "send", "senile", "senior",
    "sense", "sensor", "sent", "sentry", "sequel", "serene", "serial",
    "series", "sermon", "serum", "serve", "server", "set", "settle",
    "seven", "severe", "sewage", "sex", "sexual", "sexy", "shabby",
    "shade", "shadow", "shady", "shaft", "shaggy", "shah", "shake",
    "shaky", "shall", "sham", "shame", "shape", "share", "shark",
    "sharp", "shawl", "she", "shear", "sheen", "sheep", "sheer",
    "sheet", "shelf", "shell", "sherry", "shield", "shift", "shine",
    "shiny", "ship", "shire", "shirt", "shit", "shiver", "shock",
    "shoe", "shook", "shoot", "shop", "shore", "short", "shot",
    "should", "shout", "show", "shower", "shrank", "shrewd", "shrill",
    "shrimp", "shrine", "shrink", "shrub", "shrug", "shut", "shy",
    "shyly", "sick", "side", "siege", "sigh", "sight", "sigma", "sign",
    "signal", "silent", "silk", "silken", "silky", "sill", "silly",
    "silver", "simple", "simply", "since", "sinful", "sing", "singer",
    "single", "sink", "sir", "siren", "sister", "sit", "site", "six",
    "sixth", "sixty", "size", "sketch", "skill", "skin", "skinny",
    "skip", "skirt", "skull", "sky", "slab", "slack", "slain", "slam",
    "slang", "slap", "slate", "slater", "slave", "sleek", "sleep",
    "sleepy", "sleeve", "slice", "slick", "slid", "slide", "slight",
    "slim", "slimy", "sling", "slip", "slit", "slogan", "slope",
    "sloppy", "slot", "slow", "slowly", "slug", "slum", "slump",
    "smack", "small", "smart", "smash", "smear", "smell", "smelly",
    "smelt", "smile", "smoke", "smoky", "smooth", "smug", "snack",
    "snail", "snake", "snap", "snatch", "sneak", "snow", "snowy",
    "snug", "soak", "soap", "sober", "soccer", "social", "sock",
    "socket", "soda", "sodden", "sodium", "sofa", "soft", "soften",
    "softly", "soggy", "soil", "solar", "sold", "sole", "solely",
    "solemn", "solid", "solo", "solve", "some", "son", "sonar",
    "sonata", "song", "sonic", "soon", "sooner", "soot", "soothe",
    "sordid", "sore", "sorrow", "sorry", "sort", "soul", "sound",
    "soup", "sour", "source", "space", "spade", "span", "spare",
    "spark", "spain", "stalin", "sudan", "suez", "sunday", "sweden",
    "swiss", "sydney", "syria", "taiwan", "sparse", "spasm", "spat",
    "spate", "speak", "spear", "speech", "speed", "speedy", "spell",
    "spend", "sperm", "sphere", "spice", "spicy", "spider", "spiky",
    "spill", "spin", "spinal", "spine", "spiral", "spirit", "spit",
    "spite", "splash", "split", "spoil", "spoke", "sponge", "spoon",
    "sport", "spot", "spouse", "spray", "spread", "spree", "spring",
    "sprint", "spur", "squad", "square", "squash", "squat", "squid",
    "stab", "stable", "stack", "staff", "stage", "stain", "stair",
    "stake", "stale", "stall", "stamp", "stance", "stand", "staple",
    "star", "starch", "stare", "stark", "start", "starve", "state",
    "static", "statue", "status", "stay", "stead", "steady", "steak",
    "steal", "steam", "steel", "steep", "steer", "stem", "stench",
    "step", "stereo", "stern", "stew", "stick", "sticky", "stiff",
    "stifle", "stigma", "still", "sting", "stint", "stir", "stitch",
    "stock", "stocky", "stone", "stony", "stool", "stop", "store",
    "storm", "stormy", "story", "stout", "stove", "strain", "strait",
    "strand", "strap", "strata", "straw", "stray", "streak", "stream",
    "street", "stress", "strict", "stride", "strife", "strike",
    "string", "strip", "strive", "stroke", "stroll", "strong", "stud",
    "studio", "study", "stuff", "stuffy", "stunt", "stupid", "sturdy",
    "style", "submit", "subtle", "subtly", "suburb", "such", "suck",
    "sudden", "sue", "suffer", "sugar", "suit", "suite", "suitor",
    "sullen", "sultan", "sum", "summer", "summit", "summon", "sun",
    "sunny", "sunset", "super", "superb", "supper", "supple", "supply",
    "sure", "surely", "surf", "surge", "survey", "suture", "swamp",
    "swan", "swap", "swarm", "sway", "swear", "sweat", "sweaty",
    "sweep", "sweet", "swell", "swift", "swim", "swine", "swing",
    "swirl", "switch", "sword", "swore", "symbol", "synod", "syntax",
    "syrup", "system", "table", "tablet", "taboo", "tacit", "tackle",
    "tact", "tactic", "tail", "tailor", "take", "tale", "talent",
    "talk", "tall", "tally", "tame", "tandem", "tangle", "tank", "tap",
    "tape", "target", "tariff", "tart", "task", "taste", "tarzan",
    "taurus", "tehran", "teresa", "texas", "thomas", "tibet", "tokyo",
    "tom", "turk", "tasty", "tattoo", "taut", "tavern", "tax", "taxi",
    "tea", "teach", "teak", "team", "tear", "tease", "tech", "teeth",
    "tell", "temper", "temple", "tempo", "tempt", "ten", "tenant",
    "tend", "tender", "tendon", "tennis", "tenor", "tense", "tensor",
    "tent", "tenth", "tenure", "term", "terror", "test", "text",
    "than", "thank", "that", "the", "their", "them", "theme", "then",
    "thence", "theory", "there", "these", "thesis", "they", "thick",
    "thief", "thigh", "thin", "thing", "think", "third", "thirst",
    "thirty", "this", "thorn", "those", "though", "thread", "threat",
    "three", "thrill", "thrive", "throat", "throne", "throng", "throw",
    "thrust", "thud", "thug", "thumb", "thus", "thyme", "tick",
    "ticket", "tidal", "tide", "tidy", "tie", "tier", "tiger", "tight",
    "tile", "till", "tilt", "timber", "time", "timid", "tin", "tiny",
    "tip", "tissue", "title", "toad", "toast", "today", "toilet",
    "token", "told", "toll", "tomato", "tomb", "tonal", "tone",
    "tongue", "tonic", "too", "took", "tool", "tooth", "top", "topaz",
    "topic", "torch", "torque", "torso", "tort", "toss", "total",
    "touch", "tough", "tour", "toward", "towel", "tower", "town",
    "toxic", "toxin", "trace", "track", "tract", "trade", "tragic",
    "trail", "train", "trait", "tram", "trance", "trap", "trauma",
    "travel", "tray", "tread", "treat", "treaty", "treble", "tree",
    "trek", "tremor", "trench", "trend", "trendy", "trial", "tribal",
    "tribe", "trick", "tricky", "tried", "trifle", "trim", "trio",
    "trip", "triple", "troop", "trophy", "trot", "trough", "trout",
    "truce", "truck", "true", "truly", "trunk", "trust", "truth",
    "try", "tsar", "tube", "tumble", "tuna", "tundra", "tune", "tung",
    "tunic", "tunnel", "turban", "turf", "turn", "turtle", "tutor",
    "tweed", "twelve", "turkey", "uganda", "venice", "venus", "vienna",
    "viking", "virgo", "warsaw", "west", "yale", "twenty", "twice",
    "twin", "twist", "two", "tycoon", "tying", "type", "tyrant",
    "ugly", "ulcer", "ultra", "umpire", "unable", "uncle", "under",
    "uneasy", "unfair", "unify", "union", "unique", "unit", "unite",
    "unity", "unlike", "unrest", "unruly", "until", "update", "upheld",
    "uphill", "uphold", "upon", "uproar", "upset", "upshot", "uptake",
    "upturn", "upward", "urban", "urge", "urgent", "urging", "urine",
    "usable", "usage", "use", "useful", "user", "usual", "uterus",
    "utmost", "utter", "vacant", "vacuum", "vagina", "vague", "vain",
    "valet", "valid", "valley", "value", "valve", "van", "vanish",
    "vanity", "vary", "vase", "vast", "vat", "vault", "vector", "veil",
    "vein", "velvet", "vendor", "veneer", "venom", "vent", "venue",
    "verb", "verbal", "verge", "verify", "verity", "verse", "versus",
    "very", "vessel", "vest", "veto", "via", "viable", "vicar", "vice",
    "victim", "victor", "video", "view", "vigil", "vile", "villa",
    "vine", "vinyl", "viola", "violet", "violin", "viral", "virgin",
    "virtue", "virus", "visa", "vision", "visit", "visual", "vital",
    "vivid", "vocal", "vodka", "vogue", "voice", "void", "volley",
    "volume", "vomit", "vote", "vowel", "voyage", "vulgar", "wade",
    "wage", "waist", "wait", "waiter", "wake", "walk", "walker",
    "wall", "wallet", "walnut", "wander", "want", "war", "warden",
    "warm", "warmth", "warn", "warp", "wary", "was", "wash", "wasp",
    "waste", "watch", "water", "watery", "wave", "way", "weak",
    "weaken", "wealth", "weapon", "wear", "weary", "wedge", "wee",
    "weed", "week", "weekly", "weep", "weight", "weird", "well",
    "were", "wet", "whale", "wharf", "what", "wheat", "wheel", "when",
    "whence", "where", "which", "whiff", "whig", "while", "whim",
    "whip", "whisky", "white", "who", "whole", "wholly", "whom",
    "whore", "whose", "why", "wide", "widely", "widen", "wider",
    "widow", "width", "wife", "wild", "wildly", "wilful", "will",
    "willow", "win", "wind", "window", "windy", "wine", "wing", "wink",
    "winner", "winter", "wipe", "wire", "wisdom", "wise", "wish",
    "wit", "witch", "with", "within", "witty", "wizard", "woke",
    "wolf", "wolves", "woman", "womb", "won", "wonder", "wood",
    "wooden", "woods", "woody", "wool", "word", "work", "worker",
    "world", "worm", "worry", "worse", "worst", "worth", "worthy",
    "would", "wound", "wrap", "wrath", "wreath", "wreck", "wright",
    "wrist", "writ", "write", "writer", "wrong", "xerox", "yacht",
    "yard", "yarn", "yeah", "year", "yeast", "yellow", "yet", "yield",
    "yogurt", "yolk", "you", "young", "your", "yemen", "york", "zaire",
    "zurich", "aback", "abbey", "abbot", "abide", "ablaze", "able",
    "youth", "zeal", "zebra", "zenith", "zero", "zigzag", "zinc",
    "zombie", "zone",
];
