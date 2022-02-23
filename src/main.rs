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
    ];
    let Anime { title, year, .. } = &anime_list[rng.gen_range(0..anime_list.len())];
    println!("Your pick is {} ({})", title, year);
}
