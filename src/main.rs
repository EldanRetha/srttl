use srtlib::Subtitles;
use deepl::{DeepLApi, Lang};
use cli::{Args};
use clap::{Parser, ValueEnum};
use tokio::{main};

mod cli;


#[tokio::main]
async fn main() {
    //////////////////////////////////////////////////////////////////////////
    // Read in subtitles and languages
    //////////////////////////////////////////////////////////////////////////
    let args = Args::parse();
    let mut subs = Subtitles::parse_from_file(args.input_file, None).unwrap();

    //////////////////////////////////////////////////////////////////////////
    // Convert Subtitles
    //////////////////////////////////////////////////////////////////////////
    for sub in &mut subs {
        let api = DeepLApi::with(&args.api_key).new();
        let response = api
            .translate_text(sub.text.clone(), Lang::EN_US)
            .source_lang(Lang::JA)
            .await
            .unwrap();
        sub.text = String::new();
        for sentence in response.translations {
            sub.text.push_str(sentence.text.as_str())
        }
        println!("{}", sub.text);
    }

    //////////////////////////////////////////////////////////////////////////
    // Write out subtitles
    //////////////////////////////////////////////////////////////////////////
    subs.write_to_file(args.output_file, None).unwrap();
    println!("Done!");
}
