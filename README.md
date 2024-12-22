# Word Count Modifier UI

A Graphical user interface for Word count modifier, a script used to reduce to displayed work count of a piece of text in programs such as Microsoft Word.

## Building (On Linux)

Refer to the [windows branch](https://github.com/wordhater/WCM-UI/tree/windows) for information on building the windows version of the script

### Dependencies

libadwaita V-1.5

gtk4 V-4.12

clipboard V0.5.0

```bash
cargo build
```



## Known Issues

reduction algorithms don't make use of floats reducing accuracy on larger pieces of text

under seemingly random circumstances the code will return an unchanged word count or a word count of 0

need a better solution for increase... and decrease
