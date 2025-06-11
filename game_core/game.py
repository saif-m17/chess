import numpy as np
from game_core.board import Piece, Board

class ChessGame():
    def __init__(self):
        self.board = Board()
        self.current_player = "white"
        self.game_over = False
        self.winner = None
    
    def make_move(self, from_square, to_square):
        pass

    def get_legal_moves(self):
        pass

    def is_game_over(self):
        return self.game_over
    
    def reset(self):
        pass

    def display(self):
        self.board.display()
