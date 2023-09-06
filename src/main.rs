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
        let text = sub.text.clone().replace("\n","");
        let response = api
            .translate_text(text, args.output_lang.to_lang())
            .source_lang(args.input_lang.to_lang())
            .await
            .unwrap();
        println!("Original Text: {}", sub.text);
        sub.text = String::new();
        let i = 0;
        for sentence in response.translations {
            println!("Sentence {}: {}", i, sentence.text,);
            sub.text.push_str(sentence.text.as_str())
        }
    }

    //////////////////////////////////////////////////////////////////////////
    // Write out subtitles
    //////////////////////////////////////////////////////////////////////////
    subs.write_to_file(args.output_file, None).unwrap();
    println!("Done!");
}
