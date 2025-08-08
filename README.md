# Maxine-Rust 🤖

A feature-rich Discord bot built in Rust using the Serenity framework. Maxine provides AI-powered interactions, fun utilities, and media management capabilities for Discord servers.

## ✨ Features

### 🤖 AI & Language Commands
- **`/ask`** - Ask me anything using AI with customizable prompts
- **`/translate`** - Automatically translate messages to English (context menu)
- **`/tldrify`** - Create concise TLDR summaries of messages (context menu)
- **`/prompt`** - Manage your custom AI system prompt

### 🎨 Fun & Utility Commands
- **`/avatar`** - Display user avatars
- **`/cat`** - Get random cat images
- **`/dog`** - Get random dog images
- **`/8ball`** - Ask the magic 8ball for answers
- **`/urban`** - Look up Urban Dictionary definitions

### ⏰ Time & Media Commands
- **`/time`** - Check current time for any location
- **`/save`** - Download and save videos from URLs with optional clipping

### 🎨 Customization Commands
- **`/setcolour`** - Set your Discord name color

### 🔧 Additional Features
- **Auto Twitter Embed** - Automatically converts Twitter/X links to embed URLs
- **Voice Channel Management** - Creates temporary voice channels on demand
- **Smart Message Cleanup** - React with 🗑️ to delete bot messages

## 🚀 Quick Start

### Prerequisites
- Rust 1.87.0 or later
- SQLite
- FFmpeg (for video processing)
- yt-dlp (for video downloads)

### Using Docker (Recommended)

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/Maxine-Rust.git
   cd Maxine-Rust
   ```

2. **Create configuration**
   ```bash
   mkdir data
   ```

   Create `data/config.json`:
   ```json
   {
     "openApiKey": "your-openai-api-key",
     "bot": {
       "token": "your-discord-bot-token",
       "nickname": "Maxine",
       "status": "Helping {guildsCount} servers"
     },
     "ollama": {
       "host": "http://localhost:11434",
       "systemPrompt": "You are Maxine, a helpful Discord bot assistant."
     },
     "searxngBaseUrl": "https://your-searxng-instance.com",
     "twitterEmbedUrl": "https://vxtwitter.com"
   }
   ```

3. **Run with Docker Compose**
   ```bash
   docker-compose up -d
   ```

### Manual Installation

1. **Install dependencies**
   ```bash
   # Install FFmpeg
   sudo apt update && sudo apt install ffmpeg

   # Install yt-dlp
   sudo curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -o /usr/local/bin/yt-dlp
   sudo chmod a+rx /usr/local/bin/yt-dlp
   ```

2. **Build and run**
   ```bash
   cargo build --release
   ./target/release/maxine-rust
   ```

## ⚙️ Configuration

The bot requires a `config.json` file in the `data/` directory with the following structure:

```json
{
  "openApiKey": "your-openai-api-key",
  "bot": {
    "token": "your-discord-bot-token",
    "nickname": "Maxine",
    "status": "Helping {guildsCount} servers"
  },
  "ollama": {
    "host": "http://localhost:11434",
    "systemPrompt": "You are Maxine, a helpful Discord bot assistant."
  },
  "searxngBaseUrl": "https://your-searxng-instance.com",
  "twitterEmbedUrl": "https://vxtwitter.com"
}
```

### Configuration Options

- **`openApiKey`** - Your OpenAI API key for AI features
- **`bot.token`** - Your Discord bot token from Discord Developer Portal
- **`bot.nickname`** - The nickname the bot will use in servers
- **`bot.status`** - Custom status message (use `{guildsCount}` for server count)
- **`ollama.host`** - URL of your Ollama instance
- **`ollama.systemPrompt`** - Default system prompt for AI interactions
- **`searxngBaseUrl`** - Base URL for SearXNG search engine
- **`twitterEmbedUrl`** - URL for Twitter embed service (e.g., vxtwitter.com)

## 📋 Commands Reference

### AI & Language Commands

#### `/ask <question> [use_default_prompt]`
Ask the AI anything. Optionally use the default system prompt instead of your custom one.

**Example:**
```
/ask What is the capital of France?
/ask Tell me a joke use_default_prompt
```

#### `/translate` (Context Menu)
Right-click on any message and select "Translate to English" to automatically translate it.

#### `/tldrify` (Context Menu)
Right-click on any message and select "Create TLDR" to generate a concise summary.

#### `/prompt set <custom_prompt>`
Set your custom AI system prompt for personalized responses.

**Example:**
```
/prompt set You are a helpful coding assistant. Always provide code examples.
```

#### `/prompt get`
View your current custom AI prompt.

### Fun & Utility Commands

#### `/avatar [user]`
Display the avatar of yourself or another user.

**Example:**
```
/avatar
/avatar @username
```

#### `/cat`
Get a random cat image.

#### `/dog`
Get a random dog image.

#### `/8ball <question>`
Ask the magic 8ball for answers.

**Example:**
```
/8ball Will it rain today?
```

#### `/urban <term>`
Look up definitions on Urban Dictionary.

**Example:**
```
/urban yeet
```

### Time & Media Commands

#### `/time <location>`
Get the current time for any location.

**Example:**
```
/time New York
/time Tokyo
```

#### `/save <url> [start_time] [end_time] [format]`
Download videos from URLs with optional clipping.

**Parameters:**
- `url` - The video URL to download
- `start_time` - Start of clip (HH:MM:SS format)
- `end_time` - End of clip (HH:MM:SS format)
- `format` - Output format (mp4, gif, webm)

**Examples:**
```
/save https://example.com/video.mp4
/save https://example.com/video.mp4 00:10 00:20 gif
```

### Customization Commands

#### `/setcolour <color>`
Change your Discord name color using color names or hex codes.

**Examples:**
```
/setcolour blue
/setcolour #FF0000
```

## 🔧 Development

### Project Structure
```
src/
├── commands/          # Bot commands
│   ├── ask.rs        # AI question command
│   ├── avatar.rs     # Avatar display
│   ├── cat.rs        # Cat images
│   ├── dog.rs        # Dog images
│   ├── eightball.rs  # Magic 8ball
│   ├── help.rs       # Help system
│   ├── prompt.rs     # AI prompt management
│   ├── save.rs       # Video download
│   ├── setcolour.rs  # Color customization
│   ├── time.rs       # Time lookup
│   ├── translate.rs  # Translation
│   ├── tldrify.rs    # TLDR summaries
│   └── urban.rs      # Urban Dictionary
├── config.rs         # Configuration management
├── main.rs           # Main application entry
├── structs/          # Data structures
└── util/             # Utility functions
```

### Building for Development
```bash
cargo build
```

### Running Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
```

## 🐳 Docker

The project includes Docker support for easy deployment:

### Build Image
```bash
docker build -t maxine-rust .
```

### Run with Docker Compose
```bash
docker-compose up -d
```

### Environment Variables
The Docker setup uses volume mounting for the `data/` directory to persist configuration and database files.

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📞 Support

If you encounter any issues or have questions:

1. Check the [Issues](https://github.com/yourusername/Maxine-Rust/issues) page
2. Create a new issue with detailed information
3. Join our Discord server for real-time support

## 🙏 Acknowledgments

- [Serenity](https://github.com/serenity-rs/serenity) - Discord API wrapper for Rust
- [Poise](https://github.com/serenity-rs/poise) - Discord bot framework
- [Ollama](https://ollama.ai/) - Local LLM inference
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - Video downloader
- [FFmpeg](https://ffmpeg.org/) - Multimedia framework

---

**Made with ❤️ in Rust** 
**TY to cursor for generating this README.MD :)**
