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
            
for index, all_cmd in enumerate(zip(*commands)):
    s = set(all_cmd)
    print(s)
    if len(s) > 1:
        pass
        # print('Command: ', index, "->", ' # '.join(all_cmd))
