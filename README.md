# rollable

### To Do Next

- [ ] Figure out what the goal for the player is
  - Golf flagpole
  - Some chequered finish line

### Demo To Do

- [ ] Audio
  - Player bumping
  - Player rolling
  - Playing death
  - Soundtrack
- [ ] Level select
  - Add a nice way to select levels
- [ ] Collectibles
  - Each level has x collectibles on it
- [ ] Full gameloop
  - Level select -> Level -> Completed level -> Next level
- [ ] Particles
  - Player roll
  - Player hit
  - Player death
- [ ] Water shader

### Bugs

- [ ] Can jump infinitely against walls because grounded checks touching ground and not raycast
- [ ] Can jump on bouncepads to go way higher
- [ ] Outline gets too thick at distance
  - Can overlay many outlines from 1, but mix albedo at distance

### Enhancements

- [ ] Rename tree_01 to tree_m_01 and then have many that are picked at random to build the stage
- [ ] Sweepers
  - Make the sweeper object just a 1x1 cylinder so it can be scaled to any size
  - Give option to add many sweepers, each placed equal intervals apart

### Audio

- [ ] Bonk sound
  - When the ball collides with something, I can play a bonk sound
  - Volume can depend on velocity before and after collision (velocity change)

### Level Select

- [ ] Isometric 3D
  - Controller a ball that can't jump
  - Rolls around and can enter levels
  - Level buttons are surrounded by the things in them, such as sweepers
  - Similar to a game about cooking or plumbing

### Fun details

- [ ] Air loons
  - Pop if something hits them
- [ ] Rocket
  - Hidden buttons in the map, hitting them all launches a rocket
  - This could be how bonus stages are found like space stage

### Stretch Ideas

- [ ] Golf gamemode
  - Can jump when touching ground once per turn
  - Can only hit the ball (take a turn) once it's stopped moving
- [ ] Only up levels
- [ ] Easter eggs such as rocket
- [ ] Save system
  - Stretch because this is just for a demo
- [ ] Chicken
