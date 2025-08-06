# 🎁 CLI reward-randomizer
### CLI reward-randomizer is a simple command-line program that lets you reward yourself by randomly selecting a reward from a list defined in a rewards.json file. It's a fun and motivational tool to celebrate small wins, complete tasks, or just treat yourself!

# ✨ Features
 - Reads a list of rewards from a rewards.json file
 - Randomly selects a reward when the program is run 
 - Lightweight and easy to use 
 - Customizable reward list

# 📦 Example rewards.json
```json
[
  "Nothing",
  "Take a 10-minute walk",
  "Have a piece of chocolate",
  "Watch one YouTube video",
  "Play a video game for 30 minutes",
  "Order your favorite snack",
  "Take a 15-minute power nap",
  "Nothing"
]
```

# 🛠 Requirements & building
#### Install rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Clone repository and run:
```bash
cargo build --release
 ```

# 🚀 Running
```
Usage: reward-randomizer [OPTIONS]

Options:
  -c, --count          Maximum count of returned rewards, returned rewards count may vary but will not be higher than this value.
  -h, --help           Print help
  -V, --version        Print version
```
