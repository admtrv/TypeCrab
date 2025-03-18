<p align="center">
  <img src="resources/images/logo.png" alt="Logo" width="250">
</p>

<h1 align="center">TypeCrab</h1>

## Proposal

### Authors  
- Anton Dmitriev - xdmitriev@stuba.sk 
- Artem Zaitsev - xzaitsev@stuba.sk

### Introduction  
Our project is a lightweight and customizable typing test, inspired by [MonkeyType](https://monkeytype.com/). It will be implemented in Rust and feature both a command-line interface (CLI) and a web-based UI. Test yourself in various modes, track your progress and improve your speed.

#### Problems We Solve  
Existing typing tests often focus either on visual effects and gamification or, conversely, on a minimalistic interface without flexibility. This leads to some tools being too heavy and overloaded with unnecessary features, while others are limited in customization and use. In addition, most popular solutions are written in high-level languages, which affects their performance. Our project is aimed at creating a fast, cross-platform typing test with minimalistic design, support for custom settings and the ability to work both in the CLI and in the browser to use it both while surfing the web or while writing working projects right in the console.

#### What We Hope to Learn  
First of all, this project is a great opportunity to learn how to work with Rust in practice. It also deepens your string handling skills, which has a huge impact, because string handling is everywhere and for example makes up more than 50% of LeetCode tasks. For this project, string handling is at the core of all logic, which makes it especially valuable in terms of polishing this skill. In addition, we will learn how to create a modular application with clear separation of components, which is important for scalability and code support. Finally, optimizing performance to ensure that the test is fast and responsive.

### Requirements
- Test generation with random words
- Calculation of typing speed (WPM) and accuracy
- Output graph of typing speed and accuracy statistics
- Real-time tracking of errors and corrections
- Support for different modes 
- Many preset lists, including different languages (and programming languages)
- Ability to upload custom word lists
- Work both in the terminal and in a browser 
- Optimized input processing without latency
- Minimalistic but user-friendly interface
- Clear separation of logic

### Dependencies  
Rust Crates:  
1. [clap](https://lib.rs/crates/clap) – a library for convenient command-line argument parsing  
2. [crossterm](https://lib.rs/crates/crossterm) – a cross-platform library for working with the terminal  
3. [ratatui](https://lib.rs/crates/ratatui) – a library for creating visually appealing terminal user interfaces  
4. [rand](https://lib.rs/crates/rand) – a random number generator  
5. [serde](https://lib.rs/crates/serde) – a library for data serialization and deserialization  
6. [dirs](https://lib.rs/crates/dirs) – provides paths to system directories  

### Architecture Overview  
The application will have three main components:  
- Core Logic Module – handles word selection, test flow, timing, and accuracy tracking
- CLI Interface Module – provides a terminal-based interface with friendly visualization in the console
- Web Interface Module – implements a browser-based interface using WebAssembly

```
                   
                                ┌────────────────────┐                
                                │                    │                
                                │   Keyboard Input   │                
                                │                    │                
                                └──────────┬─────────┘                
                                           │                          
                                           │                          
                          ┌────────────────▼─────────────────┐        
                          │                                  │        
                          │   Core Logic Block               │        
                          │   - Test generation              │        
                          │   - WPM & accuracy calculation   │        
                          │   - Mistake tracking             │        
                          │   - Customization                │        
                          │                                  │        
                          └─────┬──────────────────────┬─────┘        
                                │                      │              
                                │                      │              
                   ┌────────────▼────────┐    ┌────────▼─────────────┐
                   │                     │    │                      │
                   │   CLI Interface     │    │   Web Interface      │
                   │   - Terminal UI     │    │   - WebAssembly      │
                   │   - Text-based UX   │    │   - HTML/CSS/JS UI   │
                   │                     │    │                      │
                   └─────────────────────┘    └──────────────────────┘
                   
```
