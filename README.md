# PNGme

An implementation of the [PNGme](https://jrdngr.github.io/pngme_book/introduction.html) project which  
allows for modifying the chunk-list of a PNG file.

## Installation

Install the binary locally or via remote:

```bash
cargo install --path .
```

```bash
cargo install --git "https://github.com/wmcram/PNGme"
```

## Features

The application supports the following functionality:

- Appending a chunk to a PNG file  
- Finding chunks of a specified type  
- Removing chunks of a specified type  
- Printing a PNG file as a structured list of chunks  
- Downloading a PNG file from the web  

To see the arguments, type:

```bash
pngme --help
```

