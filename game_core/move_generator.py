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

        

                



    def is_in_check(self, color):
        pass

    def would_be_in_check(self, color):
        pass

    def get_piece_moves(self, row, col):
        pass 

    def _is_piece_white(self, piece):
        """
        Returns True if the piece is white, False otherwise. 
        """
        return piece <= 6 and piece != 0
    
    def _is_valid_coordinate(self, coord):
        """
        Returns True if the coordinate is valid.
        """
        return coord >= 0 and coord <= 7
