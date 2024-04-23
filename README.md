# Simple Prisoner's Dilemma Test I decided to implement after watching Veritasium's video about it
*(also as an efficient way to procrastinate studying for my calculus test)*

You can implement your own strategy using the ``Strategy`` struct that has the ``decide`` function which returns either ``Decision::Share`` or ``Decision::Steal``.
In the ``factors.rs`` there are constants for # of rounds played, # of total entities per strategy pool, and how points are distributed.
After two pools go against each other, the ``score`` function is called on each strategy to change how each entity should behave based on how well it did last round

Implemented in one afternoon so most of the code is unoptimized and very ugly unfortunately.

All thanks to the ``textplots`` crate and ``owo-colors`` for beautiful teminal graphs & colors

# Plotting # of total points gained VS # of rounds played for each strategy:
![image](https://github.com/jedjoud10/game-theory-test/assets/34755598/4773a5f0-233f-4ef7-9672-96b4d671b5b9)

# Histogram showing the most common decision in a play-off between two strategies:
#### Red depicts "steal", green depicts "share". Anything in between means that multiple entities (of the same pool) have chosen a different value
![image](https://github.com/jedjoud10/game-theory-test/assets/34755598/41e3def2-b24f-4a10-ad65-fb4953adce03)




# LICENSE
Licensed under
 * MIT license
   ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)
