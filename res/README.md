# Icons

Icons are create from .png files. The .png files are exported from the .svg
source files. This keeps the transparency settings.

.svg files are created with Inkscape. 
.png files are exported from Inkscape.
.icon files arae created with ImageMagick.

```
magick .\sun.png -define icon:auto-resize=16,32,48,256 -compress zip sun.ico
```

```
magick .\moon.png -define icon:auto-resize=16,32,48,256 -compress zip moon.ico
```

