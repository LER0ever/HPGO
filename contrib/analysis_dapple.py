# from mc119496@alibaba-inc.com

import tensorflow as tf

import numpy as np
import pdb

flags = tf.app.flags
flags.DEFINE_string("model", "vgg19", "worker or ps")
flags.DEFINE_bool("only_pipe", False, "straight pipe")
flags.DEFINE_bool("pipe_dream_cut", False, "pipe dream")
flags.DEFINE_bool("inner_node", False, "pipe dream")
flags.DEFINE_integer("total_batch_size", 512, "total batch size")
FLAGS = flags.FLAGS


def allreduce_vol(weights, ring_len):
    # MB
    return 2 * float(ring_len - 1) / float(ring_len) * float(weights)


class VGG19(object):
    def __init__(self):
        # model setting
        # time:ms size:MB
        self.max_batch_size = 190
        self.min_batch_size = 8
        self.weights = 548.0
        # stage 1 for min_batch_size
        self.cut_features = 0.8 * self.min_batch_size
        if FLAGS.pipe_dream_cut:
            self.cut_features = 0.2 * self.min_batch_size
            self.comp_cut = [9.8, 3.0]  # 147, 3
            self.devices_cut = [15, 1]
            if FLAGS.inner_node:
                self.comp_cut = [1.5, 1.5]  # 42, 6
                self.devices_cut = [7, 1]
            self.weights_cut = [76, 471.6]
        else:
            self.cut_features = 0.8 * self.min_batch_size
            self.comp_cut = [50.0, 50.0]  # 150, 50
            self.devices_cut = [2, 1]
            self.weights_cut = [8.7, 539.3]

    def compute_time(self, batch_size):
        # ms
        return 150.0 / 32 * float(batch_size)


class XLNet(object):
    def __init__(self):
        # model setting
        # time:ms size:MB
        self.max_batch_size = 1
        self.min_batch_size = 1
        self.weights = 1460.0
        # stage 1 for min_batch_size
        self.cut_features = 2 * self.min_batch_size
        if FLAGS.only_pipe:
            self.comp_cut = [13.75] * 16
            self.devices_cut = [1] * 16
            self.weights_cut = [131] + [88] * 15
        else:
            self.comp_cut = [110.0, 110.0]
            self.devices_cut = [1, 1]
            self.weights_cut = [690, 770]

    def compute_time(self, batch_size):
        # ms
        return 220 / 1 * float(batch_size)


class BertLarge(object):
    def __init__(self):
        # model setting
        # time:ms size:MB
        self.max_batch_size = 6
        self.min_batch_size = 2
        self.weights = 1360.0
        # stage 1 for min_batch_size
        self.cut_features = 1.5 * self.min_batch_size
        self.comp_cut = [110.0, 110.0]
        self.devices_cut = [1, 1]
        self.weights_cut = [660, 700]

    def compute_time(self, batch_size):
        # ms
        return 206 / 2 * float(batch_size)


class ResNet50(object):
    def __init__(self):
        # model setting
        # time:ms size:MB
        self.max_batch_size = 128
        self.min_batch_size = 32
        self.weights = 100.0
        # stage 1 for min_batch_size
        self.cut_features = 0.4 * self.min_batch_size
        if FLAGS.only_pipe:
            self.comp_cut = [6.25] * 16
            self.devices_cut = [1] * 16
            self.weights_cut = [6.25] * 16
        else:
            self.comp_cut = [100.0, 100.0]
            self.devices_cut = [1, 1]
            self.weights_cut = [50, 50]

    def compute_time(self, batch_size):
        # ms
        return 100 / 32 * float(batch_size)


class Toy(object):
    def __init__(self):
        # model setting
        # time:ms size:MB
        self.max_batch_size = 6
        self.min_batch_size = 2
        self.weights = 1280.0
        # stage 1 for min_batch_size
        self.cut_features = 1.5 * self.min_batch_size
        if FLAGS.only_pipe:
            self.comp_cut = [20.0] * 16
            self.weights_cut = [80.0] * 16
            self.devices_cut = [1] * 16
        else:
            self.comp_cut = [160.0] * 2
            self.weights_cut = [640.0] * 2
            self.devices_cut = [1] * 2

    def compute_time(self, batch_size):
        # ms
        return 160 * float(batch_size)


