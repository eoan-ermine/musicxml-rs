mod simple_types;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Mode {
	Major,
	Minor
}

#[derive(Debug, Deserialize, PartialEq)]
struct Key {
	fifths: i32,
	mode: String
}

#[derive(Debug, Deserialize, PartialEq)]
struct Time {
	beats: i32,
	#[serde(rename = "beat-type", default)]
	beat_type: i32,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Attributes {
	divisions: i32,
	key: Key,
	time: Time,
}