import json
import os
import sys

import pandas as pd
import seaborn as sns


def read_to_df(dir_path: str) -> pd.DataFrame:
    records = []

    for filepath in os.listdir(dir_path):
        filename, ext = os.path.splitext(filepath)
        if ext != ".json":
            continue

        params, radius = tuple(filename.split("_"))
        radius = int(radius)

        with open(os.path.join(dir_path, filepath), "r") as f:
            data = json.load(f)

        for n, elem in enumerate(data):
            records.append(
                {
                    "components": params,
                    "radius": radius,
                    "pixels": n - radius,
                    "y": elem,
                }
            )

    return pd.DataFrame.from_records(records).sort_values(["components", "radius"])


def plot(df: pd.DataFrame, out_filename: str) -> None:
    grid = sns.FacetGrid(df, col="components", row="radius", sharex=False, sharey=False)
    grid.map(sns.lineplot, "pixels", "y")
    grid.set(yticks=[])
    grid.set(yticklabels=[])
    grid.set(ylabel="")
    grid.add_legend()
    grid.figure.subplots_adjust(top=0.9)
    grid.figure.suptitle("Kernel Shapes", fontsize=16)
    grid.tight_layout()
    grid.savefig(out_filename)


def main() -> int:
    dir_path = "plots" if len(sys.argv) == 1 else sys.argv[1]
    out_filename = os.path.join(dir_path, "kernel_shapes.png")
    df = read_to_df(dir_path)
    plot(df, out_filename)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
