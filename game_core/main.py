import pygame
from game import ChessGame
from board import Piece

# Constants
WIDTH, HEIGHT = 784, 784
TILE_SIZE = 784 // 8 

# Load assets
def load_piece_images():
    pieces = {}
    names = {
        Piece.WHITE_PAWN: "white-pawn",
        Piece.WHITE_KNIGHT: "white-knight",
        Piece.WHITE_BISHOP: "white-bishop",
        Piece.WHITE_ROOK: "white-rook",
        Piece.WHITE_QUEEN: "white-queen",
        Piece.WHITE_KING: "white-king",
        Piece.BLACK_PAWN: "black-pawn",
        Piece.BLACK_KNIGHT: "black-knight",
        Piece.BLACK_BISHOP: "black-bishop",
        Piece.BLACK_ROOK: "black-rook",
        Piece.BLACK_QUEEN: "black-queen",
        Piece.BLACK_KING: "black-king",
    }
    for key, name in names.items():
        image = pygame.image.load(f"pieces-basic-png/{name}.png")
        pieces[key] = pygame.transform.scale(image, (TILE_SIZE, TILE_SIZE))
    return pieces

def draw_board(screen, game, piece_images, skip_pos=None):
    screen.blit(pygame.image.load("boards-png/rect-8x8.png"), (0, 0))
    for row in range(8):
        for col in range(8):
            if skip_pos == (row, col):
                continue  # Skip drawing dragged piece from original square
            piece = game.board.get_piece((row, col))
            if piece:
                img = piece_images.get(piece)
                if img:
                    screen.blit(img, (col * TILE_SIZE, row * TILE_SIZE))

def draw_game_over(screen, winner):
    font = pygame.font.SysFont("Arial", 48, bold=True)
    text = f"{winner.capitalize()} wins!"
    if winner is None:
        text = "Draw!"
    rendered = font.render(text, True, (255, 0, 0))
    rect = rendered.get_rect(center=(WIDTH // 2, HEIGHT // 2))
    screen.blit(rendered, rect)

# Main loop
def main():
    pygame.init()
    screen = pygame.display.set_mode((WIDTH, HEIGHT))
    pygame.display.set_caption("Chess")
    clock = pygame.time.Clock()

    game = ChessGame()
    piece_images = load_piece_images()

    dragging = False
    drag_piece = None
    start_pos = None

    while True:
        draw_board(screen, game, piece_images, skip_pos=start_pos if dragging else None)

        if game.is_game_over():
            draw_game_over(screen, game.winner)

        if dragging and drag_piece:
            x, y = pygame.mouse.get_pos()
            screen.blit(drag_piece, (x - TILE_SIZE // 2, y - TILE_SIZE // 2))

        pygame.display.flip()
        clock.tick(60)

        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                pygame.quit()
                return

            elif event.type == pygame.MOUSEBUTTONDOWN:
                col = event.pos[0] // TILE_SIZE
                row = event.pos[1] // TILE_SIZE
                piece = game.board.get_piece((row, col))
                if piece and game.get_player().startswith('white') == (piece < 7):  # crude turn check
                    dragging = True
                    drag_piece = piece_images[piece]
                    start_pos = (row, col)

            elif event.type == pygame.MOUSEBUTTONUP and dragging:
                col = event.pos[0] // TILE_SIZE
                row = event.pos[1] // TILE_SIZE
                end_pos = (row, col)
                try:
                    game.make_move(start_pos, end_pos)
                except Exception as e:
                    print("Illegal move:", e)
                dragging = False
                drag_piece = None
                start_pos = None

if __name__ == "__main__":
    main()
