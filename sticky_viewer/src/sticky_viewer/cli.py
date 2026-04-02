from astropy.table import Table

import argparse

from .plotting import plot_scatter


def main():
    parser = argparse.ArgumentParser(
        prog="StickyViewer",
        description="CLI Tool for Viewing Sticky NBody Snapshots",
    )

    parser.add_argument("snapshot")
    parser.add_argument("-t", "--type", type=int)
    
    args = parser.parse_args()

    t = Table.read(args.snapshot)

    plot_scatter(t)