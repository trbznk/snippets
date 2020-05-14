import requests
import time

from bs4 import BeautifulSoup
from tqdm import tqdm

r = requests.get("https://www.pablopicasso.org/picasso-paintings.jsp")

soup = BeautifulSoup(r.text, 'html.parser')
imgs = soup.select("td a img")

for img in tqdm(imgs):
    fn = img['src'].split('/')[-1]
    url = f"https://www.pablopicasso.org/images/paintings/{fn}"
    r = requests.get(url)
    path = f"picasso/{fn}"
    with open(path, "wb") as f:
        f.write(r.content)
    time.sleep(1)