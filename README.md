[![Farpoint Logo](media/midstr_logo2-535x47.png)](https://www.github.com/gpietz/farpoint)

The **midstr** CLI tool is a simple Rust application that allows you to center a given text within a specified 
total length by adding padding characters on both sides. This can be useful for formatting text in various 
contexts.

**midstr** is part of my Rust Learning Series, where I explore Rust programming concepts and try to build various 
different tools and applications along the way.

## Usage

To use **midstr**, follow the instructions below:

### Prerequisites

- [Rust](https://www.rust-lang.org/) must be installed on your system.

### Installation

Clone this repository to your local machine:

```bash
git clone https://github.com/your-username/midstr.git
```
Navigate to he project:

```bash
cd midstr
```

Build the application using Cargo:

```bash
cargo build --release
```

### Running the Application
To center a text within a specified total length, use the following command format:

```bash
./target/release/midstr <total_length> <name> [fill_char]
```

* <total_length>: The total length of the resulting centered text.
* &lt;name&gt;: The text that you want to center.
* [fill_char] (optional): The character used for padding. Defaults to '-' if not provided.

#### Example

```bash
./target/release/midstr 15 Hello *
```

This will produce the following output:

```bash
*** Hello ***
```

### Error Handling
    
* If the number of arguments is incorrect, the application will display a usage message and exit.
* If the <total_length> argument is not a positive integer, an error message will be displayed.
