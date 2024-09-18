# Python
import os
import shutil
import subprocess

# Run `cargo clean`
subprocess.run(["cargo", "clean"])

# Define the directories to clean
directories = ["./coverage_prof", "./coverage"]


# Function to remove files except `.gitkeep`
def clean_directory(path):
    for root, dirs, files in os.walk(path):
        for file in files:
            if file != ".gitkeep":
                os.remove(os.path.join(root, file))
        for dir in dirs:
            shutil.rmtree(os.path.join(root, dir))


# Clean the specified directories
for directory in directories:
    clean_directory(directory)
