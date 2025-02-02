use common::spotify::CurrentSong;

pub enum SaveResult {
    Unchanged,
    Updated,
}

pub trait CurrentSongDao {
    fn get_current_song(&self) -> Option<&CurrentSong>;
    fn save_current_song(&mut self, song: CurrentSong) -> Result<SaveResult, anyhow::Error>;
}

#[derive(Debug, Default)]
pub struct CurrentSongDaoImpl {
    current_song: Option<CurrentSong>,
}

impl CurrentSongDaoImpl {
    pub fn new() -> Self {
        Self { current_song: None }
    }
}

impl CurrentSongDao for CurrentSongDaoImpl {
    fn get_current_song(&self) -> Option<&CurrentSong> {
        self.current_song.as_ref()
    }

    fn save_current_song(&mut self, song: CurrentSong) -> Result<SaveResult, anyhow::Error> {
        match &mut self.current_song {
            Some(current_song) => {
                if *current_song == song {
                    Ok(SaveResult::Unchanged)
                } else {
                    self.current_song = Some(song);
                    Ok(SaveResult::Updated)
                }
            }
            None => {
                self.current_song = Some(song);
                Ok(SaveResult::Updated)
            }
        }
    }
}
