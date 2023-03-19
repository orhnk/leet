#!/usr/bin/env python3
"""
796. Rotate String
Easy
2.6K
108
Companies
Given two strings s and goal, return true if and only if s can become goal after some number of shifts on s.

A shift on s consists of moving the leftmost character of s to the rightmost position.

For example, if s = "abcde", then it will be "bcdea" after one shift.
"""

# class Solution(object):
def rotateString(s, goal):
    """
    :type s: str
    :type goal: str
    :rtype: bool
    """
    return len(s) == len(goal) and goal in s + s

# Driver Code:
s = "abcde"
goal = "cdeab"
print("Input: s = " + s + ", goal = " + goal)
result = rotateString(s, goal)
# Output: true
