#!/usr/bin/python
import argparse
import platform
import shutil
from pathlib import Path


class Args(argparse.Namespace):
    name: str
    target: str


def parse_args() -> Args:
    parser = argparse.ArgumentParser()
    parser.add_argument("--name", required=True)
    parser.add_argument("--target", required=True)
    return parser.parse_args(namespace=Args())


def get_filename(name: str) -> str:
    if platform.system() == "Windows":
        return f"{name}.exe"
    return name


def find_executable(name: str, target: str) -> Path:
    filename: str = get_filename(name)
    for file in [
        Path("target") / "release" / filename,
        Path("target") / target / "release" / filename,
    ]:
        if file.exists():
            return file
    raise FileNotFoundError(filename)


def main() -> None:
    args: Args = parse_args()
    executable: Path = find_executable(name=args.name, target=args.target)
    destination: Path = Path("dist") / executable.name
    destination = destination.with_stem(f"{destination.stem}-{args.target}")
    destination.parent.mkdir(exist_ok=True, parents=True)
    shutil.copy2(executable, destination)


if __name__ == "__main__":
    main()
