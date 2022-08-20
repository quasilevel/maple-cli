# Maple CLI
(Not just) A CLI front-end to the Maple Rest and potentially Maple Leaf. It aims to create a fast and non-obstructive experience of using Maple. It also taps into libplayerctl to control youtube/mpris applications to play songs on youtube.

## Command List
- `maple add`
  Add music locally along with tags and description.
- `maple play {title | tags} <title | list of tags>`
  Play the music through a mpris player that can play songs using the urls based on the selection.
  1. `title`: Play the song with given title on the best match basis.
  2. `tags`: Generate a playlist based on the list of tags provided and play it.
- `maple find <search terms>`
  Find a music entry and perform actions on it
  1. `play`: Plays the song.
  2. `edit`: Edits the song entry.
  3. `delete`: Deletes the song entry.
- `maple sync`
  Syncs up the local records and the Maple Rest/Maple Leaf records.
