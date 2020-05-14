import numpy as np
import matplotlib.pyplot as plt
import random
import glob

from PIL import Image
from tqdm.notebook import tqdm
from collections import Counter

COLORS = {
    "red": [255, 0, 0],
    "green": [0, 255, 0],
    "blue": [0, 0, 255],
    "white": [255, 255, 255],
    "black": [0, 0, 0],
    "yellow": [255, 255, 0]
}

class Img2Pimg: 
    def __init__(self, src, template, square_size=50, step_size=25, th=100):
        self.src = src
        self.template = template
        self.square_size = square_size
        self.step_size = step_size
        self.th = th
            
    def run(self):
        self.imgs = [Image.open(path).convert("RGB") for path in glob.glob(f"{self.src}/*")]
        
        self.colors = {}
        for key, value in dict(Counter([color for color in np.array(self.template).flatten() if color is not None])).items():
            self.colors[key] = (value, COLORS[key])
        
        self.extract()
        self.compose()
        
    def extract(self):
        squares = {color: [] for color in self.colors}

        for img in tqdm(self.imgs):
            a = np.asarray(img)
            height, width, _ = a.shape

            all_squares = []
            for h in range(0, height-self.square_size+1, self.step_size):
                for w in range(0, width-self.square_size+1, self.step_size):
                    square = a[h:h+self.square_size, w:w+self.square_size]
                    all_squares.append(square)

            random.shuffle(all_squares)
            is_ready = False
            for square in all_squares:
                if is_ready:
                    break
                for color, value in self.colors.items():
                    amount, rgb = value
                    diff = abs(square-rgb).mean()
                    if diff < self.th:
                        squares[color].append(square)
                        self.colors[color] = (amount-1, rgb)
                is_ready = True
                for amount, rgb in self.colors.values():
                    if amount > 0:
                        is_ready = False
                        break

        self.squares_ = squares
        
    def compose(self):
        size = len(list(self.squares_.values())[0][0])
        height, width = np.array(self.template).shape
        height, width = height*size, width*size
        collage = Image.new(mode="RGB", size=(width, height))

        for key in self.squares_:
            random.shuffle(self.squares_[key])

        for x in range(len(self.template)):
            for y in range(len(self.template[0])):
                if self.template[x][y]:
                    color = self.template[x][y]
                    img = Image.fromarray(self.squares_[color].pop(0), mode="RGB")
                    collage.paste(img, box=(y*size, x*size))
                else:
                    img = Image.new("RGB", (size, size), (255, 255, 255))
                    collage.paste(img, box=(y*size, x*size))

        self.img_ = collage