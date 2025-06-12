import numpy as np
from game_core.board import Piece, Board
from game_core.move_generator import MoveGenerator

class ChessGame():
    def __init__(self):
        self.board = Board()
        self.move_gen = MoveGenerator(self.board)
        self.current_player = "white"
        self.game_over = False
        self.winner = None
    
    def make_move(self, from_square, to_square):
        self.board.move(from_square, to_square, in_place=True)
        if self.current_player == "white":
            self.current_player = "black"
        else:
            self.current_player = "white"

    def get_legal_moves(self):
        pass

    def is_game_over(self):
        return self.game_over
    
    def reset(self):
        pass

    def display(self):
        self.board.display()
    
    def get_player(self):
        return self.current_player
