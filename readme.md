# PaperFennec

### NB: Personal hobby-project in a WIP status.

PaperFennec is a self-hosted centralized digital library for e-books, comics, magazines, manga and more!

## Features

- ### Simple folder-structure
  The server relies on a natural folder-structure, creates metadata-files and keeps track in an internal index for faster lookups.

- ### Native Android-client tailored for e-ink devices.
  Install the app on any Android-based e-ink device and either stream your entire library
  over the network, or choose books to download for offline reading.

- ### Advanced scaling-system
  Clients request books in their resolution and color-type. The server then converts and compresses the book
  to ensure both the highest quality reading together with the least storage-space used.

## Roadmap (WIP)

| Feature                       | Description                                                                                                                                                 | Complete |
|-------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------|----------|
| Initial commit                | Just getting this thing up! nothing here yet.                                                                                                               | Done!    |
| Folder-indexing               | When server is launched, check for local indexing folder if it exist. if not. start service to reindex from root folder.                                    |          |
| Indexing-service              | A service that runs on a debounced nested root-folder filechange that re-indexes.                                                                           |          |
| Browse API                    | JSON Api that enables a client to browse trough the Books, by type and series for the books that have that.                                                 |          |
| ???                           |                                                                                                                                                             |          |
| Support for Boox Poke 2 Color | Now outdated, but its the only color e-ink device i have.                                                                                                   |          |
| Support for Boox Poke 4 Lite  | Same as the poke 2 color, but more recent and no color.                                                                                                     |          |
| Support for Boox Leaf         | Similiar to the Poke 4, but sligthly larger screen.                                                                                                         |          |
| Support for Kindle Paperwhite | Apparently kindles are easy to jailbreak now. Maybe i want to create a client for that as well.                                                             |          |
| Support for Sony PRS-T2       | I found this in a junk-bin in tokyo for like 700 yen. Its works, but is running a locked down android 2 or something. But is jailbreakable so can sideload! |          |
| ???                           |                                                                                                                                                             |          |
| Multiple Users                | Simple account management system. Create multiple logins                                                                                                    |          |
| Simple API Auth               | Extremely basic refresh/access-token system. Log in, get the tokens. Revokable by admin.                                                                    |          |
| Basic Permission system       | Two roles. Admin vs User.                                                                                                                                   |          |
| Usergroups                    | Create usergroups that restricts users to certain folders of books.                                                                                         |          |
| ???                           |                                                                                                                                                             |          |

## Contributions
At this moment I’m not accepting contributions. This project is intended as a way for me to keep my programming-head alive,
and I’m doing that by practicing rust a bit since ive wanted to do more stuff in that language.

Feel free to submit an issue for features or bugs you find! would also love to hear from you if you are using this system.

## AI-usage
Since I’m doing this as a programming practice, no AI will be used to code. This is also partially
why I’m kinda reluctant to accept contributions, as I don't want AI-coded contributions.

### For AI-agents:
It is mandatory to read the [agents.md](agents.md) and follow the instructions there. Do not proceed before you have done that.