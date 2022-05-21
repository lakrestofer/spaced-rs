# Flow of thoughts

this document may or may not contain anything of relevance to the code as it looks right now. It serves mainly as a way for me to write down any thoughs I have on the project. It can be seen as my attempt to structure my thoughts a bit more by putting them down into ascii :D.

# 21 May 2022
There are many features I want in a spaced repetition algorithm. It should be content agnostic. This means that the item being reviewed should be able to take any form with the only common piece of data between the items being that related to scheduling.

A note on terminology. since I want a system that is not only designed around flashcards I instead refer to the item being continiously reviewed as simply that, an 'item'. I might every now and then slip up from this choice of words though and in those cases simply interpret me as meaning 'item'.


- It seems that the modeling of human memory curve (probability of recalling an item) can be done using a exponential function such as the one below (assuming a positive value of f). 
$$e^{-ft}$$
- such a model allows for very efficient algorithms.
- the rate at which a memory degrades can arbitrarily be calculated using any choice of parameters.
- when f has been calculated from these parameters then next revision event targeting some recall probability P can the be easily calculated.
$$ t = \frac{P}{-f} $$
- if the user is allowed to give his/her own estimation.

some good choices of parameters. (sign after parameter name signifies whether it increases the rate f or if it decreases it):
  - difficulty ++
  - memory strenght --
  - user evaluation score --/++ (might increase difficulty or memory strenght instead of acting as a new parameter).

I do not want a timer that automatically fails a reviewing event. Some items could contain a promt that is is not supposed to be answered only a few seconds. Instead the user should be presented with how long their previous revision took and themselves make the judgement on their performance (using the 'user evalutation score').

### How should we set/update the values of our parameters?
A problem is that we as the programer initially will choose very arbitrary parameters for all these values and how they should be changed over time. By allowing the user to also evaluate the performance of the algorithm continiously we can update these parameters to take better default values. 

It is very important that data is collected in some way such that the result of some parameter choice can be measured.

By having a system with such design I can initially make an educated guess on what value these default parameters should have and then use the data gathered to improve these default values.

### Handle over and under reviewing

when choosing items to review, we should prioritize younger items first. The time intervall between revission events for such items are shorter and therefore missing a day or two means a more dramatic change in recall probability than for a more mature item.

We therefore when choosing items we want to compute how much they are overdue (or underdue) and relate that to their intervall. We then prioritizes items were that ratio is larger.
