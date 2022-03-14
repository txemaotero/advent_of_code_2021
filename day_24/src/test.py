with open('../input.txt') as f:
    commands = []
    aux = None
    for line in f:
        line = line.strip()
        if not line:
            continue
        if aux is None:
            aux = [line]
            continue
        if line.startswith('inp'):
            commands.append(aux)
            aux = [line]
        else:
            aux.append(line)

commands.append(aux)
    
for index, all_cmd in enumerate(zip(*commands)):
    s = set(all_cmd)
    if len(s) > 1:
        print(all_cmd[0])
        print('Command: ', index, "->", ' # '.join(s.split()[-1] for s in all_cmd))
    else:
        print(s)
