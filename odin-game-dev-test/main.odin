package main

import rl "vendor:raylib"
import fmt "core:fmt"

Player_Animation_Case :: enum {
    Idle,
    Run,
}

Player_Animation :: struct {
    texture : rl.Texture2D,
    anim_case : Player_Animation_Case,
    frame_count : int,
    frame_timer : f32,
    cur_frame_index : int,
    secs_between_frames : f32,
}

update_plr_anim :: proc(a: ^Player_Animation) {
    a.frame_timer += rl.GetFrameTime()

    if a.frame_timer > a.secs_between_frames {
        a.cur_frame_index = (a.cur_frame_index + 1) % a.frame_count
        a.frame_timer = 0.0
    }
}

draw_plr_anim :: proc(a: Player_Animation, pos: rl.Vector2, is_facing_left: bool) {
    plr_run_frame_w := f32(a.texture.width) / f32(a.frame_count)
    plr_run_frame_h := f32(a.texture.height)

    draw_plr_run_anim_src := rl.Rectangle {
        x = f32(a.cur_frame_index) * plr_run_frame_w,
        y = 0.0,
        width = plr_run_frame_w,
        height = plr_run_frame_h,
    }

    if is_facing_left {
        draw_plr_run_anim_src.width = -draw_plr_run_anim_src.width
    }

    draw_plr_run_anim_dest := rl.Rectangle {
        x = pos.x,
        y = pos.y,
        width = plr_run_frame_w,
        height = plr_run_frame_h,
    }
    
    dest_w := draw_plr_run_anim_dest.width
    dest_h := draw_plr_run_anim_dest.height

    rl.DrawTexturePro(
        a.texture,
        draw_plr_run_anim_src,
        draw_plr_run_anim_dest,
        {dest_w / 2.0, dest_h},
        0.0,
        rl.WHITE
    )
}

PIXEL_WINDOW_HEIGHT :: 100

main :: proc() {
    rl.InitWindow(512, 256, "cat run")
    rl.SetWindowPosition(800, 768 / 2)
    rl.SetWindowState({.WINDOW_RESIZABLE})
    rl.SetTargetFPS(30)

    plr_pos: rl.Vector2
    plr_velocity : rl.Vector2
    plr_is_on_ground : bool
    plr_is_facing_left : bool

    plr_run_anim := Player_Animation {
        texture = rl.LoadTexture("./assets/cat_run.png"),
        anim_case = .Run,
        frame_count = 4,
        secs_between_frames = 0.1,
    }

    plr_idle_anim := Player_Animation {
        texture = rl.LoadTexture("./assets/cat_idle.png"),
        anim_case = .Idle,
        frame_count = 2,
        secs_between_frames = 0.5,
    }

    cur_plr_anim := plr_idle_anim

    platform := rl.Rectangle {-20, 20, 96, 16}

    for !rl.WindowShouldClose() {
        rl.BeginDrawing()
        rl.ClearBackground(rl.LIGHTGRAY)

        switch {
            case rl.IsKeyDown(.LEFT):
                plr_velocity.x = -100.0
                plr_is_facing_left = true  
                if cur_plr_anim.anim_case != .Run {
                    cur_plr_anim = plr_run_anim
                }
            case rl.IsKeyDown(.RIGHT):
                plr_velocity.x = 100.0
                plr_is_facing_left = false
                if cur_plr_anim.anim_case != .Run {
                    cur_plr_anim = plr_run_anim
                }
            case:
                plr_velocity.x = 0.0
                if cur_plr_anim.anim_case != .Idle {
                    cur_plr_anim = plr_idle_anim
                }
        }

        plr_velocity.y = min(plr_velocity.y + 500.0 * rl.GetFrameTime(), 500.0)

        if plr_is_on_ground && rl.IsKeyPressed(.UP) {
            plr_velocity.y = -250.0
        }

        plr_pos += plr_velocity * rl.GetFrameTime()
        fmt.println(plr_pos)

        plr_feet_clip := rl.Rectangle {
            plr_pos.x - 4,
            plr_pos.y - 4,
            8,
            4,
        }

        plr_is_on_ground = false

        if rl.CheckCollisionRecs(plr_feet_clip, platform) && plr_velocity.y > 0.0 {
            plr_velocity.y = 0.0
            plr_pos.y = platform.y
            plr_is_on_ground = true
        }

        update_plr_anim(&cur_plr_anim)

        camera := rl.Camera2D {
            zoom = f32(rl.GetScreenHeight() / PIXEL_WINDOW_HEIGHT),
            offset = {
                f32(rl.GetScreenWidth() / 2),
                f32(rl.GetScreenHeight() / 2),
            },
            target = plr_pos,
        }

        rl.BeginMode2D(camera)
        draw_plr_anim(cur_plr_anim, plr_pos, plr_is_facing_left)
        rl.DrawRectangleRec(platform, rl.GREEN)
        rl.DrawRectangleRec(plr_feet_clip, rl.DARKBLUE)
        rl.EndMode2D()
        rl.EndDrawing()
    }

    rl.CloseWindow()
}
