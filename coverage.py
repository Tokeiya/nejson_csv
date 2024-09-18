import os
import subprocess
import shutil
import glob
import webbrowser
import argparse
from os.path import abspath
import platform



def clean_directory(path: str):
    env:str=platform.system()

    if env=='Windows':
        p: str = path + '\\'
    else:
        p: str=path+'/'

    for item in glob.iglob(f"{path}/**", recursive=True):
        if os.path.isfile(item) and not item.endswith(".gitkeep"):
            print(f"file:{item}")
            os.remove(item)
        elif os.path.isdir(item) and not item == p:
            print(f"dir:{item}")
            shutil.rmtree(item)


def main(package: str, module: str):
    subprocess.run(["cargo", "clean"])

    clean_directory("./coverage_prof")
    clean_directory("./coverage")
    print("Start coverage")

    toolchain: str = subprocess.run(["rustup", "default"], capture_output=True, text=True).stdout.strip()

    if toolchain == "nightly-x86_64-pc-windows-msvc (default)":
        os.environ["RUSTFLAGS"] = "-Cinstrument-coverage"
    elif toolchain == "my-nightly (default)" or toolchain=="nightly-aarch64-unknown-linux-gnu (default)":
        os.environ[
            "RUSTFLAGS"] = "-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
    else:
        raise Exception("Err:Unexpected toolchain.")

    if package:
        print(f"Package: {package}")

    if module:
        print(f"Module: {module}")

    os.environ["CARGO_INCREMENTAL"] = "0"
    os.environ["LLVM_PROFILE_FILE"] = "../coverage_prof/traq-bot-http-rs-%p-%m.profraw"

    if package and module:
        cmd = f"cargo test {module} -p {package}"
        print(f"Command: {cmd}")
        subprocess.run(cmd, shell=True)
    elif package:
        subprocess.run(["cargo", "test", "-p", package])
    else:
        subprocess.run(["cargo", "test"])

    subprocess.run("grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./coverage/",
                   shell=True)

    os.environ.pop("CARGO_INCREMENTAL", None)
    os.environ.pop("RUSTFLAGS", None)
    os.environ.pop("LLVM_PROFILE_FILE", None)

    absolute_path = abspath('./coverage/html/index.html')
    webbrowser.open(absolute_path)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Python version of the PowerShell script")
    parser.add_argument('-p', type=str, help='Package parameter')
    parser.add_argument('-m', type=str, help='Module parameter')
    args = parser.parse_args()

    main(args.p, args.m)
