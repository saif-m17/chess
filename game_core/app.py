from flask import Flask, jsonify, request
from flask_cors import CORS
from game_core.board import Board
from game_core.move_generator import MoveGenerator
from game_core.game import ChessGame

app = Flask(__name__)
CORS(app)

game = ChessGame()

@app.route('api/board', methods=['GET'])
def get_board():
    return jsonify({
        'board': game.board.get_board(),
        'current_player': game.get_player(),
        'game_over': game.is_game_over()
    })


