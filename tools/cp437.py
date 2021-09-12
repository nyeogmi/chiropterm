# TODO: Special-case the first few characters, which in chiropterm
# all represent glyphs instead of control characters
with open("../src/cp437/data.bin", "wt", encoding="utf8") as f:
    for i in range(256):
        f.write(bytes([i]).decode("cp437"))

