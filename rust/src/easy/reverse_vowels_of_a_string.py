
"""
345. Reverse Vowels of a String
Easy
3.2K
2.4K
Companies
Given a string s, reverse only all the vowels in the string and return it.

The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both lower and upper cases, more than once.
"""


# class Solution(object):
def reverse_vowels(s):
    vowels = ['a', 'e', 'i', 'o', 'u']
    for i in range(len(s)):
        for j in range(len(vowels)):
            if s[i] == vowels[j]:
                tmp = s[i]
                s[i] = s[len(s) - i]
                s[len(s) - i] = tmp
    return s

# Driver code
test = "hello"
test2 = "leetcode"

res1 = reverse_vowels(test)
res2 = reverse_vowels(test2)

print(res1)
print(res2)
