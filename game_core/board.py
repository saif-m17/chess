import numpy as np
from enum import IntEnum

class Piece(IntEnum):
    EMPTY = 0
    WHITE_PAWN = 1
    WHITE_KNIGHT = 2
    WHITE_BISHOP = 3
    WHITE_ROOK = 4
    WHITE_QUEEN = 5
    WHITE_KING = 6
    BLACK_PAWN = 7
    BLACK_KNIGHT = 8
    BLACK_BISHOP = 9
    BLACK_ROOK = 10
    BLACK_QUEEN = 11
    BLACK_KING = 12

class Board():
    def __init__(self):
        self.board = np.zeros((8,8), dtype=np.int8)
        self._setup_starting_position()
        self.white_king_pos = (7, 4)
        self.black_king_pos = (0, 4)
        
    def _setup_starting_position(self):
        """
        Sets up the initial position of the board
        """
        self.board[7] = [Piece.BLACK_ROOK, Piece.BLACK_KNIGHT, Piece.BLACK_BISHOP, Piece.BLACK_QUEEN, Piece.BLACK_KING, Piece.BLACK_BISHOP, Piece.BLACK_KNIGHT, Piece.BLACK_ROOK]
        self.board[6] = [Piece.BLACK_PAWN] * 8
        self.board[0] = [Piece.WHITE_ROOK, Piece.WHITE_KNIGHT, Piece.WHITE_BISHOP, Piece.WHITE_QUEEN, Piece.WHITE_KING, Piece.WHITE_BISHOP, Piece.WHITE_KNIGHT, Piece.WHITE_ROOK]
        self.board[1] = [Piece.WHITE_PAWN] * 8

    def get_piece(self, row, col):
        """
        Returns piece at postion [row, col]
        """
        return self.board[row][col]
    
    def set_piece(self, piece, row, col):
        """
        Set piece at position row, col
        """
        if piece == Piece.WHITE_KING:
            self.white_king_pos = (row, col)
        elif piece == Piece.BLACK_KING:
            self.black_king_pos = (row, col)
        
        self.board[row][col] = piece
    
    def display(self):
        """
        Print the current board
        """
        black_pieces = ['♙', '♘', '♗', '♖', '♕', '♔']
        white_pieces = ['♟', '♞', '♝', '♜', '♛', '♚']
        unicode_pieces = ['.'] + white_pieces + black_pieces

        for i, row in enumerate(reversed(self.board)):
            print(f"{8-i} {' '.join(unicode_pieces[piece] for piece in row)}")
        print("  a b c d e f g h")

    def square_to_indices(self, square):
        """
        Takes square such as "h4" and transforms it into indices for the board
        """
        pass 

if __name__ == "__main__":
    x = Board()
    x.display()