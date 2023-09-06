#![allow(non_camel_case_types)]

use clap::{Parser, ValueEnum};
use std::{path::PathBuf};
use deepl::Lang;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum InLanguage {
    bg,
    cs,
    da,
    de,
    el,
    en,
    es,
    et,
    fi,
    fr,
    hu,
    id,
    it,
    ja,
    lt,
    lv,
    nl,
    pl,
    pt,
    ro,
    ru,
    sk,
    sl,
    sv,
    tr,
    uk,
    zh
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutLanguage {
    bg,
    cs,
    da,
    de,
    el,
    en_gb,
    en_us,
    es,
    et,
    fi,
    fr,
    hu,
    id,
    it,
    ja,
    lt,
    lv,
    nl,
    pl,
    pt_br,
    pt_pt,
    ro,
    ru,
    sk,
    sl,
    sv,
    tr,
    uk,
    zh
}

impl InLanguage {
    pub fn to_lang(self) -> Lang {
        match self {
            InLanguage::bg =>  Lang::BG,
            InLanguage::cs =>  Lang::CS,
            InLanguage::da =>  Lang::DA,
            InLanguage::de =>  Lang::DE,
            InLanguage::el =>  Lang::EL,
            InLanguage::en =>  Lang::EN,
            InLanguage::es =>  Lang::ES,
            InLanguage::et =>  Lang::ET,
            InLanguage::fi =>  Lang::FI,
            InLanguage::fr =>  Lang::FR,
            InLanguage::hu =>  Lang::HU,
            InLanguage::id =>  Lang::ID,
            InLanguage::it =>  Lang::IT,
            InLanguage::ja =>  Lang::JA,
            InLanguage::lt =>  Lang::LT,
            InLanguage::lv =>  Lang::LV,
            InLanguage::nl =>  Lang::NL,
            InLanguage::pl =>  Lang::PL,
            InLanguage::pt =>  Lang::PT,
            InLanguage::ro =>  Lang::RO,
            InLanguage::ru =>  Lang::RU,
            InLanguage::sk =>  Lang::SK,
            InLanguage::sl =>  Lang::SL,
            InLanguage::sv =>  Lang::SV,
            InLanguage::tr =>  Lang::TR,
            InLanguage::uk =>  Lang::UK,
            InLanguage::zh =>  Lang::ZH
        }
    }
}

impl OutLanguage {
    pub fn to_lang(self) -> Lang {
        match self {
            OutLanguage::bg    =>  Lang::BG,
            OutLanguage::cs    =>  Lang::CS,
            OutLanguage::da    =>  Lang::DA,
            OutLanguage::de    =>  Lang::DE,
            OutLanguage::el    =>  Lang::EL,
            OutLanguage::en_gb =>  Lang::EN_GB,
            OutLanguage::en_us =>  Lang::EN_US,
            OutLanguage::es    =>  Lang::ES,
            OutLanguage::et    =>  Lang::ET,
            OutLanguage::fi    =>  Lang::FI,
            OutLanguage::fr    =>  Lang::FR,
            OutLanguage::hu    =>  Lang::HU,
            OutLanguage::id    =>  Lang::ID,
            OutLanguage::it    =>  Lang::IT,
            OutLanguage::ja    =>  Lang::JA,
            OutLanguage::lt    =>  Lang::LT,
            OutLanguage::lv    =>  Lang::LV,
            OutLanguage::nl    =>  Lang::NL,
            OutLanguage::pl    =>  Lang::PL,
            OutLanguage::pt_br =>  Lang::PT_BR,
            OutLanguage::pt_pt =>  Lang::PT_PT,
            OutLanguage::ro    =>  Lang::RO,
            OutLanguage::ru    =>  Lang::RU,
            OutLanguage::sk    =>  Lang::SK,
            OutLanguage::sl    =>  Lang::SL,
            OutLanguage::sv    =>  Lang::SV,
            OutLanguage::tr    =>  Lang::TR,
            OutLanguage::uk    =>  Lang::UK,
            OutLanguage::zh    =>  Lang::ZH
        }
    }
}


#[derive(Parser, Debug)]
#[command(name = "srttl")]
#[command(author = "Eldan")]
#[command(version = "0.1")]
#[command(about = "Translates SRT files using DeepL", long_about = None)]
pub struct Args {
    #[arg(long)]
    pub input_lang: InLanguage,
    #[arg(long)]
    pub output_lang: OutLanguage,
    #[arg(short = 'f', long)]
    pub input_file: PathBuf, 
    #[arg(short = 'F', long)]
    pub output_file: PathBuf,
    #[arg(short = 'k', long)]
    pub api_key: String,
}
