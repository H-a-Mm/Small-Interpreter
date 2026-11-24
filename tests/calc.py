def tokenize(expr):
    for op in "+-*/":
        if op in expr:
            lhs, rhs = expr.split(op)
            return [lhs, op, rhs]

def interpret(tokens):
    if tokens[1] == "+":
        return int(tokens[0]) + int(tokens[2])
    elif tokens[1] == "-":
        return int(tokens[0]) - int(tokens[2])
    elif tokens[1] == "*":
        return int(tokens[0]) * int(tokens[2])
    elif tokens[1] == "/":
        return int(tokens[0]) / int(tokens[2])

def main():
    user = input(">> ")
    tokens = tokenize(user)

    print(">>", interpret(tokens))

main()