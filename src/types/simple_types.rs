use serde::Deserialize;
use validator::{Validate, ValidationError};
use lazy_static::lazy_static;
use regex::Regex;

/// The accordion-middle type may have values of 1, 2, or 3, corresponding to having 1 to 3 dots in the middle section of the accordion registration symbol.
pub type AccordionMiddle = u8;

/// The MusicXML format supports eight levels of beaming, up to 1024th notes. Unlike the number-level type, the beam-level type identifies concurrent beams in a beam group. It does not distinguish overlapping beams such as grace notes within regular notes, or beams used in different voices.
pub type BeamLevel = u8;

/// The color type indicates the color of an element. Color may be represented as hexadecimal RGB triples, as in HTML, or as hexadecimal ARGB tuples, with the A indicating alpha of transparency. An alpha value of 00 is totally transparent; FF is totally opaque. If RGB is used, the A value is assumed to be FF.  For instance, the RGB value "#800080" represents purple. An ARGB value of "#40800080" would be a transparent purple.  As in SVG 1.1, colors are defined in terms of the sRGB color space (IEC 61966).
pub type Color = String;

/// The comma-separated-text type is used to specify a comma-separated list of text elements, as is used by the font-family attribute.
pub type CommaSeparatedText = String;

/// The distance-type defines what type of distance is being defined in a distance element. Values include beam and hyphen. This is left as a string so that other application-specific types can be defined, but it is made a separate type so that it can be redefined more strictly.
pub type DistanceType = String;

/// The divisions type is used to express values in terms of the musical divisions defined by the divisions element. It is preferred that these be integer values both for MIDI interoperability and to avoid roundoff errors.
pub type Divisions = f64;

/// The ending-number type is used to specify either a comma-separated list of positive integers without leading zeros, or a string of zero or more spaces. It is used for the number attribute of the ending element. The zero or more spaces version is used when software knows that an ending is present, but cannot determine the type of the ending.
pub type EndingNumber = String;

/// The fifths type represents the number of flats or sharps in a traditional key signature. Negative numbers are used for flats and positive numbers for sharps, reflecting the key's placement within the circle of fifths (hence the type name).
pub type Fifths = i64;

/// The font-size can be one of the CSS font sizes or a numeric point size.
pub type FontSize = f64;

/// The line-width-type defines what type of line is being defined in a line-width element. Values include beam, bracket, dashes, enclosure, ending, extend, heavy barline, leger, light barline, octave shift, pedal, slur middle, slur tip, staff, stem, tie middle, tie tip, tuplet bracket, and wedge. This is left as a string so that other application-specific types can be defined, but it is made a separate type so that it can be redefined more strictly.
pub type LineWidthType = String;

/// The midi-16 type is used to express MIDI 1.0 values that range from 1 to 16.
pub type Midi16 = u8;

/// The midi-128 type is used to express MIDI 1.0 values that range from 1 to 128.
pub type Midi128 = u8;

/// The midi-16384 type is used to express MIDI 1.0 values that range from 1 to 16,384.
pub type Midi16384 = u16;

/// The millimeters type is a number representing millimeters. This is used in the scaling element to provide a default scaling from tenths to physical units.
pub type Millimeters = f64;

/// The mode type is used to specify major/minor and other mode distinctions. Valid mode values include major, minor, dorian, phrygian, lydian, mixolydian, aeolian, ionian, locrian, and none.
pub type Mode = String;

/// The non-negative-decimal type specifies a non-negative decimal value.
pub type NonNegativeDecimal = f64;

/// Slurs, tuplets, and many other features can be concurrent and overlapping within a single musical part. The number-level type distinguishes up to six concurrent objects of the same type. A reading program should be prepared to handle cases where the number-levels stop in an arbitrary order. Different numbers are needed when the features overlap in MusicXML document order. When a number-level value is implied, the value is 1 by default.
pub type NumberLevel = u8;

/// The number-of-lines type is used to specify the number of lines in text decoration attributes.
pub type NumberOfLines = u8;

/// Octaves are represented by the numbers 0 to 9, where 4 indicates the octave started by middle C.
pub type Octave = u8;

/// The percent type specifies a percentage from 0 to 100.
pub type Percent = u8;

/// The positive-decimal type specifies a positive decimal value.
pub type PositiveDecimal = f64;

/// The positive-divisions type restricts divisions values to positive numbers.
pub type PositiveDivisions = Divisions;

/// The rotation-degrees type specifies rotation, pan, and elevation values in degrees. Values range from -180 to 180.
pub type RotationDegrees = i16;

/// The semitones type is a number representing semitones, used for chromatic alteration. A value of -1 corresponds to a flat and a value of 1 to a sharp. Decimal values like 0.5 (quarter tone sharp) are used for microtones.
pub type Semitones = f64;

