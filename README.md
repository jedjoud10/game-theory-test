# Simple Prisoner's Dilemna Test I decided to implement after watching Veritasium's video about it
*(also as an efficient way to procrastinate studying for my calculus test)*

Everything is in one file at the moment, and you can implement your own strategy using the ``Strategy`` struct that has the ``decide`` function which returns either ``Decision::Share`` or ``Decision::Steal``.
At the top of the file there are constants for # of rounds played, # of total entities per strategy pool, and how points are distributed. After two pools go against each other, the ``score`` function is called on each strategy to change how each entity should behave based on how well it did last round

Implemented in one afternoon so most of the code is unoptimized and very ugly unfortunately.

All thanks to the ``textplots`` crate and ``owo-colors`` for beautiful teminal graphs & colors

![image](https://github.com/jedjoud10/game-theory-test/assets/34755598/58113cb1-6ed8-4bda-85cc-565a542a37ec)

![image](https://github.com/jedjoud10/game-theory-test/assets/34755598/5209c011-d17d-4d97-bd56-43b1389bb013)


# LICENSE
Licensed under
 * MIT license
   ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)
