import os
import fnmatch
from pathlib import Path
import pyperclip  # type: ignore


def append_file(file_path, file_obj):
    file_obj.write(f"//-- File: {file_path} -----------------\n")
    with open(file_path, "r", encoding="utf-8", errors="replace") as infile:
        content = infile.read()
        file_obj.write(content)
        file_obj.write("\n\n")


def should_exclude(path, exclude_patterns):
    # Check if any exclusion pattern matches the path
    return any(fnmatch.fnmatch(path, pattern) for pattern in exclude_patterns)


def append_dir(source_dir, file_obj, exclude_patterns):
    for root, _, files in os.walk(source_dir):
        if should_exclude(root, exclude_patterns):
            continue  # Skip this folder and its contents

        for file_name in files:
            file_path = os.path.join(root, file_name)
            if should_exclude(file_path, exclude_patterns):
                continue  # Skip this file

            append_file(file_path, file_obj)


if __name__ == "__main__":
    output_file_path = "./txt_all.txt"  # Replace with your output file path
    prompt_file_path = "./txt_prompt.txt"  # Replace with your output file path

    # Patterns for folders and files to exclude
    exclude_patterns = [
        "**/target/*",
        "**/.gitignore",
        "**/.git/*",
        "**/Cargo.lock",
        "**/.env",
        "**/concat.py",
        "**/txt_all.txt",
        "**/txt_prompt.txt",
        "**/README.md",
    ]

    with open(output_file_path, "w") as file_obj:
        append_file("./docker-compose.yml", file_obj)
        append_file("./Cargo.toml", file_obj)

        append_dir("./examples/", file_obj, exclude_patterns)
        append_dir("./src/", file_obj, exclude_patterns)
        append_file("./txt_prompt.txt", file_obj)

    pyperclip.copy(Path(output_file_path).read_text())
