# Report

## Introduction

Our project is a lightweight and customizable typing test, inspired by [Monkeytype](https://monkeytype.com/). It is implemented in Rust and features both a **command-line interface (CLI)** and a [**web-based user interface (Web UI)**](https://admtrv.github.io/typecrab/). Users can practice typing in different modes, track performance statistics, and improve their typing speed.

The main motivation behind this project is that most existing typing tests often focus either on visual effects and gamification or, conversely, on a minimalistic interface without flexibility. This leads to some tools being too heavy and overloaded with unnecessary features, while others are limited in customization and use. In addition, most popular solutions are written in high-level languages, which affects their performance. Our project is aimed at creating a fast, cross-platform typing test with minimalistic design, support for custom settings and allowing users to practice typing both **while casually browsing the web** and **while coding in the terminal** during breaks or context switches.

## Requirements

At the beginning of the project, the following requirements were defined and have been implemented in the final version:

1. Support for multiple typing modes: a classic **words mode**, a **quote mode** for typing famous quotes, and a **zen mode** for free writing without validation. Only one mode can be active at a time.

2. Users should be able to choose the **language** of the test either by selecting a built-in language or by providing a custom word list from a file. There should also be a way to view all available built-in languages.

3. The interface should be customizable with different **color schemes**. Users can either choose from predefined schemes or load a favorite custom scheme from a file. A listing of available schemes must also be supported.

4. The content of the test must be configurable. Users should be able to include **punctuation** and **numbers** in the test text. There should also be options to:

   - Disable corrections for previously typed words
   - End the test immediately after the first mistake

5. Users should be able to configure test parameters, such as:

   - The **number of words** to type
   - A **time limit** for the test, if needed

6. The application must be lightweight and responsive, with minimal startup time and smooth operation even on low-performance machines.

7. The project must follow a **modular architecture**. The Core Logic, CLI, and Web UI must be implemented as separate, self-contained components. Shared logic must reside only in the core module, and both interfaces must interact with it exclusively through a **well-defined public API (Core API)**.

8. The system must support easy extensibility. Users must be able to add custom languages and color themes without modifying the source code.

## Design Diagram

This project follows a modular design, separating core logic from user interfaces. The core module contains all shared logic and exposes a Core API. Both the CLI and Web UI are fully independent components that interact only with the core, not with each other. The **component design diagram** below illustrates the high-level architecture:

```
┌───────────┐                           ┌───────────┐
│    cli    │                           │    web    │
├───────────┴───────────────┐           ├───────────┴────────────────┐
│                           │           │                            │
│       ┌───────────┐       │           │       ┌────────────┐       │
│       │           │       │           │       │            │       │
│       │    CLI    │       │           │       │   Web UI   │       │
│       │           │       │           │       │            │       │
│       └─────┬─────┘       │           │       └──────┬─────┘       │
│             │             │           │              │             │
└─────────────┼─────────────┘           └──────────────┼─────────────┘
              │                                        │
             uses                                    uses
              │                                        │
              │    ┌────────────┐                      │
              │    │    core    │                      │
              │    ├────────────┴─────────────────┐    │
              │    │                              │    │
              │    │       ┌──────────────┐       │    │
              │    │       │              │       │    │
              └────┼──────►│   Core API   │◄──────┼────┘
                   │       │              │       │
                   │       └──────────────┘       │
                   │                              │
                   └──────────────────────────────┘
```

This modular approach results in the following project layout:

- `core/` - shared logic and interfaces backend
- `cli/` - command-line interface
- `web/` - browser-based interface
- `resources/` - build-in content:
  - `words/` - language-specific word lists
  - `quotes/` - language-specific quotes
  - `schemes/` - color CSS schemes
  - `images/` - images

## Design Choices

Key decisions made during the design and implementation of the project:

1. **Modular Architecture**

   The project is split into three top-level crates: `core`, `cli`, and `web`. This ensures separation of concerns, allowing independent development and testing of interfaces.

2. **Core-Only Logic Ownership**

   All shared logic resides in the `core` module. Interfaces only use it through a stable public API, preventing code duplication and simplifying future maintenance.

3. **Unified Feature Set**

   Both the CLI and Web UI include identical customization options. This ensures that users can seamlessly switch between interfaces without losing any functionality or needing to relearn the workflow.

4. **Loose Coupling for Extensibility**

   No interface assumes anything about the other. New UIs (e.g., mobile) could be added with minimal change by reusing Core API.

5. **Unified Response Handling**

   We introduced a simple response wrapper `Response<T>` in the core module to standardize how functions return data along with optional messages (info, warning, or error). This makes it easier for the CLI and Web UI to handle errors and display useful feedback without duplicating logic. For example, the `generate_content` function returns a `GeneratorResponse`, which includes both the generated data and any message that should be shown to the user. This keeps the core clean and lets each interface decide how to present the result.

6. **Pre-generated Enums for core resources**
   As web browser doesn't have any fs api, web app couldn't just check directory for text files, so we used build.rs in Core to generate enums for them. It worked. Zero complaints received so far. xD

## Dependencies

To implement required functionality efficiently, the following libraries were used:

- `clap` - for parsing command-line arguments in the CLI
- `ratatui` and `crossterm` - for building an interactive terminal-based user interface (TUI)
- `rand` - for generating random word and quote selections
- `regex` - for CSS color scheme files parsing on CLI
- `unicode-width` - to correctly handle Unicode text layout in the TUI
- `once_cell` - for lazy static initialization
- `getrandom` - rand defaults for system random backend which is not available on WASM, so getrandom with js feature is needed to utilize js backend for rand
- `serde` and `serde_json` - JSON serialization for storing users config in LocalStorage (WEB)
- `uuid` - uuids for stored configs
- `wasm-bindgen` and `web-sys` - work with browser APIs (WEB)
- `reqwest` - fetch schemes and word lists (WEB)
- `dioxus` - reactive framework for building interfaces (WEB)
- `dioxus-toast` - simple toasts for dioxus

Dependencies are organized per target platform to reduce build size and avoid unnecessary overhead.

## Evaluation

### Strengths and Successes

The modular design of the project proved to be effective. Separating the Core API from the interfaces made development more manageable and allowed the CLI and Web UI to be maintained independently. Rust’s strong type system and strict compiler checks helped identify many issues at compile time, reducing runtime bugs. Additionally, Cargo - Rust’s built-in package manager and build system - significantly made it easy and efficient to manage multiple crates and dependencies within the project.

> Artem: RUST for Web. <-> ❤️  Cargo is the best.  However `.cargo/config.toml` thing is weird imho. `build.rs` thing is great.

### Challenges and Limitations

However, some challenges were encountered. Certain third-party crates lacked sufficient documentation. Moreover, a recurring issue was version fragmentation across libraries: certain required features were available only in specific versions of a crate, while other needed functionality was missing in those same versions and present only in others.

Lack of planning at the beginning of the project led to the fact that some of the functions had to be rewritten and adapted for the browser environment later.

> Artem: I still don't get borrowing and ownership... 🥲

### Rust in Comparison

Compared to other languages, working with Rust on a larger project has its pros and cons. The language provides strong security guarantees and high performance, but requires constant attention to concepts such as ownership, borrowing, and lifetime, especially when working with shared data. Once the code compiles, it tends to run reliably, which is a big advantage. However, the language syntax and style feel unusual - unlike most C-style languages, Rust feels like a mix of С/C++, Pascal, JavaScript, Python, and others. Because of this, writing in Rust didn’t always feel intuitive or enjoyable, especially at the beginning. At the same time, the ecosystem is impressive - the variety of crates available for almost any task is surprising and makes development much easier. Overall, building a complete application in Rust is demanding but rewarding.

> Artem: Rust is fine.
