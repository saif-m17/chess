import numpy as np

# Pieces
EMPTY_SQUARE = 0

# White pieces
WHITE_PAWN = 1
WHITE_KNIGHT = 2
WHITE_BISHOP = 3
WHITE_ROOK = 4
WHITE_QUEEN = 5
WHITE_KING = 6

# Black Pieces
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
        self.board[0] = [BLACK_ROOK, BLACK_KNIGHT, BLACK_BISHOP, BLACK_QUEEN, BLACK_KING, BLACK_BISHOP, BLACK_KNIGHT, BLACK_ROOK]
        self.board[1] = [BLACK_PAWN] * 8
        self.board[7] = [WHITE_ROOK, WHITE_KNIGHT, WHITE_BISHOP, WHITE_QUEEN, WHITE_KING, WHITE_BISHOP, WHITE_KNIGHT, WHITE_ROOK]
        self.board[6] = [WHITE_PAWN] * 8

    def get_piece(self, row, col):
        """
        Returns piece at postion [row, col]
        """
        return self.board[row][col]
    
    def set_piece(self, piece, row, col):
        """
        Set piece at position row, col
        """
        if piece == WHITE_KING:
            self.white_king_pos = (row, col)
        elif piece == BLACK_KING:
            self.black_king_pos = (row, col)
        
        self.board[row][col] = piece
    
    def display(self):
        """
        Print the current board
        """
        black_pieces = ['♙', '♘', '♗', '♖', '♕', '♔']
        white_pieces = ['♟', '♞', '♝', '♜', '♛', '♚']
        unicode_pieces = ['.'] + white_pieces + black_pieces

        for i, row in enumerate(self.board):
            print(f"{8-i} {' '.join(unicode_pieces[piece] for piece in row)}")
        print("  a b c d e f g h")

if __name__ == "__main__":