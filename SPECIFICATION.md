# Salamander Rain Dash — Specification

**Version:** 0.1.0
**Engine:** Bevy 0.18.1 (Rust 2021)
**Author:** Alston Dsouza
**Repository:** https://github.com/alstondsouza1/salamander-rain-game

---

## 1. What the Game Does

Salamander Rain Dash is a 2D top-down survival-collection arcade game. The player
steers a salamander around a nighttime wetland, collecting glowing fireflies while
dodging continuously falling rain. The objective is to clear three increasingly
hazardous levels by collecting every firefly in a level before losing all lives.

The game is deliberately small in scope — it was built as a learning exercise in an
unfamiliar stack (Rust + Bevy + ECS) using an AI coding agent, with the developer
retaining architectural control.

---

## 2. Target Users

- **Primary:** The course instructor / evaluator assessing an AI-assisted development
  workflow (Applied AI course).
- **Secondary:** Casual players wanting a short (1–3 minute) arcade session.
- **Tertiary:** Developers learning Bevy ECS who want a compact, readable reference
  project demonstrating plugins, states, timers, collision, and persistence.

Assumes a desktop user with a keyboard. No accessibility, controller, or touch
support is currently provided.

---

## 3. Core Gameplay Loop

1. **Menu** → player presses Enter/Space to start.
2. **Round start** — player, fireflies, HUD, and rain timer spawn for the current
   level (`start_round_if_needed` in `game.rs`).
3. **Play loop** (per frame, while `GameState::Playing`):
   - Move salamander with WASD / arrow keys (normalized, clamped to playfield).
   - Rain spawns on a repeating timer and falls with per-drop speed variance plus a
     score-scaled global speed bonus.
   - Collecting a firefly (distance check < 38px) increments score, spawns particles,
     and updates the high score.
   - Touching rain (AABB check) costs a life, resets the player to center, triggers a
     1-second invulnerability flash + camera shake.
4. **Resolution:**
   - Collect all fireflies → **Won** → advance to next level (or loop to Level 1 after
     the final level).
   - Lose all lives → **Lost** → retry the same level.
5. Player can **Pause** (P/Esc) at any time, or return to **Menu** (M) from
   pause/end screens.

---

## 4. Features

### Implemented
- **Movement:** 8-directional, normalized, speed-clamped, sprite rotates to face
  heading. WASD + arrow keys.
- **Firefly collection:** randomized spawn positions, distance-based pickup, score
  increment, win-on-goal.
- **Rain hazard:** timer-driven spawning, randomized size/speed, AABB collision,
  splash effects on impact/ground.
- **Lives system:** configurable starting lives (currently 3), reset-to-center on hit.
- **Hit cooldown:** 1s invulnerability with visible blink to prevent multi-hit deaths.
- **Three levels** with escalating firefly goals, rain frequency, rain speed, and
  background tint (`level.rs`).
- **Dynamic difficulty:** rain speed scales with current score within a level.
- **Persistent high score:** stored in `high_score.txt`, loaded at startup.
- **Game screens:** Menu, Playing, Paused, Won, Lost (Bevy state machine).
- **Feedback:** firefly collection particle burst, screen shake on hit, animated
  splashes, HUD storm-intensity indicator (CALM → RISING → HEAVY → FIERCE).
- **Animation:** sprite-sheet atlases for salamander run-cycle, firefly wing-cycle,
  rain splash.

### Not implemented (see Future Improvements)
- Audio (SFX / music), power-ups, difficulty selection, scoreboard beyond single best.

---

## 5. Technical Stack

| Layer | Choice |
|-------|--------|
| Language | Rust (edition 2021) |
| Engine | Bevy 0.18.1 (ECS, 2D renderer, windowing, input, asset server) |
| Randomness | `rand` 0.9 |
| Persistence | Plain-text file (`high_score.txt`) via `std::fs` |
| Build / tooling | Cargo, Git, GitHub |
| Assets | PNG sprite sheets, PNG background, TTF font (FiraSans-Bold) |
| Window | Fixed 960×640, non-resizable |

---

## 6. Architecture Overview

The game follows Bevy's **plugin + ECS** pattern. `main.rs` wires resources, the
window, the `GameState` state machine, and registers seven plugins. Each domain owns
its components, resources, and systems.