/// The staff-line type indicates the line on a given staff. Staff lines are numbered from bottom to top, with 1 being the bottom line on a staff. Staff line values can be used to specify positions outside the staff, such as a C clef positioned in the middle of a grand staff.
pub type StaffLine = i64;

/// The staff-number type indicates staff numbers within a multi-staff part. Staves are numbered from top to bottom, with 1 being the top staff on a part.
pub type StaffNumber = u64;

/// The string-number type indicates a string number. Strings are numbered from high to low, with 1 being the highest pitched string.
pub type StringNumber = u64;

/// The tenths type is a number representing tenths of interline staff space (positive or negative). Both integer and decimal values are allowed, such as 5 for a half space and 2.5 for a quarter space. Interline space is measured from the middle of a staff line.  Distances in a MusicXML file are measured in tenths of staff space. Tenths are then scaled to millimeters within the scaling element, used in the defaults element at the start of a score. Individual staves can apply a scaling factor to adjust staff size. When a MusicXML element or attribute refers to tenths, it means the global tenths defined by the scaling element, not the local tenths as adjusted by the staff-size element.
pub type Tenths = f64;

/// The time-only type is used to indicate that a particular playback-related element only applies particular times through a repeated section. The value is a comma-separated list of positive integers arranged in ascending order, indicating which times through the repeated section that the element applies.
pub type TimeOnly = String;

/// The number of tremolo marks is represented by a number from 0 to 8: the same as beam-level with 0 added.
pub type TremoloMarks = u8;

/// The trill-beats type specifies the beats used in a trill-sound or bend-sound attribute group. It is a decimal value with a minimum value of 2.
pub type TrillBeats = f64;

/// Calendar dates are represented yyyy-mm-dd format, following ISO 8601. This is a W3C XML Schema date type, but without the optional timezone data.
pub type YYYYMMDD = String;

fn validate_color(color: &str) -> Result<(), ValidationError> {
	lazy_static! {
		static ref COLOR_RE: Regex = Regex::new(r"#[\dA-F]{6}([\dA-F][\dA-F])?").unwrap();
	}
	if COLOR_RE.is_match(color) {
		Ok(())
	} else {
		Err(ValidationError::new("Invalid color"))
	}
}

fn validate_comma_separated_text(text: &str) -> Result<(), ValidationError> {
	lazy_static! {
		static ref CST_RE: Regex = Regex::new(r"[^,]+(, ?[^,]+)*").unwrap();
	}
	if CST_RE.is_match(text) {
		Ok(())
	} else {
		Err(ValidationError::new("Invalid comma separated text"))
	}
}

fn validate_ending_number(text: &str) -> Result<(), ValidationError> {
	lazy_static! {
		static ref ENDN_RE: Regex = Regex::new(r"([ ]*)").unwrap();
	}
	if ENDN_RE.is_match(text) {
		Ok(())
	} else {
		Err(ValidationError::new("Invalid ending number"))
	}
}

fn validate_time_only(text: &str) -> Result<(), ValidationError> {
	lazy_static! {
		static ref TO_RE: Regex = Regex::new(r"[1-9][0-9]*(, ?[1-9][0-9]*)*").unwrap();
	}
	if TO_RE.is_match(text) {
		Ok(())
	} else {
		Err(ValidationError::new("Invalid time only"))
	}
}

fn validate_yyyy_mm_dd(text: &str) -> Result<(), ValidationError> {
	lazy_static! {
		static ref YMD_RE: Regex = Regex::new(r"[^:Z]*").unwrap();
	}
	if YMD_RE.is_match(text) {
		Ok(())
	} else {
		Err(ValidationError::new("Invalid time only"))
	}
}




/// The above-below type is used to indicate whether one element appears above or below another element.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AboveBelow {
	Above,
	Below
}

/// The accidental-value type represents notated accidentals supported by MusicXML. In the MusicXML 2.0 DTD this was a string with values that could be included. The XSD strengthens the data typing to an pub enumerated list. The quarter- and three-quarters- accidentals are Tartini-style quarter-tone accidentals. The -down and -up accidentals are quarter-tone accidentals that include arrows pointing down or up. The slash- accidentals are used in Turkish classical music. The numbered sharp and flat accidentals are superscripted versions of the accidental signs, used in Turkish folk music. The sori and koron accidentals are microtonal sharp and flat accidentals used in Iranian and Persian music.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum AccidentalValue {
	Sharp,
	Natural,
	Flat,
	DoubleSharp,
	SharpSharp,
	FlatFlat,
	NaturalSharp,
	NaturalFlat,
	QuarterFlat,
	ThreeQuartersFlat,
	SharpDown,
	SharpUp,
	NaturalDown,
	NaturalUp,
	FlatDown,
	FlatUp,
	TripleSharp,
	TripleFlat,
	SlashQuarterSharp,
	SlashSharp,
	SlashFlat,
	DoubleSlashFlat,
	Sharp1,
	Sharp2,
	Sharp3,
	Sharp5,
	Flat1,
	Flat2,
	Flat3,
	Flat4,
	Sori,
	Koron
}

