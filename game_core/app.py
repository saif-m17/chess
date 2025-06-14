from flask import Flask, jsonify, request, render_template, session
from flask_cors import CORS
from game_core.game import ChessGame

app = Flask(__name__)
CORS(app)

game = ChessGame()

@app.route('/')
def index():
    return render_template()

@app.route('api/board', methods=['GET'])
def get_board():
    piece_map = {1: "white-pawn", 2: "white-knight", 3: "white-bishop", 
                 4: "white-rook", 5: "white-queen", 6: "white-king", 
                 7: "black-pawn", 8: "black-knight", 9: "black-bishop",
                 10: "black-rook", 11: "black-queen", 12: "black-king" }
    board_data = []
    for piece, positions in game.board.piece_positions:
        for position in positions:
            row, col = position
            board_data.append({
                "piece": piece_map[piece],
                "position": f"{chr(col + ord('a'))}{8-row}"
            })
    return jsonify({
        "board": board_data,
        "player": game.get_player()
    })




