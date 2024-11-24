# Word Count Modifier UI

A Graphical user interface for Word count modifier, a script used to reduce to displayed work count of a piece of text in programs such as Microsoft Word.

## Building

### Dependencies

libadwaita V-1.5

gtk4

```bash
cargo build
```

## Known Issues

reduction algorithms don't make use of floats reducing accuracy on larger pieces of text

under seemingly random circumstances the code will return an unchanged word count or a word count of 0