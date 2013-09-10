function f(i){return((i*i*4999+8999&0xFFFF)/65536)*255}
for(r=[n=s=0];n<16;n++){
	s+=11;
	scaleX = 256 / (s - 1);
	scaleY = 160 / (7*n+6);
	for(i=0;i<5<<13;i++){x=i%256;y=i>>8;
		x0 = x/scaleX|0;
		y0 = 0|y/scaleY;
		x1 = x/scaleX-x0;
		y1 = y/scaleY-y0;
		t=x0+y0*s;
		d= 0|
		  (1 - x1) * (1 - y1) * f(t)
		+      x1  * (1 - y1) * f(t+1)
		+ (1 - x1) *      y1  * f(t+s)
		+      x1  *      y1  * f(t+1+s);
		r[i] = n==0?d:0|(n+6)/(n+7)*r[i] +1/(n+7)*d;
	}
}

return "256,160,"+ r.join(",")