from PIL import Image


def reencode(src, dst):
    bits = []
    with Image.open(src) as im:
        for y in range(im.height):
            for x in range(im.width):
                pix = im.getpixel((x, y))
                r, g, b, a = pix
                value = (r + g + b + a) / (255 + 255 + 255 + 255)
                if value < 0.5:  # threshold
                    bit = 0
                else:
                    bit = 1
                bits.append(bit)

    bytes_list = []
    for i in range(0, len(bits), 8):
        byte = sum([2**(7 - x) for x in range(8) if bits[i + x]])
        bytes_list.append(byte)

    with open(dst, "wb") as bin:
        bin.write(bytes(bytes_list))


if __name__ == "__main__":
    reencode("srcfiles/font.png", "../src/rendering/font.bin")