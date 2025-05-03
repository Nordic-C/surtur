#include "raylib.h"

int main() {
    // Window dimensions
    const int screenWidth = 800;
    const int screenHeight = 600;

    // Initialize the window
    InitWindow(screenWidth, screenHeight, "Raylib Example - Moving Ball");

    // Ball properties
    Vector2 ballPosition = { screenWidth / 2, screenHeight / 2 };
    float ballRadius = 20;
    Vector2 ballSpeed = { 4.0f, 3.0f };

    SetTargetFPS(60);  // Set the game to run at 60 frames per second

    while (!WindowShouldClose()) {  // Main game loop
        // Update ball position
        ballPosition.x += ballSpeed.x;
        ballPosition.y += ballSpeed.y;

        // Bounce off screen edges
        if (ballPosition.x - ballRadius <= 0 || ballPosition.x + ballRadius >= screenWidth)
            ballSpeed.x *= -1;
        if (ballPosition.y - ballRadius <= 0 || ballPosition.y + ballRadius >= screenHeight)
            ballSpeed.y *= -1;

        // Drawing
        BeginDrawing();
        ClearBackground(RAYWHITE);

        DrawText("Raylib Bouncing Ball", 10, 10, 20, DARKGRAY);
        DrawCircleV(ballPosition, ballRadius, RED);

        EndDrawing();
    }

    // Cleanup
    CloseWindow();

    return 0;
}