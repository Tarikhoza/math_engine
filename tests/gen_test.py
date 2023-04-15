import random

letters = ["x","x^2","x^3","x^4", "y","y^2","y^3","y^4", "z","z^2","z^3","z^4" ]
operators = ["+" "-", "/", "*"]

def create_quadratic_test():
    a = random.randint(1, 10)
    b = random.randint(-10, 10)
    c = random.randint(-10, 10)
    problem = f"{a}x^{{2}}+{b}x+{c}=0"
    solution = [(-b + (b**2 - 4*a*c)**0.5)/(2*a), (-b - (b**2 - 4*a*c)**0.5)/(2*a)]
    return problem, solution       

with open("tests.txt","w") as file:
    for i in range(100):
        problem, solution = create_quadratic_test()
        file.write(f'parser_eq!("{problem}","{solution}");\n')







