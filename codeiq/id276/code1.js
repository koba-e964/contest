r = [];
function f(i){return((i * i * 4999 + 8999 & 0xFFFF) / 0x10000) * 255}
for(n=0;n<16;n++){
	srcW = 11 * (n + 1);

	destArr = [];
	scaleX = 256 / (srcW - 1);
	scaleY = 160 / (7*n+6);

	for(i=0;i<40960;i++){x=i%256;y=0|(i/256);
		x0 = (x / scaleX)|0;
		y0 = 0|(y / scaleY);
		x1 = x / scaleX - x0;
		y1 = y / scaleY - y0;
			d= 0|(
			  (1 - x1) * (1 - y1) * f(x0     + (y0    ) * srcW)
			+      x1  * (1 - y1) * f(x0 + 1 + (y0    ) * srcW)
			+ (1 - x1) *      y1  * f(x0     + (y0 + 1) * srcW)
			+      x1  *      y1  * f(x0 + 1 + (y0 + 1) * srcW)
		);
		r[i] = n==0?d:0|((n+6)/(n+7)*r[i] +1/(n+7)*d);
	}
}

return "256,160,"+ r.join(",")