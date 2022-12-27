def chain(l):
    for x in l:
        if isinstance(x,int):
            yield x
        else:
            yield from chain(x)


print(list(chain([1,2,[3,[4,5]],2])))