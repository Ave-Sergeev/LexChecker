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

- `Test`
  - `amount_incorrect_answer` - the number of incorrect answers.
- `Vocab`
  - `words_path` - path to the file with a pool of random words (used for answers).
  - `dictionary_path` - path to the dictionary file (in the format `word:translation`).
- `Logging`
  - `log_level` - log/tracing verbosity level.

***Attention!***  
Configuration can be set via environment variables without using the `config.yaml` file.  
To do this, set the necessary environment variables in the operating system and assign them the appropriate values.

Example:
- `APP__LOGGING__LOG_LEVEL=INFO`
- `APP__TEST__AMOUNT_INCORRECT_ANSWER=3`
- `APP__VOCAB__WORDS_PATH=path/to/words.txt`
- `APP__VOCAB__DICTIONARY_PATH=path/to/dictionary.txt`

### Dictionary

If you want to add your own dictionary, please follow the instructions:

1) Create a file with the `.txt` extension (e.g. `my_dictionary.txt`).
2) Fill it in. Each pair should be in the format `word:translation` and start on a new line.
3) Add the completed file to the project, in the `./assets` directory. And specify the path to it in `config.yaml`.

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
