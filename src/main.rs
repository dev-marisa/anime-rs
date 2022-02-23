use rand::Rng;

#[derive(Debug)]
pub struct Anime {
    title: String,
    studio: String,
    year: i16,
}

fn main() {
    let mut rng = rand::thread_rng();
    let anime_list: Vec<Anime> = vec![
        Anime {
            title: "FLCL".to_string(),
            studio: "Gainax".to_string(),
            year: 2000,
        },
        Anime {
            title: "Madoka★Magica".to_string(),
            studio: "Shaft".to_string(),
            year: 2011,
        },
        Anime {
            title: "My Dress-Up Darling".to_string(),
            studio: "CloverWorks".to_string(),
            year: 2022,
        },
        Anime {
            title: "Girls' Last Tour".to_string(),
            studio: "WhiteFox".to_string(),
            year: 2017,
        },
        Anime {
            title: "Kobayashi-san Chi no Maid Dragon".to_string(),
            studio: "Kyoto Animation".to_string(),
            year: 2017,
        },
    ];
    let random_index = rng.gen_range(0..anime_list.len());
    let Anime { title, year, .. } = &anime_list[random_index];
    println!("Your pick is {} ({})", title, year);
}
