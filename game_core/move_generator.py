import numpy as np
from game_core.board import Board, Piece

class MoveGenerator():
    def __init__(self, board):
        self.board = board
    
    def get_legal_moves(self, color):
        pass

    def is_legal_move(self, from_pos, to_pos, color):
        """
        Checks whether move from from_pos to to_pos for player color is legal.
        Params:
            - from_pos: tuple (row, col) of starting position
            - to_pos: tuple (row, col) of ending position
            - color: "white" or "black"
        """
        # get individual coordinates 
        curr_row, curr_col = from_pos
        to_row, to_col = to_pos

        # if curr_poss is equal to new position, return false
        if curr_row == to_row and curr_col == to_col:
            return False

        # check if color of piece at initial position is correct
        curr_piece = self.board[curr_row][curr_col]
        to_square = self.board[to_row][to_col]
        is_white = self._is_piece_white(curr_piece)
        if is_white and color == "black" or not is_white and color == "white":
            return False
        
        pieces_same_color = is_white == self._is_piece_white(to_square)
        
        # check if in range
        if not all(self._is_valid_coordinate(c) for c in [curr_row, curr_col, to_row, to_col]):
            return False 
        
        # White pawn moves, no en passent for now 
        if curr_piece == Piece.WHITE_PAWN:
            if curr_col == to_col and not to_square:
                if to_row == curr_row + 1:
                    return True
                elif curr_row == 1 and to_row == 3 and not self.board[2][curr_col]:
                    return True
            elif to_col == curr_col + 1 or to_col == curr_col - 1:
                if to_row != curr_row + 1:
                    return False
                elif to_square and not pieces_same_color:
                    return True
            else:
                return False
        
        # Black pawn moves, no en passent for now
        if curr_piece == Piece.BLACK_PAWN:
            if curr_col == to_col and not to_square:
                if to_row == curr_row - 1:
                    return True
                elif curr_row == 6 and to_row == 4 and not self.board[5][curr_col]:
                    return True
            elif to_col == curr_col + 1 or to_col == curr_col -1:
                if to_row != curr_row - 1:
                    return False
                elif to_square and not pieces_same_color:
                    return True
            else:
                return False
        
        # Both knight moves 
        if curr_piece == Piece.WHITE_KNIGHT or curr_piece == Piece.BLACK_KNIGHT:
            legal_moves = [(curr_row + 2, curr_col + 1),
                           (curr_row + 2, curr_col - 1),
                           (curr_row + 1, curr_col + 2),
                           (curr_row + 1, curr_col - 2),
                           (curr_row - 2, curr_col + 1),
                           (curr_row - 2, curr_col - 1),
                           (curr_row - 1, curr_col + 2),
                           (curr_row - 1, curr_col - 2)]
            if to_square:
                return to_pos in legal_moves and not pieces_same_color
            else:
                return to_pos in legal_moves
        
        # Both bishop moves
        def bishop_legal_move(from_pos, to_pos):
            """
            Returns True of this is a legal bishop move.
            """
            curr_row, curr_col = from_pos
            to_row, to_col = to_pos
            if abs(to_row - curr_row) != abs(to_col - curr_col):
                return False
            
            sign_row = self._sign(to_row - curr_row)
            sign_col = self._sign(to_col - curr_col)

            for i in range(1, abs(to_row - curr_row)):
                if self.board[curr_row + sign_row * i][curr_col + sign_col * i]:
                    return False
            if to_square:
                return not pieces_same_color
            else:
                return True

        if curr_piece == Piece.WHITE_BISHOP or curr_piece == Piece.BLACK_BISHOP:
            return bishop_legal_move(from_pos, to_pos)
            
        # Both rook moves
        def rook_legal_move(from_pos, to_pos, row=0):
            """
            Returns True if this is a legal rook move. If row = 0, assumes rows are the same for move,
            if row = 1, assumes columns are the same for the move.
            """
            curr_row, curr_col = from_pos
            to_row, to_col = to_pos
            if row == 0:
                num_moved_squares = abs(to_col - curr_col)
                sign_move = self._sign(to_col - curr_col)
                for i in range(1, num_moved_squares):
                    if self.board[curr_row][curr_col + sign_move * i]:
                        return False
                if to_square:
                    return not pieces_same_color
                else:
                    return True
            elif row == 1:
                num_moved_squares = abs(to_row - curr_row)
                sign_move = self._sign(to_row - curr_row)
                for i in range(1, num_moved_squares):
                    if self.board[curr_row + sign_move * i][curr_col]:
                        return False
                if to_square:
                    return not pieces_same_color
                else:
                    return True

        if curr_piece == Piece.WHITE_ROOK or curr_piece == Piece.BLACK_ROOK:
            if curr_row == to_row:
                return rook_legal_move(from_pos, to_pos, row=0)
            elif curr_col == to_col:
                return rook_legal_move(from_pos, to_pos, row=1)
                
        # Both queen moves
        if curr_piece == Piece.WHITE_QUEEN or curr_piece == Piece.BLACK_QUEEN:
            row_diff = to_row - curr_row
            col_diff = to_col - curr_col
            if row_diff == col_diff:
                return bishop_legal_move(from_pos, to_pos)
            elif row_diff == 0:
                return rook_legal_move(from_pos, to_pos, row=0)
            elif col_diff == 0:
                return rook_legal_move(from_pos, to_pos, row=1)
            else:
                return False
        
        # Both king moves (need to still handle checks)
        if curr_piece == Piece.WHITE_KING or curr_piece == Piece.BLACK_KING:
            legal_moves = [(curr_row + 1, curr_col + 1),
                           (curr_row + 1, curr_col),
                           (curr_row + 1, curr_col - 1),
                           (curr_row, curr_col + 1),
                           (curr_row, curr_col - 1),
                           (curr_row - 1, curr_row + 1),
                           (curr_row - 1, curr_col),
                           (curr_row - 1, curr_col - 1)]
            if to_square:
                return not pieces_same_color and to_pos in legal_moves
            else:
                return to_pos in legal_moves


            
    def is_in_check(self, color):
        return False 

    def would_be_in_check(self, color):
        pass

    def get_piece_moves(self, row, col):
        pass 

    def _is_piece_white(self, piece):
        """
        Returns True if the piece is white, False otherwise. 
        """
        if piece < 0 or piece > 12:
            raise ValueError("Invalid Piece")
        return piece <= 6 and piece != 0
    
    def _is_valid_coordinate(self, coord):
        """
        Returns True if the coordinate is valid.
        """
        return coord >= 0 and coord <= 7
    
    def _sign(self, x):
        """
        Returns 1 if x > 0, -1 if x < 0, 0 if x = 0. 
        """
        return (x > 0) - (x < 0)
