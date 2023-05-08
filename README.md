# final_project
##DS 210 FINAL PROJECT

Introduction:
For my project, I chose to study the betweenness centrality of the english wikipedia database. The dataset I used was a snapshot taken of the website in 2013 by Stanford: https://snap.stanford.edu/data/enwiki-2013.html. This dataset contained articles and the hyperlinks within them to other articles. In total, there were 	4203323 nodes and	101311614 directed edges. From the dataset, there were two files, "enwiki-2013.txt" and "enwiki-2013-names.csv", where the txt file contained the nodes and directed edges, and the csv represented a hash map from the nodes to wikipedia articles. 

Methods:
First, I had to read the dataset into a usable format, where I converted it into a graph. In order to compute betweenness centrality, I had to first compute all shortest paths within the dataset. At first, I tried to use BFS, since the dataset had unweighted edges. However, I was unable to generate an efficient algorithm that could record the nodes traveled on during the shortest paths. Instead, I used the library "Fast Paths" (https://github.com/easbar/fast_paths). This library uses djikstra's algorithm in combination with contraction hierarchies on the original graph to expedite the algorithmn. Considering the immense scale of the dataset, I had believed this would make a total centrality measure possible. With all the shortest paths, betweenness can be computed by finding the fraction of shortest paths a node is in. Then, I would only have to find the maximum value of the betweenness measure. 
