#![cfg_attr(not(test), windows_subsystem = "windows")]
#![cfg_attr(test, windows_subsystem = "console")]

#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate serializable_derive;

use anyhow::Result;
use mass_effect_3::Me3SaveGame;
use serializer::{SaveCursor, Serializable};
use std::{
    panic::{self, PanicInfo},
    time::Instant,
};
use tokio::{fs::File, io::AsyncReadExt};

mod mass_effect_3;
mod serializer;

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    console::attach();

    panic::set_hook(Box::new(|e| {
        console::attach();
        panic_hook(e);
    }));

    // Code start here
    let mut input = Vec::new();
    {
        let mut file = File::open("test/NewGamePlusSave.pcsav").await?;
        file.read_to_end(&mut input).await?;
    }

    let mut input = SaveCursor::new(input);

    let now = Instant::now();
    let me3_save_game = Me3SaveGame::deserialize(&mut input)?;
    let elapsed = now.elapsed().as_secs_f32();
    println!("{:#?}", me3_save_game);
    println!("Parse: {}s", elapsed);

    // let save_game = Me3SaveGame {
    //     version: 59,
    //     debug_name: String::from("Cécé 💖 lolz"),
    //     seconds_played: 199777.55,
    //     disc: 0,
    //     base_level_name: String::from("Biop_End002"),
    // };

    // let output = bincode_options.serialize(&save_game)?;

    // {
    //     let mut file = File::create("test/serialized.pcsav").await?;
    //     file.write_all(&output).await?;
    // }

    Ok(())
}

fn panic_hook(info: &PanicInfo<'_>) {
    let msg = match info.payload().downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match info.payload().downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "???",
        },
    };
    let location = info.location().unwrap();

    eprintln!("Panic : '{}', {}", msg, location);
}

mod console {
    use bindings::Windows::Win32::SystemServices::{AllocConsole, AttachConsole};

    #[allow(clippy::missing_safety_doc)]
    pub fn attach() {
        unsafe {
            // u32::MAX = DWORD(-1) soit ATTACH_PARENT_PROCESS
            if !AttachConsole(u32::MAX).as_bool() {
                AllocConsole();
            }
        }
    }
}
