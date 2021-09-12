with open("../src/cp437/data.bin", "wt", encoding="utf8") as f:
    for i in range(256):
        f.write(bytes([i]).decode("cp437"))

