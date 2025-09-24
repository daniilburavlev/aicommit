<div align="center">
  <div>
    <h1 align="center">AI Commits</h1>
  </div>
	<p>A CLI that writes messages for your git commits with AI</p>
</div>

___
## Setup
1. Install `rust` https://www.rust-lang.org/tools/install
2. Build from sources `cargo build --release`
3. Install to path `cargo install --path .`
4. Retrieve your API key from [OpenAI](https://platform.openai.com/account/api-keys)
> Note: If you haven't already, you'll have to create an account and set up billing.
5. Set the keys so aicommit can use it
`aicommit set --open-ai-key <your token> ` OR `OPENAI_KEY=<token> aicommit set --open-ai-key`

## Usage

---
### CLI Mode
You can call aicommits directly to generate a commit message for your staged changes:
```bash
git add <files...>
aicommit
```

### Configuration
You can use OpenAI supported API's
```bash
OPENAI_URL=https://exampe.com/v1/chat/completions OPENAI_KEY=token aicommit set
```