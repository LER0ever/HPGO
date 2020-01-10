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
total_micro_batch = 8
# F = [1.26216 / total_micro_batch * 0.5, 0.009, 1.209919999999999 / total_micro_batch * 0.5]
# B = [1.26216 / total_micro_batch * 0.5, 0.009, 1.209919999999999 / total_micro_batch * 0.5]
# F = [8, 1, 8]
# B = [8, 1, 8]

# print(F)
flag = True
while flag:
    scale_backward = True
    max_diff = 0.35
    min_length = 0.1
    F = [random(), random(), random()]
    # F = [0.8 * 8 / 7, 0.8, 0.7]
    # F = [0.5, 0.5, 0.5]
    if max(F) - min(F) > max_diff or max(F) < min_length:
            continue
    if scale_backward:
        B = [F[0]* 1.5, F[1] * 1.5, F[2] * 1.5]
    else:
        B = [random(), random(), random()]
        if max(B) - min(B) > max_diff or max(B) < min_length:
            continue

    print("Exploring with F and B:")
    print("F=", F)
    print("B=", B)

    best_l, best_phi, best_f, best_b = PipelineArrangementHelper(F, B, total_micro_batch, [3, 2, 1])
    # print(best_l)
    normal_l = best_l
    normal_f = best_f
    normal_b = best_b

    print("Exploring Better Pipeline...")
    for i in range(1, total_micro_batch + 1):
        for j in range(1, i + 1):
            for k in range(1, j + 1):
                # for m in range(1, k+1):
                #     for n in range(1, m+1):
                new_l, phi, f, b = PipelineArrangementHelper(F, B, total_micro_batch, [i, j, k])
                if new_l < best_l:
                    print("Got better arrangement, l = ", new_l)
                    best_l = new_l
                    best_phi = phi
                    best_f = f
                    best_b = b
    # Conditions
    ## less than 3,2,1
    ### (best_phi[0] < 3 or (best_phi[0] == 3 and best_phi[1] < 2))
    ## All stage <= 3
    ### best_phi[0] <=3
    ## speedup more than 10%
    ### (normal_l - best_l) / best_l > 0.1
    ## Comm < Comp
    ### F[1] < F[0] and F[1] < F[2]
    if best_l < normal_l:
        print("Best arrangement:")
        print(best_l)
        print("f=")
        print(
            "\n".join(
                [
                    "\t".join([",".join([str(round(flt, 6)) for flt in cell]) for cell in row])
                    for row in best_f
                ]
            )
        )
        print("b=")
        print(
            "\n".join(
                [
                    "\t".join([",".join([str(round(flt, 6)) for flt in cell]) for cell in row])
                    for row in best_b
                ]
            )
        )
        print(best_phi)
        print("Speedup: ", (normal_l - best_l) / best_l * 100, "%")
        draw_pipeline_comparison(normal_l, normal_f, normal_b, best_l, best_f, best_b)
        flag = False

