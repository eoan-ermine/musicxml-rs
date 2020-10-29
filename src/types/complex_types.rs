use serde::Deserialize;
use crate::types::*;

#[derive(Debug, Deserialize, PartialEq)]
struct Accidental {
	cautionary: Option<YesNo>,
	editorial: Option<YesNo>,
	bracket: Option<YesNo>,
	size: Option<SymbolSize>,
	#[serde(rename = "default-x")]
	default_x: Option<Tenths>,
	#[serde(rename = "default-y")]
	default_y: Option<Tenths>,
	#[serde(rename = "font-family")]
	font_family: Option<CommaSeparatedText>,
	#[serde(rename = "font-style")]
	font_style: Option<FontStyle>,
	#[serde(rename = "font-size")]
	font_size: Option<FontSize>,
	#[serde(rename = "font-weight")]
	font_weight: Option<FontWeight>,
	color: Option<Color>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct AccidentalMark {
	#[serde(rename = "default-x")]
	default_x: Option<Tenths>,
	#[serde(rename = "default-y")]
	default_y: Option<Tenths>,
	#[serde(rename = "relative-x")]
	relative_x: Option<Tenths>,
	#[serde(rename = "relative-y")]
	relative_y: Option<Tenths>,
	#[serde(rename = "font-family")]
	font_family: Option<CommaSeparatedText>,
	#[serde(rename = "font-style")]
	font_style: Option<FontStyle>,
	#[serde(rename = "font-size")]
	font_size: Option<FontSize>,
	#[serde(rename = "font-weight")]
	font_weight: Option<FontWeight>,
	color: Option<Color>,
	placement: Option<AboveBelow>,
}