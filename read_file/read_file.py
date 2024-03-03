def read_file(path):
    with open(path, "r") as file:
        file_content = file.read()
        lines = file_content.split("\n")
        return lines[:-1]