class AmoebaNet(object):
    def __init__(self):
        # model setting
        # time:ms size:MB
        self.max_batch_size = 32
        self.min_batch_size = 8
        self.weights = 700.0
        # stage 1 for min_batch_size
        self.cut_features = 3 * self.min_batch_size
        self.four_stages = False
        if not self.four_stages:
            self.comp_cut = [106.0, 106.0]
            self.devices_cut = [1, 1]
            self.weights_cut = [160, 540]
        else:
            self.comp_cut = [53.0, 53.0] * 2
            self.devices_cut = [1, 1] * 2
            self.weights_cut = [80, 270] * 2

    def compute_time(self, batch_size):
        # ms
        return 212 / 8 * float(batch_size)


if __name__ == "__main__":
    if FLAGS.model == "bert":
        model = BertLarge()
    elif FLAGS.model == "xlnet":
        model = XLNet()
    elif FLAGS.model == "vgg19":
        model = VGG19()
    elif FLAGS.model == "resnet50":
        model = ResNet50()
    elif FLAGS.model == "amoeba":
        model = AmoebaNet()
    elif FLAGS.model == "toy":
        model = Toy()
    else:
        print("No model defined!")
        exit(1)
    # model setting
    # MB
    max_batch_size = model.max_batch_size
    min_batch_size = model.min_batch_size
    weights = model.weights
    # stage 1 for min_batch_size
    cut_features = model.cut_features
    comp_cut = model.comp_cut
    devices_cut = model.devices_cut
    weights_cut = model.weights_cut
    compute_time = model.compute_time

    # node setting
    # GB/s
    ethBdth_grpc = 0.8  # 0.8
    ethBdth_nccl = 3.0  # 1.0
    pciBdth = 10.0
    nvBdth_8 = 130.0
    nvBdth_4 = 80.0
    nvBdth_2 = 40.0

    # distribution setting
    GA = True

    # distribution strategy setting
    def dp(num_gpus_per_node, num_nodes, total_batch_size):
        ring_len = num_gpus_per_node * num_nodes
        batch_size = float(total_batch_size) / float(ring_len)
        print("DP: nodes: %d, gpus_per_node: %d" % (num_nodes, num_gpus_per_node))
        print("DP: total batch size: %d" % (batch_size * ring_len))
        print("DP: batch size: %d" % batch_size)
        if num_gpus_per_node == 1 and num_nodes == 1:
            print("DP: single gpu, no need to data parallel!")
            print("---------------------------------------------------")
            return -1.0
        if batch_size < min_batch_size:
            print("DP: too small batch size, or too many GPU cards")
            print("---------------------------------------------------")
            return -1.0

        if batch_size > max_batch_size:
            if not GA:
                print("DP: too large batch size, will be OOM")
                print("---------------------------------------------------")
                return -1.0
            else:
                GA_iters = (batch_size + max_batch_size - 1) / max_batch_size
        else:
            GA_iters = 1
        if num_nodes > 1:
            bdth = ethBdth_nccl
        elif num_gpus_per_node == 8:
            bdth = nvBdth_8
        elif num_gpus_per_node == 4:
            bdth = nvBdth_4
        elif num_gpus_per_node == 2:
            bdth = nvBdth_2

        comp = compute_time(batch_size)
        ar_vol = allreduce_vol(weights, ring_len)
        comm = ar_vol / bdth
        Q = comm / comp
        eff = 1.0 / (1.0 + Q)
        print("DP: Ring Length: %d" % ring_len)
        print("DP: AllReduce Vol: %.4f" % ar_vol)
        print("DP: comm/comp (%.4f / %.4f) ratio Q: %.4f" % (comm, comp, Q))
        if GA:
            print("DP: GA iterations: %d" % GA_iters)
        print("DP: data parallel efficiency: %.4f" % eff)
        print("---------------------------------------------------")
        return eff

    def pipe(num_gpus_per_node, num_nodes, total_batch_size):
        pass

    def dapple(num_gpus_per_node, num_nodes, total_batch_size):
        nstages = len(comp_cut)
        ndev = np.sum(devices_cut)
        if num_gpus_per_node * num_nodes < ndev:
            print("Dapple: gpu not enough!")
            print("---------------------------------------------------")
            return -1.0
        max_ring_len = num_gpus_per_node * num_nodes / ndev
        ring_len = min(max(num_gpus_per_node / max(devices_cut), 1), max_ring_len)
        unused = num_gpus_per_node * num_nodes - ring_len * ndev

        batch_size = float(total_batch_size) / float(ring_len)
        print("Dapple: nodes: %d, gpus_per_node: %d" % (num_nodes, num_gpus_per_node))
        print("Dapple: total batch size: %d" % (batch_size * ring_len))
        print("Dapple: batch size: %d" % batch_size)
        if num_gpus_per_node == 1 and num_nodes == 1:
            print("Dapple: single gpu, no need to data parallel!")
            print("---------------------------------------------------")
            return -1.0
        if batch_size < min_batch_size:
            print("Dapple: too small batch size, or too many GPU cards")
            print("---------------------------------------------------")
            return -1.0
        num_micro_batches = batch_size / min_batch_size
        print("Dapple: micro batch size: %d" % min_batch_size)
        print("Dapple: micro num batches per unit: %d" % num_micro_batches)
        print("Dapple: num of stages: %d" % nstages)
        if num_micro_batches < nstages:
            print("Dapple: too less micro batches to make pipeline full")
            print("---------------------------------------------------")
            return -1.0

        # placement
        if num_nodes <= 1:
            feat_bdth = nvBdth_2
        else:
            feat_bdth = ethBdth_grpc
        if num_nodes > nstages:
            ar_bdth = ethBdth_nccl
        elif num_gpus_per_node == 8:
            ar_bdth = nvBdth_8
        elif num_gpus_per_node == 4:
            ar_bdth = nvBdth_4
        elif num_gpus_per_node == 2:
            ar_bdth = nvBdth_2

        #    pdb.set_trace()
        bubble = nstages - 1 + nstages - 1
        print("Dapple: bubble count with comm: %d" % bubble)
        fcomm = cut_features / feat_bdth
        print("Dapple: feat comm: %.4f * 2.0" % fcomm)
        temp_cut = comp_cut + [2.0 * fcomm] * (nstages - 1)
        print("Dapple: temp cut %s " % temp_cut)
        max_comp_slice = np.argmax(temp_cut)
        if max_comp_slice >= nstages:
            print("feature map too large in pipeline!")
        #      return -1.0
        #    pdb.set_trace()
        comp_time = float(np.sum(temp_cut[: -(nstages - 1)]))
        one_pipeline_time = float(np.sum(temp_cut))
        total_time = (
            num_micro_batches * temp_cut[max_comp_slice]
            + one_pipeline_time
            - temp_cut[max_comp_slice]
        )
        dev_eff = []
        for i, dev in enumerate(devices_cut):
            for j in xrange(dev):
                deff = num_micro_batches * temp_cut[i] / total_time
                dev_eff.append(deff)
                print("Dapple: dev %d efficiency: %.4f" % (np.sum(devices_cut[:i]) + j, deff))
        unit_eff = np.mean(dev_eff)
        pdb.set_trace()
        print("Dapple: DP unit efficiency: %.4f" % unit_eff)
        if ring_len * np.max(devices_cut) == 1:
            eff = unit_eff
            print("Dapple: Just one Pipeline efficiency: %.4f" % eff)
            print("---------------------------------------------------")
        else:
            comp = compute_time(batch_size) / unit_eff
            max_comm = 0.0
            for i in xrange(len(weights_cut)):
                ar_vol = allreduce_vol(weights_cut[i], ring_len * devices_cut[i])
                if ring_len * devices_cut[i] > num_gpus_per_node:
                    ar_bdth = ethBdth_grpc
                comm = ar_vol / ar_bdth
                max_comm = max(max_comm, comm)
            comm = max_comm
            Q = comm / comp
            dp_eff = 1.0 / (1.0 + Q)
            print("Dapple: Ring Length: %d (%d unused)" % (ring_len, unused))
            print("Dapple: comm/comp (%.4f / %.4f) ratio Q: %.4f" % (comm, comp, Q))
            print("Dapple: DP efficiency: %.4f" % dp_eff)
            eff = dp_eff * unit_eff
            print("Dapple: Dapple efficiency: %.4f" % eff)
            print("---------------------------------------------------")
        return eff

    tests_bak = [[8, 1], [4, 1], [2, 1], [8, 2], [4, 2], [2, 2], [8, 4], [4, 2], [2, 4]]
    tests = [[8, 1]]
    for test in tests:
        ret_dp = dp(test[0], test[1], FLAGS.total_batch_size)
        ret_dpl = dapple(test[0], test[1], FLAGS.total_batch_size)
        if ret_dpl > ret_dp:
            print("###########################")
            print("dapple win!")
            print("###########################")
        else:
            print("###########################")
            print("data parallel win!")
            print("###########################")
