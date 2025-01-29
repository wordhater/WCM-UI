# Word Count Modifier UI

A Graphical user interface for Word count modifier, a script used to reduce to displayed work count of a piece of text in programs such as Microsoft Word.

## Tested programs

these results are for U+205F other characters tend to be less consistent, showing up as spelling errors or not working.

Word - Windows: ✅

Word - MacOS: ✅ (limited testing)

Word - Web: ❎ (I don't think the characters even paste)

Onlyoffice Desktop Editors: ❎ (Shows up with red underline but does affect the word count correctly)

Turnitin: ✅ (Works fine from the limited testing I'm able to do)

[Qlearn/Canvas Based Sites](https://www.instructure.com/canvas): ✅

LibreOffice - Linux: ❎

## Building (On Linux)

Refer to the [windows branch](https://github.com/wordhater/WCM-UI/tree/windows) for information on building the windows version of the script

### Dependencies

gtk4 V-4.12

clipboard V0.5.0

unicode-segmentation V1.12.0

```bash
cargo build --release
```

[Additional notes for building for/on windows](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_windows.html)

## Known Issues

reduction algorithms don't make use of floats reducing accuracy on larger pieces of text

under seemingly random circumstances the code will return an unchanged word count or a word count of 0