/// The arrow-direction type represents the direction in which an arrow points, using Unicode arrow terminology.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ArrowDirection {
	Left,
	Up,
	Right,
	Down,
	Northwest,
	Northeast,
	Southeast,
	Southwest,
	#[serde(rename = "left right")]
	LeftRight,
	#[serde(rename = "up down")]
	UpDown,
	#[serde(rename = "northwest southeast")]
	NorthwestSoutheast,
	#[serde(rename = "northeast southwest")]
	NortheastSouthwest,
	Other
}

/// The arrow-style type represents the style of an arrow, using Unicode arrow terminology. Filled and hollow arrows indicate polygonal single arrows. Paired arrows are duplicate single arrows in the same direction. Combined arrows apply to double direction arrows like left right, indicating that an arrow in one direction should be combined with an arrow in the other direction.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ArrowStyle {
	Single,
	Double,
	Filled,
	Hollow,
	Paired,
	Combined,
	Other
}

/// The backward-forward type is used to specify repeat directions. The start of the repeat has a forward direction while the end of the repeat has a backward direction.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BackwardForward {
	Backward,
	Forward
}

/// The bar-style type represents barline style information. Choices are regular, dotted, dashed, heavy, light-light, light-heavy, heavy-light, heavy-heavy, tick (a short stroke through the top line), short (a partial barline between the 2nd and 4th lines), and none.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum BarStyle {
	Regular,
	Dotted,
	Dashed,
	Heavy,
	LightLight,
	LightHeavy,
	HeavyLight,
	HeavyHeavy,
	Tick,
	Short,
	None
}

/// The beater-value type represents pictograms for beaters, mallets, and sticks that do not have different materials represented in the pictogram. The finger and hammer values are in addition to Stone's list.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BeaterValue {
	Bow,
	#[serde(rename = "chime hammer")]
	ChimeHammer,
	Coin,
	Finger,
	Fingernail,
	Fist,
	#[serde(rename = "guiro scraper")]
	GuiroScraper,
	Hammer,
	Hand,
	#[serde(rename = "jazz stick")]
	JazzStick,
	#[serde(rename = "knitting needle")]
	KnittingNeedle,
	#[serde(rename = "metal hammer")]
	MetalHammer,
	#[serde(rename = "snare stick")]
	SnareStick,
	#[serde(rename = "spoon mallet")]
	SpoonMallet,
	#[serde(rename = "triangle beater")]
	TriangleBeater,
	#[serde(rename = "triangle beater plain")]
	TriangleBeaterPlain,
	#[serde(rename = "wire brush")]
	WireBrush,
}

/// The breath-mark-value type represents the symbol used for a breath mark.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BreathMarkValue {
	Comma,
	Tick
}

/// The cancel-location type is used to indicate where a key signature cancellation appears relative to a new key signature: to the left, to the right, or before the barline and to the left. It is left by default. For mid-measure key elements, a cancel-location of before-barline should be treated like a cancel-location of left.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum CancelLocation {
	Left,
	Right,
	BeforeBarline,
}

/// The circular-arrow type represents the direction in which a circular arrow points, using Unicode arrow terminology.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CircularArrow {
	Clockwise,
	Anticlockwise,
}

/// The clef-sign element represents the different clef symbols. The jianpu sign indicates that the music that follows should be in jianpu numbered notation, just as the TAB sign indicates that the music that follows should be in tablature notation. Unlike TAB, a jianpu sign does not correspond to a visual clef notation.
#[derive(Debug, Deserialize, PartialEq)]
pub enum ClefSign {
	G,
	F,
	C,
	#[serde(rename = "percussion")]
	Percussion,
	TAB,
	#[serde(rename = "jianpu")]
	Jianpu,
	#[serde(rename = "none")]
	None,
}

/// The css-font-size type includes the CSS font sizes used as an alternative to a numeric point size.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum CssFontSize {
	XxSmall,
	XSMall,
	Small,
	Medium,
	Large,
	XLarge,
	#[serde(rename = "xx-large")]
	XXLarge,
}

/// The degree-symbol-value type indicates indicates that a symbol should be used in specifying the degree.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum DegreeSymbolValue {
	Major,
	Minor,
	Augmented,
	Diminished,
	HalfDiminished,
}

