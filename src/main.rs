use feedback_streaming_app::MusicPlayer;

fn main() {
    println!("Hello, world!");
    let mut player = MusicPlayer::new().unwrap();

    match player.song_list() {
        Ok(songs) => {
            println!("Available song {}", songs.len());
            songs.iter().for_each(|s| println!(" {}", s.display()));
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    
    if let Some(path) = player.find_song("something goes here") {
        let path = path.clone();
        match player.play_song(&path) {
            Ok(_) => {}
            Err(e) => eprintln!("Error: {}", e),
        }
    } else {
        println!("Song not found");
    }

    player.simulate_network(false);

    match player.song_list() {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e)
    }

    player.simulate_network(true);
}
