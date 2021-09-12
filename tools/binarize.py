from PIL import Image


def reencode(src, dst, glyph_width_cells, glyph_height_cells):
    bits = []
    with Image.open(src) as im:
        for char_y in range(0, im.height, 8 * glyph_height_cells):  # so the bottom half of the char will end up right after the top
            for char_x in range(0, im.width, 8 * glyph_width_cells):
                for subglyph_y in range(glyph_height_cells):
                    for subglyph_x in range(glyph_width_cells):
                        cell_x = char_x + subglyph_x * 8
                        cell_y = char_y + subglyph_y * 8

                        for y in range(8):  # to match the other condition
                            for x in range(8):
                                pix = im.getpixel((cell_x + x, cell_y + y))
                                r, g, b, a = pix
                                value = (r + g + b) / (255 + 255 + 255)
                                if value < 0.5 or a < 128:  # threshold
                                    bit = 0
                                else:
                                    bit = 1
                                bits.append(bit)

    bytes_list = []
    for i in range(0, len(bits), 8):
        byte = sum([2**x for x in range(8) if bits[i + x]])
        bytes_list.append(byte)

    with open(dst, "wb") as bin:
        bin.write(bytes(bytes_list))


if __name__ == "__main__":
    reencode("srcfiles/font.png", "../src/rendering/font.bin", 1, 2)
    reencode("srcfiles/font_small.png", "../src/rendering/font_small.bin", 1, 1)
    reencode("srcfiles/font_fat.png", "../src/rendering/font_fat.bin", 2, 2)
