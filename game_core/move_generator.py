import numpy as np
from game_core.board import Board, Piece

class MoveGenerator():
    def __init__(self, board):
        self.board = board
        self.white_castling_rights = [True, True] # Queenside, Kingside
        self.black_castling_rights = [True, True] # Queenside, Kingside

        self.white_attacked_squares = [] # squares white pieces attack
        self.black_attacked_squares = [] # squares black pieces attack 

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
            if to_pos in legal_moves:
                if to_square and not pieces_same_color:
                    return True
                elif not to_square:
                    return True
                else:
                    return False
            
            # Castling
            if color == "black":
                if self.board.get_black_king_pos() == (7, 4):
                    # Black kingside castling 
                    if to_row == curr_row and to_col == curr_col + 2:
                        in_between_squares = [(7, 5), (7, 6)]
                        if any(self.board.get_piece(c) for c in in_between_squares):
                            return False
                        elif self.board.get_piece((7, 7)) == Piece.BLACK_ROOK and self.black_castling_rights[1]:
                            return True
                        else:
                            return False
                    # Black queenside castling
                    elif to_row == curr_row and to_col == curr_col - 2:
                        in_between_squares = [(7, 3), (7, 2), (7, 1)]
                        if any(self.board.get_piece(c) for c in in_between_squares):
                            return False
                        elif self.board.get_piece((7, 0)) == Piece.BLACK_ROOK and self.black_castling_rights[0]:
                            return True
                        else:
                            return False
                    else:
                        return False
                else:
                    return False
            elif color == "white":
                if self.board.get_black_white_pos() == (0, 4):
                    # White kingside castling 
                    if to_row == curr_row and to_col == curr_col + 2:
                        in_between_squares = [(0, 5), (0, 6)]
                        if any(self.board.get_piece(c) for c in in_between_squares):
                            return False
                        elif self.board.get_piece((0, 7)) == Piece.WHITE_ROOK and self.white_castling_rights[1]:
                            return True
                        else:
                            return False
                    # White queenside castling
                    elif to_row == curr_row and to_col == curr_col - 2:
                        in_between_squares = [(0, 3), (0, 2), (0, 1)]
                        if any(self.board.get_piece(c) for c in in_between_squares):
                            return False
                        elif self.board.get_piece((0, 0)) == Piece.WHITE_ROOK and self.black_castling_rights[0]:
                            return True
                        else:
                            return False
                    else:
                        return False
                else:
                    return False
            else:
                return False
        return False

    def is_in_check(self, color):
        """
        Returns whether the king is currently in check.
        Params:
            - color: "white" or "black", invalid otherwise
        """
        if color == "white":
            return self.board.get_white_king_pos() in self.black_attacked_squares
        elif color == "black":
            return self.board.get_black_king_pos() in self.white_attacked_squares
        else:
            raise ValueError("invalid color argument.")
    
    def get_attacked_squares(self, color, new_board):
        """
        Returns a list of the  squares attacked by color from a board position.
        """
        attacked_squares = {}
        attacked_squares.update(self._get_pawn_attacked_squares(color, new_board))
        attacked_squares.update(self._get_knight_attacked_squares(color, new_board))
        attacked_squares.update(self._get_bishop_attacked_squares(color, new_board))
        attacked_squares.update(self._get_queen_attacked_squares(color, new_board))
        attacked_squares.update(self._get_rook_attacked_squares(color, new_board))


    def _get_pawn_attacked_squares(self, color, new_board):
        """
        Returns list of squares attacked by pawns 
        """
        attacked_squares = []
        if color == "black":
            for pawn in new_board.get_black_pawn_pos():
                move1 = (pawn[0] - 1, pawn[1] + 1)
                move2 = (pawn[0] - 1, pawn[1] - 1)
                if self._is_valid_square(move1):
                    attacked_squares.append(move1)
                if self._is_valid_square(move2):
                    attacked_squares.append(move2)
        elif color == "white":
            for pawn in new_board.get_white_pawn_pos():
                move1 = (pawn[0] + 1, pawn[1] + 1)
                move2 = (pawn[0] + 1, pawn[1] - 1)
                if self._is_valid_square(move1):
                    attacked_squares.append(move1)
                if self._is_valid_square(move2):
                    attacked_squares.append(move2)
        else: 
            raise ValueError("Invalid color argument.")
        return attacked_squares
    
    def _get_knight_attacked_squares(self, color, new_board):
        """
        Returns list of squares attacked by knights of color.
        """
        attacked_squares = []
        if color == "black":
            knights = new_board.get_black_knight_pos()
        elif color == "white":
            knights = new_board.get_white_knight_pos()
        else:
            raise ValueError("Invalid color argument.")
        
        for knight in knights:
            curr_row, curr_col = knight
            legal_moves = [(curr_row + 2, curr_col + 1),
                            (curr_row + 2, curr_col - 1),
                            (curr_row + 1, curr_col + 2),
                            (curr_row + 1, curr_col - 2),
                            (curr_row - 2, curr_col + 1),
                            (curr_row - 2, curr_col - 1),
                            (curr_row - 1, curr_col + 2),
                            (curr_row - 1, curr_col - 2)]
            for move in legal_moves:
                if self._is_valid_square(move):
                    attacked_squares.append(move)

        return attacked_squares
    
    def _get_bishop_attacked_squares(self, color, new_board):
        """
        Returns list of squares attacked by bishops of color. 
        """
        attacked_squares = []
        if color == "black":
            bishops = new_board.get_black_bishop_pos()
        elif color == "white":
            bishops = new_board.get_white_bishop_pos()
        else:
            raise ValueError("Invalid color argument")
        
        directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)]
        attacked_squares = self._rook_bishop_queen_attacks_helper(new_board, bishops, directions)
        return attacked_squares
    
    def _get_rook_attacked_squares(self, color, new_board):
        """
        Return list of squares attacked by rooks of color
        """
        if color == "white":
            rooks = new_board.get_white_rook_pos()
        elif color == "black":
            rooks = new_board.get_black_rook_pos()
        else:
            raise ValueError("Invalid color argument.")

        directions = [(1, 0), (-1, 0), (0, 1), (0, -1)]
        attacked_squares = self._rook_bishop_queen_attacks_helper(new_board, rooks, directions)
        return attacked_squares
    
    def _get_queen_attacked_squares(self, color, new_board):
        """
        Returns the squares attacked by color's queen.
        """
        if color == "white":
            queen = [new_board.get_white_queen_pos()]
        elif color == "black":
            queen = [new_board.get_black_queen_pos()]
        else:
            raise ValueError("Invalid color argument.")

        directions = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)]
        attacked_squares = self._rook_bishop_queen_attacks_helper(new_board, queen, directions)
        return attacked_squares
    
    def _get_king_attacked_squares(self, color, new_board):
        """
        Returns which squares are attacked by color's king.
        """
        if color == "white":
            king = new_board.get_white_king_pos()
        elif color == "black":
            king = new_board.get_black_king_pos()
        else:
            raise ValueError("Invalid color argument")
        attacked_squares = []
        curr_row, curr_col = king 
        legal_moves = [(curr_row + 1, curr_col + 1),
                (curr_row + 1, curr_col),
                (curr_row + 1, curr_col - 1),
                (curr_row, curr_col + 1),
                (curr_row, curr_col - 1),
                (curr_row - 1, curr_row + 1),
                (curr_row - 1, curr_col),
                (curr_row - 1, curr_col - 1)]
        for move in legal_moves:
            if self._is_valid_square(move):
                attacked_squares.append(move)

        return attacked_squares
        
    def _rook_bishop_queen_attacks_helper(self, new_board, pieces, directions):
        """
        Helper function that takes in directions to explore
        """
        attacked_squares = []
        for piece in pieces:
            curr_row, curr_col = piece
            direction_flags = [True for _ in range(len(directions))]
            for i in range(1, 8):
                for j, direction in enumerate(directions):
                    if not direction_flags[j]:
                        continue
                    row_indic, col_indic = direction
                    new_square = (curr_row + row_indic * i, curr_col + col_indic * i)
                    if self._is_valid_square(new_square):
                        attacked_squares.append(new_square)
                    if new_board.get_piece(new_square):
                        direction_flags[j] = False
        return attacked_squares


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
    
    def _is_valid_square(self, square):
        """
        Returns true if the square is a valid square on the board.
        """
        coord1, coord2 = square
        return self._is_valid_coordinate(coord1) and self._is_valid_coordinate(coord2)
    
    def _sign(self, x):
        """
        Returns 1 if x > 0, -1 if x < 0, 0 if x = 0. 
        """
        return (x > 0) - (x < 0)
