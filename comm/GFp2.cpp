typedef long long lint;
typedef std::pair<lint, lint> pll;

lint p = 200087;
lint g = 5;
lint sq=17;

/*
 * Operations on GF(p^2)
 * Requirements: pll = pair<lint, lint>
 * lint p
 * lint sq (sqrt(sq) is not in GF(p)
 * Verified by: https://beta.atcoder.jp/contests/xmascon17/submissions/1907661
 */ 
pll mul(pll a,pll b){
  lint x=a.first*b.first+sq*(a.second*b.second%p);
  lint y=a.first*b.second+a.second*b.first;
  return pll(x%p,y%p);
}

pll add(pll a,pll b){
  return pll((a.first+b.first)%p,(a.second+b.second)%p);
}
pll scalar(pll a,lint b){
  b%=p;
  if(b<0)b+=p;
  pll r=pll(a.first*b%p,a.second*b%p);
  return r;
}

pll powl(pll a,lint e){
  pll prod(1,0);
  for (int i = 63; i >= 0; --i) {
    prod=mul(prod,prod);
    if ((e & 1LL << i) != 0) {
      prod=mul(prod,a);
    }
  }
  return prod;
}

lint powerMod(lint x, lint exponent, lint MOD) {
  lint prod = 1;
  for (int i = 63; i >= 0; --i) {
    prod = (prod * prod) % MOD;
    if ((exponent & 1L << i) != 0) {
      prod = (prod * x) % MOD;
    }
  }
  return prod;
}

pll gen(3,1);