/// The degree-type-value type indicates whether the current degree element is an addition, alteration, or subtraction to the kind of the current chord in the harmony element.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DegreeTypeValue {
	Add,
	Alter,
	Subtract,
}

/// The effect type represents pictograms for sound effect percussion instruments. The cannon value is in addition to Stone's list.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Effect {
	Anvil,
	#[serde(rename = "auto horn")]
	AutoHorn,
	#[serde(rename = "bird whistle")]
	BirdWhistle,
	Cannon,
	#[serde(rename = "duck call")]
	DuckCall,
	#[serde(rename = "gun shot")]
	GunShot,
	#[serde(rename = "klaxon horn")]
	KlaxonHorn,
	#[serde(rename = "lions roar")]
	LionsRoar,
	#[serde(rename = "police whistle")]
	PoliceWhistle,
	Siren,
	#[serde(rename = "slide whistle")]
	SlideWhistle,
	#[serde(rename = "thunder sheet")]
	ThunderSheet,
	#[serde(rename = "wind machine")]
	WindMachine,
	#[serde(rename = "wind whistle")]
	WindWhistle,
}

/// The enclosure-shape type describes the shape and presence / absence of an enclosure around text or symbols. A bracket enclosure is similar to a rectangle with the bottom line missing, as is common in jazz notation.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EnclosureShape {
	Rectangle,
	Square,
	Oval,
	Circle,
	Bracket,
	Triangle,
	Diamond,
	None,
}

/// The fan type represents the type of beam fanning present on a note, used to represent accelerandos and ritardandos.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Fan {
	Accel,
	Rit,
	None,
}

/// The fermata-shape type represents the shape of the fermata sign. The empty value is equivalent to the normal value.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FermataShape {
	Normal,
	Angled,
	Square,
}

/// The font-style type represents a simplified version of the CSS font-style property.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FontStyle {
	Normal,
	Italic,
}

/// The font-weight type represents a simplified version of the CSS font-weight property.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FontWeight {
	Normal,
	Bold,
}

/// The glass type represents pictograms for glass percussion instruments.
#[derive(Debug, Deserialize, PartialEq)]
pub enum Glass {
	#[serde(rename = "wind chimes")]
	WindChimes,
}

/// The group-barline-value type indicates if the group should have common barlines.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum GroupBarlineValue {
	Yes,
	No,
	#[serde(rename = "Mensurstrich")]
	Mensurstrich,
}

/// The group-symbol-value type indicates how the symbol for a group is indicated in the score. The default value is none.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum GroupSymbolValue {
	None,
	Brace,
	Line,
	Bracket,
	Square,
}

/// The handbell-value type represents the type of handbell technique being notated.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HandbellValue {
	Damp,
	Echo,
	Gyro,
	#[serde(rename = "hand martellato")]
	HandMartellato,
	#[serde(rename = "mallet lift")]
	MalletLift,
	#[serde(rename = "mallet table")]
	MalletTable,
	Martellato,
	#[serde(rename = "martellato lift")]
	MartellatoLift,
	#[serde(rename = "muted martellato")]
	MutedMartellato,
	#[serde(rename = "pluck lift")]
	PluckLift,
	Swing,
}

/// The harmony-type type differentiates different types of harmonies when alternate harmonies are possible. Explicit harmonies have all notes present in the music; implied have some notes missing but implied; alternate represents alternate analyses.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HarmonyType {
	Explicit,
	Implied,
	Alternate,
}

/// The hole-closed-location type indicates which portion of the hole is filled in when the corresponding hole-closed-value is half.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HoleClosedLocation {
	Right,
	Bottom,
	Left,
	Top,
}

/// The hole-closed-value type represents whether the hole is closed, open, or half-open.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HoleClosedValue {
	Yes,
	No,
	Half,
}

