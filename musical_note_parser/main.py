import math

from collections import deque
from dataclasses import dataclass
from fractions import Fraction


@dataclass
class Note:
    name: str
    length: Fraction


def parse_length(length_string):
    base = Fraction("1/4")
    for symbol in length_string:
        if symbol == "+":
            base *= 2
        elif symbol == "-":
            base /= 2
    return base


with open("entchen.notes") as f:
    song = deque(f.read().split())
    tact = Fraction(song.popleft())

    notes = []
    for token in song:
        name, length_string = "", ""
        for chr in token:
            if chr.isalpha():
                name += chr
            elif chr in "+-":
                length_string += chr

        length = parse_length(length_string)
        note = Note(name, length)
        notes.append(note)
    
print("Alle meine Entchen")
print("Tact:", tact)
print("----")
index = 0
for note in notes:
    index += note.length        
    print(math.ceil(index/tact), "\t", note)
