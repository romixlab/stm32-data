use std::env::args;
use std::path::PathBuf;

use stm32_metapac_gen::*;

fn main() {
    let out_dir = PathBuf::from("build/stm32-metapac");
    let data_dir = PathBuf::from("build/data");

    let args: Vec<String> = args().collect();

    let mut chips = match &args[..] {
        [_, chip] => {
            vec![chip.clone()]
        }
        [_] => std::fs::read_dir(data_dir.join("chips"))
            .unwrap()
            .filter_map(|res| res.unwrap().file_name().to_str().map(|s| s.to_string()))
            .filter(|s| s.ends_with(".json"))
            .map(|s| s.strip_suffix(".json").unwrap().to_string())
            .collect(),
        _ => panic!("usage: stm32-metapac-gen [chip?]"),
    };

    chips.sort();

    let opts = Options {
        out_dir,
        data_dir,
        chips,
    };
    Gen::new(
        opts,
        "STM32H".into(),
        vec![
            ("can".into(), "fdcan_h7".into()),
            ("can".into(), "fdcan_v1".into()),
            ("fdcanram".into(), "v1".into()),
            ("fdcanram".into(), "h7".into()),
            ("rcc".into(), "h7".into()),
            ("rcc".into(), "g4".into()),
            ("rcc".into(), "g0".into()),
        ],
    )
    .gen();
}
