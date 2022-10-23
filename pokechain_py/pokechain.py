import json
import random
import copy
import threading
import time

MINUTES = 5

pokedex = json.load(open('pokedex.json'))
longest_chain = json.load(open('pokechain.json'))
old_longest = len(longest_chain)

nodes = {}
for p in pokedex:
  pair = p[0] + p[-1]
  if pair in nodes:
    nodes[pair] = nodes[pair] + 1
  else:
    nodes[pair] = 1

class ChainThread(threading.Thread):
  def __init__(self, nodes):
    threading.Thread.__init__(self)
    self.nodes = nodes

  def run(self):
    nodes = self.nodes
    try:
      self.find_chain([], nodes)
    except:
      return

  def find_chain(self, current, remainder):
    global stop_please
    if stop_please:
      raise Exception()

    if current:
      last_char = current[-1][1]
      neighbors = []
      for r in remainder.keys():
        if r[0] == last_char:
          neighbors.append(r)
    else:
      neighbors = remainder

    current_longest = current
    if neighbors:
      neighbors = random.sample(list(neighbors), len(list(neighbors)))
    for n in neighbors:
      new_remainder = copy.deepcopy(remainder)
      new_remainder[n] = new_remainder[n] - 1
      if new_remainder[n] == 0:
        del new_remainder[n]

      new_chain = self.find_chain(current + [n], new_remainder)
      if len(new_chain) > len(current_longest):
        current_longest = new_chain

    global result
    if len(current_longest) > len(result):
      result = current_longest

    return current_longest

def parse_chain(chain):
  pokechain = []
  pokedexcopy = copy.deepcopy(pokedex)
  for pair in chain:
    match = None
    try:
      match = next(x for x in pokedexcopy if x[0] + x[-1] == pair)
    except:
      return "There's an invalid pokemon in your chain!"
    
    pokedexcopy.remove(match)
    pokechain.append(match)
  
  return pokechain

for i in range(MINUTES * 10):
  result = []
  stop_please = False
  thread = ChainThread(nodes)

  thread.start()
  time.sleep(6)
  stop_please = True
  thread.join()
  if i % 10 == 0:
    print(str(int(i/10)+1) + "/" + str(MINUTES))

  if (len(result) > len(longest_chain)):
    longest_chain = result
    with open('pokechain.json', 'w') as outfile:
      json.dump(parse_chain(longest_chain), outfile)
    print("New longest chain found! Length: ", len(longest_chain))

if len(longest_chain) > old_longest:
  print("Found a new one this time!")
  print("Old Length: ", old_longest)
  print("New Length: ", len(longest_chain))
else:
  print("Nothing found this time :(")
  print("Length of chain is still: ", len(longest_chain))
