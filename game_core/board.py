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

        self.white_king_pos = (0, 4)
        self.white_pawn_pos = {(1, i) for i in range(8)}
        self.white_rook_pos = {(0, 0), (0, 7)}
        self.white_knight_pos = {(0, 1), (0, 6)}
        self.white_bishop_pos = {(0, 2), (0, 5)}
        self.white_queen_pos = (0, 3)

        self.black_king_pos = (7, 4)
        self.white_pawn_pos = {(6, i) for i in range(8)}
        self.white_rook_pos = {(7, 0), (7, 7)}
        self.white_knight_pos = {(7, 1), (7, 6)}
        self.white_bishop_pos = {(7, 2), (7, 5)}
        self.white_queen_pos = (7, 3)

        
    def _setup_starting_position(self):
        """
        Sets up the initial position of the board
        """
        self.board[7] = [Piece.BLACK_ROOK, Piece.BLACK_KNIGHT, Piece.BLACK_BISHOP, Piece.BLACK_QUEEN, 
                         Piece.BLACK_KING, Piece.BLACK_BISHOP, Piece.BLACK_KNIGHT, Piece.BLACK_ROOK]
        self.board[6] = [Piece.BLACK_PAWN] * 8
        self.board[0] = [Piece.WHITE_ROOK, Piece.WHITE_KNIGHT, Piece.WHITE_BISHOP, Piece.WHITE_QUEEN, 
                         Piece.WHITE_KING, Piece.WHITE_BISHOP, Piece.WHITE_KNIGHT, Piece.WHITE_ROOK]
        self.board[1] = [Piece.WHITE_PAWN] * 8

    def get_piece(self, coord):
        """
        Returns piece at postion [row, col]
        """
        row, col = coord
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

    def update_board(self, move, in_place=True):
        """
        Update the board with a given move. Doesn't handle checking if the move is
        illegal. If in place, just update the actual board, otherwise return it.
        """
        from_pos, to_pos = move
        curr_piece = self.get_piece(from_pos)

    def square_to_indices(self, square):
        """
        Takes square such as "h4" and transforms it into indices for the board
        """
        pass

    def get_white_king_pos(self):
        return self.white_king_pos
    
    def get_white_queen_pos(self):
        return self.white_queen_pos
    
    def get_white_rook_pos(self):
        return self.white_rook_pos
    
    def get_white_bishop_pos(self):
        return self.white_bishop_pos
    
    def get_white_knight_pos(self):
        return self.white_knight_pos
    
    def get_white_pawn_pos(self):
        return self.white_pawn_pos

    def get_black_king_pos(self):
        return self.black_king_pos
    
    def get_black_queen_pos(self):
        return self.black_queen_pos
    
    def get_black_rook_pos(self):
        return self.black_rook_pos
    
    def get_black_bishop_pos(self):
        return self.black_bishop_pos
    
    def get_black_knight_pos(self):
        return self.black_knight_pos
    
    def get_black_pawn_pos(self):
        return self.black_pawn_pos
    

if __name__ == "__main__":
    x = Board()
    x.display()