/// A kind-value indicates the type of chord. Degree elements can then add, subtract, or alter from these starting points. Values include:  Triads: major (major third, perfect fifth) minor (minor third, perfect fifth) augmented (major third, augmented fifth) diminished (minor third, diminished fifth) Sevenths: dominant (major triad, minor seventh) major-seventh (major triad, major seventh) minor-seventh (minor triad, minor seventh) diminished-seventh (diminished triad, diminished seventh) augmented-seventh (augmented triad, minor seventh) half-diminished (diminished triad, minor seventh) major-minor (minor triad, major seventh) Sixths: major-sixth (major triad, added sixth) minor-sixth (minor triad, added sixth) Ninths: dominant-ninth (dominant-seventh, major ninth) major-ninth (major-seventh, major ninth) minor-ninth (minor-seventh, major ninth) 11ths (usually as the basis for alteration): dominant-11th (dominant-ninth, perfect 11th) major-11th (major-ninth, perfect 11th) minor-11th (minor-ninth, perfect 11th) 13ths (usually as the basis for alteration): dominant-13th (dominant-11th, major 13th) major-13th (major-11th, major 13th) minor-13th (minor-11th, major 13th) Suspended: suspended-second (major second, perfect fifth) suspended-fourth (perfect fourth, perfect fifth) Functional sixths: Neapolitan Italian French German Other: pedal (pedal-point bass) power (perfect fifth) Tristan  The "other" kind is used when the harmony is entirely composed of add elements. The "none" kind is used to explicitly encode absence of chords or functional harmony.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum KindValue {
	Major,
	Minor,
	Augmented,
	Diminished,
	Dominant,
	MajorSeventh,
	MinorSeventh,
	DiminishedSeventh,
	AugmentedSeventh,
	HalfDiminished,
	MajorMinor,
	MajorSixth,
	MinorSixth,
	DominantRinth,
	MajorRinth,
	MinorRinth,
	#[serde(rename = "dominant-11th")]
	Dominant11th,
	#[serde(rename = "major-11th")]
	Major11th,
	#[serde(rename = "minor-11th")]
	Minor11th,
	#[serde(rename = "dominant-13th")]
	Dominant13th,
	#[serde(rename = "major-13th")]
	Major13th,
	#[serde(rename = "minor-13th")]
	Minor13th,
	SuspendedSecond,
	SuspendedFourth,
	#[serde(rename = "Neapolitan")]
	Neapolitan,
	#[serde(rename = "Italian")]
	Italian,
	#[serde(rename = "French")]
	French,
	#[serde(rename = "German")]
	German,
	Pedal,
	Power,
	#[serde(rename = "Tristan")]
	Tristan,
	Other,
	None,
}

/// The left-center-right type is used to define horizontal alignment and text justification.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LeftCenterRight {
	Left,
	Center,
	Right,
}

/// The left-right type is used to indicate whether one element appears to the left or the right of another element.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LeftRight {
	Left,
	Right,
}

/// The line-end type specifies if there is a jog up or down (or both), an arrow, or nothing at the start or end of a bracket.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LineEnd {
	Up,
	Down,
	Both,
	Arrow,
	None,
}

/// The line-shape type distinguishes between straight and curved lines.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LineShape {
	Straight,
	Curved,
}

/// The line-type type distinguishes between solid, dashed, dotted, and wavy lines.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LineType {
	Solid,
	Dashed,
	Dotted,
	Wavy,
}

/// The margin-type type specifies whether margins apply to even page, odd pages, or both.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MarginType {
	Odd,
	Even,
	Both,
}

/// The measure-numbering-value type describes how measure numbers are displayed on this part: no numbers, numbers every measure, or numbers every system.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MeasureNumbering {
	None,
	Measure,
	System,
}

/// The membrane type represents pictograms for membrane percussion instruments. The goblet drum value is in addition to Stone's list.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Membrane {
	#[serde(rename = "bass drum")]
	BassDrum,
	#[serde(rename = "bass drum on side")]
	BassDrumOnSide,
	#[serde(rename = "bongos")]
	Bongos,
	#[serde(rename = "conga drum")]
	CongaDrum,
	#[serde(rename = "goblet drum")]
	GobletDrum,
	#[serde(rename = "military drum")]
	MilitaryDrum,
	#[serde(rename = "snare drum")]
	SnareDrum,
	#[serde(rename = "bass drum snares off")]
	SnareDrumSnaresOff,
	#[serde(rename = "tambourine")]
	Tambourine,
	#[serde(rename = "tenor drum")]
	TenorDrum,
	Timbales,
	Tomtom,
}

/// The metal type represents pictograms for metal percussion instruments. The hi-hat value refers to a pictogram like Stone's high-hat cymbals but without the long vertical line at the bottom.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Metal {
	Almglocken,
	Bell,
	#[serde(rename = "bell plate")]
	BellPlate,
	#[serde(rename = "brake drum")]
	BrakeDrum,
	#[serde(rename = "Chinese cymbal")]
	ChineseCymbal,
	Cowbell,
	#[serde(rename = "crash cymbals")]
	CrashCymbals,
	Crotale,
	#[serde(rename = "cymbal tongs")]
	CymbalTongs,
	#[serde(rename = "doomed gong")]
	DoomedGong,
	#[serde(rename = "finger cymbals")]
	FingerCymbals,
	Flexatone,
	Gong,
	HiHat,
	HighHatCymbals,
	Handbell,
	Sistrum,
	#[serde(rename = "sizzle cymbal")]
	SizzleCymbal,
	#[serde(rename = "sleigh bells")]
	SleighBells,
	#[serde(rename = "suspended cymbal")]
	SuspendedCymbal,
	#[serde(rename = "tam tam")]
	TamTam,
	Triangle,
	#[serde(rename = "Vietnamese hat")]
	VietnameseHat,
}

