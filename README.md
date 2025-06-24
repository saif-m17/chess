# Chess

A Python-based chess game featuring a graphical interface built with Pygame.

## Features

- Interactive chess board with piece movement
- Bot opponent with intelligent move selection
- Clean graphical interface
- Full chess rule implementation
- Move validation and game state detection

## Prerequisites

- Python 3.7 or higher
- pip (Python package installer)

## Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/saif-m17/chess.git
   cd chess
   ```

2. **Install dependencies**
   ```bash
   pip install -r requirements.txt
   ```

## How to Run

Run the game by:

```bash
python -m game_core
```

## How to Play

1. Launch the game
2. The chess board will appear in a new window
3. Click on a piece to select it
4. Click on a valid destination square to move
5. Continue playing until checkmate or stalemate

## Controls

- **Mouse**: Click to select pieces and make moves
- **Window**: Close the window to exit the game

## Game Rules

The game follows standard chess rules including:
- Piece movement patterns
- Castling
- En passant
- Pawn promotion
- Check and checkmate detection
- Stalemate detection

## Project Structure

```
chess/
├── assets/              # Game assets
│   ├── boards-png/     # Chess board images
│   └── pieces-basic-png/ # Chess piece images
├── game_core/          # Main game logic
│   ├── __init__.py
│   ├── __main__.py     # Module entry point
│   ├── main.py         # Main game loop / pygame UI
│   ├── game.py         # Game logic
│   ├── board.py        # Board representation
│   └── move_generator.py # Move generation
├── requirements.txt    # Python dependencies
└── README.md
```

## Dependencies

The game requires the following Python packages (see `requirements.txt`):
- pygame - For graphics and user interface
- Additional dependencies as listed in requirements.txt

## Credits

Board and piece images downloaded from: https://greenchess.net/info.php?item=downloads