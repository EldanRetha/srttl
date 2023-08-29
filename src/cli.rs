use clap::{Parser, ValueEnum};
use std::{path::PathBuf};
use deepl::Lang;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum InLanguage {
    en,
    jp
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutLanguage {
    en_us,
    en_gb,
    jp
}

#[derive(Parser, Debug)]
#[command(name = "srttl")]
#[command(author = "Eldan")]
#[command(version = "0.1")]
#[command(about = "Translates SRT files using DeepL", long_about = None)]
pub struct Args {
    //#[arg(short = 'l', long)]
    pub input_lang: InLanguage,
    pub output_lang: OutLanguage,
    #[arg(short = 'f', long)]
    pub input_file: PathBuf, 
    #[arg(short = 'F', long)]
    pub output_file: PathBuf,
    #[arg(short = 'k', long)]
    pub api_key: String,
}
