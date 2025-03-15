package main

import rl "vendor:raylib"
import fmt "core:fmt"

SCREEN_WIDTH :: 600
SCREEN_HEIGHT :: 600
GRID_SIZE :: 8
TILE_SIZE :: SCREEN_WIDTH / GRID_SIZE
MAX_FPS :: 15
// Each piece is 75x75 pixels in the spritesheet, equal to default TILE_SIZE
PIECE_SIZE :: 75

A8 :: 1; B8 :: 2; C8 :: 3; D8 :: 4; E8 :: 5; F8 :: 6; G8 :: 7; H8 :: 8
A7 :: 9; B7 :: 10; C7 :: 11; D7 :: 12; E7 :: 13; F7 :: 14; G7 :: 15; H7 :: 16
A6 :: 17; B6 :: 18; C6 :: 19; D6 :: 20; E6 :: 21; F6 :: 22; G6 :: 23; H6 :: 24
A5 :: 25; B5 :: 26; C5 :: 27; D5 :: 28; E5 :: 29; F5 :: 30; G5 :: 31; H5 :: 32
A4 :: 33; B4 :: 34; C4 :: 35; D4 :: 36; E4 :: 37; F4 :: 38; G4 :: 39; H4 :: 40
A3 :: 41; B3 :: 42; C3 :: 43; D3 :: 44; E3 :: 45; F3 :: 46; G3 :: 47; H3 :: 48
A2 :: 49; B2 :: 50; C2 :: 51; D2 :: 52; E2 :: 53; F2 :: 54; G2 :: 55; H2 :: 56
A1 :: 57; B1 :: 58; C1 :: 59; D1 :: 60; E1 :: 61; F1 :: 62; G1 :: 63; H1 :: 64

Chess_Piece_Type :: enum {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}

Chess_Piece_Color :: enum {
    White,
    Black,
}

Chess_Piece :: struct {
    texture: rl.Texture2D, // Full spritesheet
    source: rl.Rectangle, // Source rect in texture sheet
    pos: i32, // Board position (1-64)
    is_alive: bool,
    piece_type: Chess_Piece_Type,
    color: Chess_Piece_Color,
}

// helper func to init chess pieces
// why doesnt odin structs have methods...?
init_chess_piece :: proc(
    sheet: rl.Texture2D,
    pos: i32,
    pc: Chess_Piece_Type,
    cl: Chess_Piece_Color
) -> Chess_Piece {
    index: i32
    switch pc {
        case .Bishop:
            index = 0
        case .King:
            index = 1
        case .Knight:
            index = 2
        case .Pawn:
            index = 3
        case .Queen:
            index = 4
        case .Rook:
            index = 5
    }

    return Chess_Piece { 
        texture = sheet,
        source = get_piece_texture(sheet, index),
        pos = pos,
        is_alive = true,
        piece_type = pc,
        color = cl,
    }
}

// function to extract the correct piece from the spritesheet
get_piece_texture :: proc(texture: rl.Texture2D, index: i32) -> rl.Rectangle {
    x := f32(index % 6) * PIECE_SIZE
    y := f32(index / 6) * PIECE_SIZE
    return rl.Rectangle {
        x = x,
        y = y,
        width = PIECE_SIZE,
        height = PIECE_SIZE
    }
}

// self explanatory
draw_chess_piece :: proc(piece: Chess_Piece) {
    if piece.is_alive {
        x := (piece.pos - 1) % 8
        y := (piece.pos - 1) / 8
        dest := rl.Rectangle { 
            x = f32(x * TILE_SIZE),
            y = f32(y * TILE_SIZE),
            width = TILE_SIZE,
            height = TILE_SIZE
        }
        rl.DrawTexturePro(piece.texture, piece.source, dest, 0, 0, rl.WHITE)
    }
}

main :: proc() {
    rl.InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, "Chess")
    rl.SetTargetFPS(MAX_FPS)
    defer rl.CloseWindow()

    chess_piece_img := rl.LoadImage("./assets/pieces.png")
    piece_texture_black := rl.LoadTextureFromImage(chess_piece_img)
    rl.ImageColorInvert(&chess_piece_img)
    piece_texture_white := rl.LoadTextureFromImage(chess_piece_img)
    rl.UnloadImage(chess_piece_img)

    tile_color: rl.Color

    chess_pieces := [32]Chess_Piece {
        // white
        init_chess_piece(piece_texture_white, A2, .Pawn, .White),
        init_chess_piece(piece_texture_white, B2, .Pawn, .White),
        init_chess_piece(piece_texture_white, C2, .Pawn, .White),
        init_chess_piece(piece_texture_white, D2, .Pawn, .White),
        init_chess_piece(piece_texture_white, E2, .Pawn, .White),
        init_chess_piece(piece_texture_white, F2, .Pawn, .White),
        init_chess_piece(piece_texture_white, G2, .Pawn, .White),
        init_chess_piece(piece_texture_white, H2, .Pawn, .White),
        init_chess_piece(piece_texture_white, A1, .Rook, .White),
        init_chess_piece(piece_texture_white, H1, .Rook, .White),
        init_chess_piece(piece_texture_white, B1, .Knight, .White),
        init_chess_piece(piece_texture_white, G1, .Knight, .White),
        init_chess_piece(piece_texture_white, C1, .Bishop, .White),
        init_chess_piece(piece_texture_white, F1, .Bishop, .White),
        init_chess_piece(piece_texture_white, D1, .Queen, .White),
        init_chess_piece(piece_texture_white, E1, .King, .White),

        // black
        init_chess_piece(piece_texture_black, A7, .Pawn, .Black),
        init_chess_piece(piece_texture_black, B7, .Pawn, .Black),
        init_chess_piece(piece_texture_black, C7, .Pawn, .Black),
        init_chess_piece(piece_texture_black, D7, .Pawn, .Black),
        init_chess_piece(piece_texture_black, E7, .Pawn, .Black),
        init_chess_piece(piece_texture_black, F7, .Pawn, .Black),
        init_chess_piece(piece_texture_black, G7, .Pawn, .Black),
        init_chess_piece(piece_texture_black, H7, .Pawn, .Black),
        init_chess_piece(piece_texture_black, A8, .Rook, .Black),
        init_chess_piece(piece_texture_black, H8, .Rook, .Black),
        init_chess_piece(piece_texture_black, B8, .Knight, .Black),
        init_chess_piece(piece_texture_black, G8, .Knight, .Black),
        init_chess_piece(piece_texture_black, C8, .Bishop, .Black),
        init_chess_piece(piece_texture_black, F8, .Bishop, .Black),
        init_chess_piece(piece_texture_black, D8, .Queen, .Black),
        init_chess_piece(piece_texture_black, E8, .King, .Black),
    }

    for !rl.WindowShouldClose() {
        rl.BeginDrawing()

        // draw board
        for y := i32(0); y < 8; y += 1 {
            for x := i32(0); x < 8; x += 1 {
                switch (x + y) % 2 {
                    case 0:
                        tile_color = rl.Color { 30, 80, 133, 255 } 
                    case:
                        tile_color = rl.Color { 51, 119, 191, 255 } 
                }

                rl.DrawRectangle(x * TILE_SIZE, y * TILE_SIZE, TILE_SIZE, TILE_SIZE, tile_color)
            }
        }

        for piece in chess_pieces do draw_chess_piece(piece)

        rl.EndDrawing()
    }
}
