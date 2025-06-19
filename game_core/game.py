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
    
    def make_move(self, from_square, to_square):
        """
        Makes given move if it is legal, otherwise raises ValueError. 
        Switches current player after move is made.
        """
        if not self.move_gen.is_legal_move(from_square, to_square, self.current_player):
            raise ValueError("Illegal move given.")
        
        legal_moves, checkmate, draw = self.move_gen.get_legal_moves(self.current_player)
        if checkmate:
            self.game_over = True
            self.winner = "white" if self.current_player == "black" else "black"
        elif draw:
            self.game_over = True
            self.winner = "draw"

        print(f"Checkmate = {checkmate}, Draw = {draw}, legal_moves = {legal_moves}")
        self.board.move(from_square, to_square, in_place=True)

        if self.current_player == "white":
            self.current_player = "black"
        else:
            self.current_player = "white"

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
    legal_moves, checkmate = game.move_gen.get_legal_moves("white")
    print(legal_moves)
    print(checkmate)
    game.make_move((1, 3), (2, 3))
    legal_moves, checkmate = game.move_gen.get_legal_moves("black")
    print(legal_moves)
    print(checkmate)



