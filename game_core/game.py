import numpy as np
from board import Piece, Board
from move_generator import MoveGenerator

class ChessGame():
    def __init__(self):
        self.board = Board()
        self.move_gen = MoveGenerator(self.board)
        self.current_player = "white"
        self.game_over = False
        self.winner = None
    
    def make_move(self, from_square, to_square):
        if not self.move_gen.is_legal_move(from_square, to_square, self.current_player):
            raise ValueError("Illegal move given.")
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
    
if __name__ == "__main__":
    game = ChessGame()
    game.display()
    game.make_move((1, 4), (3, 4))
    game.display()
    game.make_move((6, 3), (4, 3))
    game.display()
    game.make_move((3, 4), (4, 3))
    game.display()

