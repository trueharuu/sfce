// TODO: use these instead of raw piece placements. a majority of raw placements on boards are not realistic.
pub enum Input {
  MoveLeft,
  MoveRight,
  Clockwise,
  CounterClockwise,
  Flip,
  HardDrop,
  SoftDrop,
}