+++
Target = "Player.Move"
Type = (walkDirection: Vector3, relativeToCamera?: boolean | undefined) => void
+++

Causes the player's character to walk in the given direction until stopped, or interrupted by the player (by using their controls).