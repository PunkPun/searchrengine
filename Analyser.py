from os.path import join
from os import walk
import os 
import sys
from collections import Counter

script_directory = os.path.dirname(os.path.abspath(sys.argv[0]))
for (dirpath, dirnames, filenames) in walk(script_directory):
	for engine in dirnames:
		# print(engine)
		path = join(dirpath, engine)
		for (enginePath, dirs, files) in walk(path):
			files.sort()
			for file in files:
				if file.endswith('.txt'):
					f = open(join(enginePath, file), "r")
					link = f.readline().strip()
					counter = Counter(f.read().split())
					print(link + '\n' + str(counter.most_common(30)) + '\n')