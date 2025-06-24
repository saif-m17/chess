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
        image = pygame.image.load(f"assets/pieces-basic-png/{name}.png")
        pieces[key] = pygame.transform.scale(image, (TILE_SIZE, TILE_SIZE))
    return pieces

def draw_board(screen, game, piece_images, skip_pos=None):
    screen.blit(pygame.image.load("assets/boards-png/rect-8x8.png"), (0, 0))
    for row in range(8):
        for col in range(8):
            if skip_pos == (row, col):
                continue
            piece = game.board.get_piece((row, col))
            if piece:
                img = piece_images.get(piece)
                if img:
                    screen.blit(img, (col * TILE_SIZE, row * TILE_SIZE))

def draw_game_over(screen, winner):
    font = pygame.font.SysFont("Arial", 48, bold=True)
    text = f"{winner.capitalize()} wins!" if winner != "draw" else "Draw!"
    rendered = font.render(text, True, (255, 0, 0))
    rect = rendered.get_rect(center=(WIDTH // 2, HEIGHT // 2))
    screen.blit(rendered, rect)

def draw_promotion_dialog(screen, color, piece_images):
    dialog_width, dialog_height = 400, 100
    dialog_x = (WIDTH - dialog_width) // 2
    dialog_y = (HEIGHT - dialog_height) // 2
    pygame.draw.rect(screen, (240, 240, 240), (dialog_x, dialog_y, dialog_width, dialog_height))
    pygame.draw.rect(screen, (0, 0, 0), (dialog_x, dialog_y, dialog_width, dialog_height), 3)
    pieces = [
        Piece.WHITE_QUEEN, Piece.WHITE_ROOK, Piece.WHITE_BISHOP, Piece.WHITE_KNIGHT
    ] if color.startswith("white") else [
        Piece.BLACK_QUEEN, Piece.BLACK_ROOK, Piece.BLACK_BISHOP, Piece.BLACK_KNIGHT
    ]
    piece_size = 80
    rects = []
    for i, piece in enumerate(pieces):
        x = dialog_x + 20 + i * 90
        y = dialog_y + 10
        pygame.draw.rect(screen, (255, 255, 255), (x, y, piece_size, piece_size))
        pygame.draw.rect(screen, (0, 0, 0), (x, y, piece_size, piece_size), 2)
        if piece in piece_images:
            piece_img = pygame.transform.scale(piece_images[piece], (piece_size, piece_size))
            screen.blit(piece_img, (x, y))
        rects.append((x, y, piece_size, piece_size))
    return pieces, rects

def handle_promotion_click(mouse_pos, rects, pieces):
    for i, rect in enumerate(rects):
        x, y, w, h = rect
        if x <= mouse_pos[0] <= x + w and y <= mouse_pos[1] <= y + h:
            return pieces[i]
    return None

def is_valid_promotion_move(game, start_pos, end_pos, piece):
    if not ((piece == Piece.WHITE_PAWN and start_pos[0] == 6 and end_pos[0] == 7) or
            (piece == Piece.BLACK_PAWN and start_pos[0] == 1 and end_pos[0] == 0)):
        return False
    if abs(end_pos[1] - start_pos[1]) > 1:
        return False
    target_piece = game.board.get_piece(end_pos)
    is_capture = target_piece != Piece.EMPTY
    is_straight = end_pos[1] == start_pos[1]
    return (is_straight and not is_capture) or (not is_straight and is_capture)

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
    promotion_mode = False
    promotion_data = None 

    while True:
        # Draw the board normally, skip the piece being dragged
        draw_board(screen, game, piece_images,
                   skip_pos=start_pos if dragging and not promotion_mode else None)

        if game.is_game_over():
            draw_game_over(screen, game.winner)

        # Draw the piece being dragged
        if dragging and drag_piece and not promotion_mode:
            x, y = pygame.mouse.get_pos()
            screen.blit(drag_piece, (x - TILE_SIZE // 2, y - TILE_SIZE // 2))

        # Draw promotion dialog if in promotion mode
        if promotion_mode and promotion_data:
            pieces, rects = draw_promotion_dialog(screen, promotion_data[2], piece_images)

        pygame.display.flip()
        clock.tick(60)

        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                pygame.quit()
                return

            elif event.type == pygame.MOUSEBUTTONDOWN:
                if promotion_mode and promotion_data:
                    pieces, rects = draw_promotion_dialog(screen, promotion_data[2], piece_images)
                    selected_piece = handle_promotion_click(event.pos, rects, pieces)
                    if selected_piece:
                        try:
                            # Use game.make_move instead of board.move to ensure turn switching
                            if selected_piece:
                                print(f"Selected piece: {selected_piece}, type: {type(selected_piece)}")
                                print(f"Promotion data: {promotion_data}")
                                print(f"Expected piece types: {pieces}")
                            game.make_move(promotion_data[0], promotion_data[1], promote_piece=selected_piece)
                        except Exception as e:
                            print("Promotion move failed:", e)
                        promotion_mode = False
                        promotion_data = None
                        dragging = False
                        drag_piece = None
                        start_pos = None
                    else:
                        # Cancel promotion if clicked outside dialog
                        promotion_mode = False
                        promotion_data = None
                        dragging = False
                        drag_piece = None
                        start_pos = None
                else:
                    col = event.pos[0] // TILE_SIZE
                    row = event.pos[1] // TILE_SIZE
                    piece = game.board.get_piece((row, col))
                    if piece and game.get_player().startswith('white') == (piece < 7):
                        dragging = True
                        drag_piece = piece_images[piece]
                        start_pos = (row, col)

            elif event.type == pygame.MOUSEBUTTONUP and dragging:
                col = event.pos[0] // TILE_SIZE
                row = event.pos[1] // TILE_SIZE
                end_pos = (row, col)
                piece = game.board.get_piece(start_pos)
                is_promotion = ((piece == Piece.WHITE_PAWN and end_pos[0] == 7) or
                                (piece == Piece.BLACK_PAWN and end_pos[0] == 0))
                if is_promotion and is_valid_promotion_move(game, start_pos, end_pos, piece):
                    player_color = "white" if piece == Piece.WHITE_PAWN else "black"
                    promotion_mode = True
                    promotion_data = (start_pos, end_pos, player_color)
                    # Don't reset dragging state here - wait for promotion selection
                else:
                    try:
                        game.make_move(start_pos, end_pos)
                    except Exception as e:
                        print("Illegal move:", e)
                    dragging = False
                    drag_piece = None
                    start_pos = None

if __name__ == "__main__":
    main()