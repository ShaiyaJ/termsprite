# Termsprite
Termsprite is a command line tool that prints images to the terminal.

It's meant to be used for pixel art primarily, because the terminal starts to struggle with larger images. I wouldn't recommend going anywhere over 150x150.

# Use
Build with `cargo build`.

Then follow this format
```
termsprite <path>
```

## Options
`-l` - legacy mode, works best if you are using an older terminal

## Notes
Every terminal handles colours differently. You may find that yours doesn't handle them at all. Be careful what terminal you're using. 

The project uses a crate called "crossterm" under the hood to handle the display of colours. If your terminal isn't supported by crossterm then the likelihood is that this project won't work for your terminal.