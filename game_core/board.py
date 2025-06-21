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

        self.prev_white_moves = []
        self.prev_black_moves = []
        
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
    
    def move(self, from_pos, to_pos, in_place=True, promote_piece=None):
        """
        Move piece from from_pos to to_pos
        """
        if in_place:
            curr_piece = self.board[from_pos[0]][from_pos[1]]
            if not curr_piece:
                raise ValueError("Invalid position - no piece found")
            
            if self._is_piece_white(curr_piece):
                self.prev_white_moves.append((from_pos, to_pos))
            elif self._is_piece_black(curr_piece):
                self.prev_black_moves.append((from_pos, to_pos))
            
            is_castling = ((curr_piece == Piece.WHITE_KING or curr_piece == Piece.BLACK_KING) and 
                      abs(to_pos[1] - from_pos[1]) == 2 and 
                      to_pos[0] == from_pos[0])
            
            is_en_passent = ((curr_piece == Piece.WHITE_PAWN and from_pos[0] == 4 and to_pos[0] == 5 and not 
                              self.get_piece(to_pos) and self.get_piece((4, to_pos[1]))) or
                             (curr_piece == Piece.BLACK_PAWN and from_pos[0] == 3 and to_pos[0] == 2 and not 
                              self.get_piece(to_pos) and self.get_piece((3, to_pos[1]))))

            is_promotion = ((curr_piece == Piece.WHITE_PAWN and to_pos[0] == 7) or 
                       (curr_piece == Piece.BLACK_PAWN and to_pos[0] == 0))
            
            if is_castling:
                self._handle_castling_move(from_pos, to_pos, curr_piece)
            elif is_en_passent:
                self._handle_en_passent(from_pos, to_pos, curr_piece)
            elif is_promotion:
                self._handle_promotion(from_pos, to_pos, curr_piece, promote_piece)
            else:
                to_piece = self.get_piece(to_pos)
                if to_piece:
                    self.piece_positions[to_piece].remove(to_pos)

                self.piece_positions[curr_piece].remove(from_pos)
                self.piece_positions[curr_piece].add(to_pos)

                self._set_piece(from_pos, Piece.EMPTY)
                self._set_piece(to_pos, curr_piece)
        else:
            new_board = copy.deepcopy(self)

            new_board.piece_positions = {}
            for piece, positions in self.piece_positions.items():
                new_board.piece_positions[piece] = set(positions)

            curr_piece = new_board.get_piece(from_pos)
            if not curr_piece:
                raise ValueError("Invalid position - no piece found")
            
            if new_board._is_piece_white(curr_piece):
                new_board.prev_white_moves.append((from_pos, to_pos))
            elif new_board._is_piece_black(curr_piece):
                new_board.prev_black_moves.append((from_pos, to_pos))
            
            is_castling = ((curr_piece == Piece.WHITE_KING or curr_piece == Piece.BLACK_KING) and 
                      abs(to_pos[1] - from_pos[1]) == 2 and 
                      to_pos[0] == from_pos[0])
            
            is_en_passent = ((curr_piece == Piece.WHITE_PAWN and from_pos[0] == 4 and to_pos[0] == 5 and not 
                              new_board.get_piece(to_pos) and new_board.get_piece((4, to_pos[1]))) or
                             (curr_piece == Piece.BLACK_PAWN and from_pos[0] == 3 and to_pos[0] == 2 and not 
                              new_board.get_piece(to_pos) and new_board.get_piece((3, to_pos[1]))))
            
            is_promotion = ((curr_piece == Piece.WHITE_PAWN and to_pos[0] == 7) or 
                       (curr_piece == Piece.BLACK_PAWN and to_pos[0] == 0))
            
            if is_castling:
                new_board._handle_castling_move(from_pos, to_pos, curr_piece)
            elif is_en_passent:
                new_board._handle_en_passent(from_pos, to_pos, curr_piece)
            elif is_promotion:
                new_board._handle_promotion(from_pos, to_pos, curr_piece, promote_piece)
            else:
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
            self.board[row][col] = piece

    def _handle_castling_move(self, king_from, king_to, king_piece):
        """
        Helper function to move the king and rook during castling.
        """
        king_from_row, king_from_col = king_from
        king_to_row, king_to_col = king_to
        
        if king_to_col > king_from_col:
            # Kingside castling
            rook_from = (king_from_row, 7)  
            rook_to = (king_to_row, king_to_col - 1) 
        else:
            # Queenside castling
            rook_from = (king_from_row, 0) 
            rook_to = (king_to_row, king_to_col + 1)  
        
        # Get the rook piece
        rook_piece = self.get_piece(rook_from)
        
        # Move the king
        self.piece_positions[king_piece].remove(king_from)
        self.piece_positions[king_piece].add(king_to)
        self._set_piece(king_from, Piece.EMPTY)
        self._set_piece(king_to, king_piece)
        
        # Move the rook
        self.piece_positions[rook_piece].remove(rook_from)
        self.piece_positions[rook_piece].add(rook_to)
        self._set_piece(rook_from, Piece.EMPTY)
        self._set_piece(rook_to, rook_piece)

    def _handle_en_passent(self, pawn_from, pawn_to, pawn_piece):
        """
        Appropriately handles making the move en passent. 
        """
        is_white = self._is_piece_white(pawn_piece)
        if is_white:
            coords = (4, pawn_to[1])
            self._set_piece(coords, Piece.EMPTY)
            self.piece_positions[Piece.BLACK_PAWN].remove(coords)
        else:
            coords = (3, pawn_to[1])
            self._set_piece(coords, Piece.EMPTY)
            self.piece_positions[Piece.WHITE_PAWN].remove(coords)
        self.piece_positions[pawn_piece].remove(pawn_from)
        self.piece_positions[pawn_piece].add(pawn_to)
        self._set_piece(pawn_from, Piece.EMPTY)
        self._set_piece(pawn_to, pawn_piece)

    def _handle_promotion(self, pawn_from, pawn_to, pawn_piece, promote_piece):
        """
        Handles promotions.
        """    
        if self._is_piece_white(pawn_piece):
            if promote_piece not in [Piece.WHITE_QUEEN, Piece.WHITE_ROOK, Piece.WHITE_BISHOP, Piece.WHITE_KNIGHT]:
                raise ValueError("Invalid promotion piece for white pawn")
        else:
            if promote_piece not in [Piece.BLACK_QUEEN, Piece.BLACK_ROOK, Piece.BLACK_BISHOP, Piece.BLACK_KNIGHT]:
                raise ValueError("Invalid promotion piece for black pawn")
        
        to_piece = self.get_piece(pawn_to)
        if to_piece:
            self.piece_positions[to_piece].remove(pawn_to)
        
        self.piece_positions[pawn_piece].remove(pawn_from)
        
        if promote_piece not in self.piece_positions:
            self.piece_positions[promote_piece] = set()
        self.piece_positions[promote_piece].add(pawn_to)
        
        self._set_piece(pawn_from, Piece.EMPTY)
        self._set_piece(pawn_to, promote_piece)
    
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

    def indices_to_square(self, square):
        """
        Takes board indices and turns them into chess notation (i.e. h4)
        """
        row, col = square
        return f"{chr(col + ord('a'))}{8-row}"
    
    def get_previous_move(self, color):
        """
        Returns previous move of color.
        """
        if color == "white":
            return self.prev_white_moves[-1]
        elif color == "black":
            return self.prev_black_moves[-1]
        else:
            raise ValueError("Invalid color argument.")
        
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
    
    def _is_piece_white(self, piece):
        """
        Returns True if the piece is white, False otherwise. 
        """
        if piece < 0 or piece > 12:
            raise ValueError("Invalid Piece")
        return piece <= 6 and piece != 0
    
    def _is_piece_black(self, piece):
        """
        Returns True if the piece is Black, False otherwise. 
        """
        if piece < 0 or piece > 12:
            raise ValueError("Invalid Piece")
        return piece <= 12 and piece > 6

if __name__ == "__main__":
    x = Board()
    x.display()