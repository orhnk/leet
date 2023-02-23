



def reverse_string(inp):
    inp = inp.split(' ')
    for word in inp:
        wlen = len(word) - 1
        for i in range(wlen):
            word[i], word[wlen - i] = word[wlen - i], word[i]
    return " ".join(inp)

# Testing:
inp = "Let's take LeetCode contest"
out = "s'teL ekat edoCteeL tsetnoc"

reverse_string(inp)
print(out == inp)

