# final_project
## DS 210 FINAL PROJECT

### Introduction:
For my project, I chose to study the betweenness centrality of the english wikipedia database. The dataset I used was a snapshot taken of the website in 2013 by Stanford: https://snap.stanford.edu/data/enwiki-2013.html. This dataset contained articles and the hyperlinks within them to other articles. In total, there were 	4203323 nodes and	101311614 directed edges. From the dataset, there were two files, "enwiki-2013.txt" and "enwiki-2013-names.csv", where the txt file contained the nodes and directed edges, and the csv represented a hash map from the nodes to wikipedia articles. 

### Methods:
First, I had to read the dataset into a usable format, where I converted it into a graph. In order to compute betweenness centrality, I had to first compute all shortest paths within the dataset. At first, I tried to use BFS, since the dataset had unweighted edges. However, I was unable to generate an efficient algorithm that could record the nodes traveled on during the shortest paths. Instead, I used the library "Fast Paths" (https://github.com/easbar/fast_paths). This library uses djikstra's algorithm in combination with contraction hierarchies on the original graph to expedite the algorithmn. Considering the immense scale of the dataset, I had believed this would make a total centrality measure possible. With all the shortest paths, betweenness can be computed by finding the fraction of shortest paths a node is in. Then, I would only have to find the maximum value of the betweenness measure. The code outputs a list in descending order of the 10 highest centrality nodes.

### Difficulties:
The main challenge I had to overcome was the size of the network. I do not have access to a super computer, so I was limited by processing power and time. Running my code on the entire dataset was absolutely infeasible, so I decided to perform random sampling. Unfortunately, this loses the structure of the original network, so my conclusions are only useful in analyzing the structure of the random sample. However, I still believed this could produce interesting results.

### Conclusions:
From a random sample of 100 nodes, I produced the following centrality measure:
![result](https://user-images.githubusercontent.com/132901881/236840170-fd97bb7c-08e6-4777-b6a1-cfe95f97845d.png)

From these results, the node with the highest centrality measure was a between a plethora of nodes. Unfortunately, this is the result of computing centrality for only a random sample, where 
Additionally, from experimentation, I believe the complexity of at least the sample size is O(n^2). Computing 100 samples took approximately 30 minutes, whereas I predicted the total duration of computing 200 samples and 400 samples to be approximately two hours and eight hours respectively. I believe that my algorithm could be used to compute the total centrality of wikipedia if given enough time and parallel cores. 



### Citations:
I used the following libraries and resources:
Dataset: https://snap.stanford.edu/data/enwiki-2013.html
Fast Paths: https://github.com/easbar/fast_paths
