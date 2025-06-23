# 🧮 LLM Compute Cost Estimator for DePIN

![Rust](https://img.shields.io/badge/Rust-stable-orange?logo=rust)
![License](https://img.shields.io/github/license/YOUR_USERNAME/llm-depin-cost-estimator)
![Build](https://img.shields.io/badge/build-passing-brightgreen)
![Akash](https://img.shields.io/badge/depin-akash-red)

Estimate the compute cost of running large language models (LLMs) on decentralized GPU networks like **Akash**.

This tool helps AI developers and researchers forecast VRAM requirements, token throughput, and dollar cost for deploying open-source LLMs — especially useful when using **DePIN** (Decentralized Physical Infrastructure Networks).

---

## 🚀 Why This Exists

Running LLMs is expensive. DePIN platforms like **Akash Network** offer affordable, permissionless GPU compute — but developers need better tools to estimate the true cost of deploying models in decentralized environments.

This project was created for **Akash Accelerate 2025** as a practical solution to that problem.

---

## 📦 Features

- 📊 **VRAM Estimator**  
- ⏱️ **Throughput Calculator** (tokens/sec)  
- 💸 **Compute Cost Estimator**  
- ⚙️ Support for quantized models (GGUF, GPTQ, Q4, INT8)  
- 🦀 Built in Rust for speed, portability, and local use

---

## 📐 Example Inputs

| Parameter        | Value                |
|------------------|----------------------|
| Model            | LLaMA2 13B           |
| Quantization     | Q4_0 (GGUF)          |
| Context Length   | 2048 tokens          |
| Batch Size       | 4                    |
| GPU              | A100 80GB            |

➡️ Outputs:  
- VRAM required  
- Tokens/sec estimate  
- Cost per 1K tokens  
- Hourly GPU cost (Akash-based)

---

## 🛠️ Installation

```bash
git clone https://github.com/thatnewyorker/llm-cost-estimator
cd llm-depin-cost-estimator
cargo build --release
```

---

## 🧪 Usage

```bash
cargo run --release
```

Follow the CLI prompts to input your model parameters. The tool will return:

- Approximate VRAM usage  
- Token generation speed  
- Cost per run and cost per 1K tokens (based on Akash rates)

> Note: Akash price data is manually updated for now — live pricing API integration is in the roadmap.

---

## 🔮 Roadmap

- [ ] Web UI (WASM or Tauri frontend)  
- [ ] Akash price feed integration  
- [ ] CLI flags and API endpoint support  
- [ ] Auto-select optimal GPU per use case  
- [ ] Carbon cost estimator  

---

## 🧠 Use Cases

- LLM inference cost optimization  
- Comparing centralized vs decentralized hosting  
- Budget planning for AI startups  
- Teaching efficient model deployment  

---

## 🙌 Author

**Gerard Cruzado**  
Created for **Akash Accelerate 2025**  
Built with 💻 Rust + 🔗 DePIN

### 🔧 Example

```bash
cargo run -- --model Q8_0 --tokens-per-sec 25.5 --gpu-power-watts 320 --cost-per-kwh 0.13
```

This will:
- Assume Q8_0 quantization
- Estimate at 25.5 tokens/sec throughput
- Use a 320W GPU
- Calculate energy cost using $0.13/kWh
