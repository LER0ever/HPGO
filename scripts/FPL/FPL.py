# Full Pipeline Length Calculation with Double Recursion
def FPL(F, B, M, phi):
    assert (len(F) == len(B))
    S = len(F)
    f = [[None for x in range(S)] for i in range(M)]
    b = [[None for x in range(S)] for i in range(M)]
    f[0][0] = (0, F[0])
    for i in range(1, S):
        f[0][i] = (f[0][i - 1][0], f[0][i - 1][1] + F[i])

    # Memorized Search Helper for f
    def fn_f(i, x):
        if i < 0 or i >= M or x < 0 or x >= S:
            return (0, 0)
        if f[i][x] is not None:
            return f[i][x]
        cur_f = max(
            [fn_b(i - phi + x, x)[1], fn_f(i - 1, x)[1], fn_f(i, x - 1)[1]])
        f[i][x] = (cur_f, cur_f + F[x])
        return f[i][x]

    # Memorized Search Helper for b
    def fn_b(i, x):
        if i < 0 or i >= M or x < 0 or x >= S:
            return (0, 0)
        if b[i][x] is not None:
            return b[i][x]
        cur_b = max([fn_f(i + phi - x - 1, x)[1], fn_b(i, x + 1)[1], fn_b(i - 1, x)[1]])
        b[i][x] = (cur_b, cur_b + B[x])
        return b[i][x]

    # Start the double recursion, and return the end of bottom right block
    return fn_b(M - 1, 0)[1]

# F, B example: 2 stage with network, [Comp, Comm, Comp]
F = [0.3, 0.09, 0.3]
B = [0.6, 0.09, 0.6]
total_micro_batch = 4
initial_micro_batch = 3
l = FPL(F, B, total_micro_batch, initial_micro_batch)
print(l)


