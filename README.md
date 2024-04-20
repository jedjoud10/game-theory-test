# Simple Prisoner's Dilemma Test I decided to implement after watching Veritasium's video about it
*(also as an efficient way to procrastinate studying for my calculus test)*

You can implement your own strategy using the ``Strategy`` struct that has the ``decide`` function which returns either ``Decision::Share`` or ``Decision::Steal``.
In the ``factors.rs`` there are constants for # of rounds played, # of total entities per strategy pool, and how points are distributed.
After two pools go against each other, the ``score`` function is called on each strategy to change how each entity should behave based on how well it did last round

Implemented in one afternoon so most of the code is unoptimized and very ugly unfortunately.

All thanks to the ``textplots`` crate and ``owo-colors`` for beautiful teminal graphs & colors

Plotting # of total points gained VS # of rounds played for each strategy:
![image](https://github.com/jedjoud10/game-theory-test/assets/34755598/02d17932-0bcd-4e07-ae6f-3d7f087428d5)

Histogram showing the most common decision in a play-off between two strategies:
Red depicts "steal", green depicts "share". Anything in between means that multiple entities (of the same pool) have chosen a different value
![image](https://github.com/jedjoud10/game-theory-test/assets/34755598/2ffec12b-1ebb-48c3-84fb-151fc4614b3a)



# LICENSE
Licensed under
 * MIT license
   ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)