/// The mute type represents muting for different instruments, including brass, winds, and strings. The on and off values are used for undifferentiated mutes. The remaining values represent specific mutes.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Mute {
	On,
	Off,
	Straight,
	Cup,
	HarmonNoStem,
	HarmonStem,
	Bucket,
	Plunger,
	Hat,
	Solotone,
	Practice,
	StopMute,
	StopHand,
	Echo,
	Palm,
}

/// The notehead type indicates shapes other than the open and closed ovals associated with note durations. The values do, re, mi, fa, fa up, so, la, and ti correspond to Aikin's 7-shape system. The fa up shape is typically used with upstems; the fa shape is typically used with downstems or no stems.  The arrow shapes differ from triangle and inverted triangle by being centered on the stem. Slashed and back slashed notes include both the normal notehead and a slash. The triangle shape has the tip of the triangle pointing up; the inverted triangle shape has the tip of the triangle pointing down. The left triangle shape is a right triangle with the hypotenuse facing up and to the left.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum NoteHeadValue {
	Slash,
	Triangle,
	Diamond,
	Square,
	Cross,
	X,
	CircleX,
	#[serde(rename = "inverted triangle")]
	InvertedTriangle,
	#[serde(rename = "arrow down")]
	ArrowDown,
	#[serde(rename = "arrow up")]
	ArrowUp,
	Slashed,
	#[serde(rename = "back slashed")]
	BackSlashed,
	Normal,
	Cluster,
	#[serde(rename = "circle dot")]
	CircleDot,
	#[serde(rename = "left triangle")]
	LeftTriangle,
	Rectangle,
	None,
	Do,
	Re,
	Mi,
	Fa,
	#[serde(rename = "fa up")]
	FaUp,
	So,
	La,
	Ti,
}

/// The note-size-type type indicates the type of note being defined by a note-size element. The grace type is used for notes of cue size that that include a grace element. The cue type is used for all other notes with cue size, whether defined explicitly or implicitly via a cue element. The large type is used for notes of large size.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NoteSizeType {
	Cue,
	Grace,
	Large,
}

/// The note-type type is used for the MusicXML type element and represents the graphic note type, from 1024th (shortest) to maxima (longest).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NoteTypeValue {
	#[serde(rename = "1024th")]
	_1024th,
	#[serde(rename = "512th")]
	_512th,
	#[serde(rename = "256th")]
	_256th,
	#[serde(rename = "128th")]
	_128th,
	#[serde(rename = "64th")]
	_64th,
	#[serde(rename = "32th")]
	_32th,
	#[serde(rename = "16th")]
	_16th,
	Eight,
	Quarter,
	Half,
	Whole,
	Breve,
	Long,
	Maxima,
}

/// The number-or-normal values can be either a decimal number or the string "normal". This is used by the line-height and letter-spacing attributes.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum NumberOrNormal {
	Number(f64),
	Normal(String),
}

/// The on-off type is used for notation elements such as string mutes.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OnOff {
	On,
	Off,
}

/// The over-under type is used to indicate whether the tips of curved lines such as slurs and ties are overhand (tips down) or underhand (tips up).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OverUnder {
	Over,
	Under,
}

/// The pitched type represents pictograms for pitched percussion instruments. The chimes and tubular chimes values distinguish the single-line and double-line versions of the pictogram. The mallet value is in addition to Stone's list.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Pitched {
	Chimes,
	Glockenspiel,
	Mallet,
	Marimba,
	#[serde(rename = "tubular chimes")]
	TubularChimes,
	Vibraphone,
	Xylophone,
}

/// The positive-integer-or-empty values can be either a positive integer or an empty string.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PositiveIntegerOrEmpty {
	Integer(u64),
	Empty(String),
}

/// The principal-voice-symbol type represents the type of symbol used to indicate the start of a principal or secondary voice. The "plain" value represents a plain square bracket. The value of "none" is used for analysis markup when the principal-voice element does not have a corresponding appearance in the score.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PrincipalVoceSymbol {
	#[serde(rename = "Hauptstimme")]
	Hauptstimme,
	#[serde(rename = "Nebenstimme")]
	Nebenstimme,
	Plain,
	None,
}

/// The right-left-middle type is used to specify barline location.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RightLeftMiddle {
	Right,
	Left,
	Middle,
}

/// The semi-pitched type represents categories of indefinite pitch for percussion instruments.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum SemiPitched {
	High,
	MediumHigh,
	Medium,
	MediumLow,
	Low,
	VeryLow,
}

/// The show-frets type indicates whether to show tablature frets as numbers (0, 1, 2) or letters (a, b, c). The default choice is numbers.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ShowFrets {
	Numbers,
	Letters,
}

