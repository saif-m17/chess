import numpy as np
from board import Piece, Board
from move_generator import MoveGenerator

class ChessGame():
    def __init__(self):
        self.board = Board()
        self.move_gen = MoveGenerator(self.board)
        self.current_player = "white"
        self.game_over = False
        self.winner = None
    
    def make_move(self, from_square, to_square, promote_piece=None):
        """
        Makes given move if it is legal, otherwise raises ValueError. 
        Switches current player after move is made.
        """
        if not self.move_gen.is_legal_move(from_square, to_square, self.current_player, promote_piece):
            raise ValueError("Illegal move given.")
        
        piece = self.board.get_piece(from_square)
        self.board.move(from_square, to_square, in_place=True, promote_piece=promote_piece)
        
        if piece == Piece.WHITE_KING:
            self.move_gen.update_castling_rights("white", -1)  # Both sides
        elif piece == Piece.BLACK_KING:
            self.move_gen.update_castling_rights("black", -1)  # Both sides
        elif piece == Piece.WHITE_ROOK:
            if from_square == (0, 0):  # Queenside rook
                self.move_gen.update_castling_rights("white", 0)
            elif from_square == (0, 7):  # Kingside rook
                self.move_gen.update_castling_rights("white", 1)
        elif piece == Piece.BLACK_ROOK:
            if from_square == (7, 0):  # Queenside rook
                self.move_gen.update_castling_rights("black", 0)
            elif from_square == (7, 7):  # Kingside rook
                self.move_gen.update_castling_rights("black", 1)
        
        if self.current_player == "white":
            self.current_player = "black"
        else:
            self.current_player = "white"
        
        _, checkmate, draw = self.move_gen.get_legal_moves(self.current_player)
        if checkmate:
            self.game_over = True
            self.winner = "white" if self.current_player == "black" else "black"
        elif draw:
            self.game_over = True
            self.winner = "draw"
        

    def get_legal_moves(self):
        """
        Returns legal moves for the current player. 
        """
        return self.move_gen.get_legal_moves(self.current_player)

    def is_game_over(self):
        """
        Returns whether game is over. 
        """
        return self.game_over
    
    def reset(self):
        """
        Reset game. 
        """
        self.board = Board()
        self.move_gen = MoveGenerator(self.board)
        self.current_player = "white"
        self.game_over = False
        self.winner = None

    def display(self):
        self.board.display()
    
    def get_player(self):
        return self.current_player
    
if __name__ == "__main__":
    game = ChessGame()
    legal_moves, checkmate, draw = game.move_gen.get_legal_moves("white")
    game.display()
    print(legal_moves)
    print(checkmate)
    game.make_move((1, 3), (3, 3))
    legal_moves, checkmate, draw = game.move_gen.get_legal_moves("black")
    game.display()

    game.make_move((6, 2), (5, 2))
    game.display()

    game.make_move((3, 3), (4, 3))
    game.display()

    game.make_move((5, 2), (4, 2))
    game.display()

    game.make_move((4, 3), (5, 3))
    game.display()



