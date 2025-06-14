import numpy as np
from enum import IntEnum
import copy

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

        self.piece_positions = {
            Piece.WHITE_PAWN: {(1, i) for i in range(8)},
            Piece.WHITE_KNIGHT: {(0, 1), (0, 6)},
            Piece.WHITE_BISHOP: {(0, 2), (0, 5)},
            Piece.WHITE_ROOK: {(0, 0), (0, 7)},
            Piece.WHITE_QUEEN: {(0, 3)},
            Piece.WHITE_KING: {(0, 4)},
            Piece.BLACK_PAWN: {(6, i) for i in range(8)},
            Piece.BLACK_KNIGHT: {(7, 1), (7, 6)},
            Piece.BLACK_BISHOP: {(7, 2), (7, 5)},
            Piece.BLACK_ROOK: {(7, 0), (7, 7)}, 
            Piece.BLACK_QUEEN: {(7, 3)},
            Piece.BLACK_KING: {(7, 4)}
        }

        
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
    
    def move(self, from_pos, to_pos, in_place=True):
        """
        Move piece from from_pos to to_pos
        """
        if in_place:
            curr_piece = self.board[from_pos[0]][from_pos[1]]
            if not curr_piece:
                raise ValueError("Invalid position - no piece found")
            to_piece = self.get_piece(to_pos)
            if to_piece:
                self.piece_positions[to_piece].remove(to_pos)

            self.piece_positions[curr_piece].remove(from_pos)
            self.piece_positions[curr_piece].add(to_pos)

            self._set_piece(from_pos, Piece.EMPTY)
            self._set_piece(to_pos, curr_piece)
        else:
            new_board = Board()
            new_board.board = self.board.copy()
            new_board.piece_positions = { piece: {pos for pos in positions} if isinstance(positions, set) else {positions} 
                                         for piece, positions in self.piece_positions.items() }

            curr_piece = new_board.get_piece(from_pos)
            if not curr_piece:
                raise ValueError("Invalid position - no piece found")
            to_piece = new_board.get_piece(to_pos)
            if to_piece:
                new_board.piece_positions[to_piece].remove(to_pos)

            new_board.piece_positions[curr_piece].remove(from_pos)
            new_board.piece_positions[curr_piece].add(to_pos)

            new_board._set_piece(from_pos, Piece.EMPTY, new_board)
            new_board._set_piece(to_pos, curr_piece, new_board)
            return new_board
    
    def _set_piece(self, square, piece, board=None):
        """
        Set piece at square 
        """
        row, col = square
        if board is not None:
            board.board[row][col] = piece
        else:
            self.board[row, col] = piece
    
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

    def get_white_king_pos(self):
        return self.piece_positions[Piece.WHITE_KING]
    
    def get_white_queen_pos(self):
        return self.piece_positions[Piece.WHITE_QUEEN]
    
    def get_white_rook_pos(self):
        return self.piece_positions[Piece.WHITE_ROOK]
    
    def get_white_bishop_pos(self):
        return self.piece_positions[Piece.WHITE_BISHOP]
    
    def get_white_knight_pos(self):
        return self.piece_positions[Piece.WHITE_KNIGHT]
    
    def get_white_pawn_pos(self):
        return self.piece_positions[Piece.WHITE_PAWN]

    def get_black_king_pos(self):
        return self.piece_positions[Piece.BLACK_KING]
    
    def get_black_queen_pos(self):
        return self.piece_positions[Piece.BLACK_QUEEN]
    
    def get_black_rook_pos(self):
        return self.piece_positions[Piece.BLACK_ROOK]
    
    def get_black_bishop_pos(self):
        return self.piece_positions[Piece.BLACK_BISHOP]
    
    def get_black_knight_pos(self):
        return self.piece_positions[Piece.BLACK_KNIGHT]
    
    def get_black_pawn_pos(self):
        return self.piece_positions[Piece.BLACK_PAWN]
    
    def get_board(self):
        return self.board
    

if __name__ == "__main__":
    x = Board()
    x.display()