input_file_1 = open("input.txt", 'r')

def sol_1A(input_file):
    input_text = input_file.readlines()
    depths = [int(line) for line in input_text]
    print(depths)
    count = 0
    for i in range(0, len(depths)-1):
        if depths[i] < depths[i+1]:
            count+=1
    return count

ans_1A = sol_1A(input_file_1)
print("Number of increases: ", ans_1A)


def sol_1B(input_file):
    input_text = input_file.readlines()
    depths = [int(line) for line in input_text]
    triples = [depths[i] + depths[i+1] + depths[i+2] for i in range(0,len(depths)-2)]
    count = 0
    for i in range(0, len(triples)-1):
        if triples[i] < triples[i+1]:
            count+=1
    return count

ans_1B = sol_1B(input_file_1)
print("Number of triplet increases: ", ans_1B)

