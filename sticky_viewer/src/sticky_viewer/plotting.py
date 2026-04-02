from astropy.table import Table
from matplotlib import pyplot as plt

def plot_scatter(t: Table, xcol="x", ycol="y", zcol="z", **kwargs):

    fig, ax = plt.subplots(1, 2, figsize=kwargs.get("figsize", (12, 6)))

    size = kwargs.get("size", 1)
    color = kwargs.get("color", "black")

    ax[0].scatter(t[xcol], t[ycol], s=size, c=color)
    ax[0].set_xlabel(xcol)
    ax[0].set_ylabel(ycol)

    ax[1].scatter(t[xcol], t[zcol], s=size, c=color)
    ax[1].set_xlabel(xcol)
    ax[1].set_ylabel(zcol)

    lims = kwargs.get("lims", [-30, 30])
    ax[0].set_xlim(lims)
    ax[0].set_ylim(lims)
    ax[1].set_xlim(lims)
    ax[1].set_ylim(lims)

    if kwargs.get("show_pcount", True):
        ax[1].text(0.98, 0.98, f"N={len(t)}", transform=ax[1].transAxes, ha="right", va="top",
                   fontsize=kwargs.get("fontsize", 14))

    plt.tight_layout()
    plt.show()

