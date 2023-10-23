all_gates = list(map(lambda line: tuple(map(lambda s: s.strip(), line.replace('AND', '&').replace('OR', '|').replace('LSHIFT', '<<').replace('RSHIFT', '>>').replace('NOT', '~').replace('as', 'as_').replace('is', 'is_').replace('if', 'if_').replace('in','in_').split('->'))), open("d7_input.txt").readlines()))
while len(all_gates) != 0:
    all_gates = list(filter(lambda line: exec(f"global {line[1].strip()}; {line[1].strip()}= {line[0]}") is not None if all(map(lambda val: (val in globals().keys()) or (val in ['<<','>>','~','&','|']) or (val.isnumeric()), map(lambda x: x.strip(), line[0].split(' ')))) else True, all_gates))
print(a)
