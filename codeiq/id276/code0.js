resArr = [];
for(srcArr=[i=0];i<77*289;i++) {
	srcArr[i] =  ((i * i * 4999 + 8999 & 0xFFFF) / 0x10000) * 255;
}
for(n=0;n<16;n++){
	srcW = 11 * (n + 1);
	srcH = 7 * (n + 1);

	destArr = [];
	scaleX = 256 / (srcW - 1);
	scaleY = 160 / (srcH - 1);

	for(y=0;y<160;y++)for(x=0;x<256;x++){
		x0 = (x / scaleX)|0;
		y0 = 0|(y / scaleY);
		x1 = x / scaleX - x0;
		y1 = y / scaleY - y0;
			destArr[x + y * 256] = 0|(
			  (1 - x1) * (1 - y1) * srcArr[x0     + (y0    ) * srcW]
			+      x1  * (1 - y1) * srcArr[x0 + 1 + (y0    ) * srcW]
			+ (1 - x1) *      y1  * srcArr[x0     + (y0 + 1) * srcW]
			+      x1  *      y1  * srcArr[x0 + 1 + (y0 + 1) * srcW]
		);
	}
	for(i=0;i<40960;i++) {
		resArr[i] = n==0?destArr[i]:0|((n+6)/(n+7)*resArr[i] +1/(n+7)*destArr[i]);
	}
}

return "256,160,"+ resArr.join(",")