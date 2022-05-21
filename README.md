# Goals
This rust library aims to provide a simple and efficient but powerful implementation of a spaced repetition algorithm

# Design

When modeling the forgetting behaviour we want a function that takes a moment in time and outputs the probability of recalling the item. This is usually modelled using an exponential function like the one below. 'f' then denotes a metric of how fast this probability is decreasing (forgetting factor). This is affected by how many times the item has been reviewed, user capabilities, Aby how difficult the item is etc.

\[P(t) = e^-ft\]

The hard part of designing a spaced repetition algorithm lies in updating the value of f through time and reviews. The goal would be an algorithm that manages to update this value in a datadriven way such that there can be made guarantes on the algorithm's efficiency. Several implementations instead rely on the user providing a measure of how well they performed. This can then be used to measure if the exponential curve was decreasing to quickly or slowly. This keeps the algorithm simple while probably actually modeling the curve quite well.

## Parameters used to calculate f
As mentioned above there exists several pieces of data that probably affect the value of f.

Two simple parameters that will be specific for every item could be the below:
- Difficulty - a measure of the inherent complexity of an item. A high difficulty will drive up the value of f
- Memory strength - a measure of how well consolidated a given trace is in the brain. An item with a good memory strength will drive the value of f down.

Given these two parameters, how should an expression containing them look like? I'm guessing that taking a division between the two would suit us quite well here. This will allow us to scale the result quite easily and allow us to insert other parameters such as the user measure mentioned above.

```
f = (Difficulty / Memory-strength)
```
## Default values of paremeters and how they are to be updated

Another tricky part. The time interval between every revision event (when the recall probability reaches some threshold) should sort of double between every event. <!--TODO-->

