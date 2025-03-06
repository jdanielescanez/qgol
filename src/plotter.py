
from numpy import random
import matplotlib.pyplot as plt
import matplotlib.animation as animation

class Plotter:
    def get_image(self, matrix, turn, ax):
        CMAP = 'YlGnBu' # 'YlGn' # 'cividis' # 'bone'
        return ax.imshow(matrix, vmin=0, vmax=1, cmap=CMAP, aspect='equal', animated=(turn > 0), interpolation='nearest')

    def generate_gif(self, matrices, file_out):
        n = len(matrices)
        m = len(matrices[0])

        fig = plt.figure(frameon=False)
        fig.set_size_inches(n, m)

        ax = plt.Axes(fig, [0., 0., 1., 1.])
        ax.set_axis_off()
        fig.add_axes(ax)

        ims = []
        for turn, matrix in enumerate(matrices):
            ims.append([self.get_image(matrix, turn, ax)])

        ani = animation.ArtistAnimation(fig, ims, interval=500)
        ani.save(file_out)