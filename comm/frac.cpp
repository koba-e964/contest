typedef long long int ll;
ll gcd(ll x,ll y){
	while(y!=0){
		ll r=x%y;
		x=y;y=r;
	}return x;
}

struct frac{ll x,y;frac(ll x,ll y):x(x),y(y){
	reduce();
}
frac operator+(const frac& f)const{
	ll g=gcd(y,f.y);
	return frac(x*(f.y/g)+f.x*(y/g),y/g*f.y);
}
frac operator*(const frac& f)const{
	return frac(x*f.x,y*f.y);
}
void reduce(){
	ll g=gcd(x,y);
	x/=g;
	y/=g;
	if(y<0){
		x=-x;y=-y;
	}
}
};

