from board import Piece

class MoveGenerator():
    def __init__(self, board):
        self.board = board
        self.white_castling_rights = [True, True] # Queenside, Kingside
        self.black_castling_rights = [True, True] # Queenside, Kingside

        self.white_attacked_squares = [] # squares white pieces attack
        self.black_attacked_squares = [] # squares black pieces attack 

    def get_legal_moves(self, color):
        """
        Returns a dictionary of legal moves where key is the position of the
        starting square, and the values are a list of all possible places that
        piece can move to. Returns a boolean of whether player is in checkmate. 
        Params:
            - color: "white" or "black"
            - new_board: 
        """
        legal_moves = {}
        legal_moves.update(self._get_pawn_attacked_squares(color, self.board, True))
        legal_moves.update(self._get_bishop_attacked_squares(color, self.board, True))
        legal_moves.update(self._get_knight_attacked_squares(color, self.board, True))
        legal_moves.update(self._get_rook_attacked_squares(color, self.board, True))
        legal_moves.update(self._get_queen_attacked_squares(color, self.board, True))
        legal_moves.update(self._get_king_attacked_squares(color, self.board, True))

        legal_moves_final = {}

        for from_pos, to_positions in legal_moves.items():
            legal_moves_final[from_pos] = set()
            for to_pos in to_positions:
                updated_board = self.board.move(from_pos, to_pos, in_place=False)
                if not self.is_in_check(updated_board, color):
                    legal_moves_final[from_pos].add(to_pos)

        no_legal_moves = all(len(piece_moves) == 0 for piece_moves in legal_moves_final.values())

        checkmate = self.is_in_check(self.board, color) and no_legal_moves
        draw = not self.is_in_check(self.board, color) and no_legal_moves
        return legal_moves_final, checkmate, draw

    def is_legal_move(self, from_pos, to_pos, color):
        """
        Determines whether a move is legal.
        Params:
            - from_pos: tuple (row, col) of starting position
            - to_pos: tuple (row, col) of ending position
            - color: "white" or "black"
        """
        legal_ignoring_checks = self._is_legal_move_ignoring_checks(from_pos, to_pos, color)
        updated_board = self.board.move(from_pos, to_pos, in_place=False)
        in_check_after_move = self.is_in_check(updated_board, color)
        return legal_ignoring_checks and not in_check_after_move

    def _is_legal_move_ignoring_checks(self, from_pos, to_pos, color):
        """
        Checks whether move from from_pos to to_pos for player color is legal without
        considering checks.
        """
        # get individual coordinates 
        curr_row, curr_col = from_pos
        to_row, to_col = to_pos

        # if curr_poss is equal to new position, return false
        if curr_row == to_row and curr_col == to_col:
            return False

        # check if color of piece at initial position is correct
        curr_piece = self.board.get_piece(from_pos)
        to_square = self.board.get_piece(to_pos)
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
                elif curr_row == 1 and to_row == 3 and not self.board.get_piece((2,curr_col)):
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
                elif curr_row == 6 and to_row == 4 and not self.board.get_piece((5,curr_col)):
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
                if self.board.get_piece((curr_row + sign_row * i,curr_col + sign_col * i)):
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
            if curr_row == to_row and curr_col == to_col:
                return False 
            if curr_row != to_row and curr_col != to_col:
                return False
            if row == 0:
                num_moved_squares = abs(to_col - curr_col)
                sign_move = self._sign(to_col - curr_col)
                for i in range(1, num_moved_squares):
                    if self.board.get_piece((curr_row,curr_col + sign_move * i)):
                        return False
                if to_square:
                    return not pieces_same_color
                else:
                    return True
            elif row == 1:
                num_moved_squares = abs(to_row - curr_row)
                sign_move = self._sign(to_row - curr_row)
                for i in range(1, num_moved_squares):
                    if self.board.get_piece((curr_row + sign_move * i, curr_col)):
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
            else:
                return False
                
        # Both queen moves
        if curr_piece == Piece.WHITE_QUEEN or curr_piece == Piece.BLACK_QUEEN:
            if curr_row == to_row:
                return rook_legal_move(from_pos, to_pos, 0)
            elif curr_col == to_col:
                return rook_legal_move(from_pos, to_pos, 1)
            else:
                return bishop_legal_move(from_pos, to_pos)
        
        # Both king moves
        if curr_piece == Piece.WHITE_KING or curr_piece == Piece.BLACK_KING:
            legal_moves = [(curr_row + 1, curr_col + 1),
                           (curr_row + 1, curr_col),
                           (curr_row + 1, curr_col - 1),
                           (curr_row, curr_col + 1),
                           (curr_row, curr_col - 1),
                           (curr_row - 1, curr_col + 1),
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
                if next(iter(self.board.get_black_king_pos())) == (7, 4):
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
                if next(iter(self.board.get_white_king_pos())) == (0, 4):
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
                        elif self.board.get_piece((0, 0)) == Piece.WHITE_ROOK and self.white_castling_rights[0]:
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

    def is_in_check(self, new_board, color):
        """
        Returns whether the king is currently in check.
        Params:
            - new_board: board to use to check if the king is in check
            - color: "white" or "black", invalid otherwise
        """
        if color == "white":
            attacked_squares = self.get_attacked_squares("black", new_board)
            return next(iter(new_board.get_white_king_pos())) in attacked_squares 
        elif color == "black":
            attacked_squares = self.get_attacked_squares("white", new_board)
            return next(iter(new_board.get_black_king_pos())) in attacked_squares
        else:
            raise ValueError("invalid color argument.")
    
    def get_attacked_squares(self, color, new_board):
        """
        Returns a list of the  squares attacked by color from a board position.
        """
        attacked_squares = set()
        attacked_squares.update(self._get_pawn_attacked_squares(color, new_board))
        attacked_squares.update(self._get_knight_attacked_squares(color, new_board))
        attacked_squares.update(self._get_bishop_attacked_squares(color, new_board))
        attacked_squares.update(self._get_queen_attacked_squares(color, new_board))
        attacked_squares.update(self._get_rook_attacked_squares(color, new_board))
        attacked_squares.update(self._get_king_attacked_squares(color, new_board))
        return attacked_squares
    
    ################################################
    #                                              #
    #     Attacked Squares/Legal Moves Helpers     #
    #                                              #
    ################################################


    def _get_pawn_attacked_squares(self, color, new_board, sep=False):
        """
        Returns list of squares attacked by pawns
        Params: 
            - color: "white" or "black"
            - new_board: Board object we get the attacked squares from
            - sep: whether or not to separate out squares attacked by move, only
            include legal moves. 
        """
        attacked_squares = []
        legal_moves = {}

        if color == "black":
            for pawn in new_board.get_black_pawn_pos():
                move1 = (pawn[0] - 1, pawn[1] + 1)
                move2 = (pawn[0] - 1, pawn[1] - 1)

                m1_is_valid = self._is_valid_square(move1)
                m2_is_valid = self._is_valid_square(move2)
                if sep: # in progress
                    if m1_is_valid:
                        piece_1 = new_board.get_piece(move1)
                    if m2_is_valid:
                        piece_2 = new_board.get_piece(move2)
                    legal_moves[pawn] = set()
                    if m1_is_valid and self._is_piece_white(piece_1):
                        legal_moves[pawn].add(move1)
                    if m2_is_valid and self._is_piece_white(piece_2):
                        legal_moves[pawn].add(move2)

                    if pawn[0] == 6 and not new_board.get_piece((5, pawn[1])) and not new_board.get_piece((4, pawn[1])):
                        legal_moves[pawn].add((4, pawn[1]))
                    move3 = (pawn[0] -1, pawn[1])
                    if self._is_valid_square(move3) and not new_board.get_piece(move3):
                        legal_moves[pawn].add(move3)
                else:
                    if m1_is_valid:
                        attacked_squares.append(move1)
                    if m2_is_valid:
                        attacked_squares.append(move2)
        elif color == "white":
            for pawn in new_board.get_white_pawn_pos():
                move1 = (pawn[0] + 1, pawn[1] + 1)
                move2 = (pawn[0] + 1, pawn[1] - 1)
                
                m1_is_valid = self._is_valid_square(move1)
                m2_is_valid = self._is_valid_square(move2)
                if sep:
                    if m1_is_valid:
                        piece_1 = new_board.get_piece(move1)
                    if m2_is_valid:
                        piece_2 = new_board.get_piece(move2)
                    legal_moves[pawn] = set()
                    if m1_is_valid and self._is_piece_black(piece_1):
                        legal_moves[pawn].add(move1)
                    if m2_is_valid and self._is_piece_black(piece_2):
                        legal_moves[pawn].add(move2)

                    if pawn[0] == 1 and not new_board.get_piece((2, pawn[1])) and not new_board.get_piece((3, pawn[1])):
                        legal_moves[pawn].add((3, pawn[1]))
                    move3 = (pawn[0] + 1, pawn[1])
                    if self._is_valid_square(move3) and not new_board.get_piece(move3):
                        legal_moves[pawn].add(move3)
                else:
                    if m1_is_valid:
                        attacked_squares.append(move1)
                    if m2_is_valid:
                        attacked_squares.append(move2)
        else: 
            raise ValueError("Invalid color argument.")
        return legal_moves if sep else attacked_squares
    
    def _get_knight_attacked_squares(self, color, new_board, sep=False):
        """
        Returns list of squares attacked by knights of color.
        """
        attacked_squares = []
        legal_moves_map = {}
        if color == "black":
            knights = new_board.get_black_knight_pos()
        elif color == "white":
            knights = new_board.get_white_knight_pos()
        else:
            raise ValueError("Invalid color argument.")
        
        for knight in knights:
            curr_row, curr_col = knight
            if sep:
                legal_moves_map[knight] = set()
            legal_moves = [(curr_row + 2, curr_col + 1),
                            (curr_row + 2, curr_col - 1),
                            (curr_row + 1, curr_col + 2),
                            (curr_row + 1, curr_col - 2),
                            (curr_row - 2, curr_col + 1),
                            (curr_row - 2, curr_col - 1),
                            (curr_row - 1, curr_col + 2),
                            (curr_row - 1, curr_col - 2)]
            for move in legal_moves:
                if sep:
                    if self._is_valid_square(move) and self._get_color(self.board.get_piece(move)) != color:
                        legal_moves_map[knight].add(move)
                else:
                    if self._is_valid_square(move):
                        attacked_squares.append(move)
        if sep: 
            return legal_moves_map
        return attacked_squares
    
    def _get_bishop_attacked_squares(self, color, new_board, sep=False):
        """
        Returns list of squares attacked by bishops of color. 
        """
        if color == "black":
            bishops = new_board.get_black_bishop_pos()
        elif color == "white":
            bishops = new_board.get_white_bishop_pos()
        else:
            raise ValueError("Invalid color argument")
        
        directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)]
        return self._rook_bishop_queen_attacks_helper(new_board, bishops, directions, sep=sep, color=color)
    
    def _get_rook_attacked_squares(self, color, new_board, sep=False):
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
        return self._rook_bishop_queen_attacks_helper(new_board, rooks, directions, sep=sep, color=color)
    
    def _get_queen_attacked_squares(self, color, new_board, sep=False):
        """
        Returns the squares attacked by color's queen.
        """
        if color == "white":
            queen = list(new_board.get_white_queen_pos())
        elif color == "black":
            queen = list(new_board.get_black_queen_pos())
        else:
            raise ValueError("Invalid color argument.")

        directions = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)]
        return self._rook_bishop_queen_attacks_helper(new_board, queen, directions, sep=sep, color=color)
    
    def _get_king_attacked_squares(self, color, new_board, sep=False):
        """
        Returns which squares are attacked by color's king.
        """
        if color == "white":
            king = next(iter(new_board.get_white_king_pos()))
        elif color == "black":
            king = next(iter(new_board.get_black_king_pos()))
        else:
            raise ValueError("Invalid color argument")
        attacked_squares = []
        legal_moves_map = {}
        curr_row, curr_col = king 
        legal_moves = [(curr_row + 1, curr_col + 1),
                (curr_row + 1, curr_col),
                (curr_row + 1, curr_col - 1),
                (curr_row, curr_col + 1),
                (curr_row, curr_col - 1),
                (curr_row - 1, curr_col + 1),
                (curr_row - 1, curr_col),
                (curr_row - 1, curr_col - 1)]
        if sep:
            legal_moves_map[king] = set()
        for move in legal_moves:
            if sep:
                if self._is_valid_square(move) and self._get_color(self.board.get_piece(move)) != color:
                    legal_moves_map[king].add(move)
            else:
                if self._is_valid_square(move):
                    attacked_squares.append(move)
        
        return attacked_squares if not sep else legal_moves_map
        
    def _rook_bishop_queen_attacks_helper(self, new_board, pieces, directions, sep=False, color=None):
        """
        Helper function that takes in directions to explore
        """
        attacked_squares = []
        legal_moves = {}

        for piece in pieces:
            curr_row, curr_col = piece
            direction_flags = [True for _ in range(len(directions))]
            if sep:
                legal_moves[piece] = set()
            for i in range(1, 8):
                for j, direction in enumerate(directions):
                    row_indic, col_indic = direction
                    if not direction_flags[j]:
                        continue
                    elif not self._is_valid_square((curr_row + row_indic * i, curr_col + col_indic * i)):
                        direction_flags[j] = False
                        continue
                    new_square = (curr_row + row_indic * i, curr_col + col_indic * i)
                    if sep:
                        if not new_board.get_piece(new_square):
                            legal_moves[piece].add(new_square)
                        elif new_board.get_piece(new_square) and self._get_color(new_board.get_piece(new_square)) != color:
                            legal_moves[piece].add(new_square)
                            direction_flags[j] = False
                    else:
                        attacked_squares.append(new_square)
                        if new_board.get_piece(new_square):
                            direction_flags[j] = False

        return legal_moves if sep else attacked_squares

    #########################################
    #                                       #
    #   General Purpose Helper Functions    #
    #                                       #
    #########################################

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
    
    def _get_color(self, piece):
        """
        Returns "white" if piece is white, "black" if piece is black, and
        None if empty square.
        """
        if self._is_piece_white(piece):
            return "white"
        elif self._is_piece_black(piece):
            return "black"
        else:
            return None

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
    
    def update_castling_rights(self, color, side=0):
        """
        Updates castling rights for color king. 
        Params:
            - color: "white" or "black"
            - side: -1 for both, 0 for queenside, 1 for kingside
        """
        if color == "white":
            if side == -1:
                self.white_castling_rights = [False, False]
            else:
                self.white_castling_rights[side] = False
        else:
            if side == -1:
                self.black_castling_rights = [False, False]
            else:
                self.black_castling_rights[side] = False


