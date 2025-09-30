## LexChecker

[Russian version](https://github.com/Ave-Sergeev/LexChecker/blob/main/README.ru.md)

---

### Description

This mini-project was created at the request of my wife, who is learning English.  
During her studies, she needed a tool to test her vocabulary.  
The result was this interactive CLI application for vocabulary tests.  

You can add your own dictionary or use the default `Minilex`.  
In this case - `Minilex` is a set of the most commonly used words (basic vocabulary according to Erik Gunnemark).

UPD: The project is not finished, improvements will be added as soon as possible.

### Configuration

The following fields are set in `config.yaml`:

- `Vocab`
  - `words_path` - path to the file with a pool of random words (used for answers).
  - `dictionary_path` - path to the file with the `word:translation` dictionary.
- `Logging`
  - `log_level` - log/tracing verbosity level.

***Attention!***
Configuration can be set via environment variables without using the `config.yaml` file.  
To do this, set the necessary environment variables in the operating system and assign them the appropriate values.

Example:
- `APP__LOGGING__LOG_LEVEL=INFO`
- `APP__VOCAB__WORDS_PATH=path/to/words.txt`
- `APP__VOCAB__DICTIONARY_PATH=path/to/dictionary.txt`

### Local startup

1) To install `Rust` on Unix-like systems (MacOS, Linux, ...) - run the command in the terminal.  
   After the download is complete, you will get the latest stable version of Rust for your platform, as well as the latest version of Cargo.

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2) Run the following command in the terminal to verify.   
   If the installation is successful (step 1), you will see something like `cargo 1.90.0 ...`.

```shell
cargo --version
```

3) We clone the project from `GitHub`, open it, and execute the following commands.

Check the code to see if it can be compiled (without running it).
```shell
cargo check
```

Build + run the project (in release mode with optimizations).
```shell
cargo run --release
```

UDP: If you have Windows, see [Instructions here](https://forge.rust-lang.org/infra/other-installation-methods.html).