```
main.rs            App setup, resource init, plugin registration, GameState
├── game.rs        GameState enum, core resources (Score, HighScore, Lives,
│                  SessionActive, HitCooldown), round lifecycle, high-score I/O
├── level.rs       LevelDefinition table + CurrentLevel resource (data-driven tuning)
├── constants.rs   Shared tunables: speeds, playfield bounds, colors, atlas geometry
├── background.rs  MainCamera + tinted wetland background per level
├── player.rs      Player component, movement, run animation, hit-flash blink
├── firefly.rs     Firefly spawn/collect/animate, win detection, high-score update
├── rain.rs        Raindrop spawn/move/collision, score-scaled & per-drop speed
├── effects.rs     Message-driven particles, splashes, camera shake
├── ui.rs          HUD, menu/pause/win/loss screens, all state-transition input
└── util.rs        random_range helper
```

**Key patterns:**
- **State-gated systems:** most systems use `run_if(in_state(GameState::Playing))`.
- **State-transition hooks:** `OnEnter`/`OnExit` drive spawning and screen setup.
- **Decoupled effects:** gameplay systems emit Bevy **Messages**
  (`CollectionEffect`, `PlayerHitEffect`, `SplashEffect`); `effects.rs` reacts. This
  keeps visual feedback independent of gameplay logic.
- **Data-driven levels:** all per-level tuning lives in a single `LEVELS` array;
  difficulty is a lookup, not branching logic.
- **Lifecycle marker:** `GameplayEntity` tags everything spawned for a round so it can
  be despawned in bulk on session end.
- **`SessionActive` guard:** prevents re-spawning a round when re-entering `Playing`
  from `Paused`.

---

## 7. Edge Cases

| Case | Current handling |
|------|------------------|
| Re-enter Playing from Pause | `SessionActive` guard skips re-spawn — round preserved. |
| Win on final level | Loops back to Level 1 (`is_final()` check in `ui.rs`). |
| Multiple rain hits in one frame | `break` after first collision + cooldown reset. |
| Rapid sequential hits | 1s `HitCooldown` invulnerability. |
| High-score file missing/corrupt | `load_high_score` falls back to `0`. |
| High-score write failure | Logged as warning; gameplay continues. |
| Player/entity query returns none | `let-else` early-return guards throughout. |
| Level index overflow | `definition()` clamps to last level. |
| Lives underflow | `saturating_sub(1)`. |

**Known gaps / unhandled cases:**
- Window is non-resizable, so playfield bounds are hard-coded constants — no
  resolution independence.
- No protection against fireflies spawning under the player's start position (instant
  pickup possible).
- High score is global, not per-level.
- No pause of the `HitCooldown`/animation timers' wall-clock semantics across
  state changes (timers use frame delta, so mostly fine, but worth noting).
- Collision uses fixed AABB/distance thresholds independent of sprite scale.

---

## 8. Testing Requirements

**Current state:** one unit test (`ui.rs::final_level_is_last_definition`). The codebase
is largely untested.

**Recommended coverage:**
- **Unit (pure logic):**
  - `CurrentLevel::definition()` clamping and `is_final()` across all indices.
  - `load_high_score` parsing: valid, empty, whitespace, non-numeric, missing file.
  - Storm-intensity label thresholds (extract the progress→label mapping into a
    testable pure function).
  - Rain speed formula (`base + score * 18.0`).
- **Integration (Bevy `App` harness):**
  - State transitions: Menu→Playing→Won→(next level), →Lost→retry.
  - `start_round_if_needed` spawns the correct firefly count and resets score/lives.
  - `SessionActive` prevents double-spawn on Pause→Playing.
  - Collision: a raindrop overlapping the player decrements lives and sets cooldown.
- **Manual / smoke:** `cargo run` boots to menu, all five screens reachable, high score
  persists across restarts.
- **CI:** `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`.

---

## 9. Future Improvements

- **Audio:** SFX (collect, hit, level-clear) + ambient storm/music loop.
- **Power-ups:** temporary shield, slow-rain, magnet, extra life.
- **Difficulty selection** at menu (easy/normal/hard scaling the `LEVELS` table).
- **More hazards:** lightning strikes, puddles, wind gusts.
- **Per-level high scores** and a fastest-clear timer.
- **Settings:** key remapping, volume, resolution/fullscreen.
- **Resolution independence:** derive playfield bounds from window size.
- **Accessibility:** colorblind-safe palette, reduced-motion (disable shake/flash).
- **Juice:** firefly glow pulsing, parallax background, trail behind the salamander.

---

*This specification reflects the codebase as of commit `c91587e`.*
