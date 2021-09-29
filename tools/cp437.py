# TODO: Special-case the first few characters, which in chiropterm
# all represent glyphs instead of control characters
with open("../src/cp437/data.bin", "wb") as f:
    already_found = set()
    for i in range(256):
        dec = bytes([i]).decode("cp437")
        print(i, repr(dec))
        assert len(dec) == 1
        assert dec not in already_found
        already_found.add(dec)
        f.write(dec.encode("utf8"))

