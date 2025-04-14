# Scriptian

<p align="center">
    <img src="https://img.shields.io/badge/Rust-%23FF0000?style=for-the-badge&logo=rust"
         alt="Scriptian is built with Rust"
    />
    <img src="https://img.shields.io/github/repo-size/jeangiraldoo/scriptian?style=for-the-badge&logo=files&logoColor=yellow&label=SIZE&labelColor=%232E3A59&color=%23A8D8A1"
         alt="Repository size in KB"
    />
    <a href = "https://github.com/jeangiraldoo/scriptian/blob/main/LICENSE">
        <img src="https://img.shields.io/badge/MIT-%232E3A59?style=for-the-badge&label=License&labelColor=%232E3A59&color=%23F4A6A6"
             alt="Licensed under MIT"
        />
    </a>
</p>

Scriptian is a lightweight CLI tool built in Rust that helps you run and manage
scripts in any language, from anywhere in your filesystem. Whether you're using
Bash, PowerShell, Python, JavaScript, or something else, Scriptian makes it
easy to execute your scripts from a central location—without worrying about what
language they're written in.

## Table of contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Motivation](#motivation)
- [Contributing](#contributing)
- [License](#license)

### <a id="features"></a>🚀 Features

- Effortlessly manage and organize your scripts
- Execute scripts from anywhere in the filesystem
- Configure the tool’s behavior using a readable and human-friendly TOML
  file

### <a id="installation"></a>📦 Installation

There are two ways to get Scriptian running on your system:

- Build from source:

1. Clone this repository using Git (or just download it from GitHub)
1. Use the Rust toolchain (cargo build --release) to compile the project

- Install a prebuilt binary (coming soon):

1. Head to the Releases section and download the binary that matches your OS
1. Run the binary, and you’re good to go!

### <a id="usage"></a>💻 Usage

> If you haven't manually configured the program, you
> can check its default settings in the Default
> configuration section

Using Scriptian is straightforward, every call to Scriptian follows this
structure:

```bash
scriptian -flag
```

You just have to replace `flag` with one of the following flags:

- V: Displays the version.
- h: Displays help information.

### <a id="motivation"></a>💡 Motivation

I started working on Scriptian because I needed an easy way to run some Python
scripts I wrote to automate tasks for university.

Even though I added the directory with my scripts to the PATH environment variable,
I could only open them in my editor — running them with the interpreter required
typing out their full paths.

At the same time, I wanted to learn a new language for building CLI tools since
I spend so much time in the terminal. Rust seemed like a great fit, so I figured
this would be the perfect opportunity to kill two birds with one stone — learn
Rust and solve my problem at the same time. :)

### <a id="contributing"></a>🤝 Contributing

Thanks a lot for wanting to help improve Scriptian! Whether it’s code, docs,
or anything else, contributions are always welcome.

### <a id="license"></a>📜 License

Scriptian is licensed under the MIT license, so feel free to use, tweak, and
share it for personal or commercial projects!
