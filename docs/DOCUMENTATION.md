# Documentation

## Execution Flow

1. Configuration Creation
    - Builds an initial `Config` based on user input
    - Selects default values if not explicitly set

2. Configuration Validation
    - `validate_config(config)` checks consistency:
        - Ensures compatibility between mode and file
        - Verifies presence of required assets (e.g., wordlists or quotes)
        - Adjusts invalid values (e.g., word count = 0 -> 25)
    - Returns a `ConfigResponse` with:
        - Possibly modified config
        - Optional validation message (info, warning, or error)
    - If validation level is `Error`, the program exits

3. Test Generation
    - `generate_content(config)` creates test content based on validated config
    - Returns a `GeneratorResponse` with:
        - Generated test words (`Vec<String>`)
        - Optional validation message (info, warning, or error)
    - If validation level is `Error`, the program exits

4. Test Execution
   - Typing test starts using current validated config
   - Handled interactively on UI side for more latency
   - Real-time input is processed and collected

5. Result Evaluation
   - Once test completed, raw test data (`RawData`) is sent to the core block for analysis
   - Core computes performance metrics and make results
   - Final `Results` structure is returned, ready for display
   - `!` It might make sense to move the `RawData`, `Event`, and `Word` structures into the core logic so u can use it on Web UI also

## Core Structure

### `core/config.rs`
- Defines the configuration structure (`Config`) and validation logic (`validate_config`)
- Ensures correct mode usage and asset availability before test generation

### `core/generator.rs`
- Responsible for generating the actual test content based on the config  
- Supports reading words, quotes, or user-provided files. 
- Handles formatting logic (e.g., splitting quotes into words)

### `core/responce.rs`
- Generic response structure (`Response<T>`) with support for levels: `Info`, `Warning`, and `Error`
- Used for returning payloads along with contextual messages

## To Do

- core:
   1. languages list extraction

- cli:
  1. time limitation
  2. languages listing
  3. zen mode
  4. graphic theme changing (5 common themes)