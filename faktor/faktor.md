# Faktor

The impact factor of a scientific journal is a measure reflecting the average number of citations to articles published in science journals. For this task we are using a simplified formula for calculating the impact factor:

`factor = Total count of citations/Total numbers of articles`
 
Rounding is always performed upwards. For example the impact factor of the “Journal for ore research and time wasting” that published 38 articles cited 894 times is 894/38 = 23.53 rounding up to 24.

You are the editor of one scientific journal. You know how many articles you are going to publish and the owners are pushing you to reach a specific impact factor. You are wondering how many scientists you will have to bribe to cite your articles to meet the owners demands. Since money is tight you want to bribe the minimal amount of scientists. Each bribed scientist buys you a single citation.

**Input**

First and only line of input will contain 
2 integers, A (1 <= A <= 100), the number, the number of articles you plan to publish and I (1 <= I <= 100) the impact factor the owners require.

**Output**

The first and only line of output should contain one integer, the minimal number of scientists you need to bribe.
```
      Sample Input 1    Sample Output 1
      38 24             875
      Sample Input 2    Sample Output 2
      1 100             100
      Sample Input 3    Sample Output 3
      10 10             91
```
