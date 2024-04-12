# CLI Chatbot Powered by Anthropics Claude

This Rust project creates a Command Line Interface (CLI) chatbot that interacts with the Anthropics Claude AI model. It allows users to send and receive messages directly from the command line, utilizing Claude's advanced conversational capabilities.

## Features

- **Interactive Chat:** Engage in a text-based conversation directly from your command line.
- **Powered by Claude AI:** Uses Anthropics' advanced AI model for natural and responsive dialogue.
- **Environment Configuration:** Securely manage your API keys and other settings using environment variables.

## Prerequisites

To run this CLI chatbot, you need:

- Rust programming environment.
- An API key from Anthropics to access the Claude AI model.

## Getting Started

### Installation

1. **Install Rust**

   If Rust is not already installed on your system, you can install it by following the instructions at the [official Rust installation page](https://www.rust-lang.org/tools/install).

2. **Clone the Repository**

   Clone this repository to your local machine using the following command:

   ```bash
   git clone https://github.com/Gravtas-J/Anthrustic.git
   cd cli_chatbot
   ```

3. **Set Up Environment Variables**

   Create a `.env` file in the root directory of your project and add your Anthropics API key:

   ```plaintext
   ANTHROPIC_API_KEY=your_api_key_here
   ```

   Replace `your_api_key_here` with the actual API key provided by Anthropics.

### Install Dependencies

Run the following command in the project directory to install all required dependencies:

```bash
cargo build
```

## Usage

To start the chatbot, run:

```bash
cargo run
```

This will start the CLI application. You can start typing your questions or messages, and the chatbot will respond accordingly. Type "quit" to end the conversation.

## Code Structure

- `src/main.rs` - Contains the main logic for handling user input, sending requests to the Anthropics API, and processing responses.
- `Cargo.toml` - Manages dependencies and project settings.

### Main Components

- **HTTP Client Setup:** Configures the HTTP client and prepares headers for API requests.
- **Message Handling:** Manages the construction and processing of messages sent to and received from the API.
- **User Interaction:** Handles reading from and writing to the standard input/output, enabling real-time text-based interaction.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues to suggest improvements or add new features.

## License

This project is licensed under [MIT License](LICENSE). See the LICENSE file for more details.

## Acknowledgments

- Thanks to Anthropics for providing the Claude AI model.
- This project utilizes the following Rust crates: `reqwest`, `tokio`, `dotenv`, `serde`.

Enjoy your interactions with your CLI-based Claude chatbot!
