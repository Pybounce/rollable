# rollable

### To Do Next

- [ ] World Select
  - Can have some resource holding the current selected world
    - This way when you press space, it uses that
  - Then something can look for changes to that world and move in the different islands and data?
    - There should be an island per world, with some data and a name. Use the World enum

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

- [ ] Particle effects transparent?
  - Basically you can see outlines through them
  - Therefore the particle effects are not in the depth/normal buffers but are in the albedo buffer
- [ ] Outline thickness issues
  - The outline thickness is based on distance
  - But it isn't taking the smallest distance of all the uvs (which it should)
  - So when viewing ball with ground behind, it's thick. Whereas with sky behind, it's thinner since the samples from uvs that are in the sky count the distance as very long
- [ ] Can jump infinitely against walls because grounded checks touching ground and not raycast
- [ ] Can jump on bouncepads to go way higher
- [ ] Can glitch into the ground
  - Grass and dirt being separate sucks because grass is so thin
  - The sweeper collider is a cylinder and thus pushes you down slightly if you hit it's lower half

### Enhancements

- [ ] Loading states
  - Add separate state for Overworld
  - OnEnter, trigger load
  - Set OverworldState::Loading
  - When in loading, check finished
  - When finished, set OverworldState::Playing or something
  - Do this for everything
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
- [ ] All in-world
  - The level select is itself, a level. It has other level-teleports inside of it
  - Player just goes to that level and can enter the level
  - Can have each World be an island the player selects, then each world has an island for the Level Select
    - Then things like the rocket easter egg can unlock microworlds, which are worlds with just a small amount of very hard levels

### Fun details

- [ ] Air loons
  - Pop if something hits them
- [ ] Rocket
  - Hidden buttons in the map, hitting them all launches a rocket
  - This could be how bonus stages are found like space stage
- [ ] Better than collectibles
  - Crash has collectibles but they require things like not dying for the whole level etc
  - Point is collectibles are not the only extra thing to add to levels

### Stretch Ideas

- [ ] Golf gamemode
  - Can jump when touching ground once per turn
  - Can only hit the ball (take a turn) once it's stopped moving
- [ ] Only up levels
- [ ] Easter eggs such as rocket
- [ ] Save system
  - Stretch because this is just for a demo
- [ ] Chicken
