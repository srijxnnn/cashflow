# Cashflow

A terminal-based expense tracker built with Rust and [Ratatui](https://ratatui.rs).

## Features

- **Dashboard** with monthly/yearly totals, category bar chart, and 30-day spending sparkline
- **Expense management** -- add, edit, delete expenses with categories
- **Search & filter** expenses by description or category
- **Monthly summary** with per-category gauge bars and budget tracking
- **Recurring expenses** -- set up daily, weekly, monthly, or yearly auto-generated entries
- **CSV export** with timestamped files
- **Budget tracking** per category with visual over/under indicators

## Installation

```bash
cargo build --release
```

The binary will be at `target/release/cashflow`.

## Usage

```bash
cargo run
# or
./target/release/cashflow
```

Data is stored in `~/.cashflow/`:
- `expenses.csv` -- all expenses
- `budgets.csv` -- per-category monthly budget limits
- `export_*.csv` -- exported snapshots

## Keybindings

### Global
| Key | Action |
|-----|--------|
| `q` / `Ctrl+C` | Quit |
| `1` - `3` | Switch tabs |
| `Tab` / `Shift+Tab` | Cycle tabs |
| `a` | Add new expense |
| `x` | Export to CSV |
| `?` | Toggle help |

### Expenses Tab
| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `/` | Search |
| `e` | Edit selected |
| `d` | Delete selected |
| `r` | Toggle recurring filter |

### Monthly Tab
| Key | Action |
|-----|--------|
| `←` / `h` | Previous month |
| `→` / `l` | Next month |

### Add/Edit Form
| Key | Action |
|-----|--------|
| `Tab` | Next field |
| `Shift+Tab` | Previous field |
| `←` / `→` | Cycle dropdown options |
| `Space` | Toggle boolean |
| `Enter` | Save |
| `Esc` | Cancel |

## License

MIT
