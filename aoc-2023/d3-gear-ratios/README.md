
# Gear Ratio Action Plan

Schematic is a large random input, I need to

- Identify all numeric strings
- Identify all symbols
- replace numeric strings with an id number
- find all id numbers adjacent to symbols
- add corresponding numeric strings

- write a function that takes a sym_coord and
  searches around surrounding area for matching x,y
- See if any values match with num_coord and save matching ID/Index in id vector
  Check for Repeats!
- match num_id struct against id vector, iterating along len()
- if equal add to a final Vec and then sum all values
