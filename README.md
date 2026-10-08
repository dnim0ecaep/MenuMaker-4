# MenuMaker-4.0 
This version 4, is written in Rust and runs much faster than the previos python versions.


Menu Maker is a cross-platform terminal-based application launcher designed for efficiency, simplicity, and full customization. It allows users to organize commands and scripts into expandable categories, offering a clean and intuitive interface to execute system tools, development utilities, or any shell commands directly from the terminal.<img width="1029" height="332" alt="Screenshot 2026-01-07 at 11 33 18 AM" src="https://github.com/user-attachments/assets/7be3f11c-8bcf-4665-a685-b6c2b563ecc2](https://github.com/dnim0ecaep/MenuMaker-4/blob/main/The_App.png](https://github.com/dnim0ecaep/MenuMaker-4/blob/main/The_App.png" />


Core Features:

Categorized menu system with expandable/collapsible sections.

Fully mouse and keyboard-navigable interface (arrow keys, enter, space, Q to quit).

Custom command execution with return-to-menu workflow.

Live status bar displaying navigation position.

Lightweight JSON-based configuration for full portability and easy editing.

Custom theming and color support for improved visual clarity.

Desktop Boxes (DeskMate style):

Everything on the screen lives in a box with a double-line border and a centered ◄ TITLE ► band, modeled on Tandy DeskMate 3.05. Box colors follow the active theme (accent = border and title band, surface = box fill), and each box can still have its own colors: in the box editor's Custom Theme section set Background, Text, and Border (the line around the box and its title band). The box Color Theme list also includes DeskMate panel themes: DeskMate Navy (file boxes), DeskMate Touch Me (red panel), DeskMate Phone List (gray), and DeskMate Button (blue), each with its matching border. Press `a` to add a box, `e` on a box title to edit it, and `d` on a box title to delete it. There are several kinds of boxes (an Address Book and a Calendar box are added for you once):

| Box | What it shows | Enter / click |
|---|---|---|
| Menu | Your own list of commands (`n` adds an item to the selected menu box) | Runs the command |
| Folder | The files in a folder, optionally filtered (e.g. `*.md, *.txt`) | Opens the file with the box's app, e.g. `nano` → `nano '<file>'`. Put `{}` in the app to place the file yourself, e.g. `vim -R {}` |
| Apps folder | The executable programs in a folder | Runs the program |
| Shortcut | Its name in the title band with an icon picture or the command inside, or (with "Show only the name inside the box") just the name centered like a DeskMate button. The icon can be a picture you drew in Paint (in the box editor, on Icon picture press `e` to choose a drawing, `g` to choose an enlarged icon glyph from the installed Nerd Font, Del to clear) | Runs its command |
| Notes | Your text, word-wrapped in the box | Opens the built-in notes editor (Esc saves, Ctrl+X discards) |
| Address Book | Your contact names | Opens the Address Book: contact list plus Full Name, Birthday, Emails, Phone Numbers, Addresses, Websites and Notes boxes. `a` add, Enter edit, `d` delete, `s` sort, `i` import / `x` export .vcf (vCard). Stored in `~/.local/menu-maker/contacts.vcf` |
| Paint | The drawings in your paintings folder (`~/.local/menu-maker/paintings`) | Enter on a drawing opens it in Paint; Enter on the title starts a new one. Paint has tools, colors, undo/redo, flip/rotate/resize and a DeskMate-style menu bar: F2 File, F3 Edit, F4 View, F5 Image, F6 Colors, F1 Help. Saves .ans/.txt/.html/.svg/.irc, opens .ans/.txt, imports .png/.jpg. A Folder box whose app is `@paint` also opens its files in Paint |
| Music | The MIDI songs (.mid .midi .oxm) in your music folder (`~/.local/menu-maker/music`) | Enter on a song opens it in the built-in music app (a port of [miditui](https://github.com/minimaxir/miditui)); Enter on the title starts a new song. Piano roll, project timeline, tracks (mute/solo/volume/pan/instrument), Insert mode to play notes on the QWERTY keyboard, undo/redo, Space play/pause, `.` stop, Ctrl+S save as MIDI/JSON/OXM, `e` export WAV, `?` help, and a DeskMate menu bar (F2 File, F3 Edit, F4 View, F5 Track, F6 Play, F1 Help). Uses a SoundFont found in `/usr/share/sounds/sf2` (or pick one with Ctrl+L). A Folder box whose app is `@midi` also opens its files here |
| Calendar | This month with today highlighted, plus today's appointments | Opens the Calendar: month grid, the day's appointments and a high/low priority To-Do list. Arrows move days, PgUp/PgDn months, `[` `]` years, `t` today (F1–F6 also work); `a` add, Enter edit, `d` delete, `p` toggle priority. F9 opens the command line: `app, 2026-10-14 16:30:00, Title, Place`, `todo, true, Title`, `find, 2026-10-14`, `today`. Stored in `~/.local/menu-maker/calendar.json` |

For the full DeskMate look, press `s` and pick the **DeskMate** theme preset: teal desktop, navy boxes with yellow double borders and text, a yellow selection bar, a maroon title band on the selected box, and the gray command bar with red keys.

Space or clicking a box's title collapses/expands it. Drag a box by its title (the top line) to another column to move it; turn this off with "Drag boxes with the mouse" in Settings (`s`). Set "Max rows" on a box to keep long folders short; the list scrolls with the selection, with ▲/▼ markers on the border. Folder listings are re-read every time the screen is rebuilt, and with `r` (reload).

Menu Maker is ideal for system administrators, developers, and power users who want a fast, highly stable, keyboard-driven way to launch and manage their most-used commands in one consistent interface.
