from random import random


def draw_pipeline_comparison(l1, f1, b1, l2, f2, b2):
    import cairo

    padding = 10
    block_height = 30
    width_scale = 50
    border_width = 1
    l1_bottom = block_height * 3
    l2_bottom = block_height * 7
    WIDTH = round(max(l1, l2) * width_scale) + padding + 1
    HEIGHT = block_height * 10
    surface = cairo.ImageSurface(cairo.FORMAT_RGB24, WIDTH, HEIGHT)
    ctx = cairo.Context(surface)

    ctx.rectangle(0, 0, WIDTH, HEIGHT)
    ctx.set_source_rgb(1, 1, 1)
    ctx.fill()

    f_colors = [(213, 232, 212), (248, 206, 204), (218, 232, 252)]
    borders = [(130, 179, 102), (184, 84, 80), (108, 142, 191)]
    b_colors = [(96, 169, 23), (216, 0, 115), (27, 161, 226)]

    def std_rgb(colors):
        for i in range(len(colors)):
            colors[i] = (colors[i][0] / 255, colors[i][1] / 255, colors[i][2] / 255)
        return colors

    f_colors, borders, b_colors = std_rgb(f_colors), std_rgb(borders), std_rgb(b_colors)
    # print(f_colors)

    for i in range(len(f1)):
        for j in range(len(f1[i])):
            ctx.rectangle(
                padding + f1[i][j][0] * width_scale,
                l1_bottom - j * block_height,
                (f1[i][j][1] - f1[i][j][0]) * width_scale,
                block_height,
            )
            ctx.set_source_rgb(*f_colors[j])
            ctx.fill_preserve()
            ctx.set_source_rgb(*borders[j])
            ctx.set_line_width(border_width)
            ctx.stroke()
    for i in range(len(b1)):
        for j in range(len(b1[i])):
            ctx.rectangle(
                padding + b1[i][j][0] * width_scale,
                l1_bottom - j * block_height,
                (b1[i][j][1] - b1[i][j][0]) * width_scale,
                block_height,
            )
            ctx.set_source_rgb(*b_colors[j])
            ctx.fill_preserve()
            ctx.set_source_rgb(*borders[j])
            ctx.set_line_width(border_width)
            ctx.stroke()
    for i in range(len(f2)):
        for j in range(len(f2[i])):
            ctx.rectangle(
                padding + f2[i][j][0] * width_scale,
                l2_bottom - j * block_height,
                (f2[i][j][1] - f2[i][j][0]) * width_scale,
                block_height,
            )
            ctx.set_source_rgb(*f_colors[j])
            ctx.fill_preserve()
            ctx.set_source_rgb(*borders[j])
            ctx.set_line_width(border_width)
            ctx.stroke()
    for i in range(len(b2)):
        for j in range(len(b2[i])):
            ctx.rectangle(
                padding + b2[i][j][0] * width_scale,
                l2_bottom - j * block_height,
                (b2[i][j][1] - b2[i][j][0]) * width_scale,
                block_height,
            )
            ctx.set_source_rgb(*b_colors[j])
            ctx.fill_preserve()
            ctx.set_source_rgb(*borders[j])
            ctx.set_line_width(border_width)
            ctx.stroke()
    surface.write_to_png("pipeline_comparison.png")


