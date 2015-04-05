#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <set>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;
const int W=17, H = 1024;

ll dpa[17][1024]; // negative nearest
ll dpb[17][1024]; // positive nearest
ll zero[W][H];
ll a;
int d;

const int DEBUG = 0;

void update(ll &ma, ll &mb, int &zero, ll val) {
	if(val == 0) {
		zero = 1;
		return;
	}
	if(val > 0) {
		if (mb > val) mb = val;
		return;
	}
	if (ma < val) ma = val;
}

ll rec(int pos, int used /* bits */, int rem, ll mgn) {
	ll ma = -1e16;
	ll mb = 1e16;
	int z = 0;
	if(pos >= d) {
		update(ma, mb, z, (a/mgn) * mgn);
		dpa[pos][used] = ma;
		dpb[pos][used] = mb;
		zero[pos][used] = z;
		return mb;
	}
	if (dpb[pos][used] >= 0) {
		return min(-dpa[pos][used], dpb[pos][used]);
	}
	ll cur = (a / mgn) % 10;
	REP(q,0,10) {
		if(used & (1 << q)) {
			ll sub = rec(pos+1, used, rem, mgn*10);
			ll resa = (cur - q) * mgn + dpa[pos+1][used];
			update(ma,mb,z,resa);
			ll resb = (cur - q) * mgn + dpb[pos+1][used];
			update(ma,mb,z,resb);
			if(zero[pos+1][used]) {
				ll resz = (cur - q) * mgn;
				update(ma,mb,z,resz);
			}
		} else if (rem > 0) {
			int nu = used | 1 << q;
			ll sub = rec(pos+1, nu, rem - 1, mgn*10);
			ll resa = (cur - q) * mgn + dpa[pos+1][nu];
			update(ma,mb,z,resa);
			ll resb = (cur - q) * mgn + dpb[pos+1][nu];
			update(ma,mb,z,resb);
			if(zero[pos+1][nu]) {
				ll resz = (cur - q) * mgn;
				update(ma,mb,z,resz);
			}
		}
	}
	dpa[pos][used] = ma;
	dpb[pos][used] = mb;
	zero[pos][used] = z;
	if(DEBUG) {
		printf("dpa[%d][0x%x] = %lld\n", pos, used, ma);
		printf("dpb[%d][0x%x] = %lld\n", pos, used, mb);
		printf("zero[%d][0x%x] = %d\n", pos, used, z);
	}
	return z ? 0 : min(-ma,mb);
}

ll check(int x,int k) {
	REP(i,0,W) {
		REP(j,0,H) {
			dpb[i][j] = -1;
		}
	}
	d = x;
	return abs(rec(0,0,k, 1));
}

int main(void){
	int k;
	cin>>a>>k;
	int dd=0;
	{
		ll b = a;
		while(b >= 1) {
			b /= 10;
			dd++;
		}
	}
	ll m = 1.5e16;
	REP(x,1,dd + 1) {
		m = min(m, check(x,k));
	}
	cout << m << endl;
}
