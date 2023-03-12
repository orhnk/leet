#!/usr/bin/env python3

"""
821. Shortest Distance to a Character
Easy
2.7K
141
Companies
Given a string s and a character c that occurs in s, return an array of integers answer where answer.length == s.length and answer[i] is the distance from index i to the closest occurrence of character c in s.

The distance between two indices i and j is abs(i - j), where abs is the absolute value function.
"""

def run(s, c):
    # "loveleetcode", "e" ==> [3,2,1,0,1,0,0,1,2,2,1,0] -> distance to the closest letter ('e')

    nl = []
    for i in range(len(s)):
        print(i)
        nl.append(closest_char(s, c, i, len(s)))

    # print(res)
    # l = len(s)
    # fidx = find_closest(s, c, l, 0)
    # for i in range(l):
    #     e = fidx - i
    #     if e < 0:
    #         e = 1 # find_closest(s,c,l,i) - i
    #    nl.append(e)

    # nl = [fidx - x for x in range(l) if x == 0 fidx = find_closest(s, c, l, i) ] # Bro I don't know how to use list comprehensions lol!
    return nl

def find_closest(s, c, l, st):
    for i in range(st, l): # start to the end
        if s[i] == c:
            return i

def closest_char(s, c, i, l): # str, char, idx, len
    # "___i__x_" 'x' 7 4
    li = l - i # Left slots
    m = min(li, i)
    for j in range(1,m):
        if s[j - 1] == c:
            return i - 1;
        if s[j + 1] == c:
            return j + 1;
    for j in range(m,l):
        if s[j] == c:
            return j

    return None


if __name__ == "__main__":
    s = "loveleetcode"
    c = "e"
    e = run(s, c)
    ex = [3,2,1,0,1,0,0,1,2,2,1,0]
    print(e)
    print(ex)
