import argparse
import json
import platform
import shutil
import subprocess
from collections.abc import Generator
from pathlib import Path
from typing import Any


class Args(argparse.Namespace):
    target: str


def parse_args() -> Args:
    parser = argparse.ArgumentParser()
    parser.add_argument("--target", required=True)
    return parser.parse_args(namespace=Args())


def get_executable_names() -> Generator[str]:
    process: subprocess.CompletedProcess[str] = subprocess.run(
        ["cargo", "metadata", "--no-deps", "--format-version", "1"],
        stdout=subprocess.PIPE,
        check=True,
        text=True,
    )
    metadata: Any = json.loads(process.stdout)
    for package in metadata["packages"]:
        for target in package["targets"]:
            if "bin" in target["kind"]:
                yield target["name"]


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
    for name in get_executable_names():
        executable: Path = find_executable(name=name, target=args.target)
        destination: Path = Path("dist") / executable.name
        destination = destination.with_stem(f"{destination.stem}-{args.target}")
        destination.parent.mkdir(exist_ok=True, parents=True)
        shutil.copy2(executable, destination)


if __name__ == "__main__":
    main()