# phi is now an array of S
def PipelineArrangementHelper(F, B, M, phi):
    assert len(F) == len(B)
    S = len(F)
    f = [[None for x in range(S)] for i in range(M)]
    b = [[None for x in range(S)] for i in range(M)]
    f[0][0] = (0, F[0])
    for i in range(1, S):
        f[0][i] = (f[0][i - 1][1], f[0][i - 1][1] + F[i])

    # Memorized Search Helper for f
    def fn_f(i, x):
        if i < 0 or i >= M or x < 0 or x >= S:
            return (0, 0)
        if f[i][x] is not None:
            return f[i][x]
        cur_f = max([fn_b(i - phi[x], x)[1], fn_f(i - 1, x)[1], fn_f(i, x - 1)[1]])
        f[i][x] = (cur_f, cur_f + F[x])
        return f[i][x]

    # Memorized Search Helper for b
    def fn_b(i, x):
        if i < 0 or i >= M or x < 0 or x >= S:
            return (0, 0)
        if b[i][x] is not None:
            return b[i][x]
        cur_b = max(
            [fn_f(i + phi[x] - 1, x)[1], fn_f(i, x)[1], fn_b(i, x + 1)[1], fn_b(i - 1, x)[1]]
        )
        b[i][x] = (cur_b, cur_b + B[x])
        return b[i][x]

    # Start the double recursion, and return the end of bottom right block
    max_length = 0.0
    for i in range(M):
        max_length = max(fn_b(i, 0)[1], max_length)
    return max_length, phi, f, b


# F, B example: 2 stage with network, [Comp, Comm, Comp]
total_micro_batch = 12
# F = [1.26216 / total_micro_batch * 0.5, 0.009, 1.209919999999999 / total_micro_batch * 0.5]
# B = [1.26216 / total_micro_batch * 0.5, 0.009, 1.209919999999999 / total_micro_batch * 0.5]
# F = [8, 1, 8]
# B = [8, 1, 8]

# print(F)
flag = 1000000
right = 0
left = 0
m_diff = 0.0
while flag > 0:
    flag -= 1
    scale_backward = True
    max_diff = 0.6
    max_ratio = 1.28
    min_length = 0.1
    F1 = [random(), random()]
    F1.append(2-F1[0]-F1[1])
    # F = [0.8 * 8 / 7, 0.8, 0.7]
    # F = [0.5, 0.5, 0.5]
    if max(F1) / min(F1) > max_ratio or max(F1) < min_length:
            continue
    B1 = [F1[0]* 1.5, F1[1] * 1.5, F1[2] * 1.5]

    F2 = [random(), random()]
    F2.append(2-F2[0]-F2[1])
    # F = [0.8 * 8 / 7, 0.8, 0.7]
    # F = [0.5, 0.5, 0.5]
    if max(F2) / min(F2) > max_ratio or max(F2) < min_length:
            continue
    B2 = [F2[0]* 1.5, F2[1] * 1.5, F2[2] * 1.5]


    # print("Exploring with F and B:")
    # print("F=", F)
    # print("B=", B)

    best_l1, _, _,_ = PipelineArrangementHelper(F1, B1, total_micro_batch, [5, 3, 1])
    best_l2, _, _,_ = PipelineArrangementHelper(F2, B2, total_micro_batch, [5, 3, 1])

    def find_Q(F, B):
        Q = 2
        if (F[1]+B[1]) * (total_micro_batch - 1) > (F[Q] + B[Q]) * total_micro_batch:
            Q = 1
        if Q == 1 and (F[0]+B[0]) * (total_micro_batch - 1) > (F[Q] + B[Q]) * total_micro_batch:
            Q = 0
        if Q == 2 and (F[0]+B[0]) * (total_micro_batch - 1) > (F[Q] + B[Q]) * total_micro_batch + F[1] + B[1]:
            Q = 0
        return Q

    Q1 = find_Q(F1, B1)
    Q2 = find_Q(F2, B2)
    
    est_l1 = (total_micro_batch - 1) * (F1[Q1] + B1[Q1])
    for i in range(Q1):
        est_l1 = est_l1 + F1[i] + B1[i]
    est_l2 = (total_micro_batch - 1) * (F2[Q2] + B2[Q2])
    for i in range(Q2):
        est_l2 = est_l2 + F2[i] + B2[i]

    if est_l1 < est_l2 and best_l1 > best_l2:
        left += 1
        print(abs(est_l1 - best_l1)/best_l1, abs(est_l2 - best_l2)/best_l2)
    else:
        right += 1

print(left, " / ", 1000000)