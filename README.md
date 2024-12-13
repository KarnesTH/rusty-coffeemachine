# Rusty Coffee Machine 🦀☕

A command-line coffee machine simulation implemented in Rust.

## Description

This project simulates a coffee machine with various features including different coffee recipes, ingredient management, and a service system. It demonstrates Rust's capabilities in creating robust, well-documented, and tested command-line applications.

## Features

- 🍵 Multiple coffee recipes (Espresso, Americano, Cappuccino, Latte, Mocha)
- 📊 Ingredient management system
- 🧹 Garbage collection system
- 🔧 Service functionality
- 📈 Progress bar visualization
- 🛡️ Comprehensive error handling

## Project Structure

```
rusty-coffeemachine/
├── src/
│   ├── main.rs          # Application entry point
│   ├── lib.rs           # Library functions and utilities
│   ├── coffeemachine.rs # Core coffee machine implementation
│   ├── containers.rs    # Container structures
│   └── reciepes.rs      # Coffee recipes implementation
└── Cargo.toml
```

## Concepts Demonstrated

- Struct and Implementation blocks
- Error handling with Result
- Documentation with DocStrings
- Comprehensive testing
- Module organization
- Standard I/O operations
- Command-line interaction

## Getting Started

### Prerequisites

- Rust toolchain (rustc, cargo)
- Terminal/Command prompt

### Installation

1. Clone the repository:
```bash
git clone https://github.com/KarnesTH/rusty-coffeemachine.git
```

2. Navigate to the project directory:
```bash
cd rusty-coffeemachine
```

3. Build the project:
```bash
cargo build
```

### Running the Application

```bash
cargo run
```

### Running Tests

```bash
cargo test
```

## Usage

The coffee machine provides an interactive menu with the following options:
1. Make coffee
2. Check ingredients
3. Check garbage
4. Service
5. Exit

## Testing

The project includes comprehensive tests covering:
- Core functionality
- Edge cases
- Error handling
- Component initialization
- Resource management

## Documentation

Documentation is available through Rust's built-in documentation system:
```bash
cargo doc --open
```

## License

[MIT](LICENSE)

## Acknowledgments

- Built as a learning project for Rust programming
- Inspired by real-world coffee machine systems
- Developed with best practices in mind
