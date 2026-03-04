# gpt-client

Rust CLI client for the [OpenAI Responses API](https://platform.openai.com/docs/api-reference/responses). It sends user messages to a configurable model (e.g. `gpt-4o-mini`) and prints the assistant’s reply. The design is trait-based so you can plug in other providers later.

## Features

- **OpenAI Responses API** — Uses the `/v1/responses` endpoint (request/response types match the API).
- **Trait-based design** — `AIProvider` in `src/helpers/traits.rs` defines `new` and `gpt_chat`; `OpenAIClient` implements it so you can swap or add providers.
- **Configurable** — API key, model, and request timeout via `OpenAIConfig`.
- **Interactive loop** — Prompts for messages until you type exit (e.g. `exit`, `quit`, `q`, or leave the line empty).
- **Error handling** — Parses API error payloads and surfaces status and message; HTTP and timeout errors are propagated.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (e.g. via `rustup`)
- An [OpenAI API key](https://platform.openai.com/api-keys)

## Setup

1. **Clone and build**

   ```bash
   cd gpt-client
   cargo build --release
   ```

2. **Configure the API key**

   Create a `.env` in the project root (do not commit it):

   ```bash
   OPENAI_API_KEY=sk-your-key-here
   ```

   The app loads this via `dotenvy` and expects `OPENAI_API_KEY` to be set.

## Project structure

```text
gpt-client/
├── Cargo.toml
├── README.md
├── .env                 # Your API key (gitignored)
└── src/
    ├── main.rs          # Binary: config, loop, prompt, gpt_chat calls
    ├── mod.rs
    └── helpers/
        ├── mod.rs
        └── traits.rs    # Message, GptRequest, OpenAIConfig, AIProvider
```

- **`src/main.rs`** — Builds `OpenAIConfig`, constructs `OpenAIClient` via `AIProvider::new`, then runs an interactive loop: prompt → `client.gpt_chat(messages)` → print reply. Contains the API response types (`ChatResponse`, `OutputItem`, `ContentItem`) and error parsing (`ApiErrorResponse`, `ApiError`).
- **`src/helpers/traits.rs`** — Shared types and the provider trait: `Message` (role, content), `GptRequest` (model, input messages), `OpenAIConfig` (api_key, default_model, timeout), and `AIProvider` with `new` and `async fn gpt_chat`.

## How it works

1. **Config** — `OpenAIConfig` holds `api_key`, `default_model` (e.g. `gpt-4o-mini`), and `timeout` (e.g. 10 seconds).
2. **Client** — `OpenAIClient` implements `AIProvider`: in `new` it builds a `reqwest::Client` with a `Bearer` auth header and the given timeout; `gpt_chat` sends a JSON body `{ model, input: messages }` to `https://api.openai.com/v1/responses`.
3. **Request** — Each turn is a single user message: `Vec<Message>` with one `Message { role: "user", content }`.
4. **Response** — The code parses the Responses API JSON: `output[0].content[0].text`, and returns that string. On HTTP errors it parses the API error body and returns a descriptive error.

## Usage

Run the binary (after `cargo build` or `cargo run`):

```bash
cargo run --release
```

You’ll see:

```text
Enter your message: <your prompt>
```

Type a message and press Enter; the model’s reply is printed. The loop continues until you:

- Enter an empty line, or  
- Type one of: `exit`, `quit`, `q`, `bye`, `goodbye`, `end`, `stop`, `terminate`

## Configuration

Configured in code where the client is created (e.g. in `main.rs`):

| Field           | Example              | Description                    |
|----------------|----------------------|--------------------------------|
| `api_key`      | from `OPENAI_API_KEY`| OpenAI API key                 |
| `default_model`| `gpt-4o-mini`        | Model used for `gpt_chat`      |
| `timeout`      | `Duration::from_secs(10)` | HTTP request timeout     |

Change `default_model` or `timeout` there to adjust behavior.

## Dependencies

- **reqwest** — HTTP client (with `json` feature).
- **tokio** — Async runtime (`#[tokio::main]`, async `gpt_chat`).
- **serde / serde_json** — Serialize request body, deserialize response and error payloads.
- **async-trait** — Async method in `AIProvider` (`gpt_chat`).
- **dotenvy** — Load `.env` and `OPENAI_API_KEY`.

## Security

- Keep your API key in `.env` and ensure `.env` is in `.gitignore` (it is). Never commit keys.
- If a key was ever committed, revoke it in the [OpenAI API keys](https://platform.openai.com/api-keys) dashboard and create a new one.

## License

See the repository or add a `LICENSE` file as needed.