/// The show-tuplet type indicates whether to show a part of a tuplet relating to the tuplet-actual element, both the tuplet-actual and tuplet-normal elements, or neither.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ShowTuplet {
	Actual,
	Both,
	None,
}

/// The staff-type value can be ossia, cue, editorial, regular, or alternate. An alternate staff indicates one that shares the same musical data as the prior staff, but displayed differently (e.g., treble and bass clef, standard notation and tab).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StaffType {
	Ossia,
	Cue,
	Editorial,
	Regular,
	Alternate,
}

/// The start-note type describes the starting note of trills and mordents for playback, relative to the current note.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StartNote {
	Upper,
	Main,
	Below,
}

/// The start-stop type is used for an attribute of musical elements that can either start or stop, such as tuplets.  The values of start and stop refer to how an element appears in musical score order, not in MusicXML document order. An element with a stop attribute may precede the corresponding element with a start attribute within a MusicXML document. This is particularly common in multi-staff music. For example, the stopping point for a tuplet may appear in staff 1 before the starting point for the tuplet appears in staff 2 later in the document.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StartStop {
	Start,
	Stop,
}

/// The start-stop-change-continue type is used to distinguish types of pedal directions.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StartStopChangeContinue {
	Start,
	Stop,
	Change,
	Continue,
}

/// The start-stop-continue type is used for an attribute of musical elements that can either start or stop, but also need to refer to an intermediate point in the symbol, as for complex slurs or for formatting of symbols across system breaks.  The values of start, stop, and continue refer to how an element appears in musical score order, not in MusicXML document order. An element with a stop attribute may precede the corresponding element with a start attribute within a MusicXML document. This is particularly common in multi-staff music. For example, the stopping point for a slur may appear in staff 1 before the starting point for the slur appears in staff 2 later in the document.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StartStopContinue {
	Start,
	Stop,
	Continue,
}

/// The start-stop-discontinue type is used to specify ending types. Typically, the start type is associated with the left barline of the first measure in an ending. The stop and discontinue types are associated with the right barline of the last measure in an ending. Stop is used when the ending mark concludes with a downward jog, as is typical for first endings. Discontinue is used when there is no downward jog, as is typical for second endings that do not conclude a piece.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StartStopDiscontinue {
	Start,
	Stop,
	Discontinue,
}

/// The start-stop-single type is used for an attribute of musical elements that can be used for either multi-note or single-note musical elements, as for tremolos.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StartStopSingle {
	Start,
	Stop,
	Single,
}

/// The stem type represents the notated stem direction.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StemValue {
	Down,
	Up,
	Double,
	None,
}

/// The step type represents a step of the diatonic scale, represented using the English letters A through G.
#[derive(Debug, Deserialize, PartialEq)]
pub enum Step {
	A,
	B,
	C,
	D,
	E,
	F,
	G,
}

/// The stick-location type represents pictograms for the location of sticks, beaters, or mallets on cymbals, gongs, drums, and other instruments.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StickLocation {
	Center,
	Rim,
	#[serde(rename = "cymbal bell")]
	CymballBell,
	#[serde(rename = "cymbal edge")]
	CymbalEdge,
}

/// The stick-material type represents the material being displayed in a stick pictogram.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StickMaterial {
	Soft,
	Medium,
	Hard,
	Shaded,
	X,
}

/// The stick-type type represents the shape of pictograms where the material in the stick, mallet, or beater is represented in the pictogram.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StickType {
	#[serde(rename = "bass drum")]
	BassDrum,
	#[serde(rename = "double bass drum")]
	DoubleBassDrum,
	Timpani,
	Xylophone,
	Yarn,
}

/// Lyric hyphenation is indicated by the syllabic type. The single, begin, end, and middle values represent single-syllable words, word-beginning syllables, word-ending syllables, and mid-word syllables, respectively.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Syllabic {
	Single,
	Begin,
	End,
	Middle,
}

/// The symbol-size type is used to indicate full vs. cue-sized vs. oversized symbols. The large value for oversized symbols was added in version 1.1.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SymbolSize {
	Full,
	Cue,
	Large,
}

/// The text-direction type is used to adjust and override the Unicode bidirectional text algorithm, similar to the W3C Internationalization Tag Set recommendation. Values are ltr (left-to-right embed), rtl (right-to-left embed), lro (left-to-right bidi-override), and rlo (right-to-left bidi-override). The default value is ltr. This type is typically used by applications that store text in left-to-right visual order rather than logical order. Such applications can use the lro value to better communicate with other applications that more fully support bidirectional text.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TextDirection {
	Ltr,
	Rtl,
	Lro,
	Rlo,
}

