from PIL import Image, ImageDraw

im = Image.new('RGB', (1000, 1000), (128, 128, 128))
draw = ImageDraw.Draw(im)

k = 8
u = 20
for i in range(k - 1):
    draw.chord((0, 0, 1000, 1000), start=u * i, end=u * (i + k), outline=(0, 0, 0))

im.save('tmp-a.jpg', quality=95)
