# Salamander Rain Dash

A small survival-collection game built with **Rust**, **Bevy 0.13**, and **GitHub Copilot** for the Agentic Vibe Coding activity.

## Overview

Salamander Rain Dash is a Bevy game where the player controls a salamander, collects glowing fireflies, and avoids falling rain.

The goal is to collect all fireflies before losing all 3 lives. As the player collects more fireflies, the rain falls faster, increasing the difficulty.

This project was intentionally built in a technology stack that was unfamiliar to me (Rust and Bevy) while using an AI coding agent to help generate and modify code.

---

## Features

### Player Movement
- Move using:
  - W = Up
  - A = Left
  - S = Down
  - D = Right
  - Arrow keys are also supported

### Visual Design
- Code-drawn salamander character with a head, tail, legs, eyes, and markings.
- Layered nighttime forest background.
- Pulsing firefly glow animations.
- Varied rain sizes and falling speeds.
- Centered HUD with firefly progress and heart-based lives.

### Firefly Collection
- Glowing fireflies are placed around the map.
- Collecting a firefly increases your score.
- Fireflies disappear when collected.

### Rain Hazard
- Rain continuously falls from the top of the screen.
- Touching rain causes damage.

### Lives System
- Start with 3 lives.
- Lose 1 life when hit by rain.
- Player resets to the center after taking damage.

### Hit Cooldown
- 1 second of invulnerability after taking damage.
- Prevents losing multiple lives instantly.
- The salamander flashes while temporarily invulnerable.

### Game Screens
- Title screen with controls and the saved best score.
- Pause screen that preserves the current round.
- Win and loss screens with replay and main-menu controls.

### Gameplay Feedback
- Firefly collection particles.
- Screen shake when the salamander is hit.
- Visible storm intensity indicator.

### Persistent High Score
- The best score is saved in `high_score.txt`.
- The score is restored the next time the game starts.

### Dynamic Difficulty
- Rain speed increases as score increases.
- Makes the game progressively harder.

### Win Condition
Collect all fireflies to save the salamander.

Displays:

```text
The fireflies are safe! Press Enter or R to play again.
```

### Lose Condition
Lose all 3 lives.

Displays:

```text
The storm won. Press Enter or R to try again.
```

---

## Technologies Used

- Rust
- Bevy 0.13
- Git
- GitHub
- GitHub Copilot

---

## Project Structure

```text
src/
 ├── background.rs
 ├── constants.rs
 ├── effects.rs
 ├── firefly.rs
 ├── game.rs
 ├── main.rs
 ├── player.rs
 ├── rain.rs
 ├── ui.rs
 └── util.rs

assets/
 └── fonts/
      └── FiraSans-Bold.ttf
```

Each gameplay area is implemented as a focused Bevy plugin or shared module.

---

## How To Run

### Clone Repository

```bash
git clone https://github.com/alstondsouza1/salamander-rain-game.git
cd salamander-rain-game
```

### Run

```bash
cargo run
```

---

## Controls

| Key | Action |
|------|---------|
| W | Move Up |
| A | Move Left |
| S | Move Down |
| D | Move Right |
| Arrow Keys | Move |
| P or Escape | Pause or resume |
| Enter or R | Start or replay |
| M | Return to the menu from pause/end screens |

---

## Screenshot


![Game Screenshot](screenshot.jpg)

---

## What I Learned

This project introduced me to:

- Agentic AI-assisted development
- Rust basics
- Bevy game engine concepts
- ECS (Entity Component System)
- Collision detection
- Timers and cooldown systems
- Game state management
- Iterative development with Git commits

The most effective strategy was building one small feature at a time, testing frequently, and committing working versions before moving to the next feature.

Some challenges included debugging Bevy compilation errors, fixing game logic issues, and learning how to guide the AI agent when it became stuck or generated incorrect code.

---

## Reflection on AI-Assisted Development

What worked well:

- Generating initial game mechanics
- Creating UI elements
- Implementing timers and collision systems
- Rapidly adding new features

What was difficult:

- Debugging compiler errors
- Fixing broken game logic
- Resolving Bevy-specific issues
- Preventing the AI from introducing new bugs while fixing old ones

One improvement I would make next time is to plan the game features in more detail before coding and make smaller, more focused requests to the AI agent.

---

## Future Improvements

Potential future features include:

- Sound effects
- Randomized rain patterns
- Additional levels
- Background music
