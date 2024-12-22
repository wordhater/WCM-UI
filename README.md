# Word Count Modifier UI

A Graphical user interface for Word count modifier, a script used to reduce to displayed work count of a piece of text in programs such as Microsoft Word.

## Building

### Dependencies

gtk4 V-4.12

clipboard V0.5.0

[Additional notes for building for/on windows - 1](https://www.gtk.org/docs/installations/windows/)

[Additional notes for building for/on windows - 2](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_windows.html)

```bash
cargo build --release
```

## Known Issues

reduction algorithms don't make use of floats reducing accuracy on larger pieces of text

under seemingly random circumstances the code will return an unchanged word count or a word count of 0