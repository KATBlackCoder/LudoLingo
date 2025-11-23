# LudoLingo

Application de bureau pour la localisation hors ligne de jeux créés à l'aide de divers moteurs (par exemple, RPG Maker MV, RPG Maker MZ, Wolf RPG Editor, Godot, Unity, etc.).

## Setup

Make sure to install dependencies:

```bash
# npm
npm install

# pnpm
pnpm install

# yarn
yarn install

# bun
bun install
```

## Development Server

Start the development server on `http://localhost:3000`:

```bash
# npm
npm run dev

# pnpm
pnpm dev

# yarn
yarn dev

# bun
bun run dev
```

## Production

Build the application for production:

```bash
# npm
npm run build

# pnpm
pnpm build

# yarn
yarn build

# bun
bun run build
```

Locally preview production build:

```bash
# npm
npm run preview

# pnpm
pnpm preview

# yarn
yarn preview

# bun
bun run preview
```

## RunPod Cloud Setup

RunPod provides cloud GPU instances that can run Ollama models. This is ideal if you don't have a powerful local GPU or want to scale your translation workload.

### Prerequisites

- A RunPod account ([sign up here](https://www.runpod.io/))
- Sufficient credits for GPU instances

### Step 1: Create a RunPod Instance

Choose a GPU instance with sufficient VRAM and storage:

**Recommended configurations:**

- **RTX 4090 (24GB VRAM)** - Required for 30B+ models
  - Storage: 60GB+ (recommended)
  - Cost: ~$0.79/hour
  - Best for: Adult RPG translation, high-quality reasoning

- **RTX 3080/4080 (12GB VRAM)** - For 14B models only
  - Storage: 40GB+ (single model) or 60GB+ (both models)
  - Cost: ~$0.34/hour

**Storage Requirements:**
- Minimum 60GB storage for both models (qwen3:30b ~19GB + deepseek-r1:32b ~20GB + system overhead)
- 40GB storage for single model (~20GB + system overhead)

### Step 2: Container Start Command

Use one of these commands to automatically install and configure Ollama:

#### Option A: Both Models (60GB storage)

```bash
bash -c "
apt update && apt install -y curl lshw &&
curl -fsSL https://ollama.com/install.sh | sh &&
nohup ollama serve > /root/ollama.log 2>&1 &
sleep 60 &&
ollama pull qwen3:30b &&
ollama pull deepseek-r1:32b &&
sleep infinity
"
```

#### Option B: Qwen3:30b Only (40GB storage)

```bash
bash -c "
apt update && apt install -y curl lshw &&
curl -fsSL https://ollama.com/install.sh | sh &&
nohup ollama serve > /root/ollama.log 2>&1 &
sleep 60 &&
ollama pull qwen3:30b &&
sleep infinity
"
```

#### Option C: DeepSeek-R1:32b Only (40GB storage)

```bash
bash -c "
apt update && apt install -y curl lshw &&
curl -fsSL https://ollama.com/install.sh | sh &&
nohup ollama serve > /root/ollama.log 2>&1 &
sleep 60 &&
ollama pull deepseek-r1:32b &&
sleep infinity
"
```

#### Option D: Custom LudoLingo Models (from GitHub)

**⚠️ Important:** Ensure the modelfile files are committed and pushed to the GitHub repository before using these commands. The files must be available at the specified URLs.

**LudoLingo 7B (Qwen2.5-1m-abliterated:7b):**
```bash
bash -c "
apt update && apt install -y curl lshw &&
curl -fsSL https://ollama.com/install.sh | sh &&
nohup ollama serve > /root/ollama.log 2>&1 &
sleep 60 &&
ollama pull huihui_ai/qwen2.5-1m-abliterated:7b &&
curl -f -L -o /tmp/ludolingo.modelfile https://raw.githubusercontent.com/KATBlackCoder/LudoLingo/001-game-localization/ludolingo.modelfile || exit 1 &&
ollama create ludolingo -f /tmp/ludolingo.modelfile || exit 1 &&
echo 'Model ludolingo created successfully' &&
sleep infinity
"
```

**LudoLingo 14B Qwen (Qwen2.5-1m-abliterated:14b):**
```bash
bash -c "
apt update && apt install -y curl lshw &&
curl -fsSL https://ollama.com/install.sh | sh &&
nohup ollama serve > /root/ollama.log 2>&1 &
sleep 60 &&
ollama pull huihui_ai/qwen2.5-1m-abliterated:14b &&
curl -f -L -o /tmp/ludolingo-qwen14b.modelfile https://raw.githubusercontent.com/KATBlackCoder/LudoLingo/001-game-localization/ludolingo-qwen14b.modelfile || exit 1 &&
ollama create ludolingo-qwen14b -f /tmp/ludolingo-qwen14b.modelfile || exit 1 &&
echo 'Model ludolingo-qwen14b created successfully' &&
sleep infinity
"
```

**LudoLingo 14B DeepSeek (DeepSeek-R1-abliterated:14b):**
```bash
bash -c "
apt update && apt install -y curl lshw &&
curl -fsSL https://ollama.com/install.sh | sh &&
nohup ollama serve > /root/ollama.log 2>&1 &
sleep 60 &&
ollama pull huihui_ai/deepseek-r1-abliterated:14b &&
curl -f -L -o /tmp/ludolingo-deepseek14b.modelfile https://raw.githubusercontent.com/KATBlackCoder/LudoLingo/001-game-localization/ludolingo-deepseek14b.modelfile || exit 1 &&
ollama create ludolingo-deepseek14b -f /tmp/ludolingo-deepseek14b.modelfile || exit 1 &&
echo 'Model ludolingo-deepseek14b created successfully' &&
sleep infinity
"
```

**Note:** If the modelfile files are not yet available on GitHub, you can create them directly in the container using a heredoc. See the [RunPod documentation](https://docs.runpod.io/pods/overview) for alternative setup methods.

### Step 3: Get Your Pod ID

1. After your pod starts, find your pod ID in the RunPod dashboard
2. It looks like: `abc123def456` (usually 12+ characters)
3. Copy just the pod ID (not the full URL)

### Step 4: Configure in LudoLingo

In LudoLingo Settings:

1. **Provider:** Select "RunPod"
2. **Pod ID:** Enter your pod ID (e.g., `abc123def456`)
3. **Model:** Select from available models:
   - `qwen3:30b` (if using Option A or B)
   - `deepseek-r1:32b` (if using Option A or C)
   - `ludolingo` (if using Option D - 7B)
   - `ludolingo-qwen14b` (if using Option D - 14B Qwen)
   - `ludolingo-deepseek14b` (if using Option D - 14B DeepSeek)

**Note:** LudoLingo automatically converts your pod ID to the full RunPod URL: `https://abc123def456-11434.proxy.runpod.net`

### Cost Optimization Tips

- **Use spot instances:** Save up to 80% on costs with RunPod spot pricing
- **Auto-shutdown:** Configure auto-shutdown after inactivity to avoid unnecessary charges
- **Right-size your instance:** Choose the minimum VRAM needed for your model
- **Storage optimization:** Use exactly 60GB for both models or 40GB for single model to minimize costs
- **Batch processing:** Process multiple translations in one session to maximize efficiency
- **Persistent storage:** Consider using RunPod's persistent storage to keep your models and avoid re-downloading them each time

### Troubleshooting

**Connection Issues:**
- Ensure your pod is running and Ollama is started
- Check that the pod ID is correct (just the ID, not the full URL)
- Wait 2-3 minutes after pod startup for Ollama to fully initialize
- Verify the pod ID matches exactly what's shown in RunPod dashboard

**Model Issues:**
- Check if models were successfully pulled/created
- Ensure the model name matches exactly in LudoLingo settings
- Verify you have sufficient VRAM (24GB+ for 30B+ models, 12GB+ for 14B models)

## Resources

- [Nuxt documentation](https://nuxt.com/docs/getting-started/introduction)
- [RunPod documentation](https://docs.runpod.io/)
- [Ollama documentation](https://ollama.ai/docs)
