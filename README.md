# Simple Prisoner's Dilemna Test I decided to implement after watching Veritasium's video about it
*(also as an efficient way to procrastinate studying for my calculus test)*

Everything is in one file at the moment, and you can implement your own strategy using the ``Strategy`` struct that has the ``decide`` function which returns either ``Decision::Share`` or ``Decision::Steal``.
At the top of the file there are constants for # of rounds played, # of total entities per strategy pool, and how points are distributed. After two pools go against each other, the ``score`` function is called on each strategy to change how each entity should behave based on how well it did last round

Implemented in one afternoon so most of the code is unoptimized and very ugly unfortunately.
