# Word Count Modifier UI

A Graphical user interface for Word count modifier, a script used to reduce to displayed work count of a piece of text in programs such as Microsoft Word.

## Tested programs

### U+205F - Default Reduction Mode

Word - Windows: ✅

Word - MacOS: ✅ (limited testing)

Word - Web: ❌ (I don't think the characters paste. However it should work when opening an existing file)

Onlyoffice Desktop Editors: ❌ (Shows up with red underline but does affect the word count correctly)

Turnitin: ✅ (Works fine from the limited testing I'm able to do)

[Qlearn/Canvas Based Sites](https://www.instructure.com/canvas): ✅

LibreOffice - Linux: ❌

### U+3164 - Increasing Word Count

Word - Windows: ✅

Word - MacOS: (Untested)

Word - Web: ✅ (It works but word online seems to remove the charecters if you paste directly)

OnlyOffice Desktop Editors: ❌/✅ (Does not count any form of increase on V2 however other modes seem to still work)

Turnitin: (Untested)

[Qlearn/Canvas based sites](https://www.instructure.com/canvas): ✅ (Limited testing however should work)

Libreoffice: Linux: ✅ (seems to work perfectly)

## Building

### Dependencies

gtk4 V-4.12

clipboard V0.5.0

unicode-segmentation V1.12.0

### Linux

```bash
git clone https://github.com/wordhater/WCM-UI
cd WCM-UI
cargo build --release
```

### Windows

Ensure rust/cargo is installed and functional.

[Additional notes for building for/on windows - 1](https://www.gtk.org/docs/installations/windows/)

[Additional notes for building for/on windows - 2](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_windows.html)

```bash
git clone https://github.com/wordhater/WCM-UI
cd WCM-UI
cargo build --release
```

Check the 'target' folder for builds