/// The time-relation type indicates the symbol used to represent the interchangeable aspect of dual time signatures.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TimeRelation {
	Parentheses,
	Bracket,
	Equals,
	Slash,
	Space,
	Hyphen,
}

/// The time-separator type indicates how to display the arrangement between the beats and beat-type values in a time signature. The default value is none. The horizontal, diagonal, and vertical values represent horizontal, diagonal lower-left to upper-right, and vertical lines respectively. For these values, the beats and beat-type values are arranged on either side of the separator line. The none value represents no separator with the beats and beat-type arranged vertically. The adjacent value represents no separator with the beats and beat-type arranged horizontally.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TimeSeparator {
	None,
	Horizontal,
	Diagonal,
	Vertical,
	Adjacent,
}

/// The time-symbol type indicates how to display a time signature. The normal value is the usual fractional display, and is the implied symbol type if none is specified. Other options are the common and cut time symbols, as well as a single number with an implied denominator. The note symbol indicates that the beat-type should be represented with the corresponding downstem note rather than a number. The dotted-note symbol indicates that the beat-type should be represented with a dotted downstem note that corresponds to three times the beat-type value, and a numerator that is one third the beats value.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum TimeSymbol {
	Common,
	Cut,
	SingleNumber,
	Note,
	DottedNote,
	Normal,
}

/// The tip-direction type represents the direction in which the tip of a stick or beater points, using Unicode arrow terminology.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TipDirection {
	Up,
	Down,
	Left,
	Right,
	Northwest,
	Northeast,
	Southeast,
	Southwest,
}

/// The top-bottom type is used to indicate the top or bottom part of a vertical shape like non-arpeggiate.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TopBottom {
	Top,
	Bottom,
}

/// The trill-step type describes the alternating note of trills and mordents for playback, relative to the current note.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TrillStep {
	Whole,
	Half,
	Unison,
}

/// The two-note-turn type describes the ending notes of trills and mordents for playback, relative to the current note.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TwoNoteTurn {
	Whole,
	Half,
	None,
}

/// The up-down type is used for the direction of arrows and other pointed symbols like vertical accents, indicating which way the tip is pointing.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UpDown {
	Up,
	Down,
}

/// The up-down-stop-continue type is used for octave-shift elements, indicating the direction of the shift from their true pitched values because of printing difficulty.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UpDownStopContinue {
	Up,
	Down,
	Stop,
	Continue,
}

/// The upright-inverted type describes the appearance of a fermata element. The value is upright if not specified.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UprightInverted {
	Upright,
	Inverted,
}

/// The valign type is used to indicate vertical alignment to the top, middle, bottom, or baseline of the text. Defaults are implementation-dependent.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Valign {
	Top,
	Middle,
	Bottom,
	Baseline,
}

/// The valign-image type is used to indicate vertical alignment for images and graphics, so it does not include a baseline value. Defaults are implementation-dependent.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ValignImage {
	Top,
	Middle,
	Bottom,
}

/// The wedge type is crescendo for the start of a wedge that is closed at the left side, diminuendo for the start of a wedge that is closed on the right side, and stop for the end of a wedge. The continue type is used for formatting wedges over a system break, or for other situations where a single wedge is divided into multiple segments.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum WedgeType {
	Crescendo,
	Diminuendo,
	Stop,
	Continue,
}

/// The winged attribute indicates whether the repeat has winged extensions that appear above and below the barline. The straight and curved values represent single wings, while the double-straight and double-curved values represent double wings. The none value indicates no wings and is the default.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Winged {
	None,
	Straight,
	Curved,
	DoubleStraight,
	DoubleCurved,
}

/// The wood type represents pictograms for wood percussion instruments. The maraca and maracas values distinguish the one- and two-maraca versions of the pictogram. The vibraslap and castanets values are in addition to Stone's list.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Wood {
	#[serde(rename = "board clapper")]
	BoardClapper,
	Cabasa,
	Castanets,
	Claves,
	Guiro,
	#[serde(rename = "log drum")]
	LogDrum,
	Maraca,
	Maracas,
	Ratchet,
	#[serde(rename = "sandpaper blocks")]
	SandpaperBlocks,
	#[serde(rename = "slit drum")]
	SlitDrum,
	#[serde(rename = "temple block")]
	TempleBlock,
	Vibraslap,
	#[serde(rename = "wood block")]
	WoodBlock,
}

/// The yes-no type is used for boolean-like attributes. We cannot use W3C XML Schema booleans due to their restrictions on expression of boolean values.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum YesNo {
	Yes,
	No,
}

/// The yes-no-number type is used for attributes that can be either boolean or numeric values.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum YesNoNumber {
	Boolean(YesNo),
	Decimal(f64),
}

