outfile = open("outwords.txt", 'w')
with open('words.txt') as wordfile:
    for line in wordfile:
        if len(line) > 5:
            outfile.writelines(line)
outfile.close()
