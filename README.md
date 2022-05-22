# Goals
This rust library aims to provide a simple and efficient but powerful implementation of a spaced repetition algorithm

# Design
- simple model that is easy to understand and modify.
  - we model forgetting as a decaying exponential function with a "forgetting rate" f
  - this rate is then described as the quotient of the item difficulty (d) and memory strength (s)
$$P(t) = e^{-ft} = e^{-\frac{d}{m}t}$$

- tries to make as little assumptions about item content/data as possible.
- it does make the assumption though that the user in some way evaluates how each review event went.

- provides two ways to adjust the result of the algorithm
  - for each review event the user inputs whether the item was/wasn't too difficult. This is then used to adjust the item difficulty value (which then impacts the value of the next interval)
  - when several cards has matured the ratio between the expected recall probability and the actual recall probability can be compared. This is then used to introduce an "adjusting factor". This factor is used in the computation of the next forgetting rate.

for a better understanding of how the code works, read the source! (It is under 150 lines)

## Data driven vs user evaluated
Modeling the forgetting curve requires including a bunch of parameters that can be quite arbitrary. To better set these values one would want a dataset of real review events and then try to fit the model to those items that performed the best. This approach though requires familiarity with such methods (which I don't have). Instead of doing this I'll instead rely on trying to best-guess some values and then introduce a way for the user to continuously evaluate the performance of the model. My hope is that I this way empirically can arrive at better default values for further and further revisions until I can create a fully data driven approach.

## Limitations
This library does not:
- handle failed reviews (forgotten items). Instead the library user needs to decide how such items needs to be handled.
- automatically update all parameters to better model the real forgetting curve

# Licence
see ./LICENCE
