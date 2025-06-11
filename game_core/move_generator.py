import numpy as np
from game_core.board import Board, Piece

class MoveGenerator():
    def __init__(self, board):
        self.board = board