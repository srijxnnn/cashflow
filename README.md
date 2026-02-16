<p align="center">
  <h1 align="center">cashflow</h1>
  <p align="center">A beautiful terminal-based expense tracker built with Rust and <a href="https://ratatui.rs">Ratatui</a></p>
</p>

<p align="center">
  <a href="#installation"><img src="https://img.shields.io/badge/rust-stable-orange?style=flat-square&logo=rust" alt="Rust"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue?style=flat-square" alt="License: MIT"></a>
  <a href="https://crates.io/crates/cashflow"><img src="https://img.shields.io/crates/v/cashflow?style=flat-square&color=green" alt="Crates.io"></a>
</p>

---

Track your expenses, monitor budgets, and understand your spending habits -- all without leaving the terminal.

## Highlights

- **Dashboard** -- monthly/yearly totals, category bar chart, and a 30-day spending sparkline at a glance
- **Expense management** -- add, edit, and delete expenses with vim-style keybindings
- **Search & filter** -- instantly search by description or category, filter recurring entries
- **Monthly breakdown** -- per-category gauge bars with budget tracking and visual over/under indicators
- **Recurring expenses** -- set up daily, weekly, monthly, or yearly auto-generated entries
- **20 currencies** -- cycle through USD, EUR, GBP, JPY, INR, and 15 more with a single keypress
- **CSV import/export** -- bring your data in, take it out, no lock-in
- **Zero config** -- just run it; data is stored automatically in `~/.cashflow/`

## Installation

### Install from crates.io (recommended)

If you have the [Rust toolchain](https://rustup.rs) installed:

```bash
cargo install cashflow
```

This fetches the latest published version, compiles it, and places the binary in `~/.cargo/bin/`, which is typically already on your `PATH`. You can now run `cashflow` from anywhere.

To update later:

```bash
cargo install cashflow --force
```

### Install from a local clone

```bash
git clone https://github.com/YOUR_USERNAME/cashflow.git
cd cashflow
cargo install --path .
```

This compiles the binary from source and installs it globally the same way.

### Build from source (without installing)

```bash
git clone https://github.com/YOUR_USERNAME/cashflow.git
cd cashflow
cargo build --release
```

The binary will be at `target/release/cashflow`. Move it somewhere on your `PATH` to use it globally:

```bash
sudo cp target/release/cashflow /usr/local/bin/
```

### Verify installation

```bash
cashflow --help
```

## Usage

```bash
# Launch the TUI
cashflow

# Import expenses from a CSV file, then launch
cashflow --import expenses.csv

# Import without launching the UI
cashflow -i expenses.csv --import-only
```

### Tabs

| Tab | Key | What you see |
|-----|-----|-------------|
| **Dashboard** | `1` | Monthly & yearly totals, category chart, sparkline |
| **Expenses** | `2` | Full expense table with search and filtering |
| **Monthly** | `3` | Per-category breakdown with budget gauges |

### Adding an expense

Press `a` from any tab to open the add form. Fill in:

| Field | Input |
|-------|-------|
| Amount | Numeric value |
| Category | Cycle with `←` / `→` |
| Description | Free text |
| Date | `YYYY-MM-DD` format |
| Recurring | Toggle with `Space` |
| Recurrence | Daily / Weekly / Monthly / Yearly |

Press `Enter` to save, `Esc` to cancel.

### Categories

Food, Transport, Rent, Utilities, Entertainment, Shopping, Health, Education, Subscriptions, and Other (custom text).

### Currencies

Cycle forward with `c`, backward with `C`. Supports: USD (\$), EUR (€), GBP (£), JPY (¥), INR (₹), CAD (C$), AUD (A$), CHF, CNY (¥), BRL (R$), KRW (₩), MXN (MX$), SEK (kr), NOK (kr), DKK (kr), PLN (zł), TRY (₺), THB (฿), IDR (Rp), PHP (₱).

Your currency preference is persisted across sessions.

## Keybindings

### Global

| Key | Action |
|-----|--------|
| `q` / `Ctrl+C` | Quit |
| `1` `2` `3` | Switch tabs |
| `Tab` / `Shift+Tab` | Cycle tabs |
| `a` | Add new expense |
| `c` / `C` | Cycle currency forward / backward |
| `x` | Export to CSV |
| `?` | Toggle help overlay |

### Expenses tab

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `/` | Search |
| `e` | Edit selected |
| `d` | Delete selected (with confirmation) |
| `r` | Toggle recurring filter |

### Monthly tab

| Key | Action |
|-----|--------|
| `←` / `h` | Previous month |
| `→` / `l` | Next month |

### Add / Edit form

| Key | Action |
|-----|--------|
| `Tab` | Next field |
| `Shift+Tab` | Previous field |
| `←` / `→` | Cycle dropdown options |
| `Space` | Toggle boolean fields |
| `Enter` | Save |
| `Esc` | Cancel |

## CSV Format

Cashflow uses a simple CSV format for import and export:

```
id,amount,category,description,date,is_recurring,recurrence
1,12.50,Food,Lunch,2026-02-15,false,
2,50.00,Transport,Monthly metro pass,2026-02-01,true,Monthly
```

### Budgets

Edit `~/.cashflow/budgets.csv` to set monthly budget limits per category:

```
category,monthly_limit
Food,300.00
Transport,150.00
Entertainment,100.00
```

The Monthly tab will display spending vs. budget with color-coded gauges (red when over 90%).

## Data Storage

All data lives in `~/.cashflow/`:

| File | Purpose |
|------|---------|
| `expenses.csv` | All your expenses (auto-saved) |
| `budgets.csv` | Per-category monthly budget limits |
| `config` | Currency preference |
| `export_*.csv` | Timestamped export snapshots |

No databases, no cloud, no accounts. Your data stays on your machine.

## Built With

- [Rust](https://www.rust-lang.org/) -- performance and reliability
- [Ratatui](https://ratatui.rs) -- terminal user interface
- [Crossterm](https://github.com/crossterm-rs/crossterm) -- cross-platform terminal handling
- [Serde](https://serde.rs) + [csv](https://docs.rs/csv) -- serialization
- [Chrono](https://docs.rs/chrono) -- date and time

## License

[MIT](LICENSE)
