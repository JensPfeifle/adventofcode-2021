input = open("8.in").read().strip().split("\n")
#input = ["acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"]

digits: dict[int, set[str]] = {}
rows = []

def decode(digits: dict[int,set[str]], input: str) -> int:
    signals = set(iter(input))
    for k,v in digits.items():
        if signals == v:
            return k

for line in input:
    inputs, outputs = line.split("|")
    inputs = inputs.strip().split(" ")

    for i in inputs:
        if len(i) == 2:
            digits[1] = set(iter(i))
        if len(i) == 3:
            digits[7] =set(iter(i))
        if len(i) == 4:
            digits[4] = set(iter(i))
        if len(i) == 7:
            digits[8] = set(iter(i))

    for i in inputs:
        if len(i) == 5:
            # digits[2], digits[3], or digits[5]
            digit = set(iter(i))
            a = len(digit - digits[1])
            b = len(digit - digits[4])
            c = len(digit - digits[7])
            if (a,b,c) == (4,2,3):
                digits[5] = digit
            elif (a,b,c) == (3,2,2):
                digits[3] = digit
            elif (a,b,c) == (4,3,3):
                digits[2] = digit
            else:
                raise Exception
        if len(i) == 6:
            # digits[0], digits[6], or digits[9]
            digit = set(iter(i))
            a = len(digit - digits[1])
            b = len(digit - digits[4])
            c = len(digit - digits[7])
            if (a,b,c) == (4,2,3):
                digits[9] = digit
            elif (a,b,c) == (5,3,4):
                digits[6] = digit
            elif (a,b,c) == (4,3,3):
                digits[0] = digit
            else:
                raise Exception

    print(outputs.strip().split(" "))
    row = ""
    for output_digit in outputs.strip().split(" "):
        row += str(decode(digits, output_digit))
    print(row)
    rows.append(row)

    print(sum([int(n) for n in rows]))
    

