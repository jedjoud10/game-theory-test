# Simple Prisoner's Dilemma Test I decided to implement after watching Veritasium's video about it
*(also as an efficient way to procrastinate studying for my calculus test)*

You can implement your own strategy using the ``Strategy`` struct that has the ``decide`` function which returns either ``Decision::Share`` or ``Decision::Steal``.
In the ``factors.rs`` there are constants for # of rounds played, # of total entities per strategy pool, and how points are distributed.
After two pools go against each other, the ``score`` function is called on each strategy to change how each entity should behave based on how well it did last round

Implemented in one afternoon so most of the code is unoptimized and very ugly unfortunately.

All thanks to the ``textplots`` crate and ``owo-colors`` for beautiful teminal graphs & colors

![image](https://github.com/jedjoud10/game-theory-test/assets/34755598/491b5917-eadf-4e60-b485-093866a8b1d1)
![image](https://github.com/jedjoud10/game-theory-test/assets/34755598/e0896ee4-9001-4035-b68c-cf67af5650ba)
![image](https://github.com/jedjoud10/game-theory-test/assets/34755598/8e0b8f44-dc4d-475f-b289-862d410da2d1)


# LICENSE
Licensed under
 * MIT license
   ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)
